use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use arboard::Clipboard;

fn clipboard_path() -> Result<PathBuf, String> {
    let home = if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").map_err(|_| "USERPROFILE not set".to_string())
    } else {
        std::env::var("HOME").map_err(|_| "HOME not set".to_string())
    }?;
    Ok(PathBuf::from(home).join(".tk_clipboard"))
}

fn read_clipboard() -> Result<String, String> {
    let path = clipboard_path()?;
    if !path.exists() {
        return Err("Clipboard is empty".to_string());
    }
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

fn file_fallback_disabled_error() -> String {
    "System clipboard unavailable and file fallback is disabled; pass --allow-file-fallback to persist clipboard content to the local fallback file".to_string()
}

fn write_clipboard_file(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.create(true).write(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata().map_err(|e| e.to_string())?.permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn write_clipboard(content: &str, allow_file_fallback: bool) -> Result<PathBuf, String> {
    if !allow_file_fallback {
        return Err(file_fallback_disabled_error());
    }
    let path = clipboard_path()?;
    write_clipboard_file(&path, content)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_path_ends_with_tk_clipboard() {
        let path = clipboard_path().unwrap();
        assert!(path.to_string_lossy().ends_with(".tk_clipboard"));
    }

    #[test]
    fn test_write_clipboard_requires_explicit_file_fallback() {
        let err = write_clipboard("secret", false).unwrap_err();
        assert!(err.contains("--allow-file-fallback"));
    }

    #[test]
    fn test_write_clipboard_file_round_trips() {
        let dir = std::env::temp_dir().join("tk-test-clipboard-file");
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join(".tk_clipboard");
        write_clipboard_file(&path, "secret").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "secret");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn test_write_clipboard_file_sets_private_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = std::env::temp_dir().join("tk-test-clipboard-perms");
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join(".tk_clipboard");
        write_clipboard_file(&path, "secret").unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

pub fn run(args: &crate::ClipArgs) -> Result<(), String> {
    if args.r#in {
        let content = match &args.value {
            Some(val) => val.clone(),
            None => {
                let mut buf = String::new();
                std::io::stdin()
                    .read_to_string(&mut buf)
                    .map_err(|e| e.to_string())?;
                buf
            }
        };

        let json = crate::commands::json_enabled();
        // Prefer the real system clipboard; fall back to a file-backed store
        // (e.g. headless Linux, no X11/Wayland) and say so honestly.
        match Clipboard::new().and_then(|mut cb| cb.set_text(content.clone())) {
            Ok(()) => {
                if json {
                    return crate::commands::emit_json(&serde_json::json!({
                        "copied_bytes": content.len(),
                        "target": "system",
                    }));
                }
                println!("Copied {} bytes to the system clipboard", content.len());
            }
            Err(_) => {
                let path = write_clipboard(&content, args.allow_file_fallback)?;
                if json {
                    return crate::commands::emit_json(&serde_json::json!({
                        "copied_bytes": content.len(),
                        "target": "file",
                        "path": path.display().to_string(),
                    }));
                }
                eprintln!(
                    "System clipboard unavailable; stored {} bytes in {}",
                    content.len(),
                    path.display()
                );
            }
        }
    } else {
        // `-o` and the default both read; the system clipboard wins, else the file store.
        let text = match Clipboard::new().and_then(|mut cb| cb.get_text()) {
            Ok(text) => text,
            Err(_) => read_clipboard()?,
        };
        if crate::commands::json_enabled() {
            return crate::commands::emit_json(&serde_json::json!({ "content": text }));
        }
        print!("{}", text);
    }
    Ok(())
}
