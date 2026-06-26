# Proof: TK-UPGRADE-002 JSON Runtime Errors

Date: 2026-06-26
Scope: Added integration tests for JSON-mode runtime errors.
Source ticket/spec:
`docs/tickets/TK-UPGRADE-002-json-runtime-error-contract-tests.md`
Git state: `main...origin/main`, dirty with the current docs/test slice.

## Changed Files

- `tests/cli.rs`
- `docs/tickets/TK-UPGRADE-002-json-runtime-error-contract-tests.md`
- `docs/tickets/INDEX.md`
- `docs/proofs/2026-06-26-tk-upgrade-002-json-runtime-errors.md`
- `docs/proofs/README.md`

## Commands

```powershell
cargo test --test cli json_runtime_error
cargo fmt --all -- --check
cargo test --all
cargo clippy --all-targets -- -D warnings
```

## Result

Pass.

- `cargo test --test cli json_runtime_error`: 2 passed, 0 failed.
- `cargo fmt --all -- --check`: passed.
- `cargo test --all`: 58 unit tests and 47 CLI/MCP integration tests passed.
- `cargo clippy --all-targets -- -D warnings`: passed.

The new tests assert that JSON-mode runtime failures:

- exit non-zero,
- leave stdout empty,
- emit parseable JSON on stderr,
- include an `error` string.

## Decision Gates

No decision gates were hit. This was a test-only slice and did not change CLI
behavior, MCP behavior, public README text, release behavior, credentials, or
deployment state.

## Gaps

- This does not implement the broader `SPEC-003` CLI output and exit-code
  contract.
- JSON runtime-error coverage is not exhaustive across every command.

## Handoff

`TK-UPGRADE-002` is done. The next low-risk candidates are `TK-UPGRADE-004`
for MCP inventory/schema tests or `TK-UPGRADE-003` after the MCP error-semantics
gate is resolved.
