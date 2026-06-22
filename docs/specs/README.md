# Specs Registry

Specs are durable design contracts. They should describe what must be true,
where the code lives, how the change rolls out, and how to prove it.

## Status Vocabulary

| Status | Meaning |
| --- | --- |
| `draft` | Proposed or newly created; useful but not yet accepted as durable truth. |
| `accepted` | Agreed contract for implementation and review. |
| `implemented` | Contract has shipped and verification evidence exists. |
| `superseded` | Replaced by another spec. |
| `archived` | Historical context only. |

## Active Specs

| Spec | Status | Scope |
| --- | --- | --- |
| [SPEC-000: Planning System](SPEC-000-planning-system.md) | draft | Repo-local docs, maps, tickets, runbooks, and proof lanes. |

## Candidate Future Specs

| Candidate | Why |
| --- | --- |
| CLI output contract | Pin text vs JSON behavior, exit-code expectations, color rules, and path formatting. |
| Git-aware traversal contract | Make `.git`, ignore handling, hidden-file behavior, and recursive traversal semantics consistent. |
| MCP tool contract | Pin exposed tools, input schemas, error semantics, and parity with CLI JSON output. |
| Archive extraction safety contract | Define supported archive formats, zip-slip/tar traversal protection, overwrite behavior, and entry counts. |

## Spec Rules

- Start from [TEMPLATE.md](TEMPLATE.md).
- Keep current state and proposed design separate.
- Include exact file touchpoints and verification commands.
- Link related tickets, ADRs, runbooks, and proofs.
- Update this registry when a spec changes status.

## Related Lanes

- Plans sequence implementation work across one or more specs.
- Tickets are scoped work items that should point back to a spec or map.
- ADRs record durable decisions and tradeoffs.
- Runbooks capture commands and recovery steps.
- Proofs record validation output and gaps.
