//! A minimal Model Context Protocol (MCP) server exposing `tk`'s read-only
//! commands as agent tools.
//!
//! MCP over stdio is newline-delimited JSON-RPC 2.0 — no `Content-Length`
//! framing — so the loop reads one JSON object per line and writes one compact
//! JSON object per line. This is handled synchronously with no async runtime,
//! keeping the binary dependency-lean.
//!
//! Each tool maps to a `tk` subcommand. Rather than re-enter command logic
//! in-process (and capture stdout), the server re-invokes the current
//! executable with `--format json`, which guarantees identical behavior to the
//! CLI and keeps output machine-readable. Only read-only commands are exposed,
//! so a connected model can never mutate the filesystem through `tk`.

use std::io::{BufRead, Write};
use std::process::Command;

use serde_json::{json, Value};

/// Protocol version advertised when a client doesn't request one.
const DEFAULT_PROTOCOL_VERSION: &str = "2025-06-18";

/// One exposed tool: its name, human description, and JSON Schema for inputs.
struct ToolDef {
    name: &'static str,
    description: &'static str,
    schema: Value,
}

fn string_prop(desc: &str) -> Value {
    json!({ "type": "string", "description": desc })
}
fn bool_prop(desc: &str) -> Value {
    json!({ "type": "boolean", "description": desc })
}
fn int_prop(desc: &str) -> Value {
    json!({ "type": "integer", "description": desc })
}

/// The curated, read-only tool set. Side-effecting commands (`extract`,
/// `clip`, `dups --delete`) and ones redundant with an agent's built-in file
/// reading (`cat`, `head`, `preview`) are intentionally omitted.
fn tool_defs() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "ls",
            description: "List directory contents (name, type, size, mtime).",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Directory to list (default: current dir)"),
                    "all": bool_prop("Include hidden entries"),
                    "long": bool_prop("Include permissions and link counts")
                }
            }),
        },
        ToolDef {
            name: "tree",
            description: "Directory tree as a nested structure. Respects .gitignore.",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Root directory (default: current dir)"),
                    "depth": int_prop("Maximum depth to descend"),
                    "all": bool_prop("Include hidden entries"),
                    "dirs_only": bool_prop("Show directories only")
                }
            }),
        },
        ToolDef {
            name: "find",
            description: "Find files/dirs by name substring. Respects .gitignore.",
            schema: json!({
                "type": "object",
                "properties": {
                    "pattern": string_prop("Name substring to match"),
                    "path": string_prop("Root directory (default: current dir)"),
                    "ignore_case": bool_prop("Case-insensitive matching"),
                    "ext": string_prop("Filter by file extension, e.g. rs"),
                    "type": json!({ "type": "string", "enum": ["f", "d"], "description": "f=files, d=dirs" })
                },
                "required": ["pattern"]
            }),
        },
        ToolDef {
            name: "search",
            description: "Search file contents (grep-like). Returns {path,line,text} matches. Respects .gitignore and skips binaries.",
            schema: json!({
                "type": "object",
                "properties": {
                    "pattern": string_prop("Substring to search for"),
                    "path": string_prop("Root directory (default: current dir)"),
                    "ignore_case": bool_prop("Case-insensitive matching"),
                    "ext": string_prop("Restrict to a file extension, e.g. rs"),
                    "files_with_matches": bool_prop("Return only matching file paths")
                },
                "required": ["pattern"]
            }),
        },
        ToolDef {
            name: "stats",
            description: "File/dir/byte counts for a tree, optionally broken down by extension or directory.",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Root directory (default: current dir)"),
                    "by_type": bool_prop("Break down by file extension"),
                    "by_directory": bool_prop("Break down per directory"),
                    "max_depth": int_prop("Maximum directory depth")
                }
            }),
        },
        ToolDef {
            name: "dups",
            description: "Find duplicate files by SHA-256 content hash (read-only; never deletes).",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Root directory (default: current dir)"),
                    "min_size": string_prop("Minimum file size, e.g. 1kib, 1mb")
                }
            }),
        },
        ToolDef {
            name: "largest",
            description: "Largest files (or directories) under a tree.",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Root directory (default: current dir)"),
                    "count": int_prop("Number of entries to return"),
                    "directories": bool_prop("Rank directories instead of files")
                }
            }),
        },
        ToolDef {
            name: "recent",
            description: "Recently modified files within a time window.",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Root directory (default: current dir)"),
                    "count": int_prop("Number of files to return"),
                    "days": int_prop("How far back to look, in days"),
                    "ext": string_prop("Filter by file extension")
                }
            }),
        },
        ToolDef {
            name: "empty",
            description: "Find empty files and/or directories.",
            schema: json!({
                "type": "object",
                "properties": {
                    "path": string_prop("Root directory (default: current dir)"),
                    "files": bool_prop("Empty files only"),
                    "dirs": bool_prop("Empty directories only")
                }
            }),
        },
        ToolDef {
            name: "count",
            description: "Count lines, words, chars, and bytes for the given files.",
            schema: json!({
                "type": "object",
                "properties": {
                    "files": json!({ "type": "array", "items": { "type": "string" }, "description": "Files to count" })
                },
                "required": ["files"]
            }),
        },
        ToolDef {
            name: "checksum",
            description: "Compute file checksums (sha256/224/384/512, md5).",
            schema: json!({
                "type": "object",
                "properties": {
                    "files": json!({ "type": "array", "items": { "type": "string" }, "description": "Files to hash" }),
                    "algorithm": string_prop("sha256 (default), sha224, sha384, sha512, md5")
                },
                "required": ["files"]
            }),
        },
        ToolDef {
            name: "info",
            description: "File details (size, type, mime, timestamps) or a system overview when no file is given.",
            schema: json!({
                "type": "object",
                "properties": {
                    "file": string_prop("Path to a specific file (omit for system overview)")
                }
            }),
        },
    ]
}

fn arg_str(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(Value::as_str).map(str::to_string)
}
fn arg_bool(args: &Value, key: &str) -> bool {
    args.get(key).and_then(Value::as_bool).unwrap_or(false)
}
fn arg_int(args: &Value, key: &str) -> Option<i64> {
    args.get(key).and_then(Value::as_i64)
}

/// Translate a tool name + JSON arguments into `tk` CLI arguments. The global
/// `--format json --color never` flags are appended by the caller.
fn build_args(name: &str, args: &Value) -> Result<Vec<String>, String> {
    let mut v: Vec<String> = Vec::new();

    match name {
        "ls" => {
            v.push("ls".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if arg_bool(args, "all") {
                v.push("-a".into());
            }
            if arg_bool(args, "long") {
                v.push("-l".into());
            }
        }
        "tree" => {
            v.push("tree".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if let Some(d) = arg_int(args, "depth") {
                v.push("-L".into());
                v.push(d.to_string());
            }
            if arg_bool(args, "all") {
                v.push("-a".into());
            }
            if arg_bool(args, "dirs_only") {
                v.push("-d".into());
            }
        }
        "find" => {
            v.push("ff".into());
            v.push(arg_str(args, "pattern").ok_or("missing required 'pattern'")?);
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if arg_bool(args, "ignore_case") {
                v.push("-i".into());
            }
            if let Some(e) = arg_str(args, "ext") {
                v.push("-e".into());
                v.push(e);
            }
            if let Some(t) = arg_str(args, "type") {
                v.push("-t".into());
                v.push(t);
            }
        }
        "search" => {
            v.push("search".into());
            v.push(arg_str(args, "pattern").ok_or("missing required 'pattern'")?);
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if arg_bool(args, "ignore_case") {
                v.push("-i".into());
            }
            if let Some(e) = arg_str(args, "ext") {
                v.push("-e".into());
                v.push(e);
            }
            if arg_bool(args, "files_with_matches") {
                v.push("-l".into());
            }
        }
        "stats" => {
            v.push("stats".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if arg_bool(args, "by_type") {
                v.push("-t".into());
            }
            if arg_bool(args, "by_directory") {
                v.push("-d".into());
            }
            if let Some(d) = arg_int(args, "max_depth") {
                v.push("--max-depth".into());
                v.push(d.to_string());
            }
        }
        "dups" => {
            v.push("dups".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if let Some(m) = arg_str(args, "min_size") {
                v.push("-m".into());
                v.push(m);
            }
        }
        "largest" => {
            v.push("largest".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if let Some(c) = arg_int(args, "count") {
                v.push("-n".into());
                v.push(c.to_string());
            }
            if arg_bool(args, "directories") {
                v.push("-d".into());
            }
        }
        "recent" => {
            v.push("recent".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if let Some(c) = arg_int(args, "count") {
                v.push("-n".into());
                v.push(c.to_string());
            }
            if let Some(d) = arg_int(args, "days") {
                v.push("-d".into());
                v.push(d.to_string());
            }
            if let Some(e) = arg_str(args, "ext") {
                v.push("-e".into());
                v.push(e);
            }
        }
        "empty" => {
            v.push("empty".into());
            if let Some(p) = arg_str(args, "path") {
                v.push(p);
            }
            if arg_bool(args, "files") {
                v.push("-f".into());
            }
            if arg_bool(args, "dirs") {
                v.push("-d".into());
            }
        }
        "count" => {
            v.push("count".into());
            let files = args
                .get("files")
                .and_then(Value::as_array)
                .ok_or("missing required 'files' array")?;
            for f in files {
                if let Some(s) = f.as_str() {
                    v.push(s.to_string());
                }
            }
        }
        "checksum" => {
            v.push("checksum".into());
            let files = args
                .get("files")
                .and_then(Value::as_array)
                .ok_or("missing required 'files' array")?;
            for f in files {
                if let Some(s) = f.as_str() {
                    v.push(s.to_string());
                }
            }
            if let Some(a) = arg_str(args, "algorithm") {
                v.push("-a".into());
                v.push(a);
            }
        }
        "info" => {
            v.push("info".into());
            if let Some(f) = arg_str(args, "file") {
                v.push("-f".into());
                v.push(f);
            }
        }
        other => return Err(format!("unknown tool: {}", other)),
    }

    Ok(v)
}

/// Run a tool by re-invoking this executable with `--format json`.
/// Returns `(text, is_error)`.
fn call_tool(name: &str, arguments: &Value) -> (String, bool) {
    let mut cli_args = match build_args(name, arguments) {
        Ok(a) => a,
        Err(e) => return (e, true),
    };
    cli_args.extend([
        "--format".into(),
        "json".into(),
        "--color".into(),
        "never".into(),
    ]);

    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => return (format!("cannot locate tk executable: {}", e), true),
    };

    match Command::new(exe).args(&cli_args).output() {
        Ok(out) => {
            if out.status.success() {
                (String::from_utf8_lossy(&out.stdout).to_string(), false)
            } else {
                let err = String::from_utf8_lossy(&out.stderr).to_string();
                let msg = if err.trim().is_empty() {
                    format!("tk {} exited with status {}", name, out.status)
                } else {
                    err
                };
                (msg, true)
            }
        }
        Err(e) => (format!("failed to run tk {}: {}", name, e), true),
    }
}

fn tools_list_result() -> Value {
    let tools: Vec<Value> = tool_defs()
        .into_iter()
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
                "inputSchema": t.schema,
            })
        })
        .collect();
    json!({ "tools": tools })
}

fn success(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

/// Handle a single decoded request, returning a response to write — or `None`
/// for notifications (messages with no `id`), which take no reply.
fn handle(msg: &Value) -> Option<Value> {
    let method = msg.get("method").and_then(Value::as_str).unwrap_or("");

    // Notifications carry no id and must not be answered.
    let id = msg.get("id").cloned()?;

    match method {
        "initialize" => {
            let protocol = msg
                .get("params")
                .and_then(|p| p.get("protocolVersion"))
                .and_then(Value::as_str)
                .unwrap_or(DEFAULT_PROTOCOL_VERSION);
            Some(success(
                id,
                json!({
                    "protocolVersion": protocol,
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "tk", "version": env!("CARGO_PKG_VERSION") }
                }),
            ))
        }
        "ping" => Some(success(id, json!({}))),
        "tools/list" => Some(success(id, tools_list_result())),
        "tools/call" => {
            let params = msg.get("params").cloned().unwrap_or(Value::Null);
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let arguments = params.get("arguments").cloned().unwrap_or(json!({}));
            if name.is_empty() {
                return Some(error(id, -32602, "missing tool name"));
            }
            let (text, is_error) = call_tool(name, &arguments);
            Some(success(
                id,
                json!({
                    "content": [ { "type": "text", "text": text } ],
                    "isError": is_error
                }),
            ))
        }
        other => Some(error(id, -32601, &format!("method not found: {}", other))),
    }
}

/// Run the MCP server loop over stdin/stdout until EOF.
pub fn run() -> Result<(), String> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        // Strip a leading UTF-8 BOM (U+FEFF isn't whitespace, so `trim` leaves
        // it) that some transports prepend to the first message.
        let trimmed = line.trim_start_matches('\u{feff}').trim();
        if trimmed.is_empty() {
            continue;
        }

        let msg: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                // Parse errors get a null-id JSON-RPC error per the spec.
                let resp = error(Value::Null, -32700, &format!("parse error: {}", e));
                writeln!(out, "{}", resp).map_err(|e| e.to_string())?;
                out.flush().map_err(|e| e.to_string())?;
                continue;
            }
        };

        if let Some(resp) = handle(&msg) {
            writeln!(out, "{}", resp).map_err(|e| e.to_string())?;
            out.flush().map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_echoes_protocol_and_advertises_tools() {
        let req = json!({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": { "protocolVersion": "2025-06-18" }
        });
        let resp = handle(&req).unwrap();
        assert_eq!(resp["result"]["protocolVersion"], "2025-06-18");
        assert_eq!(resp["result"]["serverInfo"]["name"], "tk");
        assert!(resp["result"]["capabilities"]["tools"].is_object());
    }

    #[test]
    fn notification_gets_no_response() {
        let note = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });
        assert!(handle(&note).is_none());
    }

    #[test]
    fn tools_list_includes_curated_set() {
        let req = json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" });
        let resp = handle(&req).unwrap();
        let tools = resp["result"]["tools"].as_array().unwrap();
        let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
        assert!(names.contains(&"search"));
        assert!(names.contains(&"dups"));
        // Side-effecting commands must not be exposed.
        assert!(!names.contains(&"extract"));
        assert!(!names.contains(&"clip"));
    }

    #[test]
    fn unknown_method_is_method_not_found() {
        let req = json!({ "jsonrpc": "2.0", "id": 3, "method": "bogus/method" });
        let resp = handle(&req).unwrap();
        assert_eq!(resp["error"]["code"], -32601);
    }

    #[test]
    fn build_args_maps_search_flags() {
        let args = json!({ "pattern": "TODO", "path": "src", "ignore_case": true });
        let v = build_args("search", &args).unwrap();
        assert_eq!(v, vec!["search", "TODO", "src", "-i"]);
    }

    #[test]
    fn build_args_requires_pattern() {
        let v = build_args("find", &json!({}));
        assert!(v.is_err());
    }

    #[test]
    fn build_args_rejects_unknown_tool() {
        assert!(build_args("rm", &json!({})).is_err());
    }
}
