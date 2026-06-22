---
id: ARCH-001
title: route tree traversal through the shared walker contract
lens: architecture
severity: S2
confidence: high
effort: M
blast_radius: 1 module, 2 commands, 1 MCP tool
files:
  - src/commands/tree.rs:68-80
  - src/commands/tree.rs:111-122
  - src/commands/mod.rs:147-161
depends_on: []
tags: [traversal, git-aware]
status: done
---

## What

`tree` builds its own `ignore::WalkBuilder` in both JSON and text renderers
instead of going through the repo's shared traversal boundary. The shared walker
has the `.git` exclusion contract, but `tree` does not inherit it.

## Why it matters

This is already causing behavior drift: `tree -a` and `ltd -L` can surface
`.git` while other recursive commands intentionally do not. Future traversal
policy changes will need to be patched in multiple places.

## Evidence

- `src/commands/tree.rs:68-80` - JSON tree construction creates a local `WalkBuilder` and filters hidden entries manually.
- `src/commands/tree.rs:111-122` - text tree rendering repeats a second local `WalkBuilder` and the same hidden-entry filter.
- `src/commands/mod.rs:147-161` - the shared walker applies gitignore settings and explicitly filters any `.git` path component.

## Suggested direction

Move tree entry enumeration behind one helper that uses the same `.git` and
ignore semantics as `walk_entries`, or add a shared shallow-walk helper for the
tree renderer.

## Open questions

Should `ltd` remain a separate command with custom semantics, or become a thin
shortcut for `tree -L`?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
