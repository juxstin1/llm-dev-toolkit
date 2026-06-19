use std::fs;
use std::path::Path;
use std::time::SystemTime;

use serde::Serialize;

use super::{ansi, color_enabled};
use crate::LsArgs;

#[derive(PartialEq)]
enum EntryKind {
    Dir,
    Symlink,
    File,
}

impl EntryKind {
    fn as_str(&self) -> &'static str {
        match self {
            EntryKind::Dir => "dir",
            EntryKind::Symlink => "symlink",
            EntryKind::File => "file",
        }
    }
}

#[derive(Serialize)]
struct LsEntry {
    name: String,
    #[serde(rename = "type")]
    kind: &'static str,
    size: u64,
    modified: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nlink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_target: Option<String>,
}

pub fn run(args: &LsArgs) -> Result<(), String> {
    let path = args.path.as_deref().unwrap_or(".");
    list_entries(Path::new(path), args.all, args.long)
}

pub fn run_all(args: &LsArgs) -> Result<(), String> {
    let path = args.path.as_deref().unwrap_or(".");
    list_entries(Path::new(path), true, false)
}

pub fn run_long(args: &LsArgs) -> Result<(), String> {
    let path = args.path.as_deref().unwrap_or(".");
    list_entries(Path::new(path), true, true)
}

fn classify(entry: &fs::DirEntry) -> EntryKind {
    let ft = entry.file_type();
    if let Ok(t) = ft {
        if t.is_symlink() {
            return EntryKind::Symlink;
        }
        if t.is_dir() {
            return EntryKind::Dir;
        }
    }
    if let Ok(meta) = entry.metadata() {
        if meta.is_dir() {
            return EntryKind::Dir;
        }
    }
    EntryKind::File
}

fn color(kind: &EntryKind) -> &'static str {
    match kind {
        EntryKind::Dir => ansi::BLUE,
        EntryKind::Symlink => ansi::CYAN,
        EntryKind::File => ansi::WHITE,
    }
}

fn color_start(kind: &EntryKind) -> &'static str {
    if color_enabled() {
        color(kind)
    } else {
        ""
    }
}

fn color_end() -> &'static str {
    if color_enabled() {
        ansi::RESET
    } else {
        ""
    }
}

fn indicator(kind: &EntryKind) -> &'static str {
    match kind {
        EntryKind::Dir => "/",
        EntryKind::Symlink => "@",
        EntryKind::File => "",
    }
}

fn is_hidden(entry: &fs::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(unix)]
fn format_permissions(mode: u32, kind: &EntryKind) -> String {
    let ft = match kind {
        EntryKind::Dir => 'd',
        EntryKind::Symlink => 'l',
        _ => '-',
    };
    let mut s = String::with_capacity(10);
    s.push(ft);
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    s
}

#[cfg(not(unix))]
fn format_permissions(_mode: u32, kind: &EntryKind) -> String {
    let ft = match kind {
        EntryKind::Dir => 'd',
        EntryKind::Symlink => 'l',
        _ => '-',
    };
    format!("{}---------", ft)
}

#[cfg(unix)]
fn get_nlink(meta: &fs::Metadata) -> String {
    use std::os::unix::fs::MetadataExt;
    meta.nlink().to_string()
}

#[cfg(not(unix))]
fn get_nlink(_meta: &fs::Metadata) -> String {
    "-".to_string()
}

#[cfg(unix)]
fn total_blocks_count(entries: &[fs::DirEntry]) -> u64 {
    use std::os::unix::fs::MetadataExt;
    entries
        .iter()
        .filter_map(|e| fs::symlink_metadata(e.path()).ok())
        .map(|m| m.blocks())
        .sum()
}

fn format_date(meta: &fs::Metadata) -> String {
    match meta.modified() {
        Ok(modified) => match modified.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => super::format_time(d.as_secs() as i64),
            Err(_) => "------------".to_string(),
        },
        Err(_) => "------------".to_string(),
    }
}

fn read_entries(path: &Path, show_all: bool) -> Result<Vec<fs::DirEntry>, String> {
    let dir = fs::read_dir(path)
        .map_err(|e| format!("Cannot read directory '{}': {}", path.display(), e))?;

    let mut entries = Vec::new();
    for entry in dir {
        match entry {
            Ok(e) => {
                if !show_all && is_hidden(&e) {
                    continue;
                }
                entries.push(e);
            }
            Err(e) => {
                eprintln!("Warning: cannot read entry: {}", e);
            }
        }
    }
    Ok(entries)
}

fn sort_entries(entries: &mut [fs::DirEntry]) {
    entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();

        if a_is_dir != b_is_dir {
            return b_is_dir.cmp(&a_is_dir);
        }

        let a_name = a.file_name().to_string_lossy().to_lowercase();
        let b_name = b.file_name().to_string_lossy().to_lowercase();
        a_name.cmp(&b_name)
    });
}

#[cfg(unix)]
fn perms_string(meta: &fs::Metadata, kind: &EntryKind) -> String {
    use std::os::unix::fs::PermissionsExt;
    format_permissions(meta.permissions().mode(), kind)
}

#[cfg(not(unix))]
fn perms_string(_meta: &fs::Metadata, kind: &EntryKind) -> String {
    format_permissions(0, kind)
}

fn list_entries_json(entries: &[fs::DirEntry], long: bool) -> Result<(), String> {
    let mut out = Vec::with_capacity(entries.len());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let kind = classify(entry);
        let meta = fs::symlink_metadata(entry.path()).ok();
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = meta
            .as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64);

        let (permissions, nlink, link_target) = if long {
            let permissions = meta.as_ref().map(|m| perms_string(m, &kind));
            let nlink = meta.as_ref().map(get_nlink);
            let link_target = if kind == EntryKind::Symlink {
                fs::read_link(entry.path())
                    .ok()
                    .map(|t| t.display().to_string())
            } else {
                None
            };
            (permissions, nlink, link_target)
        } else {
            (None, None, None)
        };

        out.push(LsEntry {
            name,
            kind: kind.as_str(),
            size,
            modified,
            permissions,
            nlink,
            link_target,
        });
    }
    super::emit_json(&out)
}

fn list_entries(path: &Path, show_all: bool, long: bool) -> Result<(), String> {
    let mut entries = read_entries(path, show_all)?;
    sort_entries(&mut entries);

    if super::json_enabled() {
        return list_entries_json(&entries, long);
    }

    if long {
        let mut max_links_len = 1usize;
        let mut max_size_len = 0usize;
        for entry in &entries {
            if let Ok(meta) = fs::symlink_metadata(entry.path()) {
                let nlink = get_nlink(&meta);
                max_links_len = max_links_len.max(nlink.len());
                let size = super::format_size(meta.len());
                max_size_len = max_size_len.max(size.len());
            }
        }

        #[cfg(unix)]
        {
            let total = total_blocks_count(&entries);
            println!("total {}", total);
        }

        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_string();
            let kind = classify(entry);

            let meta = match fs::symlink_metadata(entry.path()) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Warning: cannot read metadata for '{}': {}", name, e);
                    continue;
                }
            };

            #[cfg(unix)]
            let perms = {
                use std::os::unix::fs::PermissionsExt;
                format_permissions(meta.permissions().mode(), &kind)
            };
            #[cfg(not(unix))]
            let perms = format_permissions(0, &kind);

            let nlink = get_nlink(&meta);
            let size = super::format_size(meta.len());
            let date = format_date(&meta);

            let link_target = if kind == EntryKind::Symlink {
                match fs::read_link(entry.path()) {
                    Ok(target) => format!(" -> {}", target.display()),
                    Err(_) => String::new(),
                }
            } else {
                String::new()
            };

            println!(
                "{} {:>links$} {:>size$} {} {}{}{}{}{}",
                perms,
                nlink,
                size,
                date,
                color_start(&kind),
                name,
                indicator(&kind),
                color_end(),
                link_target,
                links = max_links_len,
                size = max_size_len,
            );
        }
    } else {
        for entry in &entries {
            let name = entry.file_name().to_string_lossy().to_string();
            let kind = classify(entry);
            println!(
                "{}{}{}{}",
                color_start(&kind),
                name,
                color_end(),
                indicator(&kind)
            );
        }
    }

    Ok(())
}
