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
| [SPEC-000: Planning System](SPEC-000-planning-system.md) | implemented | Repo-local docs, maps, tickets, runbooks, proof lanes, and autonomous loop. |
| [SPEC-001: CLI Ergonomics And Workflow Shortcuts](SPEC-001-cli-ergonomics.md) | accepted | Additive shortcuts, help polish, natural file info, and future `show`/`scan` workflows. |
| [SPEC-006: Git Integration Commands](SPEC-006-git-integration.md) | draft | `tk status`/`diff`/`log`/`branch` with structured JSON for agents. |
| [SPEC-007: Context Window Manager](SPEC-007-context-window.md) | draft | `tk context` — token-aware file concatenation with max-tokens budgeting. |
| [SPEC-008: Symbol Extraction](SPEC-008-symbol-extraction.md) | draft | `tk symbols` — function/class/struct/interface extraction from source. |
| [SPEC-009: Project Detection](SPEC-009-project-detection.md) | draft | `tk detect` — auto-detect language, build, test, lint from config files. |
| [SPEC-010: Web Scraping Utility](SPEC-010-web-scraper.md) | draft | `tk fetch`/`tk scrape` — HTTP fetching with readability extraction + cleanup. |
| [SPEC-011: MCP File Reading](SPEC-011-mcp-file-reading.md) | draft | `read_file`/`read_lines` MCP tools for agents without built-in file readers. |

## Candidate Future Specs

| Candidate | Why |
| --- | --- |
| `SPEC-002`: Git-aware traversal contract | Make `.git`, ignore handling, hidden-file behavior, and recursive traversal semantics consistent. |
| `SPEC-003`: CLI output and exit-code contract | Pin text vs JSON behavior, exit-code expectations, color rules, and path formatting beyond the ergonomics scope. |
| `SPEC-004`: MCP tool contract | Pin exposed tools, input schemas, error semantics, and parity with CLI JSON output. |
| `SPEC-005`: Archive extraction safety contract | Define supported archive formats, zip-slip/tar traversal protection, overwrite behavior, and entry counts. |

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
