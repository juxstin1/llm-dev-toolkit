use std::collections::HashMap;
use std::path::Path;

use serde::Serialize;

use super::{ansi, paint, rel_to, walk_entries, WalkConfig};
use crate::LargestArgs;

#[derive(Serialize)]
struct SizedPath {
    path: String,
    size: u64,
}

pub fn run(args: &LargestArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");

    if args.directories {
        largest_dirs(root, args.count)
    } else {
        largest_files(root, args.count)
    }
}

fn largest_files(root: &str, count: usize) -> Result<(), String> {
    let mut entries = Vec::new();

    let config = WalkConfig {
        root,
        show_all: true,
        ..Default::default()
    };

    for entry in walk_entries(&config) {
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        let meta = match entry.path().metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        entries.push((meta.len(), entry.path().to_path_buf()));
    }

    entries.sort_by_key(|e| std::cmp::Reverse(e.0));
    entries.truncate(count);

    let root_path = Path::new(root);
    if super::json_enabled() {
        let out: Vec<SizedPath> = entries
            .iter()
            .map(|(size, path)| SizedPath {
                path: rel_to(path, root_path).display().to_string(),
                size: *size,
            })
            .collect();
        return super::emit_json(&out);
    }

    for (i, (size, path)) in entries.iter().enumerate() {
        let rel = rel_to(path, root_path);
        println!(
            "{:>4}.  {:>9}  {}",
            i + 1,
            super::format_size(*size),
            rel.display()
        );
    }

    Ok(())
}

fn largest_dirs(root: &str, count: usize) -> Result<(), String> {
    let mut dir_sizes: HashMap<std::path::PathBuf, u64> = HashMap::new();

    let config = WalkConfig {
        root,
        show_all: true,
        ..Default::default()
    };

    for entry in walk_entries(&config) {
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }

        let meta = match entry.path().metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        if let Some(parent) = entry.path().parent() {
            *dir_sizes.entry(parent.to_path_buf()).or_insert(0) += meta.len();
        }
    }

    let mut sorted: Vec<_> = dir_sizes.into_iter().collect();
    sorted.sort_by_key(|e| std::cmp::Reverse(e.1));
    sorted.truncate(count);

    let root_path = Path::new(root);
    if super::json_enabled() {
        let out: Vec<SizedPath> = sorted
            .iter()
            .map(|(path, size)| {
                let rel = rel_to(path, root_path);
                let display = if rel.as_os_str().is_empty() {
                    ".".to_string()
                } else {
                    rel.display().to_string()
                };
                SizedPath {
                    path: display,
                    size: *size,
                }
            })
            .collect();
        return super::emit_json(&out);
    }

    for (i, (path, size)) in sorted.iter().enumerate() {
        let rel = rel_to(path, root_path);
        let display = if rel.as_os_str().is_empty() {
            "."
        } else {
            rel.to_str().unwrap_or("?")
        };
        let dir = paint(ansi::BLUE, &format!("{}/", display));
        println!("{:>4}.  {:>9}  {}", i + 1, super::format_size(*size), dir);
    }

    Ok(())
}
