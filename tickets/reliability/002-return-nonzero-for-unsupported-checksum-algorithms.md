---
id: REL-002
title: return nonzero for unsupported checksum algorithms
lens: reliability
severity: S2
confidence: high
effort: S
blast_radius: 1 module, 1 command, 1 MCP tool
files:
  - src/commands/checksum.rs:21-30
  - src/commands/checksum.rs:68-80
  - src/commands/checksum.rs:82-107
depends_on: []
tags: [exit-code, checksum, mcp]
status: done
---

## What

Unsupported checksum algorithms are converted into per-file errors, but the
command still returns `Ok(())`. The CLI can print `Unsupported algorithm` while
exiting successfully.

## Why it matters

Scripts and MCP callers can treat an invalid user request as success. That makes
automation unreliable and hides configuration mistakes.

## Evidence

- `src/commands/checksum.rs:21-30` - unknown algorithms become `Err("Unsupported algorithm: ...")`.
- `src/commands/checksum.rs:68-80` - all files are mapped into `Result` values instead of validating the algorithm once.
- `src/commands/checksum.rs:82-107` - JSON embeds the error and text mode prints it, but neither branch returns a command-level `Err`.

## Suggested direction

Validate `args.algorithm` before hashing. Return `Err` for unsupported
algorithms so the global error handler produces a non-zero exit.

## Open questions

Should missing/unreadable files remain per-file errors with exit 0, or should
any checksum failure make the command fail?

## Implementation

Implemented in build slices 1-7. Proof:
`docs/proofs/2026-06-22-build-slices-1-7.md`.
