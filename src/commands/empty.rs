use serde::Serialize;

use super::{walk_entries, WalkConfig};
use crate::EmptyArgs;

#[derive(Serialize)]
struct EmptyEntry {
    path: String,
    #[serde(rename = "type")]
    kind: &'static str,
}

pub fn run(args: &EmptyArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let show_files = args.files || !args.dirs;
    let show_dirs = args.dirs || !args.files;
    let json = super::json_enabled();

    let config = WalkConfig {
        root,
        show_all: false,
        ..Default::default()
    };

    let mut out: Vec<EmptyEntry> = Vec::new();

    for entry in walk_entries(&config) {
        let ft = match entry.file_type() {
            Some(ft) => ft,
            None => continue,
        };

        if ft.is_file() && show_files {
            let meta = match entry.path().metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            if meta.len() == 0 {
                let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
                if json {
                    out.push(EmptyEntry {
                        path: rel.display().to_string(),
                        kind: "file",
                    });
                } else {
                    println!("E  {}", rel.display());
                }
            }
        }

        if ft.is_dir() && show_dirs {
            let mut read = match entry.path().read_dir() {
                Ok(r) => r,
                Err(_) => continue,
            };
            if read.next().is_none() {
                let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
                if json {
                    out.push(EmptyEntry {
                        path: rel.display().to_string(),
                        kind: "dir",
                    });
                } else {
                    println!("D  {}", rel.display());
                }
            }
        }
    }

    if json {
        return super::emit_json(&out);
    }

    Ok(())
}
