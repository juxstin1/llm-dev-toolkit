# Proofs

Proofs record exact validation evidence: command, date, context, output
summary, changed files, decision gates, handoff state, and any gaps. They are
not a substitute for rerunning checks before a release or implementation PR.

## Naming

Use `YYYY-MM-DD-short-topic.md`.

## Template

````markdown
# Proof: Short Topic

Date: YYYY-MM-DD
Scope: What was checked.
Source ticket/spec: Link or `none`.
Git state: Branch, upstream, and dirty state.

## Changed Files

- `path`

## Commands

```powershell
command here
```

## Result

Pass/fail summary and important output.

## Decision Gates

Any product, architecture, MCP, security, deployment, credential, destructive,
or external-service decisions encountered.

## Gaps

Anything not checked.

## Handoff

Current status, next action, and whether the source ticket is ready, blocked, or
done.
````

## Current Proofs

- [2026-06-26-current-baseline-and-autonomous-loop.md](2026-06-26-current-baseline-and-autonomous-loop.md)
- [2026-06-26-tk-upgrade-002-json-runtime-errors.md](2026-06-26-tk-upgrade-002-json-runtime-errors.md)
- [2026-06-26-professional-github-readme.md](2026-06-26-professional-github-readme.md)
- [2026-06-22-spec0-setup.md](2026-06-22-spec0-setup.md)
- [2026-06-22-ticket-runover.md](2026-06-22-ticket-runover.md)
- [2026-06-22-build-slices-1-7.md](2026-06-22-build-slices-1-7.md)
- [2026-06-22-sec-001-clipboard-fallback.md](2026-06-22-sec-001-clipboard-fallback.md)
- [2026-06-22-maint-001-demo-ci.md](2026-06-22-maint-001-demo-ci.md)
- [2026-06-22-arch-002-mcp-tool-table.md](2026-06-22-arch-002-mcp-tool-table.md)
- [2026-06-22-cli-ergonomics-spec.md](2026-06-22-cli-ergonomics-spec.md)
