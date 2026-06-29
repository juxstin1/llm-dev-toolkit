---
title: "SPEC-011: MCP File Reading"
status: draft
date: 2026-06-29
scope: "Expose read-only file reading tools in the MCP server"
owners: []
related: ["SPEC-004 (future MCP tool contract)"]
packages: []
---

# SPEC-011: MCP File Reading

> The tk MCP server currently omits file-reading tools (cat, preview, head, tail) as "redundant with agent built-ins." Add a `read_file` tool so MCP clients without their own file reader can still inspect file contents through tk.

## Problem

The MCP server exposes 12 read-only tools — but none of them let an agent actually *read a file*. The README states this is because agents have built-in file reading, but:
- Not all MCP clients do (OpenCode, custom agents)
- Even clients with file reading benefit from tk's syntax highlighting, line ranges, and consistent output format
- It makes tk MCP self-sufficient: one server connection covers both file *inspection* and file *reading*

## Goals

- Add `read_file` tool to MCP: read a file by path with optional line range
- Add `read_lines` tool: read a specific range of lines from a file (for large files)
- Respect `.gitignore` and binary file detection (refuse to output binary to stdout)
- Enforce a max file size (configurable, default 1MB) to prevent OOM
- Line-numbered output by default
- Text mode only — no binary/image reading

## Non-Goals

- No file writing, editing, or deletion (read-only contract stays)
- No directory listing (already covered by `ls` and `tree`)
- No image/binary preview — out of scope for a text-oriented tool
- No syntax highlighting in MCP mode (agent doesn't need colors)

## Proposed Design

### `read_file` MCP tool

```
Input: { path: "src/main.rs", max_size: 1048576 }
Output:
{
  "content": "1: fn main() {\n2:     println!(\"hello\");\n3: }\n",
  "path": "src/main.rs",
  "size": 42,
  "lines": 3,
  "truncated": false,
  "binary": false
}
```

Parameters:
- `path` (required, string): File path to read
- `max_size` (optional, integer): Max bytes to read (default: 1048576 = 1MB)
- `offset` (optional, integer): Line number to start from (1-indexed, default: 1)
- `limit` (optional, integer): Max lines to return (default: all)

### `read_lines` MCP tool

Convenience wrapper for reading specific line ranges without loading the whole file into memory (uses `std::fs::read_to_string` and splits on newlines, capped by `max_size`).

```
Input: { path: "src/main.rs", start_line: 10, end_line: 20 }
```

Returns the same shape as `read_file` but with `start_line`/`end_line` fields.

### Safety

- Binary detection: same `is_binary()` check used by `search`/`count` commands — if file contains null bytes, return error "refusing to read binary file"
- Size limit: refuse if file > `max_size` (before reading)
- Path traversal protection: reject paths containing `..` components (defense-in-depth — the re-invocation pattern already limits scope since the MCP server re-execs `tk` which naturally scopes to the working directory)

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `src/mcp.rs` | Add `read_file` and `read_lines` tool defs + arg builders | MCP exposure |
| `src/commands/view.rs` | Ensure `cat`/`preview` can accept `--format json` (currently text-only) | Reuse via re-invocation |
| `src/mcp.rs` | Add binary check and size-limit to `call_tool` | Safety |
| `tests/mcp_tests.rs` | Add unit tests for `read_file` tool handler | Verify contract |
| `docs/specs/README.md` | Register spec | — |

## Rollout Plan

Single commit. The implementation re-uses the existing MCP re-invocation pattern: `read_file` calls `tk cat --format json <path>`, `read_lines` calls `tk head --format json -n <count> <path>` (or a dedicated range call).

## Acceptance Criteria

- MCP `tools/list` includes `read_file` and `read_lines`
- `read_file("src/main.rs")` returns file content with line numbers
- `read_file("src/main.rs", offset=5, limit=3)` returns lines 5-7
- Binary file returns an error with `isError: true`
- File > 1MB returns an error or truncated result with `truncated: true`
- Path with `..` is rejected

## Verification

```bash
# Send a tools/call request via stdin
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"read_file","arguments":{"path":"Cargo.toml"}}}' | tk mcp
```

## Rollback

Revert the commit. Additive change to MCP only — CLI is unaffected.

## Open Questions

- Should we add `grep_file` MCP tool? Redundant with existing `search` tool — no.
- Should we add `read_json` MCP tool that parses a JSON file and returns structured fields? Interesting idea but out of scope — `tk json` commands already cover this via re-invocation.
- Should we expose `preview` with syntax highlighting? Agent doesn't need colors, so plain `cat` output is sufficient.
