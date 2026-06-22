# Proof: Build Order Slices 1-7

Date: 2026-06-22
Scope: Implement and verify the first seven build-order slices from
`tickets/_build_order.md`.

## Tickets Covered

- `ARCH-001`, `TK-BUG-001`, `TK-BUG-007`: tree traversal and `ltd` hidden-entry behavior.
- `REL-001`, `TK-BUG-002`: `stats -d` directory/file counts.
- `REL-004`, `TK-BUG-006`: leading-dot `search -e` filters.
- `REL-002`, `TK-BUG-004`: unsupported checksum algorithms return non-zero.
- `REL-003`, `TK-BUG-005`: invalid `ff -t` and `sort --by` values fail.
- `REL-005`: invalid `dups -m` size filters fail.
- `REL-006`, `TK-BUG-003`: `info -f` classifies symlink paths.

## Changes Verified

- `tree -a` and JSON tree output still exclude `.git`.
- `ltd -L` no longer shows hidden dot entries by default.
- `stats -d --format json` counts files and directories separately.
- `search -e .rs` matches `search -e rs` in text and JSON modes.
- `checksum -a sha1` exits non-zero; MCP `checksum` reports `isError`.
- `ff -t x`, `sort --by nope`, and `dups -m nonsense` fail with helpful errors.
- `info -f --format json` reports symlink paths as `type: "symlink"` when the platform allows symlink creation.

## Commands

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
target\release\tk.exe --version
```

## Result

- `cargo fmt --all -- --check` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo test --all` passed with 54 unit tests and 40 integration tests.
- `cargo build --release` passed.
- `target\release\tk.exe --version` printed `tk 0.1.0`.

## Remaining Build Order

Slices 8-10 remain open:

- `SEC-001`: harden file-backed clipboard fallback.
- `MAINT-001`: add demo project CI coverage.
- `ARCH-002`: unify MCP tool schema and argument builders.
