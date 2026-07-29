---
title: "SPEC-004: MCP Tool Schema And CLI Parity"
status: implemented
date: 2026-07-29
owners:
  - repository maintainer
---

# SPEC-004: MCP Tool Schema And CLI Parity

## Contract

`tk mcp` exposes exactly 23 canonical tools. Human CLI aliases and the newer
`show`, `scan`, and `completions` commands do not add MCP tool names.

Each tool call:

1. requires an argument object;
2. rejects unknown properties;
3. validates required keys, scalar types, string-array items, non-negative
   integers, and enumerated values;
4. maps valid arguments to the canonical CLI;
5. launches the current executable with `--format json --color never`;
6. reports command failures with MCP `isError: true`.

Every advertised input schema sets `additionalProperties: false`. Mutating CLI
commands such as `clip`, `extract`, Spec0 installation, and duplicate deletion
remain unavailable through MCP. `fetch` and `scrape` perform outbound HTTP
requests but do not write local files through their MCP schemas.

## Verification

Tests lock the sorted tool-name inventory, schema closure, every builder,
mixed-type arrays, negative integers, unknown keys, non-object arguments, and
representative child-command success and failure paths.
