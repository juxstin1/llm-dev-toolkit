# `checksum` exits successfully for unsupported algorithms

## Summary

`tk checksum -a <unsupported>` prints an error for each file but still exits with status 0.

## Repro

```powershell
cargo run -- checksum -a sha1 Cargo.toml
```

## Actual

The command prints:

```text
Unsupported algorithm: sha1
```

but exits successfully.

## Expected

An unsupported algorithm should return a non-zero exit status, especially because this is an argument/usage error rather than a per-file read failure.

## Likely Cause

`src/commands/checksum.rs` stores per-file `Result`s and, in text mode, prints errors to stderr without converting any failure into a command-level `Err`.

## Suggested Fix

Validate the algorithm once before parallel hashing and return `Err` immediately if it is unsupported. Alternatively, track whether any result failed and return `Err` after printing all per-file errors.

## Test Coverage

Add an integration test that runs `tk checksum -a sha1 Cargo.toml` and asserts:

- non-zero exit status
- stderr contains `Unsupported algorithm: sha1`
