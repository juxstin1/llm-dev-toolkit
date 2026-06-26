# Autonomous Loop Runbook

Snapshot date: 2026-06-26.

Use this runbook when a request asks for Spec0, autonomous development,
subagents, or "make the project legitimate" style improvement work. The loop is
designed to keep Codex moving without asking for every small reversible choice,
while preserving decision gates for product meaning, architecture, security,
credentials, deployments, and destructive actions.

## Operating Contract

- Start from the live checkout, not older proof notes.
- Work from an implementation-ready ticket or create one before product code
  changes.
- Keep each slice small enough to verify with the standard Rust gates.
- Use subagents only for bounded tasks with disjoint write scopes.
- Record proof for completed work before claiming completion.
- Ask only at decision gates or when the next step is not safely reversible.

Reversible calls Codex may make without asking:

- Docs, tests, and local proof updates.
- Small code changes that follow an established local pattern.
- Installing locked development dependencies such as `npm ci` when needed for a
  declared check.
- Running local build, lint, test, and smoke checks.

Stop and ask before:

- Public CLI behavior changes that alter command meaning or output contracts.
- MCP schema or error-semantics changes that downstream clients may depend on,
  unless already covered by an accepted spec or ticket.
- Architecture, release, versioning, security, privacy, credential, deployment,
  or paid-service decisions.
- Destructive filesystem or git actions.
- Moving or deleting historical ticket/proof artifacts.

## Loop

1. Orient
   - Run `git status --short --branch`.
   - If freshness matters, run `git fetch --all --prune`, then compare the
     active branch with its upstream.
   - Read `docs/README.md`, `docs/tickets/INDEX.md`, this runbook, and the
     relevant spec or latest proof.
   - Use `rg --files` and targeted `rg` to find source, tests, docs, and noisy
     paths.

2. Research
   - Gather file-backed evidence for the suspected gap.
   - Prefer a read-only subagent for independent research questions.
   - Keep external research limited to current official docs when library,
     protocol, or tool behavior may have changed.
   - Return evidence as paths, commands, outputs, and open questions.

3. Ticket
   - If no ready ticket exists, draft one from
     `docs/tickets/TEMPLATE.md`.
   - A ticket is not ready until it names the problem, current evidence,
     non-goals, read/write scope, acceptance criteria, verification commands,
     proof target, rollback, and decision gates.
   - Use `draft` for research output and `ready` only when a future agent can
     implement without rediscovering the problem.

4. Plan
   - Split the ticket into the smallest useful implementation slice.
   - Assign subagents only when write scopes do not overlap.
   - Keep the critical path local when integration risk is high.

5. Implement
   - Make the first useful slice real.
   - Stay inside the ticket's write scope.
   - If the implementation expands the behavior contract, update the ticket or
     spec before continuing.

6. Verify
   - Run the narrowest relevant check first.
   - Run the standard gates when Rust behavior, MCP behavior, or public CLI
     behavior changed:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

   - For the demo surface:

```powershell
cd demo
npm ci
npm run check
```

7. Proof
   - Add or update a proof under `docs/proofs/`.
   - Include git state, source ticket/spec, commands, pass/fail result, changed
     files, gaps, and any decision gates hit.

8. Handoff
   - Update `docs/tickets/INDEX.md` when a ticket changes status.
   - Final reports should name changed files, verification, residual risks, and
     the next ready or draft ticket.

## Subagent Contract

Every subagent task must include:

- Goal.
- Context.
- Read scope.
- Write scope.
- Non-goals.
- Acceptance criteria.
- Verification command or check.
- Decision gates.
- Required final report format.

Worker instructions:

- You are not alone in the codebase.
- Do not revert edits made by others.
- Stay inside the write scope unless reporting a blocker.
- Return changed paths, verification commands, and residual risks.

Use read-only explorers for research and workers only for implementation slices
with disjoint file ownership.
