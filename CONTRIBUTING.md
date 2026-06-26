# Contributing

Thanks for improving `tk`. Keep changes small, evidence-backed, and easy to
verify.

## Development Setup

1. Install a stable Rust toolchain from <https://rustup.rs/>.
2. Clone the repository.
3. Build and test locally.

```bash
git clone https://github.com/juxstin1/llm-dev-toolkit.git
cd llm-dev-toolkit
cargo test --all
```

The demo project under `demo/` uses Node.js 22:

```bash
cd demo
npm ci
npm run check
```

## Before Opening A PR

Run the standard gates:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo build --release
```

If you touch the demo:

```bash
cd demo
npm ci
npm run check
```

## Pull Request Scope

- Keep each PR focused on one behavior, spec, bug, or maintenance slice.
- Add or update tests for behavior changes.
- Update README, docs, tickets, or proof logs when user-visible contracts
  change.
- Avoid unrelated refactors in feature or bug-fix PRs.
- Do not commit generated output from `target/`, `demo/node_modules/`, or
  `demo/out/`.

## Issues And Specs

Use issues for bugs, feature requests, and behavior changes. For larger work,
start with the repo-local planning docs:

- `docs/README.md`
- `docs/tickets/INDEX.md`
- `docs/specs/README.md`
- `docs/runbooks/autonomous-loop.md`

## Coding Style

- Prefer existing command-module patterns.
- Keep command output stable unless the change is explicitly about output.
- Preserve JSON output as machine-readable data with no ANSI escapes.
- Keep MCP tools read-only unless a future accepted spec changes that contract.
- Return helpful non-zero errors for invalid inputs.

## Security

Do not put credentials, tokens, exploit details, or sensitive user data in
issues, PRs, commits, tests, or proof logs. Follow [SECURITY.md](SECURITY.md)
for vulnerability reports.
