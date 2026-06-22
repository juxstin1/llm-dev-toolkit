# Proof: Ticket Runover

Date: 2026-06-22
Scope: Fresh repo runover and ticket creation for verified CLI behavior gaps.

## Commands

```powershell
git status --short --branch
gh auth status
target\release\tk.exe --format json ff Cargo -t x
target\release\tk.exe --format json sort . --by nope
target\release\tk.exe --format json search "fn main" src -e rs
target\release\tk.exe --format json search "fn main" src -e .rs
target\release\tk.exe tree -L 1
target\release\tk.exe ltd -L 1
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```

## Result

`gh auth status` still reports an invalid token for the `juxstin1` GitHub
account, so live GitHub issues could not be created from this environment.

Verified behavior gaps:

- `ff Cargo -t x` succeeds and returns Cargo files instead of rejecting `x`.
- `sort . --by nope` succeeds and emits name-sorted entries instead of
  rejecting `nope`.
- `search "fn main" src -e rs` finds `src\main.rs`.
- `search "fn main" src -e .rs` returns an empty JSON array.
- `tree -L 1` hides hidden entries.
- `ltd -L 1` shows hidden entries, including `.git`.

Standard Rust checks passed:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all` with 54 unit tests and 28 integration tests passing.

## Tickets Created

- `TK-BUG-005`: Validate enum-like CLI options.
- `TK-BUG-006`: Normalize leading-dot extension filters for `search`.
- `TK-BUG-007`: Make `ltd -L` match `tree -L` hidden-file behavior.

## Gaps

No product code was changed. Archive extraction safety was not re-tested beyond
the existing unit test suite.
