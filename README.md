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
- **Parallel where it counts** — duplicate detection and checksums fan out across CPU cores (`rayon`), with an optional `--threads` cap.
- **Pipe-friendly** — honors `--color=auto|always|never` and `NO_COLOR`, and exits cleanly on a broken pipe (`tk tree | head` won't panic).
- **Agent workflow toolkit** — bundles Spec0 slash-command prompts for Claude Code, Codex, and OpenCode.
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

A global `--color <auto|always|never>` flag controls ANSI styling (default `auto`; also respects the `NO_COLOR` environment variable).

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
| `clip` | | Read/write the system clipboard (`-i` in, `-o` out) |
| `info` | | File details (`-f <path>`) or a system overview |
| `spec0` | | List, print, or install Spec0 agent workflow commands |

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
tk spec0 list                  # show bundled Spec0 commands and install targets
tk spec0 install --agent all --scope user # install Spec0 into local agent config dirs
```

## Spec0 Agent Commands

Spec0 is a small agent workflow framework for moving from rough intent to a bounded, verifiable implementation loop:

1. Orient around the repo, branch, dirty files, and local agent instructions.
2. Frame the goal, non-goals, assumptions, target files, and verification.
3. Plan small implementation slices.
4. Execute the next useful slice.
5. Verify with the narrowest meaningful checks.
6. Leave a compact handoff.

The toolkit bundles Spec0 as reusable prompts under [`spec0/`](spec0/) and can install them into the local command surfaces used by Claude Code, Codex, and OpenCode.

### Installed commands

| Command | Purpose |
|---|---|
| `/spec0` | Run the full orient, frame, plan, execute, verify, handoff loop |
| `/spec0-plan` | Convert a rough request into a bounded implementation plan |
| `/spec0-exec` | Execute the next planned slice and verify it |
| `/spec0-review` | Review current work against the Spec0 frame |
| `/spec0-handoff` | Summarize status for the next agent session |

### Install targets

`tk spec0 install` writes the same workflow prompts to the right location for each agent:

| Agent | User scope | Project scope | Invoke |
|---|---|---|---|
| Claude Code | `~/.claude/commands/*.md` | `.claude/commands/*.md` | `/spec0` |
| OpenCode | `~/.config/opencode/commands/*.md` | `.opencode/commands/*.md` | `/spec0` |
| Codex | `~/.agents/skills/spec0/SKILL.md` plus `~/.codex/prompts/*.md` | `.agents/skills/spec0/SKILL.md` | `$spec0` skill or `/prompts:spec0` shim |

Codex custom prompts are deprecated in favor of Skills, so the durable Codex path is the installed `spec0` skill. The prompt shims are included for CLI/IDE slash-menu convenience.

### Install examples

```bash
tk spec0 list
tk spec0 print spec0-plan
tk spec0 install --agent all --scope user --dry-run
tk spec0 install --agent all --scope user
tk spec0 install --agent all --scope project --dir .
tk spec0 install --agent codex --scope project --dir .
tk spec0 install --agent claude --scope user --force
```

Existing files are skipped by default; pass `--force` when you intentionally want to overwrite them.

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
