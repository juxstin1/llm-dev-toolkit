---
title: "SPEC-003: CLI Output And Error Contract"
status: implemented
date: 2026-07-29
owners:
  - repository maintainer
---

# SPEC-003: CLI Output And Error Contract

## Contract

- Text is the default output; `--format json`, `--json`, and `-j` select JSON.
- `--color never` and `--no-color` prevent ANSI output.
- Existing JSON success shapes remain command-specific and stable.
- Runtime failures return nonzero. In JSON mode stdout is empty and stderr is
  one object with an `error` string.
- Syntax errors reported by Clap return nonzero using Clap's diagnostic format.
- Recursive commands reject a missing or inaccessible root instead of silently
  returning empty success.
- Errors opening the root are fatal. Descendant entries that disappear or
  become unreadable during traversal are skipped so long scans can return the
  remaining accessible results.
- CLI flags override project config, which overrides global config.

## Verification

Integration tests cover conflicting global flags, JSON runtime errors, missing
walk roots, aliases, short flags, configured defaults, and ANSI suppression.
Every behavior change must preserve the existing canonical command.
