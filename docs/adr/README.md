# ADRs

Architecture Decision Records capture durable choices and tradeoffs. Use this
lane when a decision should survive beyond a single ticket.

## Naming

Use `ADR-NNN-short-title.md`, starting at `ADR-001`.

## Template

```markdown
# ADR-001: Short Title

Date: YYYY-MM-DD
Status: proposed

## Context

## Decision

## Consequences

## Alternatives Considered

## Verification Or Follow-Up
```

## Candidate ADRs

- Whether MCP should continue shelling back into the current executable for CLI
  JSON parity, or dispatch in-process for speed.
- Versioning and release policy for the `tk` binary.
- Where ticket drafts should live once GitHub issues exist.
