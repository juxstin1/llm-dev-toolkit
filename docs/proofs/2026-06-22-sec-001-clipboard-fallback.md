# Proof: SEC-001 Clipboard Fallback Hardening

Date: 2026-06-22
Scope: Harden file-backed clipboard persistence for `SEC-001`.

## Changes Verified

- `clip -i` no longer writes to the persistent fallback file unless
  `--allow-file-fallback` is supplied.
- Fallback-file writes use `OpenOptions` and set `0600` permissions on Unix.
- Existing fallback reads remain supported when the system clipboard is
  unavailable.
- `clip --help` documents the explicit fallback opt-in.
- README command docs mention the persistent fallback flag.

## Commands

```powershell
cargo fmt --all -- --check
cargo test commands::clip::tests
cargo test test_tk_clip_help_documents_file_fallback_opt_in
cargo test --all
cargo clippy --all-targets -- -D warnings
cargo build --release
target\release\tk.exe clip --help
```

## Result

- Focused clipboard tests passed.
- `cargo test --all` passed with 56 unit tests and 41 integration tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo build --release` passed.
- `target\release\tk.exe clip --help` includes `--allow-file-fallback`.

## Platform Note

The Unix permission assertion is compiled only on Unix. This Windows proof run
verified the fallback opt-in behavior, fallback file round-trip helper, CLI help,
and release build.
