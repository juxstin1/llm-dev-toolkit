# Proof: CLI Ergonomics Spec

Date: 2026-06-22
Scope: Finish `SPEC-001` for additive CLI ergonomics and workflow shortcuts.

## Evidence

- Four read-only subagent passes reviewed command aliases, workflow friction,
  docs/help discoverability, and implementation compatibility.
- Local help output was inspected for root help and representative commands:
  `ff`, `search`, `tree`, alias invocations, `json`, and `info`.
- Current implementation evidence came from `src/main.rs`, `src/mcp.rs`,
  `README.md`, and `tests/cli.rs`.

## Commands

```powershell
git diff --check
cargo fmt --all -- --check
cargo test --all
npm.cmd run check
cargo clippy --all-targets -- -D warnings
cargo build --release
```

## Result

`SPEC-001` is accepted and registered in `docs/specs/README.md`. It defines the
additive alias/flag contract, natural `info PATH`, help/README polish, and
follow-on `show` and `scan` workflows with JSON contracts, rollout,
acceptance criteria, verification, rollback, and remaining open questions.

- `git diff --check` passed.
- `cargo fmt --all -- --check` passed.
- `cargo test --all` passed with 58 unit tests and 41 integration tests.
- `npm.cmd run check` passed in `demo/`.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo build --release` passed.

## Gaps

No product code was changed in this slice. Implementation remains future work.
