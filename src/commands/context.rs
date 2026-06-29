use clap::Args;
use serde::Serialize;
use std::path::Path;

#[derive(Args)]
pub struct ContextArgs {
    paths: Vec<String>,
    #[arg(
        long,
        help = "Maximum token budget (estimated, ~3.5 bytes/token for code)"
    )]
    max_tokens: Option<usize>,
    #[arg(long, help = "Include pattern (glob)")]
    include: Option<String>,
    #[arg(long, help = "Exclude pattern (glob)")]
    exclude: Option<String>,
    #[arg(long, help = "Disable line numbers in output")]
    no_line_numbers: bool,
}

#[derive(Serialize)]
struct ContextResult {
    total_tokens: usize,
    total_bytes: usize,
    max_tokens: Option<usize>,
    truncated: bool,
    files: Vec<ContextFile>,
}

#[derive(Serialize)]
struct ContextFile {
    path: String,
    tokens: usize,
    bytes: usize,
    lines: usize,
    truncated: bool,
    content: String,
}

pub fn run(args: &ContextArgs) -> Result<(), String> {
    crate::config::require_feature("context")?;

    let paths = if args.paths.is_empty() {
        vec![".".to_string()]
    } else {
        args.paths.clone()
    };

    let mut files = Vec::new();

    for path_str in &paths {
        let p = Path::new(path_str);
        if p.is_dir() {
            collect_from_dir(p, &mut files, args)?;
        } else if p.is_file() {
            if let Some(file) = read_file(p, args) {
                files.push(file);
            }
        }
    }

    let total_bytes: usize = files.iter().map(|f| f.bytes).sum();
    let total_tokens = estimate_tokens(
        &files
            .iter()
            .map(|f| f.content.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
    );
    let mut truncated = false;

    let mut output_files = files;

    // Apply max_tokens truncation
    if let Some(max) = args.max_tokens {
        if total_tokens > max {
            // Drop files from the end until we fit
            let mut running = 0usize;
            let mut keep_until = output_files.len();
            for (i, file) in output_files.iter().enumerate() {
                let file_tokens = estimate_tokens(&file.content);
                if running + file_tokens > max && i > 0 {
                    keep_until = i;
                    truncated = true;
                    break;
                }
                running += file_tokens;
            }
            output_files.truncate(keep_until);
            // Mark the last file as possibly truncated
            if truncated {
                output_files.last_mut().unwrap().truncated = true;
            }
        }
    }

    let result = ContextResult {
        total_tokens,
        total_bytes,
        max_tokens: args.max_tokens,
        truncated,
        files: output_files,
    };

    if crate::commands::json_enabled() {
        crate::commands::emit_json(&result)
    } else {
        for file in &result.files {
            println!("# --- {} ---", file.path);
            print!("{}", file.content);
            if !file.content.ends_with('\n') {
                println!();
            }
            println!();
        }
        if result.truncated {
            eprintln!("-- truncated --");
        }
        Ok(())
    }
}

fn collect_from_dir(
    dir: &Path,
    files: &mut Vec<ContextFile>,
    args: &ContextArgs,
) -> Result<(), String> {
    let walk = crate::commands::WalkConfig {
        root: dir.to_str().unwrap_or("."),
        show_all: false,
        max_depth: None,
    };

    for entry in crate::commands::walk_entries(&walk) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        // Globbing skipped for simplicity in v1
        if let Some(file) = read_file(path, args) {
            files.push(file);
        }
    }
    Ok(())
}

fn read_file(path: &Path, args: &ContextArgs) -> Option<ContextFile> {
    let content = std::fs::read_to_string(path).ok()?;
    let bytes = content.len();
    let lines = content.matches('\n').count();

    let body = if args.no_line_numbers {
        content.clone()
    } else {
        let numbered: Vec<String> = content
            .lines()
            .enumerate()
            .map(|(i, line)| format!("{}:{}", i + 1, line))
            .collect();
        numbered.join("\n")
    };

    Some(ContextFile {
        path: path.to_string_lossy().to_string(),
        tokens: estimate_tokens(&content),
        bytes,
        lines,
        truncated: false,
        content: body,
    })
}

/// Estimate tokens using ~3.5 bytes/token heuristic for code.
fn estimate_tokens(text: &str) -> usize {
    let bytes = text.len();
    (bytes + 1) / 4 // ~4 bytes per token
}
