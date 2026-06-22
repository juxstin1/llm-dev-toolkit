# Proof: ARCH-002 MCP Tool Table

Date: 2026-06-22
Scope: Unify MCP tool schemas and CLI argument builders.

## Changes Verified

- `ToolDef` now owns the MCP tool name, description, schema, and CLI argument
  builder.
- `tools/list` still advertises schemas from the tool table.
- `tools/call` now resolves the same tool table before building CLI args.
- Unit coverage verifies every advertised tool has a builder.
- Unit coverage verifies flag mapping for all 12 exposed MCP tools.

## Commands

```powershell
cargo fmt --all -- --check
cargo test mcp::tests
cargo test --all
npm.cmd run check
cargo clippy --all-targets -- -D warnings
cargo build --release
```

## Result

- `cargo test mcp::tests` passed with 9 MCP unit tests.
- `cargo test --all` passed with 58 unit tests and 41 integration tests.
- `npm.cmd run check` passed in `demo/`.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo build --release` passed.

## Build Order Status

All build-order slices are implemented and verified.
