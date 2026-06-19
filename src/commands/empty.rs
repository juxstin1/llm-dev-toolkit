use super::{walk_entries, WalkConfig};
use crate::EmptyArgs;

pub fn run(args: &EmptyArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let show_files = args.files || !args.dirs;
    let show_dirs = args.dirs || !args.files;

    let config = WalkConfig {
        root,
        show_all: false,
        ..Default::default()
    };

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
                println!("E  {}", rel.display());
            }
        }

        if ft.is_dir() && show_dirs {
            let mut read = match entry.path().read_dir() {
                Ok(r) => r,
                Err(_) => continue,
            };
            if read.next().is_none() {
                let rel = entry.path().strip_prefix(root).unwrap_or(entry.path());
                println!("D  {}", rel.display());
            }
        }
    }

    Ok(())
}
