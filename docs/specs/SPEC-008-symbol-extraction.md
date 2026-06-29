---
title: "SPEC-008: Symbol Extraction"
status: draft
date: 2026-06-29
scope: "Add tk symbols command for extracting function, class, struct, and interface definitions"
owners: []
related: []
packages: []
---

# SPEC-008: Symbol Extraction

> `tk symbols` extracts top-level and nested definitions (functions, classes, structs, traits, interfaces, enums) from source files using language-aware regex patterns, giving agents a codebase skeleton without reading every line.

## Problem

Agents currently read entire files to understand what symbols they contain. For large files this is wasteful — a 500-line file might have 3-4 public functions that are all the agent needs to know about. There's no standard CLI tool that gives a compact "table of contents" for source files across languages.

## Goals

- Extract symbol definitions from source files: functions, classes, structs, traits, interfaces, enums, type aliases
- Support Rust, Python, JavaScript, TypeScript, Go, Java initially (85%+ of agent-served codebases)
- `--kind fn|class|struct|trait|interface|enum|all` to filter by kind
- `--public-only` to show only public/exported symbols
- JSON output with name, kind, file, line range, signature line
- Fast: pure regex, no AST parser, no tree-sitter binary
- Respect .gitignore, walk directories

## Non-Goals

- No tree-sitter or language server integration (future if regex proves insufficient)
- No call graph or reference resolution
- No type information beyond what's in the signature
- No doc comment extraction (nice-to-have, but adds complexity)

## Proposed Design

### `tk symbols [paths...] [options]`

```
tk symbols src/
tk symbols src/main.rs --kind fn --public-only
tk symbols src/ --format json
```

Text output:
```
src/main.rs:
  fn main()                                        (1:0)
  struct Config                                    (15:0)
  enum Status                                      (30:0)

src/lib.rs:
  pub fn add(a: i32, b: i32) -> i32                (1:0)
  pub trait Serializable                            (10:0)
```

JSON output:
```json
[
  {
    "path": "src/main.rs",
    "symbols": [
      { "name": "main", "kind": "fn", "signature": "fn main()", "line": 1, "public": false },
      { "name": "Config", "kind": "struct", "signature": "struct Config", "line": 15, "fields": 3 },
      { "name": "Status", "kind": "enum", "signature": "enum Status", "line": 30, "variants": 2 }
    ]
  }
]
```

### Extraction strategy

Per-language regex patterns compiled once at startup. Patterns target the line containing the definition keyword. Multi-line signatures are detected by looking ahead for `{` or `where` clauses (limited lookahead, not a full parser).

| Language | Patterns |
| --- | --- |
| Rust | `fn\s+(\w+)`, `struct\s+(\w+)`, `enum\s+(\w+)`, `trait\s+(\w+)`, `impl\s+(\w+)` |
| Python | `def\s+(\w+)`, `class\s+(\w+)` |
| JS/TS | `function\s+(\w+)`, `class\s+(\w+)`, `interface\s+(\w+)`, `type\s+(\w+)`, `enum\s+(\w+)` |
| Go | `func\s+(\w+)`, `type\s+\w+\s+struct`, `type\s+\w+\s+interface` |
| Java | `(public|private|protected)?\s*(static)?\s*(class|interface|enum)\s+(\w+)` |

### Public detection

- Rust: `pub fn` / `pub struct` etc.
- Python: no leading `_` on name
- JS/TS: `export function` / `export class` / `export default`
- Go: capitalized name (PascalCase = exported)
- Java: `public` keyword

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `src/commands/mod.rs` | Add `pub mod symbols;` | Register |
| `src/commands/symbols.rs` | Create | Implementation |
| `src/main.rs` | Add `Commands::Symbols(SymbolsArgs)` | CLI routing |
| `tests/` | Add test fixtures per language | Verify extraction |
| `docs/specs/README.md` | Register spec | — |

## Rollout Plan

Single PR. Start with Rust + Python + TS (the big three for tk's audience). Go and Java in a follow-up if patterns are solid.

## Acceptance Criteria

- `tk symbols src/main.rs` shows all symbol kinds for that file
- `--kind fn` filters to functions only
- `--public-only` correctly filters per language conventions
- JSON output is valid and parseable
- Non-source files are skipped gracefully

## Verification

```bash
tk symbols src/commands/mod.rs
tk symbols src/ --kind fn --format json | jq 'map(.symbols | length) | add'
tk symbols --public-only Cargo.toml  # -> empty output (not code)
```

## Rollback

Revert the PR. Additive.

## Open Questions

- Should `impl` blocks in Rust be extracted? They're not really "symbols" but they are important for understanding Rust code. Proposal: `impl <Type>` as a symbol with `kind: "impl"`.
- What about generics? Extracting `fn foo<T: Debug>(x: T)` would require multi-line regex. Start with single-line signatures only.
