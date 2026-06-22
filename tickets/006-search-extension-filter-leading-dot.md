# `search -e .rs` returns no matches while `search -e rs` works

## Summary

`tk search -e` does not trim a leading dot from the requested extension. This
is inconsistent with commands like `ff-ext` and `recent`, which accept either
`rs` or `.rs`.

## Repro

```powershell
target\release\tk.exe --format json search "fn main" src -e rs
target\release\tk.exe --format json search "fn main" src -e .rs
```

## Actual

`-e rs` finds `src\main.rs`.

`-e .rs` returns an empty JSON array.

## Expected

Both `-e rs` and `-e .rs` should match Rust files, or the CLI should reject
leading-dot extensions consistently across commands. Accepting both is the more
user-friendly behavior and matches existing command behavior elsewhere.

## Likely Cause

`src/commands/search.rs` compares `file_path.extension()` directly to `args.ext`
with `eq_ignore_ascii_case`, but does not call `trim_start_matches('.')`.

## Suggested Fix

Normalize `args.ext` once in `search` text and JSON paths before comparing.

## Test Coverage

Add integration tests for `search -e rs` and `search -e .rs` returning the same
matches in text and JSON modes.
