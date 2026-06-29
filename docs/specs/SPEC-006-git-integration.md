---
title: "SPEC-006: Git Integration Commands"
status: draft
date: 2026-06-29
scope: "Add git status, diff, log, and branch subcommands to tk"
owners: []
related: ["SPEC-001 (CLI ergonomics)"]
packages: []
---

# SPEC-006: Git Integration Commands

> `tk` gains the four most-used git operations — `status`, `diff`, `log`, `branch` — with structured JSON output so agents can understand project state without parsing porcelain.

## Problem

Agents working in a repo spend a significant portion of every session asking git questions: what branch am I on, what changed, what's the commit history. Currently they either shell out to `git` directly (unstructured output, fragile parsing, no JSON contract) or use agent-specific built-ins that don't generalize across tools (Claude Code vs Codex vs OpenCode).

## Goals

- `tk status` — working tree state: branch, staged/unstaged/untracked files, ahead/behind counts
- `tk diff` — staged or unstaged diffs as structured hunks (or unified diff text)
- `tk log` — commit history with hash, author, date, message, optionally filtered by path
- `tk branch` — list branches, highlight current, show upstream tracking
- Every command has `--format json` output suitable for agent consumption
- Fast: sub-second for typical repos; delegates to `git` plumbing, not libgit2

## Non-Goals

- No `git commit`, `git push`, `git pull` — mutation is out of scope for tk's read-only ethos
- No `git merge`, `git rebase`, `git cherry-pick` — complexity doesn't justify the agent use case
- No libgit2 dependency — shelling out to `git` plumbing is faster and always in sync with the user's git version
- No blame, bisect, stash, or submodule commands (future candidate)

## Proposed Design

Each command shells out to `git` plumbing commands and parses the output into structured data. The `tk` binary never writes to `.git/`. The `git` binary is assumed to be on `PATH` (with a clear error message if not).

### `tk status`

```
tk status [path] [--porcelain]
```

Internally: `git status --branch --porcelain=v1` then parse.

JSON shape:
```json
{
  "branch": "main",
  "upstream": "origin/main",
  "ahead": 2,
  "behind": 0,
  "staged": [
    { "path": "src/main.rs", "status": "M", "old_path": null }
  ],
  "unstaged": [
    { "path": "Cargo.toml", "status": "M" }
  ],
  "untracked": ["new_file.rs"],
  "merge_conflicts": []
}
```

### `tk diff`

```
tk diff [--staged] [--cached] [paths...] [--context N]
```

Internally: `git diff --unified=N [--cached] [--] <paths>` then parse unified diff.

JSON shape (structured hunks):
```json
[
  {
    "file": "src/main.rs",
    "status": "modified",
    "hunks": [
      {
        "old_start": 10, "old_lines": 7,
        "new_start": 10, "new_lines": 9,
        "header": "@@ -10,7 +10,9 @@ fn main() {",
        "lines": [
          { "type": "context", "content": "  let x = 1;", "old_line": 10, "new_line": 10 },
          { "type": "delete",  "content": "  let y = 2;", "old_line": 11, "new_line": null },
          { "type": "add",     "content": "  let y = 3;",  "old_line": null, "new_line": 11 }
        ]
      }
    ]
  }
]
```

Text mode (default): emit unified diff text for human reading.
JSON mode: emit structured hunks for agent consumption.

### `tk log`

```
tk log [--count N] [--since DATE] [--until DATE] [--author PATTERN] [paths...]
```

Internally: `git log --format=... --max-count=N [--since=...] [--until=...] [--author=...] [--] <paths>`.

JSON shape:
```json
[
  {
    "hash": "a1b2c3d",
    "abbreviated_hash": "a1b2c3d",
    "author": { "name": "Jane", "email": "jane@example.com", "date": "2026-06-28T10:00:00Z" },
    "committer": { "name": "Jane", "email": "jane@example.com", "date": "2026-06-28T10:00:00Z" },
    "subject": "Fix off-by-one in search bounds",
    "body": "The upper bound was off by one...",
    "refs": "HEAD -> main, origin/main"
  }
]
```

### `tk branch`

```
tk branch
```

Internally: `git branch --list --format=...` or `git for-each-ref`.

JSON shape:
```json
{
  "current": "main",
  "branches": [
    { "name": "main", "current": true, "upstream": "origin/main", "ahead": 0, "behind": 0 },
    { "name": "feat/x", "current": false, "upstream": null, "ahead": 3, "behind": 1 }
  ]
}
```

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `src/commands/mod.rs` | Add `pub mod git;` | Register new module |
| `src/commands/git.rs` | Create | All git subcommand implementations |
| `src/main.rs` | Add `Commands::Status/Diff/Log/Branch` variants + args structs | CLI routing |
| `src/mcp.rs` | Add `status`, `diff`, `log`, `branch` to tool definitions | Expose via MCP |
| `docs/specs/README.md` | Register spec | Spec tracking |
| `tests/` | Add integration tests | Verify git output parsing |

## Rollout Plan

Single PR implementing all four commands. Each command has its own commit for review clarity.

## Acceptance Criteria

- `tk status --format json` produces valid JSON matching the schema above
- `tk diff` on a dirty repo shows expected hunks (tested against known fixture)
- `tk log -n 3` returns the last 3 commits
- `tk branch` shows current branch with `*` in text mode, `current: true` in JSON
- All commands error gracefully (non-zero exit, stderr message) when run outside a git repo
- MCP server exposes all four tools

## Verification

```bash
# Outside a git repo
tk status        # -> "Error: not a git repository"
tk diff          # -> "Error: not a git repository"

# Inside a git repo with known state
tk status --format json | jq '.branch'
tk diff --format json | jq 'length'
tk log -n 1 --format json | jq '.[0].hash'
tk branch --format json | jq '.current'
```

## Rollback

Revert the PR. The `git` subcommands are additive — no existing behavior changes.

## Open Questions

- Should `tk diff` default to staged or unstaged? (Proposal: unstaged, matching `git diff` default)
- Should we add `tk diff --stat` for a summary view? (Nice-to-have, not in initial scope)
- Should `tk log` support `--oneline` shorthand? (Yes — text mode default should be oneline-style)
