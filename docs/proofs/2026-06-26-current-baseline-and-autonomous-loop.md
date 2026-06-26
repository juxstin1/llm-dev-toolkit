# Proof: Current Baseline And Autonomous Loop

Date: 2026-06-26
Scope: Updated `main` to the latest `origin/main`, verified the Rust CLI/MCP
and demo surfaces, and added the repo-local autonomous Spec0 loop.
Source ticket/spec: `SPEC-000`, user `/goal`.
Git state: `main...origin/main`; clean before docs edits, dirty with only docs
changes after the loop patch.

## Changed Files

- `docs/README.md`
- `docs/CODEBASE_MAP.md`
- `docs/plans/spec-development-roadmap.md`
- `docs/proofs/README.md`
- `docs/proofs/2026-06-26-current-baseline-and-autonomous-loop.md`
- `docs/runbooks/README.md`
- `docs/runbooks/autonomous-loop.md`
- `docs/specs/README.md`
- `docs/specs/SPEC-000-planning-system.md`
- `docs/tickets/INDEX.md`
- `docs/tickets/TEMPLATE.md`
- `tickets/_build_order.md`

## Commands

```powershell
git fetch --all --prune
git switch main
git merge --ff-only origin/main
git status --short --branch
cargo fmt --all -- --check
cargo test --all
cargo clippy --all-targets -- -D warnings
cargo build --release
cd demo
npm.cmd run check
npm.cmd ci
npm.cmd run check
cd ..
git diff --check
rg -n "Known Current Bug Queue|four known|four confirmed|SPEC-001.*Git-aware|pick the traversal bug|Status: in progress|draft planning layer" docs tickets -g "!docs/proofs/*"
```

## Result

Pass.

- `git fetch --all --prune` showed the old feature branch remotes were deleted
  and `origin/main` advanced to `14f5d33`.
- `git switch main` and `git merge --ff-only origin/main` fast-forwarded the
  checkout from `de98e4b` to `14f5d33`.
- `cargo fmt --all -- --check` passed.
- `cargo test --all` passed: 58 unit tests and 45 CLI/MCP integration tests.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo build --release` passed.
- The first `npm.cmd run check` failed because `demo/node_modules` was missing.
  `npm.cmd ci` installed 189 locked packages with 0 vulnerabilities, then
  `npm.cmd run check` passed.
- `git diff --check` passed.
- The stale-text guard returned no matches outside proof files after the wording
  cleanup.

## Decision Gates

- No product code changed.
- Public README and bundled `spec0/commands/*.md` prompts were not changed in
  this slice because prompt/docs behavior is user-facing.
- Root `tickets/` drafts were not moved or deleted.
- Draft upgrade candidates that change public CLI or MCP behavior remain
  `draft` until their decision gates are resolved.

## Gaps

- No markdown link checker is configured.
- No live GitHub issue creation was attempted.
- CI was not queried; this proof is local Windows verification.

## Handoff

The planning layer now has an autonomous loop runbook, a ticket template, proof
requirements, and draft upgrade candidates. `TK-UPGRADE-002` was selected as
the first low-risk test-only slice after this control-plane update. Write
`SPEC-003`/`SPEC-004` before broader CLI/MCP behavior changes.
