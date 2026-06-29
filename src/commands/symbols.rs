#![allow(
    clippy::type_complexity,
    clippy::unnecessary_map_or,
    clippy::unnecessary_sort_by,
    unused_variables
)]

use clap::Args;
use serde::Serialize;
use std::path::Path;

#[derive(Args)]
pub struct SymbolsArgs {
    paths: Vec<String>,
    #[arg(
        short = 'k',
        long,
        help = "Symbol kind filter: fn, class, struct, trait, interface, enum, all"
    )]
    kind: Option<String>,
    #[arg(short = 'p', long, help = "Public/exported symbols only")]
    public_only: bool,
}

#[derive(Serialize)]
struct SymbolFile {
    path: String,
    symbols: Vec<Symbol>,
}

#[derive(Serialize)]
struct Symbol {
    name: String,
    kind: String,
    signature: String,
    line: usize,
    public: bool,
}

pub fn run(args: &SymbolsArgs) -> Result<(), String> {
    crate::config::require_feature("symbols")?;

    let paths = if args.paths.is_empty() {
        vec![".".to_string()]
    } else {
        args.paths.clone()
    };

    let mut results = Vec::new();
    let kind_filter = args.kind.as_deref().unwrap_or("all");

    for path_str in &paths {
        let p = Path::new(path_str);
        if p.is_dir() {
            collect_from_dir(p, kind_filter, args.public_only, &mut results)?;
        } else if p.is_file() {
            if let Some(symbols) = extract_symbols(p, kind_filter, args.public_only) {
                results.push(SymbolFile {
                    path: path_str.clone(),
                    symbols,
                });
            }
        }
    }

    if crate::commands::json_enabled() {
        crate::commands::emit_json(&results)
    } else {
        for file in &results {
            println!("{}:", file.path);
            for sym in &file.symbols {
                println!("  {} {} (line {})", sym.kind, sym.signature, sym.line);
            }
            if !file.symbols.is_empty() {
                println!();
            }
        }
        Ok(())
    }
}

fn collect_from_dir(
    dir: &Path,
    kind_filter: &str,
    public_only: bool,
    results: &mut Vec<SymbolFile>,
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
        if let Some(symbols) = extract_symbols(path, kind_filter, public_only) {
            if !symbols.is_empty() {
                results.push(SymbolFile {
                    path: path.to_string_lossy().to_string(),
                    symbols,
                });
            }
        }
    }
    Ok(())
}

fn extract_symbols(path: &Path, kind_filter: &str, public_only: bool) -> Option<Vec<Symbol>> {
    let ext = path.extension()?.to_str()?;
    let content = std::fs::read_to_string(path).ok()?;

    let patterns: Vec<(&str, &str, fn(&str, &str) -> bool)> = match ext {
        "rs" => vec![
            (
                "fn",
                r"(?m)^\s*(pub\s+)?(unsafe\s+)?fn\s+(\w+)",
                |pre, _name| pre.contains("pub"),
            ),
            (
                "struct",
                r"(?m)^\s*(pub\s+)?struct\s+(\w+)",
                |pre, _name| pre.contains("pub"),
            ),
            ("enum", r"(?m)^\s*(pub\s+)?enum\s+(\w+)", |pre, _name| {
                pre.contains("pub")
            }),
            ("trait", r"(?m)^\s*(pub\s+)?trait\s+(\w+)", |pre, _name| {
                pre.contains("pub")
            }),
            ("type", r"(?m)^\s*(pub\s+)?type\s+(\w+)", |pre, _name| {
                pre.contains("pub")
            }),
        ],
        "py" => vec![
            ("fn", r"(?m)^\s*(async\s+)?def\s+(\w+)", |_pre, name| {
                !name.starts_with('_')
            }),
            ("class", r"(?m)^\s*class\s+(\w+)", |_pre, name| {
                !name.starts_with('_')
            }),
        ],
        "ts" | "tsx" => vec![
            (
                "fn",
                r"(?m)^\s*(export\s+)?(function\s+(\w+)|const\s+(\w+)\s*=\s*(async\s+)?=>?)",
                |pre, _name| pre.contains("export"),
            ),
            (
                "class",
                r"(?m)^\s*(export\s+)?(abstract\s+)?class\s+(\w+)",
                |pre, _name| pre.contains("export"),
            ),
            (
                "interface",
                r"(?m)^\s*(export\s+)?interface\s+(\w+)",
                |pre, _name| pre.contains("export"),
            ),
            ("type", r"(?m)^\s*(export\s+)?type\s+(\w+)", |pre, _name| {
                pre.contains("export")
            }),
            ("enum", r"(?m)^\s*(export\s+)?enum\s+(\w+)", |pre, _name| {
                pre.contains("export")
            }),
        ],
        "js" | "jsx" => vec![
            (
                "fn",
                r"(?m)^\s*(export\s+)?(function\s+(\w+)|const\s+(\w+)\s*=\s*(async\s+)?=>?)",
                |_pre, _name| true,
            ),
            (
                "class",
                r"(?m)^\s*(export\s+)?(default\s+)?class\s+(\w+)",
                |_pre, _name| true,
            ),
        ],
        "go" => vec![
            ("fn", r"(?m)^\s*func\s+(\w+)", |_pre, name| {
                name.chars().next().map_or(false, |c| c.is_uppercase())
            }),
            ("struct", r"(?m)^\s*type\s+(\w+)\s+struct", |_pre, name| {
                name.chars().next().map_or(false, |c| c.is_uppercase())
            }),
            (
                "interface",
                r"(?m)^\s*type\s+(\w+)\s+interface",
                |_pre, name| name.chars().next().map_or(false, |c| c.is_uppercase()),
            ),
        ],
        "java" => vec![
            (
                "class",
                r"(?m)^\s*(public\s+)?(abstract\s+)?(final\s+)?class\s+(\w+)",
                |pre, _name| pre.contains("public"),
            ),
            (
                "interface",
                r"(?m)^\s*(public\s+)?interface\s+(\w+)",
                |pre, _name| pre.contains("public"),
            ),
            ("enum", r"(?m)^\s*(public\s+)?enum\s+(\w+)", |pre, _name| {
                pre.contains("public")
            }),
        ],
        _ => return None,
    };

    let mut symbols = Vec::new();

    for (kind, re_str, is_pub) in &patterns {
        if kind_filter != "all" && kind_filter != *kind {
            continue;
        }
        let re = regex::Regex::new(re_str).ok()?;
        for cap in re.captures_iter(&content) {
            let name = cap
                .iter()
                .last()
                .and_then(|m| m)
                .map(|m| m.as_str())
                .unwrap_or("unknown");
            let pre = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let public = is_pub(pre, name);
            if public_only && !public {
                continue;
            }
            // Find line number
            let mat = cap.get(0).unwrap();
            let line = content[..mat.start()].matches('\n').count() + 1;

            symbols.push(Symbol {
                name: name.to_string(),
                kind: kind.to_string(),
                signature: mat.as_str().trim().to_string(),
                line,
                public,
            });
        }
    }

    if symbols.is_empty() {
        return None;
    }

    symbols.sort_by(|a, b| a.line.cmp(&b.line));
    Some(symbols)
}
