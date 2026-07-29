use clap::Args;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub struct ShowArgs {
    #[arg(value_name = "FILE[:LINE]")]
    target: String,
    #[arg(
        short = 'C',
        long,
        default_value = "5",
        help = "Lines around the target"
    )]
    context: usize,
}

#[derive(Serialize)]
struct ShowResult {
    path: String,
    line: Option<usize>,
    context: usize,
    start_line: usize,
    end_line: usize,
    lines: Vec<ShowLine>,
}

#[derive(Serialize)]
struct ShowLine {
    line: usize,
    text: String,
    target: bool,
}

fn split_target(target: &str) -> Result<(PathBuf, Option<usize>), String> {
    let Some((candidate, suffix)) = target.rsplit_once(':') else {
        return Ok((PathBuf::from(target), None));
    };
    let Ok(line) = suffix.parse::<usize>() else {
        return Ok((PathBuf::from(target), None));
    };
    if line == 0 {
        return Err("line number must be greater than zero".into());
    }
    if candidate
        .rsplit_once(':')
        .is_some_and(|(_, possible_line)| possible_line.parse::<usize>().is_ok())
    {
        return Err("FILE:LINE:COLUMN is not supported; pass FILE:LINE".into());
    }
    Ok((PathBuf::from(candidate), Some(line)))
}

pub fn run(args: &ShowArgs) -> Result<(), String> {
    let (path, target_line) = split_target(&args.target)?;
    let display_path = path.to_string_lossy().to_string();
    let metadata =
        std::fs::metadata(&path).map_err(|e| format!("Cannot access '{}': {}", display_path, e))?;
    if !metadata.is_file() {
        return Err(format!("'{}' is not a file", display_path));
    }
    if super::is_binary(Path::new(&path)) {
        return Err(format!("Cannot show binary file '{}'", display_path));
    }
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read '{}': {}", display_path, e))?;
    let source: Vec<&str> = content.lines().collect();
    if source.is_empty() {
        return Err(format!("'{}' is empty", display_path));
    }

    let (start, end) = match target_line {
        Some(line) if line > source.len() => {
            return Err(format!(
                "line {} is outside '{}' ({} lines)",
                line,
                display_path,
                source.len()
            ))
        }
        Some(line) => (
            line.saturating_sub(args.context).max(1),
            line.saturating_add(args.context).min(source.len()),
        ),
        None => (1, source.len().min(40)),
    };

    let lines = (start..=end)
        .map(|number| ShowLine {
            line: number,
            text: source[number - 1].to_string(),
            target: target_line == Some(number),
        })
        .collect::<Vec<_>>();
    let result = ShowResult {
        path: display_path,
        line: target_line,
        context: args.context,
        start_line: start,
        end_line: end,
        lines,
    };

    if super::json_enabled() {
        super::emit_json(&result)
    } else {
        for line in &result.lines {
            let marker = if line.target { '>' } else { ' ' };
            println!("{} {:>6} | {}", marker, line.line, line.text);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_windows_path_and_line() {
        let (path, line) = split_target(r"C:\repo\src\main.rs:120").unwrap();
        assert_eq!(path, PathBuf::from(r"C:\repo\src\main.rs"));
        assert_eq!(line, Some(120));
    }

    #[test]
    fn rejects_column_suffix() {
        assert!(split_target("src/main.rs:12:4").is_err());
    }
}
