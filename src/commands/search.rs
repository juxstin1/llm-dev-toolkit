use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::commands;
use crate::commands::{ansi, color_enabled, paint, walk_entries, WalkConfig};

pub fn run(args: &crate::RgArgs) -> Result<(), String> {
    let pattern = &args.pattern;
    let root = args.path.as_deref().unwrap_or(".");

    let config = WalkConfig {
        root,
        show_all: false,
        ..Default::default()
    };

    for entry in walk_entries(&config) {
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }
        let file_path = entry.path();

        if let Some(ref ext) = args.ext {
            match file_path.extension().and_then(|e| e.to_str()) {
                Some(e) if e.eq_ignore_ascii_case(ext) => {}
                _ => continue,
            }
        }

        if commands::is_binary(file_path) {
            continue;
        }

        if args.files_with_matches {
            search_file_files(file_path, pattern, args.ignore_case)
        } else if let Some(ctx) = args.context {
            search_file_context(file_path, pattern, args.ignore_case, ctx)
        } else {
            search_file_lines(file_path, pattern, args.ignore_case, args.line_number)
        }?;
    }

    Ok(())
}

/// Does `line` contain `pattern` (optionally case-insensitively)?
fn line_matches(line: &str, pattern: &str, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(&pattern.to_lowercase())
    } else {
        line.contains(pattern)
    }
}

/// Format a matched line's content: highlighted when color is on, plain otherwise.
fn content(line: &str, pattern: &str, ignore_case: bool) -> String {
    if color_enabled() {
        highlight_match(line, pattern, ignore_case)
    } else {
        line.to_string()
    }
}

fn search_file_files(path: &Path, pattern: &str, ignore_case: bool) -> Result<bool, String> {
    let file = File::open(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| format!("{}: {}", path.display(), e))?;
        if line_matches(&line, pattern, ignore_case) {
            println!("{}", paint(ansi::MAGENTA, &path.display().to_string()));
            return Ok(true);
        }
    }
    Ok(false)
}

fn search_file_lines(
    path: &Path,
    pattern: &str,
    ignore_case: bool,
    show_number: bool,
) -> Result<bool, String> {
    let file = File::open(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let reader = BufReader::new(file);
    let mut found = false;

    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("{}: {}", path.display(), e))?;
        if !line_matches(&line, pattern, ignore_case) {
            continue;
        }
        found = true;
        let path_str = paint(ansi::MAGENTA, &path.display().to_string());
        let body = content(&line, pattern, ignore_case);
        if show_number {
            let num = paint(ansi::YELLOW, &(i + 1).to_string());
            println!("{}:{}:{}", path_str, num, body);
        } else {
            println!("{}:{}", path_str, body);
        }
    }
    Ok(found)
}

fn search_file_context(
    path: &Path,
    pattern: &str,
    ignore_case: bool,
    context: usize,
) -> Result<bool, String> {
    let file = File::open(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.map_err(|e| format!("{}: {}", path.display(), e))?);
    }

    let match_indices: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| line_matches(l, pattern, ignore_case).then_some(i))
        .collect();

    if match_indices.is_empty() {
        return Ok(false);
    }

    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut gs = match_indices[0];
    let mut ge = match_indices[0];

    for &idx in &match_indices[1..] {
        if idx <= ge + 2 * context + 1 {
            ge = idx;
        } else {
            groups.push((
                gs.saturating_sub(context),
                std::cmp::min(ge + context, lines.len().saturating_sub(1)),
            ));
            gs = idx;
            ge = idx;
        }
    }
    groups.push((
        gs.saturating_sub(context),
        std::cmp::min(ge + context, lines.len().saturating_sub(1)),
    ));

    let is_match = |i: usize| match_indices.binary_search(&i).is_ok();

    for (g_idx, &(first, last)) in groups.iter().enumerate() {
        if g_idx > 0 {
            println!("--");
        }
        for (offset, line) in lines[first..=last].iter().enumerate() {
            let i = first + offset;
            let path_str = paint(ansi::MAGENTA, &path.display().to_string());
            let num = paint(ansi::YELLOW, &(i + 1).to_string());
            if is_match(i) {
                let body = content(line, pattern, ignore_case);
                println!("{}:{}:{}", path_str, num, body);
            } else {
                // A context line: '-' separator distinguishes it from a match.
                println!("{}:{}-{}", path_str, num, line);
            }
        }
    }

    Ok(true)
}

fn chars_eq(a: char, b: char, ignore_case: bool) -> bool {
    if !ignore_case {
        return a == b;
    }
    a.eq_ignore_ascii_case(&b) || a.to_lowercase().eq(b.to_lowercase())
}

/// Highlight every non-overlapping occurrence of `pattern` in `line`.
///
/// Operates on char boundaries throughout, so it never panics on multibyte
/// input — unlike byte-offset slicing against a `to_lowercase()` copy, whose
/// length can differ from the original (e.g. Turkish `İ`).
fn highlight_match(line: &str, pattern: &str, ignore_case: bool) -> String {
    if pattern.is_empty() {
        return line.to_string();
    }

    let line_chars: Vec<(usize, char)> = line.char_indices().collect();
    let pat_chars: Vec<char> = pattern.chars().collect();
    let plen = pat_chars.len();

    let mut out = String::with_capacity(line.len() + 16);
    let mut i = 0;
    while i < line_chars.len() {
        let is_match = i + plen <= line_chars.len()
            && (0..plen).all(|k| chars_eq(line_chars[i + k].1, pat_chars[k], ignore_case));
        if is_match {
            let start = line_chars[i].0;
            let end = line_chars
                .get(i + plen)
                .map(|(off, _)| *off)
                .unwrap_or(line.len());
            out.push_str(ansi::RED);
            out.push_str(&line[start..end]);
            out.push_str(ansi::RESET);
            i += plen;
        } else {
            out.push(line_chars[i].1);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_ascii_match() {
        let out = highlight_match("hello world", "world", false);
        assert_eq!(out, "hello \x1b[31mworld\x1b[0m");
    }

    #[test]
    fn test_highlight_all_occurrences() {
        let out = highlight_match("aXaXa", "a", false);
        assert_eq!(out, "\x1b[31ma\x1b[0mX\x1b[31ma\x1b[0mX\x1b[31ma\x1b[0m");
    }

    #[test]
    fn test_highlight_case_insensitive() {
        let out = highlight_match("Hello", "hello", true);
        assert_eq!(out, "\x1b[31mHello\x1b[0m");
    }

    #[test]
    fn test_highlight_no_match() {
        let out = highlight_match("hello", "xyz", false);
        assert_eq!(out, "hello");
    }

    #[test]
    fn test_highlight_multibyte_no_panic() {
        // Turkish dotted capital I lowercases to two chars; must not panic.
        let out = highlight_match("İstanbul café", "café", true);
        assert!(out.contains("café"));
    }

    #[test]
    fn test_highlight_empty_pattern() {
        let out = highlight_match("hello", "", false);
        assert_eq!(out, "hello");
    }
}
