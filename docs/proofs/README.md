# Proofs

Proofs record exact validation evidence: command, date, context, output
summary, and any gaps. They are not a substitute for rerunning checks before a
release or implementation PR.

## Naming

Use `YYYY-MM-DD-short-topic.md`.

## Template

````markdown
# Proof: Short Topic

Date: YYYY-MM-DD
Scope: What was checked.

## Commands

```powershell
command here
```

## Result

Pass/fail summary and important output.

## Gaps

Anything not checked.
````

## Current Proofs

- [2026-06-22-spec0-setup.md](2026-06-22-spec0-setup.md)
- [2026-06-22-ticket-runover.md](2026-06-22-ticket-runover.md)
