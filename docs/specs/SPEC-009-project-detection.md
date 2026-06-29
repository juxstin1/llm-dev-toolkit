---
title: "SPEC-009: Project Detection"
status: draft
date: 2026-06-29
scope: "Add tk detect command for auto-detecting project type, language, and toolchain"
owners: []
related: []
packages: []
---

# SPEC-009: Project Detection

> `tk detect` examines a directory and identifies the project type, language, build system, test framework, and key config files — giving agents an instant orientation to any codebase.

## Problem

When an agent enters a new repo, it has to probe for common files to figure out what it's dealing with: is this Rust or Python? Does it use pytest or Jest? Where's the config? This probing wastes 2-5 tool calls every session. A single `tk detect` call replaces all of them.

## Goals

- Detect primary language from config files and source extensions
- Identify build system, package manager, test framework, linter/formatter
- Surface paths to key config files (`Cargo.toml`, `package.json`, `tsconfig.json`, etc.)
- Report project name, version if detectable
- `--format json` for agent consumption
- `--verbose` for full detection details
- Fast: pure file existence checks + simple heuristics, no analysis

## Non-Goals

- No deep analysis (package version resolution, dependency tree, etc.)
- No runtime detection (Node version, Python version, etc.)
- No `tk init` or project scaffolding — detection only

## Proposed Design

### `tk detect [path] [--verbose]`

```
tk detect
tk detect /some/project --format json
tk detect --verbose
```

Text output (default):
```
Language:   Rust
Build:      Cargo
Test:       cargo test
Lint:       cargo clippy
Config:     Cargo.toml (1.0.0)
```

JSON output:
```json
{
  "name": "llm-dev-toolkit",
  "version": "0.1.0",
  "language": "Rust",
  "build_system": "cargo",
  "test_framework": "cargo-test",
  "linter": "clippy",
  "formatter": "rustfmt",
  "config_files": {
    "Cargo.toml": { "exists": true, "path": "Cargo.toml" },
    "rust-toolchain.toml": { "exists": false }
  },
  "scripts": {
    "test": "cargo test --all",
    "build": "cargo build --release",
    "lint": "cargo clippy --all-targets -- -D warnings"
  }
}
```

### Detection matrix

Heuristics checked in order of specificity:

| Project type | Detection signal | Language | Build | Test | Lint |
|---|---|---|---|---|---|
| Rust | `Cargo.toml` | Rust | cargo | cargo test | clippy |
| Node/JS | `package.json` | JavaScript | npm/yarn/pnpm | jest/vitest/mocha | eslint |
| Node/TS | `package.json` + `tsconfig.json` | TypeScript | npm/yarn/pnpm | jest/vitest | eslint/tsc |
| Python | `pyproject.toml` with `[build-system]` | Python | pip/poetry | pytest | ruff/flake8 |
| Python | `setup.py` or `requirements.txt` | Python | pip | pytest | flake8 |
| Go | `go.mod` | Go | go build | go test | golangci-lint |
| Java | `pom.xml` | Java | maven | mvn test | checkstyle |
| Java | `build.gradle` | Java | gradle | gradle test | — |
| Docker | `Dockerfile` | — | docker | — | hadolint |
| Generic | Most files `.rs` | Rust | — | — | — |
| Generic | Most files `.py` | Python | — | — | — |
| Generic | Most files `.ts`/`.tsx` | TypeScript | — | — | — |

### Detection is cumulative, not exclusive

A monorepo can have multiple languages. `--verbose` shows all detected systems. The primary language is the one with the most source files by extension.

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `src/commands/mod.rs` | Add `pub mod detect;` | Register |
| `src/commands/detect.rs` | Create | Detection logic |
| `src/main.rs` | Add `Commands::Detect(DetectArgs)` | CLI routing |
| `src/mcp.rs` | Add `detect` tool to MCP definitions | Agent access |
| `tests/` | Add fixtures for each project type | Verify detection |
| `docs/specs/README.md` | Register spec | — |

## Rollout Plan

Single PR with the detection matrix. Start with the top 5 (Rust, TS/JS, Python, Go, Java) — those cover 90%+ of repos agents encounter on this machine.

## Acceptance Criteria

- `tk detect` in a Rust project shows "Language: Rust"
- `tk detect` in a Node/TS project shows "Language: TypeScript"
- `tk detect --format json` produces valid JSON
- `tk detect` in an empty directory shows "Language: unknown"
- `tk detect` in a directory with no config files but with `.rs` files detects Rust via extension heuristic
- MCP exposes the tool

## Verification

```bash
tk detect                                       # in this repo
tk detect /some/node/project --format json
tk detect /empty/dir                            # -> language: unknown
```

## Rollback

Revert the PR. Additive.

## Open Questions

- Should scripts include the full command or just the tool name? Full command with common flags (more useful for agents).
- Should `tk detect` recommend install commands? Not in initial version — that's a `tk doctor` feature in the future.
- What about monorepo detection (Nx, Turborepo, workspaces)? Detect from root-level workspace config, but don't dive into each package. `--verbose` can list them.
