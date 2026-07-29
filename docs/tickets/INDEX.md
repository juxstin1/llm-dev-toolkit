# Ticket Index

This is the repo-local active work queue. Root-level `tickets/*.md` files are
historical GitHub issue body drafts created before GitHub CLI auth was fixed.

Implementation sequencing lives in [`tickets/_build_order.md`](../../tickets/_build_order.md).
New tickets should use [TEMPLATE.md](TEMPLATE.md).

If no ticket is `ready`, run the
[autonomous loop](../runbooks/autonomous-loop.md) to research and draft the next
ticket before changing product code.

## Status Values

| Status | Meaning |
| --- | --- |
| `draft` | Captured and not yet accepted for implementation. |
| `ready` | Scoped, reproducible, and ready to implement. |
| `in_progress` | Actively being implemented. |
| `blocked` | Waiting on an external dependency or decision. |
| `done` | Implemented and verified. |

## Active Bugs

| ID | Priority | Status | Draft | Scope | Next Action |
| --- | --- | --- | --- | --- | --- |
| `TK-BUG-001` | P1 | done | [`tickets/001-tree-a-includes-git.md`](../../tickets/001-tree-a-includes-git.md) | `tree`/`ltd` should never include `.git`. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |
| `TK-BUG-002` | P1 | done | [`tickets/002-stats-directory-overcounts-files.md`](../../tickets/002-stats-directory-overcounts-files.md) | `stats -d` should not count directories as files. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |
| `TK-BUG-003` | P2 | done | [`tickets/003-info-symlink-misclassified.md`](../../tickets/003-info-symlink-misclassified.md) | `info -f` should classify symlinks correctly. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |
| `TK-BUG-004` | P2 | done | [`tickets/004-checksum-invalid-algorithm-exits-zero.md`](../../tickets/004-checksum-invalid-algorithm-exits-zero.md) | Unsupported checksum algorithms should exit non-zero. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |
| `TK-BUG-005` | P2 | done | [`tickets/005-validate-enum-like-cli-options.md`](../../tickets/005-validate-enum-like-cli-options.md) | Invalid enum-like CLI values should fail instead of falling back silently. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |
| `TK-BUG-006` | P2 | done | [`tickets/006-search-extension-filter-leading-dot.md`](../../tickets/006-search-extension-filter-leading-dot.md) | `search -e .rs` should behave like `search -e rs`. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |
| `TK-BUG-007` | P2 | done | [`tickets/007-ltd-shows-hidden-entries-by-default.md`](../../tickets/007-ltd-shows-hidden-entries-by-default.md) | `ltd -L` should not show hidden entries by default. | Implemented; see [`proof`](../proofs/2026-06-22-build-slices-1-7.md). |

## Creation Path

The local drafts above are now implemented and verified. Create live GitHub
issues only if you want archival issue records after GitHub CLI auth is fixed.

After GitHub CLI auth is repaired:

```powershell
gh issue create --title "`tree -a` includes `.git` despite git-aware contract" --body-file tickets/001-tree-a-includes-git.md
gh issue create --title "`stats -d` overcounts files for directories" --body-file tickets/002-stats-directory-overcounts-files.md
gh issue create --title "`info -f` likely misclassifies symlinks" --body-file tickets/003-info-symlink-misclassified.md
gh issue create --title "`checksum` exits successfully for unsupported algorithms" --body-file tickets/004-checksum-invalid-algorithm-exits-zero.md
gh issue create --title "Validate enum-like CLI options instead of silently accepting invalid values" --body-file tickets/005-validate-enum-like-cli-options.md
gh issue create --title "`search -e .rs` returns no matches while `search -e rs` works" --body-file tickets/006-search-extension-filter-leading-dot.md
gh issue create --title "`ltd -L` shows hidden entries by default unlike `tree -L`" --body-file tickets/007-ltd-shows-hidden-entries-by-default.md
```

Once live issue numbers exist, add them to this index.

## Ready Queue

No tickets are ready right now. Promote one draft candidate only after its
decision gates are resolved and it has acceptance criteria, verification, proof
target, rollback, and handoff notes.

## Completed Upgrade Tickets

| ID | Priority | Status | Ticket | Scope | Proof |
| --- | --- | --- | --- | --- | --- |
| `TK-UPGRADE-002` | P1 | done | [JSON runtime error contract tests](TK-UPGRADE-002-json-runtime-error-contract-tests.md) | Test-only coverage for JSON-mode runtime errors. | [proof](../proofs/2026-06-26-tk-upgrade-002-json-runtime-errors.md) |
| `TK-UPGRADE-001` | P0 | done | [SPEC-003](../specs/SPEC-003-cli-output-errors.md) | Missing or unreadable recursive roots fail clearly. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |
| `TK-UPGRADE-003` | P1 | done | [SPEC-004](../specs/SPEC-004-mcp-tool-contract.md) | MCP rejects malformed argument values. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |
| `TK-UPGRADE-004` | P1 | done | [SPEC-004](../specs/SPEC-004-mcp-tool-contract.md) | MCP inventory and closed schemas are contract-tested. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |
| `TK-UPGRADE-005` | P1 | done | [SPEC-001](../specs/SPEC-001-cli-ergonomics.md) | Natural `tk info PATH` preserves `tk info -f PATH`. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |
| `TK-UPGRADE-006` | P1 | done | [SPEC-001](../specs/SPEC-001-cli-ergonomics.md) | Public aliases and fast flags are visible and tested. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |
| `TK-UPGRADE-007` | P2 | done | [SPEC-003](../specs/SPEC-003-cli-output-errors.md) | Leading-dot extension filters are normalized and tested. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |
| `TK-UPGRADE-008` | P2 | done | [SPEC-003](../specs/SPEC-003-cli-output-errors.md) | Recursive disk usage is opt-in through `info --disk-usage`. | [roadmap proof](../proofs/2026-07-29-roadmap-completion.md) |

## Draft Upgrade Candidates

All candidates from the 2026-06-26 audit were resolved in the 0.5.0 roadmap
slice. The table remains as historical context.

| ID | Priority | Status | Candidate | Likely Touchpoints | Next Action |
| --- | --- | --- | --- | --- | --- |
| `TK-UPGRADE-001` | P0 | done | Fail clearly on missing or unreadable walk roots instead of silently dropping walker errors. | `src/commands/mod.rs`, `tests/cli.rs` | Closed. |
| `TK-UPGRADE-003` | P1 | done | Harden MCP argument validation for non-string arrays and invalid integer values. | `src/mcp.rs`, `tests/cli.rs` | Closed. |
| `TK-UPGRADE-004` | P1 | done | Lock MCP tool inventory and schema shape against the README contract. | `src/mcp.rs`, `tests/cli.rs`, `README.md` | Closed. |
| `TK-UPGRADE-005` | P1 | done | Add natural `tk info PATH` while preserving `tk info -f PATH`. | `src/main.rs`, `src/commands/info.rs`, `tests/cli.rs`, `README.md` | Closed. |
| `TK-UPGRADE-006` | P1 | done | Make documented aliases visible and contract-tested in help. | `src/main.rs`, `tests/cli.rs`, `README.md` | Closed. |
| `TK-UPGRADE-007` | P2 | done | Normalize extension filter behavior across `search`, `ff-ext`, and `recent`. | `src/commands/find.rs`, `src/commands/recent.rs`, `tests/cli.rs` | Closed. |
| `TK-UPGRADE-008` | P2 | done | Make no-arg `tk info` cheaper or explicitly opt into disk usage. | `src/main.rs`, `src/commands/info.rs`, `tests/cli.rs`, `README.md` | Closed. |
