use std::collections::BTreeMap;

use super::format_size;
use super::{walk_entries, WalkConfig};
use crate::StatsArgs;

pub fn run(args: &StatsArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let mut total_files = 0u64;
    let mut total_dirs = 0u64;
    let mut total_bytes = 0u64;
    let mut by_ext: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut by_dir: BTreeMap<String, (u64, u64, u64)> = BTreeMap::new();

    let config = WalkConfig {
        root,
        show_all: true,
        max_depth: args.max_depth,
    };

    for entry in walk_entries(&config) {
        let path = entry.path();
        let Some(ft) = entry.file_type() else {
            continue;
        };

        if ft.is_dir() {
            total_dirs += 1;
            if args.directory {
                let parent = path
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                by_dir
                    .entry(parent)
                    .and_modify(|(f, d, _)| {
                        *f += 1;
                        *d += 1;
                    })
                    .or_insert((0, 1, 0));
            }
        } else if ft.is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            total_files += 1;
            total_bytes += size;

            if args.by_type {
                let ext = path
                    .extension()
                    .map(|e| format!(".{}", e.to_string_lossy()))
                    .unwrap_or_else(|| "(no ext)".to_string());
                by_ext
                    .entry(ext)
                    .and_modify(|(c, s)| {
                        *c += 1;
                        *s += size;
                    })
                    .or_insert((1, size));
            }

            if args.directory {
                let parent = path
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                by_dir
                    .entry(parent)
                    .and_modify(|(f, _d, s)| {
                        *f += 1;
                        *s += size;
                    })
                    .or_insert((1, 0, size));
            }
        }
    }

    println!("  Files:    {:>10}", total_files);
    println!("  Dirs:     {:>10}", total_dirs);
    println!("  Total:    {:>10}", format_size(total_bytes));
    println!();

    if args.by_type {
        for (ext, (count, size)) in &by_ext {
            println!(
                "  {:<10} {:>6} files  {:>10}",
                ext,
                count,
                format_size(*size)
            );
        }
    }

    if args.directory {
        for (dir, (files, dirs, size)) in &by_dir {
            println!(
                "  {}  {} files, {} dirs, {}",
                dir,
                files,
                dirs,
                format_size(*size)
            );
        }
    }

    Ok(())
}
