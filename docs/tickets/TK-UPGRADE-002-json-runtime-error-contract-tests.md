# TK-UPGRADE-002: JSON Runtime Error Contract Tests

Status: done
Priority: P1
Created: 2026-06-26
Related spec: `SPEC-003` candidate
Proof target: `docs/proofs/2026-06-26-tk-upgrade-002-json-runtime-errors.md`

## Problem

`tk --format json` is intended to keep stdout empty and emit a structured
`{"error": ...}` JSON object on stderr when command execution returns an error.
That contract is implemented in `src/main.rs`, but the integration tests mostly
cover text-mode invalid inputs.

## Current State Evidence

| Area | Evidence |
| --- | --- |
| Runtime error path | `src/main.rs` emits `serde_json::json!({ "error": e })` to stderr when `commands::json_enabled()` is true and a command returns `Err`. |
| Existing text tests | `tests/cli.rs` covers invalid checksum, `ff -t`, `sort --by`, and `dups -m` in text mode. |
| Test gap | No focused integration test asserts JSON stderr shape for runtime errors. |

## Goals

- Add integration tests for JSON-mode runtime errors.
- Assert stdout stays empty on error.
- Assert stderr is parseable JSON with an `error` string.

## Non-Goals

- No CLI behavior changes.
- No source changes unless the tests expose a bug.
- No broad `SPEC-003` implementation.

## Scope

Read scope:

- `src/main.rs`
- `src/commands/*.rs`
- `tests/cli.rs`

Write scope:

- `tests/cli.rs`
- `docs/tickets/TK-UPGRADE-002-json-runtime-error-contract-tests.md`
- `docs/proofs/2026-06-26-tk-upgrade-002-json-runtime-errors.md`
- `docs/tickets/INDEX.md`

Do not touch:

- Public README behavior docs.
- MCP schema or implementation.
- CLI parser behavior.

## Acceptance Criteria

- JSON-mode unsupported checksum algorithm exits non-zero.
- JSON-mode JSON object expectation failure exits non-zero.
- Both failures leave stdout empty.
- Both failures emit stderr parseable as JSON with an `error` string.

## Verification

```powershell
cargo test --test cli json_runtime_error
cargo fmt --all -- --check
cargo test --all
```

## Subagent Contract

Goal: Add focused integration tests for JSON-mode runtime errors.

Context:

- This is a test-only slice.
- The worker is not alone in the codebase.
- Do not revert edits made by others.

Read scope:

- `tests/cli.rs`
- `src/main.rs`
- `src/commands/checksum.rs`
- `src/commands/json.rs`

Write scope:

- `tests/cli.rs`

Decision gates:

- Stop if source behavior needs to change.
- Stop before changing public CLI behavior or JSON error semantics.
- Stop if verification requires external services.

Required final report:

```text
Summary:
Changed files:
Verification:
Decision gates hit:
Residual risks:
```

## Rollback

Remove the added tests and this ticket/proof update.

## Handoff

Implemented as a test-only slice in `tests/cli.rs`. No runtime source behavior
changed. See
`docs/proofs/2026-06-26-tk-upgrade-002-json-runtime-errors.md`.
