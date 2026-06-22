---
id: ARCH-002
title: unify MCP tool schema and argument builders
lens: architecture
severity: S3
confidence: high
effort: M
blast_radius: 1 module, 12 MCP tools
files:
  - src/mcp.rs:43
  - src/mcp.rs:209
  - src/mcp.rs:423-424
  - src/main.rs:402-428
depends_on: []
tags: [mcp, schema-drift]
status: done
---

## What

MCP tool names, JSON schemas, and CLI argument translation live in separate
manual structures. Adding or changing one MCP tool requires updating multiple
branches in `src/mcp.rs` and keeping them aligned with `src/main.rs` dispatch.

## Why it matters

The current shape works, but it makes MCP parity fragile. A future tool or flag
can appear in `tools/list` but map incorrectly in `tools/call`, or vice versa,
without a compile-time signal.

## Evidence

- `src/mcp.rs:43` - `tool_defs()` builds the advertised MCP tool schemas.
- `src/mcp.rs:209` - `build_args()` separately maps tool names and JSON arguments to CLI args.
- `src/mcp.rs:423-424` - `tools_list_result()` consumes `tool_defs()` but not `build_args()`.
- `src/main.rs:402-428` - CLI dispatch is another independent command map.

## Suggested direction

Introduce a single MCP tool table that owns name, schema, and builder together,
then have `tools/list` and `tools/call` both consume that table.

## Open questions

Is the current duplication intentional to keep schemas easy to read, or would a
small table abstraction be acceptable?

## Implementation

Implemented in build slice 10. `ToolDef` now owns both schema metadata and the
CLI argument builder, and tests cover all advertised MCP builders. Proof:
`docs/proofs/2026-06-22-arch-002-mcp-tool-table.md`.
