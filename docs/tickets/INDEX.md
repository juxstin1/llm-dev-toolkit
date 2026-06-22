# Ticket Index

This is the repo-local active work queue. Root-level `tickets/*.md` files are
GitHub issue body drafts created before GitHub CLI auth was fixed.

Implementation sequencing lives in [`tickets/_build_order.md`](../../tickets/_build_order.md).

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
