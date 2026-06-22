# `stats -d` overcounts files for directories

## Summary

The per-directory breakdown in `tk stats -d` increments the file count when visiting directory entries.

## Repro

```powershell
cargo run -- --format json stats . -d --max-depth 2
```

## Actual

Some `by_directory` entries report file counts that include child directories.

Example from the current repo:

```json
{
  "directory": ".",
  "files": 11,
  "dirs": 5
}
```

The root only has 6 files at depth 1, but the file count is inflated by the 5 directories.

## Expected

The `files` field should count files only. The `dirs` field should count directories only.

## Likely Cause

In `src/commands/stats.rs`, the directory branch updates `by_dir` with:

```rust
.and_modify(|(f, d, _)| {
    *f += 1;
    *d += 1;
})
```

The `*f += 1` appears incorrect for directory entries.

## Suggested Fix

Remove the file increment from the directory-entry branch.

## Test Coverage

Add a focused integration or unit test with a directory containing known file and directory counts, then assert `stats -d --format json` reports separate file and directory totals correctly.
