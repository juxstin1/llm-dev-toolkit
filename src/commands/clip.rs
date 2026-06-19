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
        let content = if let Some(ref val) = args.value {
            val.clone()
        } else {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| e.to_string())?;
            buf
        };

        if let Ok(mut cb) = Clipboard::new() {
            cb.set_text(content.clone()).map_err(|e| e.to_string())?;
        } else {
            write_clipboard(&content)?;
        }
        println!("Copied to clipboard ({} bytes)", content.len());
    } else if args.out {
        if let Ok(mut cb) = Clipboard::new() {
            let text = cb.get_text().map_err(|e| e.to_string())?;
            print!("{}", text);
        } else {
            let content = read_clipboard()?;
            print!("{}", content);
        }
    } else if let Ok(mut cb) = Clipboard::new() {
        let text = cb.get_text().map_err(|e| e.to_string())?;
        print!("{}", text);
    } else {
        let content = read_clipboard()?;
        print!("{}", content);
    }
    Ok(())
}
