# Proof: Version 0.5 Roadmap Completion

Date: 2026-07-29
Scope: CI compatibility, recursive-root reliability, layered configuration,
CLI ergonomics, MCP validation, `show`, `scan`, completions, dependency audit,
and release packaging.
Source specs: `SPEC-001`, `SPEC-003`, `SPEC-004`, and `SPEC-012`.
Git state: `codex/finish-roadmap`; changes uncommitted at proof capture.

## Implemented Artifacts

- Rust 1.97 Clippy compatibility in duplicate-group iteration.
- Checked recursive roots shared by find, search, stats, duplicate, recent,
  largest, empty, symbol, and context commands.
- Typed per-command TOML defaults with CLI precedence.
- Visible aliases, fast flags, natural `info PATH`, explicit disk-usage scan,
  and root-help recipes.
- Closed MCP schemas, strict value validation, locked default 23-tool inventory,
  and 21-tool inventory when built without network features.
- `show FILE:LINE`, `scan`, and generated PowerShell/Bash/Zsh/Fish completions.
- `llm-dev-toolkit` crates.io package with a `tk` binary.
- Tagged release workflow for Linux x86-64, Windows x86-64, and macOS ARM64
  archives, SHA-256 files, GitHub Release creation, and crates.io publication.
- Demo lockfile updates for `fast-uri` and `postcss` advisories.

## Verification

```powershell
cargo +1.97.0 fmt --all -- --check
cargo +1.97.0 clippy --all-targets -- -D warnings
cargo +1.97.0 test --all
cargo +1.97.0 clippy --all-targets --no-default-features -- -D warnings
cargo +1.97.0 test --all --no-default-features
cargo +1.97.0 build --release --locked
cargo +1.97.0 build --release --locked --no-default-features
cargo +1.97.0 package --locked --allow-dirty
npm.cmd ci
npm.cmd run check
npm.cmd audit --audit-level=high
yq eval '.' .github\workflows\ci.yml
yq eval '.' .github\workflows\release.yml
git diff --check
```

Results:

- Default Rust suite: 64 unit tests plus 70 integration tests passed.
- Network-disabled suite: 64 unit tests plus 69 integration tests passed.
- Strict Clippy passed for both feature sets on Rust 1.97.0.
- Locked release builds passed with and without the `net` feature.
- `cargo package` verified `llm-dev-toolkit 0.5.0`; the root-anchored manifest
  contains 46 files and the compressed crate size is 86.3 KiB.
- Demo TypeScript passed from a clean `npm ci`; npm reported zero known
  vulnerabilities at high severity or above.
- Both GitHub Actions files parsed as YAML.
- No whitespace errors were present.

## Decision Gates

- The crates.io name `tk` is owned by an unrelated Tk GUI bindings project.
  Package name `llm-dev-toolkit` was selected; executable name remains `tk`.
- Config was completed rather than removing its feature and command sections.
- Human aliases and new aggregate commands remain outside MCP, preserving the
  canonical tool inventory.
- Bare `info` no longer recursively sizes the current directory; users opt in
  with `--disk-usage` or `[commands.info] disk-usage = true`.

## External Setup Required

Before the first tag, the repository maintainer must create the GitHub
environment `crates-io` and add `CRATES_IO_TOKEN`. The release workflow itself
cannot be executed locally because GitHub-hosted runners, release permissions,
and that secret are external state.

## Handoff

Implementation status: done. No product-code blocker remains. Next action is
review, commit, push, and open a pull request; after merge, configure the
crates.io environment and tag `v0.5.0` using the release runbook.
