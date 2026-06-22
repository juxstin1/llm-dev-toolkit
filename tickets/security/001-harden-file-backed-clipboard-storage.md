---
id: SEC-001
title: harden file-backed clipboard storage
lens: security
severity: S3
confidence: medium
effort: M
blast_radius: 1 module, 1 command
files:
  - src/commands/clip.rs:6-12
  - src/commands/clip.rs:23-28
  - src/commands/clip.rs:59-80
depends_on: []
tags: [local-data, clipboard]
status: done
---

## What

When the system clipboard is unavailable, `clip -i` persists arbitrary clipboard
content into `$HOME/.tk_clipboard` using normal file creation defaults. The
fallback is useful, but there is no permission hardening or explicit opt-in for
persisting potentially sensitive clipboard content.

## Why it matters

Clipboard data often includes credentials, tokens, or private text. Persisting
that data to a stable home-directory file can surprise users on shared or
managed machines.

## Evidence

- `src/commands/clip.rs:6-12` - fallback storage path is always `$HOME/.tk_clipboard` or `%USERPROFILE%\.tk_clipboard`.
- `src/commands/clip.rs:23-28` - `write_clipboard()` creates parent dirs and writes content with `std::fs::write`.
- `src/commands/clip.rs:59-80` - when `Clipboard::new().set_text()` fails, the command writes to the file fallback.

## Suggested direction

Use restrictive file permissions where supported, document the persistence
behavior in command help, and consider requiring an explicit fallback flag for
file-backed writes.

## Open questions

Is the file fallback a must-have behavior for headless environments, or can
write failure be surfaced instead?

## Implementation

Implemented in build slice 8. Fallback persistence remains available for
headless environments, but writes require explicit `--allow-file-fallback`
opt-in. Proof: `docs/proofs/2026-06-22-sec-001-clipboard-fallback.md`.
