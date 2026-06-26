---
title: "SPEC-000: Planning System"
status: implemented
date: 2026-06-26
scope: "Docs-only setup for repo-local specs, maps, tickets, ADRs, runbooks, proofs, and the autonomous loop."
owners: []
related:
  - "../CODEBASE_MAP.md"
  - "../plans/spec-development-roadmap.md"
  - "../tickets/INDEX.md"
packages:
  - "tk"
---

# SPEC-000: Planning System

> `llm-dev-toolkit` should have a small, evidence-backed planning layer that
> lets future work start from maps, specs, tickets, runbooks, proof logs, and an
> autonomous loop instead of rediscovering the repo each time.

## Problem

The repo has a compact Rust CLI/MCP implementation and a growing planning
surface. Without an explicit local control plane, future work can drift into ad
hoc notes, duplicate tickets, or behavior changes without a clear spec,
verification trail, subagent boundary, and handoff.

## Current State Evidence

| Area | Evidence |
| --- | --- |
| CLI entry point | `src/main.rs` defines `Commands` and dispatches to modules. |
| Shared behavior | `src/commands/mod.rs` owns JSON output, color state, walker behavior, hashing, and binary detection. |
| MCP entry point | `src/mcp.rs` exposes read-only tools and shells back into the current executable with `--format json`. |
| Tests | `tests/cli.rs` covers CLI, JSON, color, tree traversal, validation behavior, Spec0 install, and MCP calls. |
| CI | `.github/workflows/ci.yml` runs fmt, clippy, tests, and release build on Linux, macOS, and Windows. |
| Existing queue | `docs/tickets/INDEX.md` lists the completed initial queue and draft upgrade candidates. |
| Autonomous loop | `docs/runbooks/autonomous-loop.md` defines research, ticket readiness, subagent contracts, verification, proofs, and handoff. |

## Goals

- Provide a clear front door for planning docs.
- Keep current-state evidence tied to exact paths and commands.
- Make specs implementation-ready without changing product code.
- Give tickets, ADRs, runbooks, and proofs stable homes.
- Preserve the existing root `tickets/` drafts as historical issue bodies.
- Define an autonomous loop for future research, implementation, testing, proof
  capture, and handoff.

## Non-Goals

- No Rust source changes in this setup.
- No CI changes in this setup.
- No GitHub issue creation unless live issue records are needed and GitHub CLI
  auth is healthy.
- No claim that runtime observations remain current without a live recheck.
- No migration or deletion of historical root ticket drafts.

## Proposed Design

Add `docs/` as the repo-local planning layer:

- `docs/README.md` is the front door.
- `docs/CODEBASE_MAP.md` maps current code, tests, config, and noisy paths.
- `docs/specs/` holds durable contracts and templates.
- `docs/plans/` holds phase plans and implementation sequences.
- `docs/tickets/` indexes active work and links ticket drafts.
- `docs/adr/` records durable decisions.
- `docs/runbooks/` records commands, checks, and recovery.
- `docs/proofs/` records validation evidence.
- `docs/runbooks/autonomous-loop.md` records the autonomous Spec0/subagent
  workflow.

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `docs/README.md` | Add | Planning front door. |
| `docs/CODEBASE_MAP.md` | Add | Current repo orientation. |
| `docs/specs/README.md` | Add | Spec registry and rules. |
| `docs/specs/TEMPLATE.md` | Add | Standard spec shape. |
| `docs/specs/SPEC-000-planning-system.md` | Add | Contract for this setup. |
| `docs/plans/spec-development-roadmap.md` | Add | First plan and follow-up sequence. |
| `docs/tickets/INDEX.md` | Add | Active queue index. |
| `docs/adr/README.md` | Add | Decision log lane. |
| `docs/runbooks/README.md` | Add | Operational command lane. |
| `docs/runbooks/autonomous-loop.md` | Add | Autonomous loop and subagent contract. |
| `docs/proofs/README.md` | Add | Validation evidence lane. |
| `docs/tickets/TEMPLATE.md` | Add | Implementation-ready ticket shape. |
| `tickets/*.md` | Link only | Preserve existing GitHub issue drafts. |

## API Or Data Contract

Docs use relative repo paths. Ticket ids use `TK-BUG-NNN` until a real external
issue id exists. Spec ids use `SPEC-NNN`. ADR ids use `ADR-NNN`.

Planning status values are:

- `draft`
- `accepted`
- `implemented`
- `superseded`
- `archived`

Ticket status values are:

- `draft`
- `ready`
- `in_progress`
- `blocked`
- `done`

## Rollout Plan

1. Add the docs scaffold.
2. Link the existing root ticket drafts from `docs/tickets/INDEX.md`.
3. Run `git diff --check`.
4. Add the autonomous loop runbook and future-ticket template.
5. For implementation work, pick one ticket and point it at a spec or create a
   focused spec first.
6. Create live GitHub issues only if archival issue records are needed.

## Acceptance Criteria

- `docs/README.md` exists and links every planning lane.
- `docs/CODEBASE_MAP.md` names current entry points, tests, CI, and noisy paths.
- `docs/specs/README.md` registers `SPEC-000`.
- `docs/tickets/INDEX.md` lists completed work, readiness rules, and draft
  upgrade candidates.
- `docs/runbooks/autonomous-loop.md` defines research, ticket, implementation,
  verification, proof, handoff, and subagent rules.
- The diff is docs-only, aside from the existing root `tickets/` drafts.
- `git diff --check` passes.

## Verification

Run:

```powershell
git status --short --branch
git diff --check
```

Optional product verification remains:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

## Rollback

Remove or revert the docs touched by this spec to roll back the planning layer.
Product code is unaffected.

## Open Questions

- Should live GitHub issues be created for historical tickets once `gh auth` is
  healthy?
- Should future accepted tickets stay repo-local or sync to GitHub issues?
