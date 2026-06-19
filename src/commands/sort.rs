use std::fs;
use std::path::Path;

use crate::commands::{ansi, paint};

#[derive(Clone)]
struct Entry {
    name: String,
    is_dir: bool,
    size: u64,
    modified: i64,
    extension: String,
}

pub fn run(args: &crate::SortArgs) -> Result<(), String> {
    let dir = args.path.as_deref().unwrap_or(".");
    let entries = collect_entries(dir)?;

    let mut sorted = sort_entries(entries, &args.by, args.reverse, args.dirs_first);

    if args.count > 0 && args.count < sorted.len() {
        sorted.truncate(args.count);
    }

    for entry in &sorted {
        let size_str = crate::commands::format_size(entry.size);
        let time_str = crate::commands::format_time(entry.modified);
        let name = if entry.is_dir {
            paint(ansi::BLUE, &entry.name)
        } else {
            entry.name.clone()
        };
        println!("{:>8} {} {}", size_str, time_str, name);
    }

    Ok(())
}

fn collect_entries(dir: &str) -> Result<Vec<Entry>, String> {
    let rd = fs::read_dir(dir).map_err(|e| format!("Cannot read directory '{}': {}", dir, e))?;
    let mut entries = Vec::new();

    for entry in rd {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.metadata().map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        let ext = Path::new(&name)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        entries.push(Entry {
            is_dir: meta.is_dir(),
            size: meta.len(),
            modified: meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            extension: ext,
            name,
        });
    }

    Ok(entries)
}

fn sort_entries(mut entries: Vec<Entry>, by: &str, reverse: bool, dirs_first: bool) -> Vec<Entry> {
    match by {
        "size" => entries.sort_by_key(|e| e.size),
        "date" => entries.sort_by_key(|e| e.modified),
        "ext" => entries.sort_by(|a, b| a.extension.cmp(&b.extension)),
        _ => entries.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    if reverse {
        entries.reverse();
    }

    if dirs_first {
        let dirs: Vec<Entry> = entries.iter().filter(|e| e.is_dir).cloned().collect();
        let files: Vec<Entry> = entries.iter().filter(|e| !e.is_dir).cloned().collect();
        let mut result = dirs;
        result.extend(files);
        result
    } else {
        entries
    }
}
