# tk — LLM Dev Toolkit

A fast, single-binary command-line toolkit for the everyday file chores that come up while working in a codebase: listing, finding, searching, hashing, inspecting, and tidying. Think of it as a handful of `ls`/`find`/`grep`/`tree`/`wc`-style tools bundled into one `tk` binary, with sensible defaults for development work — most commands respect `.gitignore` and skip the `.git` directory automatically.

[![CI](https://github.com/juxstin1/llm-dev-toolkit/actions/workflows/ci.yml/badge.svg)](https://github.com/juxstin1/llm-dev-toolkit/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

<p align="center">
  <img src="media/tk-demo.gif" alt="tk in action — ll, tree, search, stats, and largest" width="820">
</p>

## Highlights

- **Git-aware by default** — file walks honor `.gitignore`, `.git/info/exclude`, and your global gitignore, and never descend into `.git/`.
- **Single static binary** — one `tk` executable, no runtime dependencies.
- **Machine-readable** — every command takes `--format json` for raw, parseable output (numeric sizes, unix timestamps, no ANSI), and `tk mcp` runs as an MCP server so LLM agents can call the tools directly.
- **Parallel where it counts** — duplicate detection and checksums fan out across CPU cores (`rayon`), with an optional `--threads` cap.
- **Pipe-friendly** — honors `--color=auto|always|never` and `NO_COLOR`, and exits cleanly on a broken pipe (`tk tree | head` won't panic).
- **Cross-platform** — Linux, macOS, and Windows.

## Install

Requires a [Rust toolchain](https://rustup.rs/) (stable).

```bash
# From a local clone
git clone https://github.com/juxstin1/llm-dev-toolkit.git
cd llm-dev-toolkit
cargo install --path .

# Or build without installing
cargo build --release   # binary at target/release/tk
```

## Usage

```
tk <command> [options]
tk --help            # list all commands
tk <command> --help  # help for a specific command
```

Two global flags apply to every command:

- `--color <auto|always|never>` controls ANSI styling (default `auto`; also respects the `NO_COLOR` environment variable).
- `--format <text|json>` controls output (default `text`). `json` emits a parseable structure — raw byte sizes as numbers, modification times as unix seconds, and no ANSI — for piping into `jq`, scripts, or an LLM tool. Color is automatically disabled under `--format json`.

### Commands

| Command | Aliases | Description |
|---|---|---|
| `ls` | `l` | List directory contents (`-a` hidden, `-l` long format) |
| `la` | | Shortcut for `ls -a` |
| `ll` | | Shortcut for `ls -al` |
| `tree` | `lt` | Directory tree (`-L` depth, `-a` hidden, `-d` dirs only) |
| `ltd` | | Tree with a depth limit (`-L`) |
| `ff` | `fd`, `find` | Find files by name substring (`-i`, `-e <ext>`, `-t f\|d`) |
| `ff-ext` | | Find files by extension |
| `ff-name` | | Find by name substring or glob (`-g`) |
| `search` | `grep` | Search file contents (`-i`, `--line-number`, `-C <n>`, `-l`, `-e <ext>`) |
| `cat` | | Concatenate and print files (`-n` line numbers) |
| `preview` | | Syntax-highlighted file preview (`-l <lang>`, `-n`) |
| `head` / `tail` | | First / last N lines (`-n <count>`) |
| `count` | | Count lines, words, chars, bytes (`-l -w -c -b`, `wc`-compatible) |
| `stats` | | File/dir/byte statistics (`-d` per-dir, `-t` per-extension) |
| `dups` | | Find duplicate files by SHA-256 (`-m <min-size>`, `-d` delete, `--threads`) |
| `recent` | | Recently modified files (`-n`, `-d <days>`, `-e <ext>`) |
| `largest` | | Largest files or directories (`-n`, `-d` dirs) |
| `empty` | | Find empty files (`-f`) and directories (`-d`) |
| `sort` | | Sort directory entries by name/size/date/ext (`-b`, `-r`, `-n`, `-d`) |
| `checksum` | | File checksums — sha256/224/384/512, md5 (`-a`, `--threads`) |
| `extract` | | Extract `.zip`, `.tar`, `.tar.gz`/`.tgz`, `.gz` (`-o <dir>`) |
| `json` | | `format` / `validate` / `keys` for JSON (file or stdin) |
| `clip` | | Read/write the system clipboard (`-i` in, `-o` out; `--allow-file-fallback` permits persistent fallback storage when the system clipboard is unavailable) |
| `info` | | File details (`-f <path>`) or a system overview |
| `mcp` | | Run as an MCP server over stdio (read-only tools for LLM agents) |

### Examples

```bash
tk ll                          # long listing, including hidden entries
tk tree -L 2                   # tree, two levels deep
tk ff config -e toml              # files whose name contains "config" with a .toml extension
tk search "TODO" -i --line-number # case-insensitive content search with line numbers
tk grep "fn main" -C 2            # matches with 2 lines of surrounding context
tk dups -m 1mib                # duplicate files at least 1 MiB, by content hash
tk largest -n 10               # ten biggest files under the current tree
tk recent -d 1                 # files modified in the last day
tk count -l src/**/*.rs        # line counts (matches `wc -l`)
tk checksum -a sha512 file.iso # SHA-512 of a file
cat data.json | tk json format # pretty-print JSON from stdin
tk stats --format json -t      # per-extension stats as JSON
tk tree -L 2 --format json | jq '.children[].name'
```

## For LLM agents

`tk` is built to be driven by an LLM as well as a human.

**Structured output.** Append `--format json` to any command and it returns a parseable structure instead of formatted text — raw numeric sizes, unix timestamps, and no ANSI escapes — so a model (or `jq`) never has to parse prose:

```bash
$ tk largest -n 2 --format json
[
  { "path": "media/tk-demo.gif", "size": 1048576 },
  { "path": "Cargo.lock", "size": 41231 }
]
```

**MCP server.** `tk mcp` speaks the [Model Context Protocol](https://modelcontextprotocol.io) over stdio, exposing a curated set of **read-only** tools (`ls`, `tree`, `find`, `search`, `stats`, `dups`, `largest`, `recent`, `empty`, `count`, `checksum`, `info`). Side-effecting commands (`extract`, `clip`, `dups --delete`) and ones redundant with an agent's built-in file reading (`cat`, `head`, `preview`) are deliberately left out, so a connected model can inspect a tree but never mutate it.

Point any MCP client at the binary. For example, in a Claude Code / Cursor MCP config:

```jsonc
{
  "mcpServers": {
    "tk": { "command": "tk", "args": ["mcp"] }
  }
}
```

The transport is newline-delimited JSON-RPC 2.0 — no async runtime, no sidecar process. Each tool call re-invokes `tk … --format json` internally, so MCP results are identical to the CLI's JSON output.

## Development

```bash
cargo build            # debug build
cargo test             # unit + integration tests
cargo clippy --all-targets
cargo fmt
```

Tests live alongside each command module (`#[cfg(test)]`) and as end-to-end CLI tests in [`tests/cli.rs`](tests/cli.rs).

The demo GIF at the top is a [Remotion](https://remotion.dev) project under [`demo/`](demo/) — `cd demo && npm install && npm run studio` to edit it, or `npm run render` to re-render.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
