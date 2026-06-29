---
title: "SPEC-007: Context Window Manager"
status: draft
date: 2026-06-29
scope: "Add tk context command for token-aware file concatenation"
owners: []
related: []
packages: []
---

# SPEC-007: Context Window Manager

> `tk context` concatenates files with path headers and optionally truncates to a token budget. Built for agents that need to pack the most relevant files into a limited context window.

## Problem

Every LLM agent session starts with the same ritual: "read me these 5 files so I understand the codebase." Agents waste context budget on boilerplate (imports, test helpers, config) and have no standard way to say "give me the most important N thousand tokens from these paths." Existing tools like `cat`, `head`, and `wc -c` are untyped token-agnostic and don't handle truncation gracefully.

## Goals

- Accept a list of file paths and/or directory trees
- Prepend each file with a clear path header (`# --- path/to/file ---`)
- Optionally cap output to a token budget (`--max-tokens N`)
- Estimate token count using a fast approximate method (tiktoken-style heuristic, not an LLM call)
- Smart truncation: prefer dropping less-important regions (comments, blank lines, trailing content) rather than hard-cutting mid-line
- `--format json` output with structured chunks for agent-side re-ranking
- Glob patterns, .gitignore-aware

## Non-Goals

- No actual LLM tokenizer dependency (no tiktoken C bindings, no Python interop)
- No semantic reordering or relevance scoring — that's the agent's job
- No streaming output (files are read, concatenated, potentially truncated, then emitted)
- No file editing or mutation

## Proposed Design

### `tk context [paths...] [options]`

```
tk context src/main.rs src/lib.rs
tk context src/ --max-tokens 4000 --include '*.rs' --exclude '*_test.rs'
tk context src/ --max-tokens 8000 --format json
```

Output (text mode):
```
# --- src/main.rs ---
1: fn main() {
2:     println!("hello");
3: }

# --- src/lib.rs ---
1: pub fn add(a: i32, b: i32) -> i32 {
2:     a + b
3: }
```

JSON shape:
```json
{
  "total_tokens": 142,
  "total_bytes": 1024,
  "max_tokens": null,
  "truncated": false,
  "files": [
    {
      "path": "src/main.rs",
      "tokens": 42,
      "bytes": 310,
      "lines": 20,
      "truncated": false,
      "content": "fn main() {\n    println!(\"hello\");\n}\n"
    }
  ]
}
```

### Token estimation

Use a fast approximate model: count bytes ÷ 3.5 for code (empirical observations show ~3.5 bytes/token for Rust/TS/Python code). Optionally add per-line detection for comments/blanks to improve accuracy. This is intentionally fuzzy — exact tokenization requires the model's tokenizer.

### Truncation strategy

When `--max-tokens` is set:
1. Collect all file contents in order (paths listed first or discovered via walk)
2. Count estimated tokens
3. If under budget, emit everything
4. If over budget, truncate file-by-file from the *bottom* of the list, dropping files entirely if needed to fit
5. Mark each file with `truncated: true` if it was partially or fully dropped

### Walk mode

When a path is a directory, walk it respecting .gitignore and hidden-file rules (same as other tk commands). Glob `--include`/`--exclude` patterns filter within the walk.

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `src/commands/mod.rs` | Add `pub mod context;` | Register new module |
| `src/commands/context.rs` | Create | Implementation |
| `src/main.rs` | Add `Commands::Context(ContextArgs)` | CLI routing |
| `Cargo.toml` | No new deps needed (uses std, ignore, serde) | — |
| `tests/` | Add unit + integration tests | Verify truncation, JSON, walk |
| `docs/specs/README.md` | Register spec | — |

## Rollout Plan

Single PR. Core feature is the concat + truncation; JSON output and walk mode are in the same commit.

## Acceptance Criteria

- `tk context foo.txt bar.txt` outputs both files with path headers
- `tk context foo.txt --max-tokens 10` truncates to ~10 tokens
- `--format json` returns valid JSON with `total_tokens` and `files[].content`
- Walk mode respects .gitignore and hidden files
- Graceful error on missing file

## Verification

```bash
echo "hello world" > /tmp/test_context.txt
tk context /tmp/test_context.txt
tk context /tmp/test_context.txt --max-tokens 1 --format json | jq '.truncated'
```

## Rollback

Revert the PR. Additive change.

## Open Questions

- Should `--max-tokens` accept model-specific tokenizer presets (e.g. `--tokenizer cl100k_base`)? Future feature if there's demand.
- Should we support `--from-stdin` to pipe filenames in? Yes, useful for `tk ff "*.rs" | tk context --from-stdin`.
- Header format: markdown-style `# --- path ---` or plain `==== path ====`? Markdown-style is more readable and parsable.
