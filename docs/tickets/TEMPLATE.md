# Ticket Template

Status: draft
Priority: P2
Created: YYYY-MM-DD
Related spec: TBD
Proof target: `docs/proofs/YYYY-MM-DD-short-topic.md`

## Problem

What user-visible, agent-visible, or maintenance problem should this fix?

## Current State Evidence

| Area | Evidence |
| --- | --- |
| Source | `path/to/file.rs` line or symbol. |
| Test gap | Existing test path or missing coverage. |
| Docs | README, spec, runbook, or proof reference. |

## Goals

- TBD

## Non-Goals

- TBD

## Scope

Read scope:

- TBD

Write scope:

- TBD

Do not touch:

- TBD

## Acceptance Criteria

- TBD

## Verification

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```

## Subagent Contract

Goal: TBD

Context:

- TBD

Read scope:

- TBD

Write scope:

- TBD

Decision gates:

- Stop before changing public CLI behavior, MCP schema/error semantics,
  architecture, security, credentials, deployments, or destructive operations.
- Stop if the task needs files outside the write scope.
- Stop if verification requires unavailable external services.

Required final report:

```text
Summary:
Changed files:
Verification:
Decision gates hit:
Residual risks:
```

## Rollback

How to revert or contain the change.

## Handoff

What should the next agent know after this ticket is implemented or blocked?
