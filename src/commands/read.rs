use clap::Args;
use serde::Serialize;
use std::path::Path;

#[derive(Args)]
pub struct ReadFileArgs {
    pub path: String,
    #[arg(long, default_value = "1048576", help = "Maximum bytes to read")]
    pub max_size: u64,
    #[arg(long, help = "Starting line number, 1-indexed")]
    pub offset: Option<usize>,
    #[arg(long, help = "Maximum number of lines to return")]
    pub limit: Option<usize>,
}

#[derive(Args)]
pub struct ReadLinesArgs {
    pub path: String,
    #[arg(long, help = "First line to read, 1-indexed")]
    pub start_line: Option<usize>,
    #[arg(long, help = "Last line to read, inclusive")]
    pub end_line: Option<usize>,
}

#[derive(Serialize)]
struct ReadResult {
    path: String,
    total_lines: usize,
    returned_lines: usize,
    truncated: bool,
    lines: Vec<String>,
}

fn is_binary(path: &Path) -> bool {
    use std::io::Read;
    match std::fs::File::open(path) {
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

pub fn run_read_file(args: &ReadFileArgs) -> Result<(), String> {
    let path = Path::new(&args.path);
    if !path.exists() {
        return Err(format!("file not found: {}", args.path));
    }
    if is_binary(path) {
        return Err(format!("refusing to read binary file: {}", args.path));
    }
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read '{}': {}", args.path, e))?;
    let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
    if meta.len() > args.max_size {
        return Err(format!(
            "file '{}' is {} bytes, exceeding max_size of {}",
            args.path,
            meta.len(),
            args.max_size
        ));
    }

    let all_lines: Vec<&str> = contents.lines().collect();
    let total_lines = all_lines.len();
    let offset = args.offset.unwrap_or(1).max(1) - 1;
    let limit = args.limit.unwrap_or(total_lines);

    let selected: Vec<String> = all_lines
        .iter()
        .skip(offset)
        .take(limit)
        .enumerate()
        .map(|(i, line)| format!("{:>6}: {}", offset + i + 1, line))
        .collect();

    let truncated = offset + limit < total_lines;

    if super::json_enabled() {
        let result = ReadResult {
            path: args.path.clone(),
            total_lines,
            returned_lines: selected.len(),
            truncated,
            lines: selected,
        };
        super::emit_json(&result)
    } else {
        for line in &selected {
            println!("{}", line);
        }
        if truncated {
            eprintln!(
                "... truncated ({} of {} lines shown)",
                selected.len(),
                total_lines
            );
        }
        Ok(())
    }
}

pub fn run_read_lines(args: &ReadLinesArgs) -> Result<(), String> {
    let path = Path::new(&args.path);
    if !path.exists() {
        return Err(format!("file not found: {}", args.path));
    }
    if is_binary(path) {
        return Err(format!("refusing to read binary file: {}", args.path));
    }
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read '{}': {}", args.path, e))?;

    let all_lines: Vec<&str> = contents.lines().collect();
    let total_lines = all_lines.len();
    let start = args.start_line.unwrap_or(1).max(1) - 1;
    let end = args.end_line.unwrap_or(total_lines).min(total_lines);

    let selected: Vec<String> = all_lines
        .iter()
        .enumerate()
        .skip(start)
        .take(end.saturating_sub(start))
        .map(|(i, line)| format!("{:>6}: {}", i + 1, line))
        .collect();

    let truncated = end < total_lines;

    if super::json_enabled() {
        let result = ReadResult {
            path: args.path.clone(),
            total_lines,
            returned_lines: selected.len(),
            truncated,
            lines: selected,
        };
        super::emit_json(&result)
    } else {
        for line in &selected {
            println!("{}", line);
        }
        if truncated {
            eprintln!(
                "... truncated ({} of {} lines shown)",
                selected.len(),
                total_lines
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_binary_rejection() {
        let tmp = std::env::temp_dir().join("tk-test-read-binary.bin");
        std::fs::write(&tmp, b"\x00\x01\x02").unwrap();
        let args = ReadFileArgs {
            path: tmp.to_string_lossy().to_string(),
            max_size: 1048576,
            offset: None,
            limit: None,
        };
        assert!(run_read_file(&args).is_err());
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_read_file_nonexistent() {
        let args = ReadFileArgs {
            path: "/nonexistent/file.txt".to_string(),
            max_size: 1048576,
            offset: None,
            limit: None,
        };
        assert!(run_read_file(&args).is_err());
    }

    #[test]
    fn test_read_lines_basic() {
        let tmp = std::env::temp_dir().join("tk-test-read-lines.txt");
        std::fs::write(&tmp, "a\nb\nc\nd\ne\n").unwrap();
        let args = ReadLinesArgs {
            path: tmp.to_string_lossy().to_string(),
            start_line: Some(2),
            end_line: Some(4),
        };
        assert!(run_read_lines(&args).is_ok());
        std::fs::remove_file(&tmp).unwrap();
    }
}
