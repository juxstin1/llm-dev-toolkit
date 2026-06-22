# Build Order

This order sequences the open ticket set for implementation. It groups duplicate
or overlapping tickets by root cause, keeps early changes small, and leaves
decision-heavy work until after the low-risk correctness fixes.

## Principles

- Add the failing test first for every behavior ticket.
- Keep each build slice reversible and scoped to one behavior surface.
- Prefer contained S-sized correctness fixes before broad refactors.
- Do not start MCP schema cleanup until CLI behavior is stable.
- Record proof output under `docs/proofs/` for every completed slice.

## Current Status

Slices 1-7 are implemented and verified in
[`docs/proofs/2026-06-22-build-slices-1-7.md`](../docs/proofs/2026-06-22-build-slices-1-7.md).
Slice 8 is implemented and verified in
[`docs/proofs/2026-06-22-sec-001-clipboard-fallback.md`](../docs/proofs/2026-06-22-sec-001-clipboard-fallback.md).
Slice 9 is implemented and verified in
[`docs/proofs/2026-06-22-maint-001-demo-ci.md`](../docs/proofs/2026-06-22-maint-001-demo-ci.md).
Slice 10 remains open.

## Ordered Slices

| Order | Slice | Tickets | Why Here | Likely Touchpoints | Verification |
| --- | --- | --- | --- | --- | --- |
| 1 | Unify tree traversal semantics | `ARCH-001`, `TK-BUG-001`, `TK-BUG-007` | Fixes the README-visible `.git` leak and the `ltd` hidden-entry drift in one module. This is the clearest contract violation and unlocks a clean traversal spec. | `src/commands/tree.rs`, `tests/cli.rs` | `cargo test --all`; targeted tests for `tree -a`, JSON tree, and `ltd -L`. |
| 2 | Correct stats directory counts | `REL-001`, `TK-BUG-002` | Small, isolated correctness fix with high confidence and low blast radius. | `src/commands/stats.rs`, `tests/cli.rs` | Fixture asserting `stats -d --format json` file/dir counts. |
| 3 | Normalize search extension filters | `REL-004`, `TK-BUG-006` | Small user-facing consistency fix; keeps search behavior stable before broader CLI validation work. | `src/commands/search.rs`, `tests/cli.rs` | Compare `search -e rs` and `search -e .rs` in text and JSON modes. |
| 4 | Fail unsupported checksum algorithms | `REL-002`, `TK-BUG-004` | Establishes non-zero exit behavior for invalid arguments before generalizing validation patterns. | `src/commands/checksum.rs`, `tests/cli.rs`, optionally `src/mcp.rs` tests | Assert non-zero CLI exit; assert MCP `checksum` reports `isError` for invalid algorithm if exposed through MCP. |
| 5 | Reject invalid fixed-value CLI options | `REL-003`, `TK-BUG-005` | Builds on the checksum exit-code decision; converts silent fallbacks into explicit parse or command errors. | `src/main.rs`, `src/commands/find.rs`, `src/commands/sort.rs`, `tests/cli.rs` | Invalid `ff -t x` and `sort --by nope` fail with helpful stderr. |
| 6 | Fail invalid duplicate size filters | `REL-005` | Same validation family as slice 5, but separate because `dups --delete` raises the behavioral stakes. | `src/commands/dups.rs`, `tests/cli.rs` | Invalid `dups -m nonsense` exits non-zero; valid sizes still pass. |
| 7 | Classify symlink paths in info | `REL-006`, `TK-BUG-003` | Cross-platform test design may need a skip/fallback, so keep it after the easier CLI correctness work. | `src/commands/info.rs`, `tests/cli.rs` | Platform-aware symlink test; `info -f` reports the inspected path as `symlink`. |
| 8 | Harden file-backed clipboard fallback | `SEC-001` | Needs a human decision on whether fallback persistence remains implicit. Defer until correctness queue is stable. | `src/commands/clip.rs`, README/help text, tests if feasible | Unit test fallback path behavior; manual check on at least one headless or simulated clipboard-failure path. |
| 9 | Add demo project CI coverage | `MAINT-001` | Independent maintainability improvement; safe after core CLI work so CI churn does not mix with behavior fixes. | `.github/workflows/ci.yml`, `demo/package.json`, maybe `demo/tsconfig.json` | `npm ci`; a lightweight `npm run check` or equivalent once added. |
| 10 | Unify MCP schema and builders | `ARCH-002` | Broader refactor across the MCP surface; best done after CLI validation/error semantics settle. | `src/mcp.rs`, `tests/cli.rs` | Existing MCP tests plus at least one tool-call test for each changed builder path. |

## Parallel Work

These can run in parallel if separate branches or agents are used:

- Slice 2 and slice 3 touch different modules.
- Slice 8 and slice 9 are independent of the core CLI correctness queue, but
  both may need human decisions before implementation.

Avoid parallelizing slices 4, 5, and 6 unless the team first agrees on the CLI
invalid-argument error contract.

## Suggested Spec Dependencies

| Spec | Should Precede |
| --- | --- |
| `SPEC-001`: Git-aware traversal and hidden-file behavior | Slice 1, if a formal contract is desired before implementation. |
| `SPEC-002`: CLI text, JSON output, and exit-code contract | Slices 4, 5, and 6. |
| `SPEC-003`: MCP tool schema and CLI parity contract | Slice 10. |

## Completion Definition

Each slice is complete when:

- The relevant ticket(s) are updated or closed.
- Tests cover the trigger condition.
- `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
  and `cargo test --all` pass.
- A proof note exists under `docs/proofs/`.
