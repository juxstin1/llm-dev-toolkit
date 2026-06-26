# tk - LLM Dev Toolkit

`tk` is a fast Rust CLI for the file-inspection chores that come up while
working in a codebase: listing, finding, searching, hashing, inspecting,
counting, and summarizing project trees.

It is built for both humans and agents. Human output is readable by default;
`--format json` gives scripts and LLM tools stable structured output; `tk mcp`
exposes a read-only Model Context Protocol server; and `tk spec0` installs a
small agent workflow for disciplined plan/execute/verify loops.

[![CI](https://github.com/juxstin1/llm-dev-toolkit/actions/workflows/ci.yml/badge.svg)](https://github.com/juxstin1/llm-dev-toolkit/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)

<p align="center">
  <img src="media/tk-demo.gif" alt="tk in action: ll, tree, search, stats, and largest" width="820">
</p>

## Why Use It

- **One binary for common repo inspection**: familiar `ls`, `find`, `grep`,
  `tree`, `wc`, checksum, duplicate, JSON, and archive workflows in one command.
- **Git-aware traversal**: recursive commands honor ignore files and never walk
  into `.git/`.
- **Agent-native output**: every command supports `--format json` with raw
  numeric fields and no ANSI escape codes.
- **Read-only MCP server**: agents can inspect a project tree through curated
  filesystem tools without mutation access.
- **Spec0 workflow installer**: ship reusable `/spec0` prompts to Claude Code,
  Codex, and OpenCode command surfaces.
- **Cross-platform CI**: Linux, macOS, and Windows are checked on every pull
  request.

## Install

`tk` currently builds from source. Install a stable Rust toolchain first:
<https://rustup.rs/>.

```bash
git clone https://github.com/juxstin1/llm-dev-toolkit.git
cd llm-dev-toolkit
cargo install --path .
```

Or build without installing:

```bash
cargo build --release
./target/release/tk --help
```

On Windows, run:

```powershell
cargo build --release
.\target\release\tk.exe --help
```

## Quick Start

```bash
tk ll
tk tree -L 2
tk ff config -e toml
tk search "TODO" src --line-number
tk stats --format json -t
tk largest -n 10
tk checksum -a sha512 file.iso
tk spec0 list
```

Global flags:

- `--color <auto|always|never>` controls ANSI color output. `NO_COLOR` is
  respected.
- `--format <text|json>` chooses human output or stable machine-readable JSON.

## Command Map

| Command | Aliases | Purpose |
| --- | --- | --- |
| `ls` | `l` | List directory contents. |
| `la` | | Shortcut for `ls -a`. |
| `ll` | | Shortcut for `ls -al`. |
| `tree` | `lt` | Show a directory tree. |
| `ltd` | | Show a tree with a required depth limit. |
| `ff` | `fd`, `find` | Find files or directories by name substring. |
| `ff-ext` | | Find files by extension. |
| `ff-name` | | Find names by substring or glob. |
| `search` | `grep` | Search file contents. |
| `cat` | | Print files. |
| `preview` | | Syntax-highlighted preview. |
| `head` / `tail` | | Show first or last lines. |
| `count` | | Count lines, words, chars, and bytes. |
| `stats` | | Summarize file, directory, and byte counts. |
| `dups` | | Find duplicate files by SHA-256. |
| `recent` | | List recently modified files. |
| `largest` | | List largest files or directories. |
| `empty` | | Find empty files and directories. |
| `sort` | | Sort one directory by name, size, date, or extension. |
| `checksum` | | Compute SHA and MD5 checksums. |
| `extract` | | Extract `.zip`, `.tar`, `.tar.gz`, `.tgz`, and `.gz` archives. |
| `json` | | Format, validate, or inspect JSON. |
| `clip` | | Read or write the system clipboard. |
| `info` | | Show file details or a system overview. |
| `spec0` | | List, print, or install Spec0 agent workflow commands. |
| `mcp` | | Run the read-only MCP server over stdio. |

## Machine-Readable Output

Append `--format json` to get parseable output:

```bash
tk largest -n 2 --format json
```

Example shape:

```json
[
  { "path": "media/tk-demo.gif", "size": 5669205 },
  { "path": "Cargo.lock", "size": 43850 }
]
```

Runtime errors in JSON mode keep stdout empty and emit a JSON object on stderr:

```json
{ "error": "Unsupported algorithm: sha1" }
```

## MCP Server

`tk mcp` speaks newline-delimited JSON-RPC 2.0 over stdio and exposes a curated
read-only tool set:

`ls`, `tree`, `find`, `search`, `stats`, `dups`, `largest`, `recent`, `empty`,
`count`, `checksum`, and `info`.

Side-effecting commands such as `extract`, `clip`, and `dups --delete` are not
exposed through MCP.

Example MCP config:

```jsonc
{
  "mcpServers": {
    "tk": {
      "command": "tk",
      "args": ["mcp"]
    }
  }
}
```

## Spec0 Agent Workflow

Spec0 is the local agent workflow bundled with this repository. It keeps agentic
development grounded in a small loop:

1. Orient around repo state, branch, dirty files, and instructions.
2. Frame the goal, non-goals, assumptions, target files, and verification.
3. Plan small implementation slices.
4. Execute the next useful slice.
5. Verify with the narrowest meaningful checks first.
6. Handoff changed files, checks, risks, and next actions.

Install prompts for supported agents:

```bash
tk spec0 list
tk spec0 install --agent all --scope user --dry-run
tk spec0 install --agent all --scope user
tk spec0 install --agent all --scope project --dir .
```

Install targets:

| Agent | User scope | Project scope |
| --- | --- | --- |
| Claude Code | `~/.claude/commands/*.md` | `.claude/commands/*.md` |
| OpenCode | `~/.config/opencode/commands/*.md` | `.opencode/commands/*.md` |
| Codex | `~/.agents/skills/spec0/SKILL.md` plus `~/.codex/prompts/*.md` | `.agents/skills/spec0/SKILL.md` |

See [spec0/README.md](spec0/README.md) and
[docs/runbooks/autonomous-loop.md](docs/runbooks/autonomous-loop.md) for the
repo-local development loop.

## Development

Required tools:

- Stable Rust toolchain.
- Node.js 22 for the Remotion demo check.

Standard verification:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

Demo verification:

```bash
cd demo
npm ci
npm run check
```

The demo GIF is a [Remotion](https://remotion.dev) project under
[demo/](demo/). Use `npm run studio` to edit it and `npm run render` to produce
the MP4 output.

## Project Docs

- [Spec0 front door](docs/README.md)
- [Codebase map](docs/CODEBASE_MAP.md)
- [Specs](docs/specs/README.md)
- [Ticket queue](docs/tickets/INDEX.md)
- [Runbooks](docs/runbooks/README.md)
- [Proof logs](docs/proofs/README.md)

## Contributing

Contributions are welcome when they are scoped, tested, and aligned with the
existing command contracts. Start with [CONTRIBUTING.md](CONTRIBUTING.md) and
open an issue for behavior changes before writing a broad PR.

## Security

Do not disclose vulnerabilities in public issues with exploit details. See
[SECURITY.md](SECURITY.md) for the reporting path.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
