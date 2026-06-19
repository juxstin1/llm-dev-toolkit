use std::collections::BTreeMap;
use std::io::Read;

use rayon::prelude::*;
use serde::Serialize;
use sha2::Sha256;

use super::format_size;
use super::{hash_file, walk_entries, with_rayon, WalkConfig};
use crate::DupsArgs;

#[derive(Serialize)]
struct DupFile {
    path: String,
    size: u64,
}

#[derive(Serialize)]
struct DupGroup {
    hash: String,
    files: Vec<DupFile>,
}

fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_lowercase();
    let (num_str, multiplier) = if s.ends_with("kib") {
        (&s[..s.len() - 3], 1024u64)
    } else if s.ends_with("mib") {
        (&s[..s.len() - 3], 1024u64 * 1024)
    } else if s.ends_with("gib") {
        (&s[..s.len() - 3], 1024u64 * 1024 * 1024)
    } else if s.ends_with("kb") {
        (&s[..s.len() - 2], 1000u64)
    } else if s.ends_with("mb") {
        (&s[..s.len() - 2], 1000u64 * 1000)
    } else if s.ends_with("gb") {
        (&s[..s.len() - 2], 1000u64 * 1000 * 1000)
    } else if s.ends_with('b') {
        (&s[..s.len() - 1], 1u64)
    } else {
        (s.as_str(), 1u64)
    };
    let num: u64 = num_str.parse().ok()?;
    Some(num * multiplier)
}

/// Fill `buf` from `r`, looping over short reads until full or EOF.
/// Guarantees the two readers in [`files_identical`] stay byte-aligned.
fn read_full<R: Read>(r: &mut R, buf: &mut [u8]) -> std::io::Result<usize> {
    let mut total = 0;
    while total < buf.len() {
        match r.read(&mut buf[total..])? {
            0 => break,
            n => total += n,
        }
    }
    Ok(total)
}

/// True only if `a` and `b` have identical contents *right now*. Re-reading at
/// delete time guards against a hash collision or a file changing after the scan.
fn files_identical(a: &str, b: &str) -> Result<bool, String> {
    let ma = std::fs::metadata(a).map_err(|e| format!("{}: {}", a, e))?;
    let mb = std::fs::metadata(b).map_err(|e| format!("{}: {}", b, e))?;
    if ma.len() != mb.len() {
        return Ok(false);
    }

    let mut ra =
        std::io::BufReader::new(std::fs::File::open(a).map_err(|e| format!("{}: {}", a, e))?);
    let mut rb =
        std::io::BufReader::new(std::fs::File::open(b).map_err(|e| format!("{}: {}", b, e))?);
    let mut ba = [0u8; 65536];
    let mut bb = [0u8; 65536];
    loop {
        let na = read_full(&mut ra, &mut ba).map_err(|e| format!("{}: {}", a, e))?;
        let nb = read_full(&mut rb, &mut bb).map_err(|e| format!("{}: {}", b, e))?;
        if na != nb || ba[..na] != bb[..nb] {
            return Ok(false);
        }
        if na == 0 {
            return Ok(true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_one() {
        assert_eq!(parse_size("1"), Some(1));
    }

    #[test]
    fn test_parse_size_1kib() {
        assert_eq!(parse_size("1kib"), Some(1024));
    }

    #[test]
    fn test_parse_size_1mib() {
        assert_eq!(parse_size("1mib"), Some(1024 * 1024));
    }

    #[test]
    fn test_parse_size_1kb() {
        assert_eq!(parse_size("1kb"), Some(1000));
    }

    #[test]
    fn test_parse_size_1mb() {
        assert_eq!(parse_size("1mb"), Some(1000 * 1000));
    }

    #[test]
    fn test_parse_size_1gb() {
        assert_eq!(parse_size("1gb"), Some(1000 * 1000 * 1000));
    }

    #[test]
    fn test_parse_size_empty() {
        assert_eq!(parse_size(""), None);
    }

    #[test]
    fn test_files_identical_same_content() {
        let dir = std::env::temp_dir();
        let a = dir.join("tk-ident-a.txt");
        let b = dir.join("tk-ident-b.txt");
        std::fs::write(&a, b"the same bytes\n").unwrap();
        std::fs::write(&b, b"the same bytes\n").unwrap();
        assert_eq!(
            files_identical(&a.to_string_lossy(), &b.to_string_lossy()),
            Ok(true)
        );
        std::fs::remove_file(&a).unwrap();
        std::fs::remove_file(&b).unwrap();
    }

    #[test]
    fn test_files_identical_same_size_different_bytes() {
        let dir = std::env::temp_dir();
        let a = dir.join("tk-ident-c.txt");
        let b = dir.join("tk-ident-d.txt");
        std::fs::write(&a, b"aaaaa").unwrap();
        std::fs::write(&b, b"aaaab").unwrap();
        assert_eq!(
            files_identical(&a.to_string_lossy(), &b.to_string_lossy()),
            Ok(false)
        );
        std::fs::remove_file(&a).unwrap();
        std::fs::remove_file(&b).unwrap();
    }

    #[test]
    fn test_files_identical_different_size() {
        let dir = std::env::temp_dir();
        let a = dir.join("tk-ident-e.txt");
        let b = dir.join("tk-ident-f.txt");
        std::fs::write(&a, b"short").unwrap();
        std::fs::write(&b, b"much longer content").unwrap();
        assert_eq!(
            files_identical(&a.to_string_lossy(), &b.to_string_lossy()),
            Ok(false)
        );
        std::fs::remove_file(&a).unwrap();
        std::fs::remove_file(&b).unwrap();
    }
}

pub fn run(args: &DupsArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let min_bytes = args.min_size.as_deref().and_then(parse_size).unwrap_or(1);

    let mut groups: BTreeMap<String, Vec<(String, u64)>> = BTreeMap::new();

    let config = WalkConfig {
        root,
        show_all: true,
        ..Default::default()
    };

    let mut entries: Vec<(std::path::PathBuf, u64)> = Vec::new();
    for entry in walk_entries(&config) {
        let path = entry.path();
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }
        let meta = match entry.metadata() {
            Ok(m) => m,
            _ => continue,
        };
        if meta.len() < min_bytes {
            continue;
        }
        entries.push((path.to_path_buf(), meta.len()));
    }

    let results: Vec<(String, String, u64)> = with_rayon(args.threads, || {
        entries
            .par_iter()
            .filter_map(|(path, size)| {
                hash_file::<Sha256>(path)
                    .ok()
                    .map(|hash| (hash, path.to_string_lossy().to_string(), *size))
            })
            .collect()
    });

    for (hash, name, size) in results {
        groups.entry(hash).or_default().push((name, size));
    }

    if super::json_enabled() {
        // Emit duplicate groups; deletion is an interactive action and is
        // never performed in JSON mode.
        let out: Vec<DupGroup> = groups
            .iter()
            .filter(|(_, files)| files.len() >= 2)
            .map(|(hash, files)| DupGroup {
                hash: hash.clone(),
                files: files
                    .iter()
                    .map(|(path, size)| DupFile {
                        path: path.clone(),
                        size: *size,
                    })
                    .collect(),
            })
            .collect();
        return super::emit_json(&out);
    }

    let mut found = false;
    for (_hash, files) in groups.iter() {
        if files.len() < 2 {
            continue;
        }
        found = true;
        println!("{}", "-".repeat(60));
        for (path, size) in files {
            println!("  {}  ({})", path, format_size(*size));
        }
    }

    if !found {
        println!("No duplicate files found.");
        return Ok(());
    }

    if args.delete {
        println!();
        print!("Delete duplicates (keep first in each group)? [y/N] ");
        use std::io::{stdin, stdout, Write};
        stdout().flush().map_err(|e| e.to_string())?;
        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }

        let mut deleted = 0usize;
        let mut failures: Vec<String> = Vec::new();

        for (_hash, files) in groups.iter() {
            if files.len() < 2 {
                continue;
            }
            let (keep, _) = &files[0];
            for (path, _size) in files.iter().skip(1) {
                match files_identical(keep, path) {
                    Ok(true) => match std::fs::remove_file(path) {
                        Ok(()) => {
                            println!("  Deleted: {}", path);
                            deleted += 1;
                        }
                        Err(e) => failures.push(format!("{}: {}", path, e)),
                    },
                    Ok(false) => failures.push(format!(
                        "{}: no longer identical to {}, skipped",
                        path, keep
                    )),
                    Err(e) => failures.push(e),
                }
            }
        }

        println!("\nDeleted {} file(s).", deleted);
        if !failures.is_empty() {
            eprintln!("{} could not be deleted:", failures.len());
            for f in &failures {
                eprintln!("  {}", f);
            }
            return Err(format!("{} deletion(s) failed", failures.len()));
        }
    }

    Ok(())
}
