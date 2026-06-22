---
id: REL-006
title: classify symlink paths in info
lens: reliability
severity: S3
confidence: medium
effort: M
blast_radius: 1 module, 1 command
files:
  - src/commands/info.rs:71
  - src/commands/info.rs:88
  - src/commands/ls.rs:229
depends_on: []
tags: [symlink, cross-platform]
status: done
---

## What

`info -f` uses followed metadata to classify the path, then checks
`meta.is_symlink()`. On symlink paths, followed metadata describes the target,
not the link path.

## Why it matters

`tk info -f <symlink>` can report the target type instead of the inspected path
type, while `ls` uses symlink-aware metadata.

## Evidence

- `src/commands/info.rs:71` - `file_info()` calls `fs::metadata(path)`, which follows symlinks.
- `src/commands/info.rs:88` - classification checks `meta.is_symlink()` after following the link.
- `src/commands/ls.rs:229` - `ls` uses `fs::symlink_metadata()` for symlink-aware output.

## Suggested direction

Use `symlink_metadata` for path classification and optionally gather followed
target metadata separately if target details are useful.

## Open questions

How should tests handle Windows environments where symlink creation may require
developer mode or elevated privileges?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
