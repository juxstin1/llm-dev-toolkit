# Validate enum-like CLI options instead of silently accepting invalid values

## Summary

Some CLI options describe a fixed set of valid values, but invalid values are
accepted and then silently ignored or treated as a default.

## Repro

```powershell
target\release\tk.exe --format json ff Cargo -t x
target\release\tk.exe --format json sort . --by nope
```

## Actual

`ff -t x` succeeds and returns both `Cargo.lock` and `Cargo.toml`, as if no type
filter was provided.

`sort --by nope` succeeds and appears to fall back to sorting by name.

## Expected

Invalid enum-like option values should fail fast with a non-zero exit and a
clear error message.

Expected valid values:

- `ff -t`: `f` or `d`
- `sort --by`: `name`, `size`, `date`, or `ext`

## Likely Cause

`FfArgs.type_filter` and `SortArgs.by` are plain `String` values in
`src/main.rs`. `src/commands/find.rs` ignores unknown type filters, and
`src/commands/sort.rs` uses the default name sort branch for unknown sort keys.

## Suggested Fix

Use clap `ValueEnum` types or explicit validation before command execution.
Apply the same validation pattern consistently with checksum algorithm handling
once that ticket is fixed.

## Test Coverage

Add integration tests that assert non-zero exit status and helpful stderr for:

- `tk ff Cargo -t x`
- `tk sort . --by nope`
