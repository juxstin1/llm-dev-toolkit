# `info -f` likely misclassifies symlinks

## Summary

`tk info -f <path>` uses `fs::metadata`, which follows symlinks. The code then checks `meta.is_symlink()`, but metadata for the target generally will not identify the original path as a symlink.

## Repro

On a platform where symlink creation is available:

```powershell
New-Item -ItemType SymbolicLink -Path link-to-readme.md -Target README.md
cargo run -- --format json info -f link-to-readme.md
```

## Actual

The symlink is expected to be reported as a regular file because `fs::metadata` follows the link.

## Expected

The symlink path should be reported as `"type": "symlink"` or equivalent, consistent with `tk ls` behavior.

## Likely Cause

`src/commands/info.rs` calls `fs::metadata(path)` in `file_info`, while `src/commands/ls.rs` correctly uses `fs::symlink_metadata` for symlink-aware classification.

## Suggested Fix

Use `fs::symlink_metadata` for path classification. If target metadata is also useful, include it separately rather than using followed metadata as the source of truth for the path itself.

## Test Coverage

Add a symlink test where supported, with a platform-aware skip/fallback for environments that cannot create symlinks.
