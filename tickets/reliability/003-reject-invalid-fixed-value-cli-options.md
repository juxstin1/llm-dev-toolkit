---
id: REL-003
title: reject invalid fixed-value CLI options
lens: reliability
severity: S2
confidence: high
effort: S
blast_radius: 2 modules, 2 commands
files:
  - src/main.rs:126
  - src/commands/find.rs:42-46
  - src/commands/sort.rs:95-100
depends_on: []
tags: [cli-validation]
status: done
---

## What

Fixed-value options are modeled as strings and invalid values silently fall back
to broad behavior. `ff -t x` behaves like no type filter, and `sort --by nope`
falls back to name sorting.

## Why it matters

Mistyped automation can return plausible but wrong results without any failure
signal.

## Evidence

- `src/main.rs:126` - `FfArgs.type_filter` is an `Option<String>`, not an enum.
- `src/commands/find.rs:42-46` - unknown `-t` values fall through `_ => {}` and apply no filter.
- `src/commands/sort.rs:95-100` - unknown `--by` values fall through to name sorting.

## Suggested direction

Use clap `ValueEnum` types or explicit validation for fixed-value flags before
running command logic.

## Open questions

Should invalid values be clap parse errors, or command-level errors formatted by
the existing global handler?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
