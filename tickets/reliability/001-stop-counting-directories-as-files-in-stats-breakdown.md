---
id: REL-001
title: stop counting directories as files in stats breakdown
lens: reliability
severity: S2
confidence: high
effort: S
blast_radius: 1 module, 1 command
files:
  - src/commands/stats.rs:55-68
  - src/commands/stats.rs:89-100
depends_on: []
tags: [stats, json-output]
status: done
---

## What

`stats -d` increments the per-directory file count while processing directory
entries. The file branch also increments file counts, so directory rows can
report inflated `files` values.

## Why it matters

The JSON and text breakdowns become misleading for any user or MCP consumer
using `stats -d` to reason about repository shape.

## Evidence

- `src/commands/stats.rs:55-68` - the directory branch updates `by_dir` and increments both `f` and `d`.
- `src/commands/stats.rs:89-100` - the file branch also increments `f`, which is the only branch that should count files.

## Suggested direction

Remove the `*f += 1` update from the directory branch and add a fixture that
asserts known `files` and `dirs` counts in `--format json` output.

## Open questions

Should `by_directory` include the root row, or only directories that directly
contain counted children?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
