---
title: "Build Order: Gap Specs"
type: plan
date: 2026-06-29
status: draft
supersedes: []
related:
  - SPEC-006 (git integration)
  - SPEC-007 (context window)
  - SPEC-008 (symbol extraction)
  - SPEC-009 (project detection)
  - SPEC-010 (web scraper)
  - SPEC-011 (MCP file reading)
  - SPEC-012 (config system)
---

# Build Order: Gap Specs

## 1. Audit Summary

### File-Conflict Matrix

Every command spec touches the same three files. This is the primary sequencing constraint:

| File | Touched by |
| --- | --- |
| `src/main.rs` | SPEC-006, -007, -008, -009, -010, -012 (config) |
| `src/commands/mod.rs` | SPEC-006, -007, -008, -009, -010 |
| `src/mcp.rs` | SPEC-006, -007, -008, -009, -011 |
| `Cargo.toml` | SPEC-010 (`ureq` + `scraper`), SPEC-012 (`toml` for config parse) |
| New files | Each spec creates `src/commands/<name>.rs` |

Verdict: **No two specs can be implemented in parallel without merge conflicts** on `main.rs` (Commands enum + match arms) and `mod.rs` (module declarations). Sequential implementation in a single branch is the safest path.

### Dependency Chain

```
SPEC-012 (config)     -- standalone, no dependencies
  |
  v
SPEC-009 (detect)     -- reads filesystem state, no deps on other commands
SPEC-006 (git)        -- shells out to git binary, no Rust deps
SPEC-008 (symbols)    -- regex patterns, no Rust deps
  |
  v
SPEC-007 (context)    -- reads files, could consume git diff + symbols later
  |
  v
SPEC-011 (MCP file)   -- only touches mcp.rs, fully independent
SPEC-010 (scraper)    -- only spec with new Cargo deps, feature-gated
```

No spec strictly depends on another at the code level. The arrows above are conceptual ordering for cleanliness, not hard prerequisites.

### Risk Assessment

| Spec | Risk | Why |
| --- | --- | --- |
| 006 (git) | Low | Shells out to `git`, pure text parsing, no new deps |
| 008 (symbols) | Low | Regex-only, well-understood patterns |
| 009 (detect) | Low | File-existence checks + heuristics |
| 011 (MCP read) | Low | Wraps existing `cat` via re-invocation |
| 007 (context) | Medium | Token estimation is heuristic; truncation logic needs edge-case testing |
| 010 (scraper) | Medium | First network dep, needs feature gate, error handling for flaky HTTP |
| 012 (config) | Medium | Design must be extensible for all future specs; config file discovery adds complexity |

### Command-Name Audit

No name collisions between specs. MCP tool names `read_file`/`read_lines` (SPEC-011) don't overlap with CLI commands.

### Conceptual Overlap

- SPEC-006 (git) and SPEC-009 (detect): both can report git state. SPEC-009 should delegate to SPEC-006's `status` output for branch info rather than re-parsing `.git/HEAD`.
- SPEC-007 (context) and SPEC-008 (symbols): `tk context --symbols` could inline symbol extraction. Not in initial scope but worth keeping the API compatible.
- SPEC-010 (scraper) and SPEC-007 (context): future `tk context` could accept scraped URLs as input sources alongside local files.

---

## 2. Config System (SPEC-012)

The user needs to customize which features are enabled. This must land first so every subsequent spec can integrate with it from day one.

### Design

**Config file locations** (checked in order, later overrides earlier):
1. `~/.config/tk/config.toml` (or `$XDG_CONFIG_HOME/tk/config.toml`)
2. `.tkconfig.toml` in current directory or any parent (project-local, overrides global)

**Format** (TOML, using `toml` crate with serde):

```toml
[features]
# Toggle command groups. Disabled commands show a clear "not enabled" message.
# Defaults: all true (backward compatible).
git = true
fetch = false
symbols = true
detect = true
context = true

[defaults]
# Global defaults overridden by CLI flags.
format = "text"     # "text" or "json"
color = "auto"      # "auto", "always", "never"

[commands.status]
# Per-command default arguments.
branch = true

[commands.diff]
context = 3
staged = false

[commands.context]
max-tokens = 4000
```

**Behavior:**
- Config is optional — missing = all features on, same as today.
- Disabled commands still parse (clap arg tree is intact) but show:
  ```
  $ tk fetch https://example.com
  Error: 'fetch' is not enabled. Set [features].fetch = true in ~/.config/tk/config.toml
  ```
- Feature gating is runtime-configurable, not compile-time (except SPEC-010 which uses `--features net`).
- `tk config` subcommand: print current effective config, show config file path.

### Cargo dep to add

```toml
toml = "0.8"
```

`serde` + `serde_json` already present.

### File touchpoints

| File | Action |
| --- | --- |
| `src/config.rs` | Create — config loading, merging, querying |
| `src/main.rs` | Load config at startup, pass to command dispatch |
| `src/commands/mod.rs` | No change |
| `Cargo.toml` | Add `toml` dep |

### Acceptance

- `tk config` prints effective config as TOML
- Setting `[features].git = false` makes `tk status` return an error
- Setting `[defaults].format = "json"` makes every command default to JSON output
- Project-local `.tkconfig.toml` overrides global `~/.config/tk/config.toml`
- With no config file, all features are enabled (identical to current behavior)

---

## 3. Build Order

All work in a single branch (`feat/gap-specs`) with sequential commits. Each commit is independently buildable and testable.

```
feat/gap-specs
│
├── commit 1: Config system (SPEC-012)
│   └── src/config.rs, Cargo.toml (+toml), main.rs (load at startup)
│   └── builds, tests pass, tk config works
│
├── commit 2: Project detection — tk detect (SPEC-009)
│   └── src/commands/detect.rs, mod.rs, main.rs
│   └── detects Rust/TS/Python/Go/Java projects
│
├── commit 3: Git integration — tk status/diff/log/branch (SPEC-006)
│   └── src/commands/git.rs, mod.rs, main.rs
│   └── shells out to git plumbing, no new Rust deps
│
├── commit 4: Symbol extraction — tk symbols (SPEC-008)
│   └── src/commands/symbols.rs, mod.rs, main.rs
│   └── regex patterns for Rust/Python/TS/Go/Java
│
├── commit 5: Context window — tk context (SPEC-007)
│   └── src/commands/context.rs, mod.rs, main.rs
│   └── file concatenation + token heuristic + --max-tokens
│
├── commit 6: MCP file reading — read_file/read_lines (SPEC-011)
│   └── src/mcp.rs only (adds 2 tools + arg builders)
│   └── no CLI changes, pure MCP addition
│
└── commit 7: Web scraper — tk fetch/tk scrape (SPEC-010)
    └── src/commands/fetch.rs, mod.rs, main.rs, Cargo.toml (+ureq, +scraper)
    └── feature-gated behind --features net
    └── tk fetch without feature gives clear compile error
```

### Why this order

| Reason | Detail |
| --- | --- |
| Config first | Every subsequent command can check `[features]` gates from day one |
| Simple before complex | Detect → Git → Symbols are all no-new-deps, low-risk |
| Context after git+symbols | Context conceptually could consume git/symbols output (future) |
| MCP late | Pure MCP change, no CLI implications, easy to verify |
| Scraper last | Only spec needing new deps + feature gate; cleanest as a capstone |

### Parallelism warning

Do NOT split these into parallel PRs. Every spec touches `main.rs` and `mod.rs`. Merging parallel branches will create conflicts on the `Commands` enum and match arms every time. Sequential commits in one branch avoid this entirely.

---

## 4. Config Integration Per Spec

Each spec should register itself with the config system:

| Spec | Config section | Gate key |
| --- | --- | --- |
| SPEC-006 (git) | `[features]` | `git` |
| SPEC-007 (context) | `[features]` | `context` |
| SPEC-008 (symbols) | `[features]` | `symbols` |
| SPEC-009 (detect) | `[features]` | `detect` |
| SPEC-010 (scraper) | `[features]` | `fetch` |
| SPEC-011 (MCP read) | no gate (always on) | — |

Per-command defaults in `[commands.<name>]` for flags like `--context N`, `--max-tokens N`, etc.

---

## 5. Verification Strategy

After each commit:

```bash
cargo build --release
cargo clippy --all-targets -- -D warnings
cargo test --all
```

After SPEC-010 (scraper), also:

```bash
cargo build --release --features net
cargo test --all --features net
cargo build --release --no-default-features  # verify scraper excluded
```

Full integration sweep after the final commit:

```bash
tk config                                    # print effective config
tk detect                                    # should show Rust
tk status --format json | jq '.branch'       # git status
tk diff                                      # working tree diff
tk log -n 3                                  # recent commits
tk branch                                    # current branch
tk symbols src/ --kind fn                    # function symbols
tk context src/main.rs --max-tokens 100      # truncated context
tk fetch https://example.com                 # scraper (--features net)
```

---

## 6. Open Questions

- **Config format**: TOML vs JSON vs YAML? TOML is standard for Rust CLI tools (cargo, clippy use it), serde support is mature, and the toml crate has zero deps beyond serde. Recommend TOML.
- **Config hot-reload**: Not needed — config is read once at startup. If they change config, they restart tk.
- **Project config security**: Should `.tkconfig.toml` be in `.gitignore`? Recommend NOT — it should be committed for team-shared defaults. Personal overrides go in `~/.config/tk/config.toml`.
- **SPEC-010 feature gate**: Should `--features net` be a Cargo feature or a runtime config? Cargo feature controls whether `ureq` + `scraper` are compiled. Runtime `[features].fetch` controls whether the command is usable. Both: Cargo for compilation, runtime for usability.
