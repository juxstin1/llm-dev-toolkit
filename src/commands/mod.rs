pub mod checksum;
pub mod clip;
pub mod count;
pub mod dups;
pub mod empty;
pub mod extract;
pub mod find;
pub mod info;
pub mod json;
pub mod largest;
pub mod ls;
pub mod recent;
pub mod search;
pub mod sort;
pub mod stats;
pub mod tree;
pub mod view;

use chrono::{DateTime, Local};
use clap::ValueEnum;
use ignore::WalkBuilder;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use std::time::UNIX_EPOCH;

static COLOR_ENABLED: OnceLock<bool> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, ValueEnum)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}

pub fn init_color(choice: ColorChoice) {
    let enabled = match choice {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => atty::is(atty::Stream::Stdout) && std::env::var("NO_COLOR").is_err(),
    };
    let _ = COLOR_ENABLED.set(enabled);
}

pub fn color_enabled() -> bool {
    COLOR_ENABLED.get().copied().unwrap_or(true)
}

/// ANSI escape codes shared across commands. Use [`paint`] rather than
/// emitting these directly so output honors `--color`.
pub mod ansi {
    pub const BLUE: &str = "\x1b[34m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RESET: &str = "\x1b[0m";
}

/// Wrap `text` in an ANSI `code`, or return it unstyled when color is disabled.
pub fn paint(code: &str, text: &str) -> String {
    if color_enabled() {
        format!("{}{}{}", code, text, ansi::RESET)
    } else {
        text.to_string()
    }
}

/// A path relative to `root`, falling back to the full path when it isn't a prefix.
pub fn rel_to<'a>(path: &'a Path, root: &Path) -> &'a Path {
    path.strip_prefix(root).unwrap_or(path)
}

pub fn with_rayon<F, R>(threads: Option<usize>, f: F) -> R
where
    F: FnOnce() -> R + Send,
    R: Send,
{
    match threads {
        Some(n) => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n)
                .build()
                .expect("Failed to build rayon thread pool");
            pool.install(f)
        }
        None => f(),
    }
}

/// Configuration for the shared directory walker
pub struct WalkConfig<'a> {
    pub root: &'a str,
    pub show_all: bool,
    pub max_depth: Option<usize>,
}

impl<'a> Default for WalkConfig<'a> {
    fn default() -> Self {
        Self {
            root: ".",
            show_all: false,
            max_depth: None,
        }
    }
}

/// Walk entries with consistent error handling.
///
/// The `.git` directory is always skipped — even with `show_all: true` — so
/// commands like `dups`, `stats`, and `largest` never descend into git
/// internals (and `dups --delete` can never offer to delete them).
pub fn walk_entries(config: &WalkConfig) -> impl Iterator<Item = ignore::DirEntry> {
    let mut builder = WalkBuilder::new(config.root);
    builder
        .hidden(!config.show_all)
        .parents(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true);
    if let Some(depth) = config.max_depth {
        builder.max_depth(Some(depth));
    }
    builder
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().components().any(|c| c.as_os_str() == ".git"))
}

pub fn format_size(size: u64) -> String {
    humansize::format_size(size, humansize::BINARY)
}

pub fn format_time(secs: i64) -> String {
    let secs = if secs < 0 { 0 } else { secs as u64 };
    let dt = DateTime::<Local>::from(UNIX_EPOCH + std::time::Duration::from_secs(secs));
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn is_binary(path: &Path) -> bool {
    use std::io::Read;
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut buf = [0u8; 8192];
            match file.read(&mut buf) {
                Ok(n) => buf[..n].contains(&0x00),
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_zero() {
        let s = format_size(0);
        assert!(s.contains("0 B"), "Expected '0 B' in '{}'", s);
    }

    #[test]
    fn test_format_size_kib() {
        let s = format_size(1024);
        assert!(s.contains("KiB"), "Expected 'KiB' in '{}'", s);
    }

    #[test]
    fn test_format_time_epoch() {
        let s = format_time(0);
        let ok = s.contains("1970") || s.contains("1969");
        assert!(ok, "Expected 1970-ish date in '{}'", s);
    }

    #[test]
    fn test_is_binary_text_file() {
        let tmp = std::env::temp_dir().join("tk-test-mod-is-binary.txt");
        std::fs::write(&tmp, b"hello world\n").unwrap();
        assert!(!is_binary(&tmp));
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_walk_config_default() {
        let config = WalkConfig::default();
        assert_eq!(config.root, ".");
        assert!(!config.show_all);
        assert_eq!(config.max_depth, None);
    }

    #[test]
    fn test_with_rayon_default_uses_global() {
        let result = with_rayon(None, || 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_with_rayon_threads_1() {
        let result = with_rayon(Some(1), rayon::current_num_threads);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_paint_wraps_when_enabled() {
        // color_enabled() defaults to true when the OnceLock is unset (test context).
        assert_eq!(
            paint(ansi::BLUE, "x"),
            format!("{}x{}", ansi::BLUE, ansi::RESET)
        );
    }

    #[test]
    fn test_rel_to_strips_prefix() {
        let p = Path::new("/root/a/b.txt");
        assert_eq!(rel_to(p, Path::new("/root")), Path::new("a/b.txt"));
    }

    #[test]
    fn test_rel_to_falls_back_when_not_prefix() {
        let p = Path::new("/other/a.txt");
        assert_eq!(rel_to(p, Path::new("/root")), p);
    }
}
