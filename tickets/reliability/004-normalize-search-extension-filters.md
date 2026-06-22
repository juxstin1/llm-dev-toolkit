---
id: REL-004
title: normalize search extension filters
lens: reliability
severity: S3
confidence: high
effort: S
blast_radius: 1 module, 1 command, 1 MCP tool
files:
  - src/commands/search.rs:37-41
  - src/commands/search.rs:82-86
  - src/commands/recent.rs:35-36
  - src/commands/find.rs:81
depends_on: []
tags: [search, cli-consistency]
status: done
---

## What

`search -e` compares the supplied extension literally. `search -e rs` matches
Rust files, but `search -e .rs` returns no matches. Other extension-aware
commands normalize away a leading dot.

## Why it matters

The inconsistency makes a common user input look like a valid no-match search,
which is easy to trust in automation.

## Evidence

- `src/commands/search.rs:37-41` - text search compares `file_path.extension()` directly to `args.ext`.
- `src/commands/search.rs:82-86` - JSON search repeats the same direct comparison.
- `src/commands/recent.rs:35-36` - `recent` trims a leading dot before comparing extensions.
- `src/commands/find.rs:81` - `ff-ext` also trims a leading dot.

## Suggested direction

Normalize `args.ext` once in `search` and use the normalized value in both text
and JSON paths.

## Open questions

Should extension normalization be a shared helper across commands?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
