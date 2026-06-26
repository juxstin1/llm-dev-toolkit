# Spec Development Roadmap

Snapshot date: 2026-06-26.

## Why This Exists

`tk` is small enough to move quickly, but it now has multiple behavior surfaces:
human CLI text, structured JSON, MCP tool output, recursive filesystem walking,
archive extraction, and cross-platform CI. The planning layer keeps these
surfaces explicit before implementation work changes behavior.

## Doc Lanes

| Lane | Purpose |
| --- | --- |
| Map | Current code and verification orientation. |
| Spec | Durable behavior or architecture contract. |
| Plan | Sequenced implementation path. |
| Ticket | One scoped work slice. |
| ADR | Durable decision and tradeoff. |
| Runbook | Commands, health checks, recovery, and local operations. |
| Proof | Evidence that a claim was checked. |

## Phase 0: Planning Layer

Status: implemented.

Deliverables:

- `docs/README.md`
- `docs/CODEBASE_MAP.md`
- `docs/specs/README.md`
- `docs/specs/TEMPLATE.md`
- `docs/specs/SPEC-000-planning-system.md`
- `docs/tickets/INDEX.md`
- `docs/adr/README.md`
- `docs/runbooks/README.md`
- `docs/runbooks/autonomous-loop.md`
- `docs/proofs/README.md`

Acceptance:

- Docs describe current state and avoid product-code changes.
- Completed ticket drafts and draft upgrade candidates are indexed.
- `git diff --check` passes.

## Phase 1: Stabilize Known Bugs

Status: implemented for the first seven bug tickets. See
[`docs/proofs/2026-06-22-build-slices-1-7.md`](../proofs/2026-06-22-build-slices-1-7.md).

Candidate order is summarized here and expanded in
[`tickets/_build_order.md`](../../tickets/_build_order.md):

1. `TK-BUG-001`: Make `tree` and `ltd` honor the `.git` exclusion contract.
2. `TK-BUG-002`: Fix `stats -d` per-directory file counts.
3. `TK-BUG-007`: Make `ltd -L` match `tree -L` hidden-file behavior.
4. `TK-BUG-004`: Make unsupported checksum algorithms exit non-zero.
5. `TK-BUG-005`: Validate enum-like CLI options.
6. `TK-BUG-006`: Normalize leading-dot extension filters for `search`.
7. `TK-BUG-003`: Fix `info -f` symlink classification with platform-aware tests.

Each bug fix should include:

- A focused test that fails before the fix.
- A code change scoped to the relevant module.
- A proof note under `docs/proofs/` with command output.

## Phase 2: Write Behavior Contracts

Status: ongoing. `SPEC-001` now covers CLI ergonomics and workflow
shortcuts.

Candidate specs:

- `SPEC-001`: CLI ergonomics and workflow shortcuts.
- `SPEC-002`: Git-aware traversal and hidden-file behavior.
- `SPEC-003`: CLI text, JSON output, and exit-code contract.
- `SPEC-004`: MCP tool schema and CLI parity contract.
- `SPEC-005`: Archive extraction safety and overwrite contract.

Acceptance:

- Each spec has file touchpoints, acceptance criteria, verification, rollback,
  and open questions.
- Existing tests map to at least one spec where practical.

## Phase 3: Packaging And Release Notes

Candidate work:

- Release checklist runbook.
- Versioning policy ADR.
- Demo GIF update runbook.
- Proof template for cross-platform CI and local release builds.

## Phase 4: Autonomous Upgrade Loop

Status: implemented for docs/control-plane; ongoing for ticket execution.

The loop lives in
[`docs/runbooks/autonomous-loop.md`](../runbooks/autonomous-loop.md). It defines
research, ticket readiness, subagent contracts, implementation, verification,
proof capture, and handoff.

Next upgrade work should promote one draft candidate from
[`docs/tickets/INDEX.md`](../tickets/INDEX.md) to `ready`, then implement it in a
small verified slice.

## First Follow-Ups

- Promote one low-risk draft upgrade ticket to `ready` after any decision gate
  is resolved.
- Write `SPEC-003` before broader CLI output or exit-code changes.
- Write `SPEC-004` before broader MCP schema or error-semantics changes.
- Fix GitHub CLI auth only if live archival issues are needed.
