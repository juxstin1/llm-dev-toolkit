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

## Draft Upgrade Candidates

These candidates came from a 2026-06-26 read-only audit and need ticket files
before implementation.

| ID | Priority | Status | Candidate | Likely Touchpoints | Next Action |
| --- | --- | --- | --- | --- | --- |
| `TK-UPGRADE-001` | P0 | draft | Fail clearly on missing or unreadable walk roots instead of silently dropping walker errors. | `src/commands/mod.rs`, `tests/cli.rs` | Write `SPEC-003` or a ticket-local exit-code contract, then add failing tests. |
| `TK-UPGRADE-003` | P1 | draft | Harden MCP argument validation for non-string arrays and invalid integer values. | `src/mcp.rs`, `tests/cli.rs` | Resolve MCP error-semantics gate or cover it under `SPEC-004`. |
| `TK-UPGRADE-004` | P1 | draft | Lock MCP tool inventory and schema shape against the README contract. | `src/mcp.rs`, `tests/cli.rs`, `README.md` | Create a test-first ticket; source changes only if the test exposes drift. |
| `TK-UPGRADE-005` | P1 | draft | Add natural `tk info PATH` while preserving `tk info -f PATH`. | `src/main.rs`, `src/commands/info.rs`, `tests/cli.rs`, `README.md` | Product CLI decision gate before implementation. |
| `TK-UPGRADE-006` | P1 | draft | Make documented aliases visible and contract-tested in help. | `src/main.rs`, `tests/cli.rs`, `README.md` | Product help-output decision gate before implementation. |
| `TK-UPGRADE-007` | P2 | draft | Normalize extension filter behavior across `search`, `ff-ext`, and `recent`. | `src/commands/find.rs`, `src/commands/recent.rs`, `tests/cli.rs` | Product behavior decision gate before implementation. |
| `TK-UPGRADE-008` | P2 | draft | Make no-arg `tk info` cheaper or explicitly opt into disk usage. | `src/main.rs`, `src/commands/info.rs`, `tests/cli.rs`, `README.md` | UX/performance decision gate before implementation. |
