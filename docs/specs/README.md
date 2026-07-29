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
| [SPEC-001: CLI Ergonomics And Workflow Shortcuts](SPEC-001-cli-ergonomics.md) | implemented | Visible aliases, short flags, natural info paths, `show`, `scan`, and completions. |
| [SPEC-003: CLI Output And Error Contract](SPEC-003-cli-output-errors.md) | implemented | Text/JSON selection, ANSI control, runtime failures, and root validation. |
| [SPEC-004: MCP Tool Schema And CLI Parity](SPEC-004-mcp-tool-contract.md) | implemented | Closed schemas, strict validation, 23 canonical tools, and CLI child dispatch. |
| [SPEC-006: Git Integration Commands](SPEC-006-git-integration.md) | implemented | `tk status`/`diff`/`log`/`branch` with structured JSON for agents. |
| [SPEC-007: Context Window Manager](SPEC-007-context-window.md) | implemented | Token-aware file concatenation with max-token budgeting. |
| [SPEC-008: Symbol Extraction](SPEC-008-symbol-extraction.md) | implemented | Function/class/struct/interface extraction from source. |
| [SPEC-009: Project Detection](SPEC-009-project-detection.md) | implemented | Detect language, build, test, lint, and formatter configuration. |
| [SPEC-010: Web Scraping Utility](SPEC-010-web-scraper.md) | implemented | Feature-gated HTTP fetching, selectors, and text cleanup. |
| [SPEC-011: MCP File Reading](SPEC-011-mcp-file-reading.md) | implemented | Bounded `read_file` and `read_lines` MCP tools. |
| [SPEC-012: Layered Configuration](SPEC-012-config-system.md) | implemented | Global/project TOML, feature gates, defaults, and CLI precedence. |

## Candidate Future Specs

| Candidate | Why |
| --- | --- |
| `SPEC-002`: Git-aware traversal contract | Make `.git`, ignore handling, hidden-file behavior, and recursive traversal semantics consistent. |
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
