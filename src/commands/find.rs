use super::{rel_to, walk_entries, WalkConfig};
use std::path::Path;

fn name_matches(name: &str, pattern: &str, ignore_case: bool) -> bool {
    if ignore_case {
        name.to_lowercase().contains(&pattern.to_lowercase())
    } else {
        name.contains(pattern)
    }
}

fn ext_matches(path: &Path, target_ext: &str, ignore_case: bool) -> bool {
    match path.extension() {
        Some(e) => {
            if ignore_case {
                e.to_string_lossy().to_lowercase() == target_ext.to_lowercase()
            } else {
                e == target_ext
            }
        }
        None => false,
    }
}

pub fn run_name(args: &crate::FfArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let root_path = Path::new(root);

    let config = WalkConfig {
        root,
        show_all: false,
        max_depth: None,
    };

    let mut matches: Vec<String> = Vec::new();
    for entry in walk_entries(&config) {
        let ft = match entry.file_type() {
            Some(ft) => ft,
            None => continue,
        };

        if let Some(ref tf) = args.type_filter {
            match tf.as_str() {
                "f" if !ft.is_file() => continue,
                "d" if !ft.is_dir() => continue,
                _ => {}
            }
        }

        if let Some(ref ext) = args.ext {
            if !ext_matches(entry.path(), ext, args.ignore_case) {
                continue;
            }
        }

        let name = entry.file_name().to_string_lossy();
        if !name_matches(&name, &args.pattern, args.ignore_case) {
            continue;
        }

        matches.push(rel_to(entry.path(), root_path).display().to_string());
    }

    emit_paths(matches)
}

/// Print one path per line, or a JSON array of paths under `--format json`.
fn emit_paths(matches: Vec<String>) -> Result<(), String> {
    if super::json_enabled() {
        return super::emit_json(&matches);
    }
    for m in &matches {
        println!("{}", m);
    }
    Ok(())
}

pub fn run_ext(args: &crate::FfExtArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let root_path = Path::new(root);
    let ext = args.ext.trim_start_matches('.');

    let config = WalkConfig {
        root,
        ..Default::default()
    };

    let mut matches: Vec<String> = Vec::new();
    for entry in walk_entries(&config) {
        let ft = match entry.file_type() {
            Some(ft) => ft,
            None => continue,
        };

        if !ft.is_file() {
            continue;
        }

        match entry.path().extension() {
            Some(e) if e == ext => {}
            _ => continue,
        }

        matches.push(rel_to(entry.path(), root_path).display().to_string());
    }

    emit_paths(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_name_matches_exact() {
        assert!(name_matches("hello.rs", "hello", false));
    }

    #[test]
    fn test_name_matches_case_sensitive_fail() {
        assert!(!name_matches("hello.rs", "HELLO", false));
    }

    #[test]
    fn test_name_matches_case_insensitive() {
        assert!(name_matches("hello.rs", "HELLO", true));
    }

    #[test]
    fn test_name_matches_no_match() {
        assert!(!name_matches("hello.rs", "xyz", false));
    }

    #[test]
    fn test_ext_matches_rs() {
        assert!(ext_matches(Path::new("hello.rs"), "rs", false));
    }

    #[test]
    fn test_ext_matches_case_sensitive_fail() {
        assert!(!ext_matches(Path::new("hello.rs"), "RS", false));
    }

    #[test]
    fn test_ext_matches_case_insensitive() {
        assert!(ext_matches(Path::new("hello.rs"), "RS", true));
    }

    #[test]
    fn test_matches_glob_exact() {
        assert!(matches_glob("hello.rs", "hello.rs", false));
    }

    #[test]
    fn test_matches_glob_wildcard() {
        assert!(matches_glob("hello.rs", "*.rs", false));
    }

    #[test]
    fn test_matches_glob_no_match() {
        assert!(!matches_glob("hello.rs", "*.txt", false));
    }

    #[test]
    fn test_matches_glob_case_insensitive() {
        assert!(matches_glob("HELLO.RS", "*.rs", true));
    }

    #[test]
    fn test_matches_glob_bad_pattern() {
        assert!(!matches_glob("hello.rs", "[invalid", false));
    }
}

fn matches_glob(name: &str, pattern: &str, ignore_case: bool) -> bool {
    let pat = if ignore_case {
        glob::Pattern::new(&pattern.to_lowercase()).ok()
    } else {
        glob::Pattern::new(pattern).ok()
    };
    match pat {
        Some(p) => {
            if ignore_case {
                p.matches(&name.to_lowercase())
            } else {
                p.matches(name)
            }
        }
        None => false,
    }
}

pub fn run_name_pattern(args: &crate::FfNameArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let root_path = Path::new(root);

    let config = WalkConfig {
        root,
        ..Default::default()
    };

    let mut matches: Vec<String> = Vec::new();
    for entry in walk_entries(&config) {
        let name = entry.file_name().to_string_lossy();

        let matched = if args.glob {
            matches_glob(&name, &args.pattern, args.ignore_case)
        } else {
            name_matches(&name, &args.pattern, args.ignore_case)
        };
        if !matched {
            continue;
        }

        matches.push(rel_to(entry.path(), root_path).display().to_string());
    }

    emit_paths(matches)
}
