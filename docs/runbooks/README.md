# Runbooks

Runbooks hold commands and operational checks for this repo. Recheck the live
checkout before relying on any command output captured in older proof files.

## Standard Rust Verification

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

## Quick CLI Smoke Checks

```powershell
cargo run -- --help
cargo run -- --format json stats src -t
cargo run -- --format json tree src -L 1
target\release\tk.exe --version
```

## GitHub Issue Creation

GitHub CLI is installed, but `gh auth status` reported an invalid token during
the setup pass. Repair auth before creating live issues:

```powershell
gh auth login -h github.com
gh auth status
```

Then use the commands in [tickets/INDEX.md](../tickets/INDEX.md).

## Demo Project

The demo is a Remotion project under `demo/`.

```powershell
cd demo
npm install
npm run studio
npm run render
```

The generated output path is ignored by `demo/.gitignore`.
