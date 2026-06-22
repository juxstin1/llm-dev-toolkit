# Codebase Map

Snapshot date: 2026-06-22.

This map describes the current checkout. Generated and dependency folders such
as `target/`, `demo/node_modules/`, and `demo/out/` should be excluded from
normal search and review.

## Product Surface

| Surface | Entry Point | Purpose |
| --- | --- | --- |
| Rust CLI | `src/main.rs` | Defines the `tk` command, global flags, subcommands, and dispatch. |
| Command modules | `src/commands/*.rs` | Implements file listing, search, stats, checksums, archive extraction, JSON helpers, and clipboard support. |
| MCP server | `src/mcp.rs` | Exposes a curated read-only tool set over newline-delimited JSON-RPC on stdio. |
| Remotion demo | `demo/src/index.ts` and `demo/src/Root.tsx` | Renders the marketing/demo terminal animation. |
| CI | `.github/workflows/ci.yml` | Runs format, clippy, tests, and release build across Linux, macOS, and Windows. |

## Rust Layout

| Path | Ownership |
| --- | --- |
| `Cargo.toml` | Package metadata and Rust dependencies. |
| `src/main.rs` | Clap parser, command enum, broken pipe guard, command dispatch. |
| `src/mcp.rs` | MCP tool definitions, argument translation, JSON-RPC handling, subprocess invocation. |
| `src/commands/mod.rs` | Shared output format, color handling, ignore-aware walker, hashing, binary detection. |
| `src/commands/ls.rs` | `ls`, `la`, and `ll`. |
| `src/commands/tree.rs` | `tree` and `ltd`. |
| `src/commands/find.rs` | `ff`, `fd`, `ff-ext`, and `ff-name`. |
| `src/commands/search.rs` | `search` and `grep` alias. |
| `src/commands/stats.rs` | Tree statistics and breakdowns. |
| `src/commands/dups.rs` | Duplicate detection and interactive delete flow. |
| `src/commands/checksum.rs` | Parallel file checksums. |
| `src/commands/extract.rs` | Zip, tar, tar.gz, tgz, and gz extraction. |
| `src/commands/view.rs` | `cat`, `preview`, `head`, and `tail`. |
| `src/commands/json.rs` | JSON format, validate, and keys. |
| `src/commands/info.rs` | File and system information. |
| `src/commands/clip.rs` | Clipboard read/write with file fallback. |
| `src/commands/largest.rs` | Largest files or directories. |
| `src/commands/recent.rs` | Recently modified files. |
| `src/commands/empty.rs` | Empty file and directory detection. |
| `src/commands/sort.rs` | One-directory sorting. |
| `src/commands/count.rs` | wc-style counts. |

## Tests And Verification

| Path or Command | Purpose |
| --- | --- |
| `tests/cli.rs` | End-to-end CLI and MCP integration tests. |
| `#[cfg(test)]` modules under `src/` | Unit tests for parsing, highlighting, hashing, duplicate checks, and archive safety. |
| `cargo fmt --all -- --check` | Formatting gate. |
| `cargo clippy --all-targets -- -D warnings` | Lint gate. |
| `cargo test --all` | Unit and integration test gate. |
| `cargo build --release` | Packaging/build gate. |

## Current Planning Artifacts

| Path | Role |
| --- | --- |
| `docs/specs/SPEC-000-planning-system.md` | Contract for this planning layer. |
| `docs/plans/spec-development-roadmap.md` | First roadmap for specs and bug fixes. |
| `docs/tickets/INDEX.md` | Active queue index. |
| `tickets/*.md` | GitHub issue body drafts created before GitHub auth was fixed. |

## Known Current Bug Queue

- `tree -a` includes `.git` despite the git-aware contract.
- `stats -d` overcounts files for directories.
- `info -f` likely misclassifies symlinks.
- `checksum` exits successfully for unsupported algorithms.
- Invalid enum-like CLI option values are accepted silently.
- `search -e .rs` does not match files while `search -e rs` does.
- `ltd -L` shows hidden entries by default unlike `tree -L`.

See [tickets/INDEX.md](tickets/INDEX.md) for the current queue.
