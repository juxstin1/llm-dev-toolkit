# Runbooks

Runbooks hold commands and operational checks for this repo. Recheck the live
checkout before relying on any command output captured in older proof files.

## Standard Rust Verification

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
cargo build --release --no-default-features
cargo package --locked
```

## Autonomous Spec0 Loop

Use [autonomous-loop.md](autonomous-loop.md) for research, ticket drafting,
implementation, verification, proof capture, subagent contracts, and handoff.

## Releases

Use [release.md](release.md) to validate a version, create its tag, publish
platform archives and checksums, and publish the `llm-dev-toolkit` crate.

## Quick CLI Smoke Checks

```powershell
cargo run -- --help
cargo run -- --format json stats src -t
cargo run -- --format json tree src -L 1
target\release\tk.exe --version
```

## GitHub Issue Creation

Confirm GitHub CLI authentication before creating or changing live issues:

```powershell
gh auth status
```

Then use the commands in [tickets/INDEX.md](../tickets/INDEX.md).

## Demo Project

The demo is a Remotion project under `demo/`.

```powershell
cd demo
npm ci
npm run check
npm run studio
npm run render
```

The generated output path is ignored by `demo/.gitignore`.
