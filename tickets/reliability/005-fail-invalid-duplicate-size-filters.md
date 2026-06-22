---
id: REL-005
title: fail invalid duplicate size filters
lens: reliability
severity: S2
confidence: high
effort: S
blast_radius: 1 module, 1 command
files:
  - src/main.rs:211-212
  - src/commands/dups.rs:24
  - src/commands/dups.rs:172-174
depends_on: []
tags: [dups, cli-validation]
status: done
---

## What

Invalid `dups -m` values are treated as if the user supplied no valid minimum
size. The parser returns `None`, and command execution falls back to `1` byte.

## Why it matters

A typo in a duplicate scan can expand the candidate set dramatically. With
`--delete`, the command still prompts, but the prompt is based on a much broader
scan than requested.

## Evidence

- `src/main.rs:211-212` - the help documents `-m` as a minimum file size input.
- `src/commands/dups.rs:24` - `parse_size()` returns `Option<u64>` with no error detail.
- `src/commands/dups.rs:172-174` - parse failure falls through `unwrap_or(1)`.

## Suggested direction

Make size parsing return `Result<u64, String>` and surface invalid values as a
command error.

## Open questions

Should decimal sizes such as `1.5mb` be accepted or rejected with a clear error?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
