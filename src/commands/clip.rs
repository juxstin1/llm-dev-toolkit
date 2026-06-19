use std::io::Read;
use std::path::PathBuf;

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

fn write_clipboard(content: &str) -> Result<(), String> {
    let path = clipboard_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_path_ends_with_tk_clipboard() {
        let path = clipboard_path().unwrap();
        assert!(path.to_string_lossy().ends_with(".tk_clipboard"));
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
                write_clipboard(&content)?;
                let path = clipboard_path()?;
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
