use std::path::Path;
use std::time::SystemTime;

use super::{ansi, paint, rel_to, walk_entries, WalkConfig};
use crate::RecentArgs;

pub fn run(args: &RecentArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let cutoff = SystemTime::now()
        .checked_sub(std::time::Duration::from_secs(args.days * 86400))
        .ok_or("Failed to compute cutoff time")?;

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

        if let Some(ref ext) = args.ext {
            let ext = ext.trim_start_matches('.');
            match entry.path().extension() {
                Some(e) if e == ext => {}
                _ => continue,
            }
        }

        let meta = match entry.path().metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified = match meta.modified() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if modified >= cutoff {
            entries.push((modified, entry.path().to_path_buf()));
        }
    }

    entries.sort_by(|a, b| b.0.cmp(&a.0));
    entries.truncate(args.count);

    for (modified, path) in &entries {
        let date = match modified.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => super::format_time(d.as_secs() as i64),
            Err(_) => "------------".to_string(),
        };
        let size = match path.metadata() {
            Ok(m) => super::format_size(m.len()),
            Err(_) => "?".to_string(),
        };

        let rel = rel_to(path, Path::new(root));
        let name = if path.is_dir() {
            paint(ansi::BLUE, &format!("{}/", rel.display()))
        } else {
            rel.display().to_string()
        };
        println!("{}  {:>8}  {}", date, size, name);
    }

    Ok(())
}
