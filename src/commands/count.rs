use std::fs::File;
use std::io::Read;

use serde::Serialize;

use crate::CountArgs;

const BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

#[derive(Serialize)]
struct CountRecord {
    path: String,
    lines: u64,
    words: u64,
    chars: u64,
    bytes: u64,
}

fn count_file(path: &str) -> Result<(u64, u64, u64, u64), String> {
    let mut file = File::open(path).map_err(|e| format!("{}: {}", path, e))?;
    let mut raw = Vec::new();
    file.read_to_end(&mut raw)
        .map_err(|e| format!("{}: {}", path, e))?;

    let byte_count = raw.len() as u64;
    let content = if raw.len() >= 3 && raw[..3] == BOM {
        &raw[3..]
    } else {
        &raw[..]
    };

    let content_str = String::from_utf8_lossy(content);
    let char_count = content_str.chars().count() as u64;
    // Match `wc -l`: count newline bytes, so a final line without a trailing
    // newline is not counted (unlike `str::lines`, which would count it).
    let line_count = content.iter().filter(|&&b| b == b'\n').count() as u64;
    let word_count = content_str.split_whitespace().count() as u64;

    Ok((line_count, word_count, char_count, byte_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_simple_string() {
        let tmp = std::env::temp_dir().join("tk-test-count-simple.txt");
        std::fs::write(&tmp, b"hello world\nfoo bar baz\n").unwrap();
        let (lines, words, chars, bytes) = count_file(&tmp.to_string_lossy()).unwrap();
        assert_eq!(lines, 2);
        assert_eq!(words, 5);
        assert_eq!(chars, 24);
        assert_eq!(bytes, 24);
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_count_bom_stripping() {
        let tmp = std::env::temp_dir().join("tk-test-count-bom.txt");
        let mut data = vec![0xEFu8, 0xBB, 0xBF];
        data.extend_from_slice(b"hello\n");
        std::fs::write(&tmp, &data).unwrap();
        let (lines, words, chars, bytes) = count_file(&tmp.to_string_lossy()).unwrap();
        assert_eq!(lines, 1);
        assert_eq!(words, 1);
        assert_eq!(chars, 6);
        assert_eq!(bytes, 9);
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_count_no_trailing_newline() {
        // `wc -l` counts newlines, so "a\nb" (no trailing newline) is 1 line.
        let tmp = std::env::temp_dir().join("tk-test-count-notrail.txt");
        std::fs::write(&tmp, b"a\nb").unwrap();
        let (lines, words, chars, bytes) = count_file(&tmp.to_string_lossy()).unwrap();
        assert_eq!(lines, 1);
        assert_eq!(words, 2);
        assert_eq!(chars, 3);
        assert_eq!(bytes, 3);
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_count_empty_file() {
        let tmp = std::env::temp_dir().join("tk-test-count-empty.txt");
        std::fs::write(&tmp, b"").unwrap();
        let (lines, words, chars, bytes) = count_file(&tmp.to_string_lossy()).unwrap();
        assert_eq!(lines, 0);
        assert_eq!(words, 0);
        assert_eq!(chars, 0);
        assert_eq!(bytes, 0);
        std::fs::remove_file(&tmp).unwrap();
    }
}

pub fn run(args: &CountArgs) -> Result<(), String> {
    if super::json_enabled() {
        // JSON always carries every metric; the -l/-w/-c/-b flags only shape
        // text columns, which don't apply here.
        let mut out = Vec::with_capacity(args.files.len());
        for path in &args.files {
            let (lines, words, chars, bytes) = count_file(path)?;
            out.push(CountRecord {
                path: path.clone(),
                lines,
                words,
                chars,
                bytes,
            });
        }
        return super::emit_json(&out);
    }

    let show_lines = args.lines || (!args.words && !args.chars && !args.bytes);
    let show_words = args.words || (!args.lines && !args.chars && !args.bytes);
    let show_bytes = args.bytes || (!args.words && !args.chars && !args.lines);
    let show_chars = args.chars;

    let mut total_lines = 0u64;
    let mut total_words = 0u64;
    let mut total_chars = 0u64;
    let mut total_bytes = 0u64;

    for path in &args.files {
        let (lines, words, chars, bytes) = count_file(path)?;
        total_lines += lines;
        total_words += words;
        total_chars += chars;
        total_bytes += bytes;

        if show_lines {
            print!("{:>8}\t", lines);
        }
        if show_words {
            print!("{:>8}\t", words);
        }
        if show_chars {
            print!("{:>8}\t", chars);
        }
        if show_bytes {
            print!("{:>8}\t", bytes);
        }
        println!("{}", path);
    }

    if args.files.len() > 1 {
        if show_lines {
            print!("{:>8}\t", total_lines);
        }
        if show_words {
            print!("{:>8}\t", total_words);
        }
        if show_chars {
            print!("{:>8}\t", total_chars);
        }
        if show_bytes {
            print!("{:>8}\t", total_bytes);
        }
        println!("total");
    }

    Ok(())
}
