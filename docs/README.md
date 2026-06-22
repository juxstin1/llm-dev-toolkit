# Spec0 Front Door

This repository uses a lightweight spec-first planning layer for the `tk` CLI
and MCP server. Product code stays in `src/`; durable planning context lives
under `docs/`.

Current status: draft planning layer, created 2026-06-22. Recheck the live
checkout before treating runtime facts as current.

| Need | Use |
| --- | --- |
| Codebase orientation | [CODEBASE_MAP.md](CODEBASE_MAP.md) |
| Durable design contracts | [specs/README.md](specs/README.md) |
| Multi-slice plans | [plans/spec-development-roadmap.md](plans/spec-development-roadmap.md) |
| Active implementation queue | [tickets/INDEX.md](tickets/INDEX.md) |
| Architecture decisions | [adr/README.md](adr/README.md) |
| Commands and operations | [runbooks/README.md](runbooks/README.md) |
| Validation evidence | [proofs/README.md](proofs/README.md) |

## Working Rules

- Separate current-state evidence from proposed design.
- Name exact files and commands.
- Keep specs implementation-ready but small enough for a focused PR.
- Record verification evidence in `docs/proofs/` when behavior is checked.
- Do not change product code as part of planning-only updates.

## Current Queue

The first bug queue was drafted as GitHub issue bodies under `tickets/`.
`docs/tickets/INDEX.md` is the repo-local planning index that points to those
drafts until real GitHub issue numbers exist.
