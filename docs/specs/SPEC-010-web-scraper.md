---
title: "SPEC-010: Web Scraping Utility"
status: draft
date: 2026-06-29
scope: "Add tk scrape command and tk fetch subcommand for web content retrieval with proper cleanup"
owners: []
related: []
packages: []
---

# SPEC-010: Web Scraping Utility

> `tk scrape` and `tk fetch` fetch URLs and extract clean content (text, markdown, or structured data) with proper connection cleanup, timeouts, rate limiting, and cache management. Designed for agents that need web context without orphaned connections or brittle HTML parsing.

## Problem

LLM agents constantly fetch web content: docs pages, GitHub issues, blog posts, API references. The usual approaches are fragile — shelling out to `curl` (no cleanup, no structured output), ad-hoc Python scripts (dependency-heavy), or agent built-in fetchers (inconsistent across tools). A dedicated scraper in `tk` gives every agent a consistent, well-behaved fetch path with guaranteed teardown.

## Goals

- `tk fetch <url>` — fetch a URL and emit clean text/markdown
- `tk scrape <url>` — fetch + extract main content (article/markdown body), stripping navigation/ads/headers
- Connection cleanup: timeout, TCP connection pooling, explicit close on completion
- Rate limiting: configurable delay between requests, max concurrent connections
- Cache: optional disk cache with TTL (avoid re-fetching docs every session)
- User-agent control, cookie handling, redirect following
- `--format json` with metadata (status, content-type, content-length, fetch time)
- Respect robots.txt (optional, opt-in via `--respect-robots`)
- Output to stdout by default or `--output file`

## Non-Goals

- No headless browser / JS rendering — HTML-to-text only. JS rendering is a separate feature (requires browser dep).
- No DOM manipulation or element-specific queries (use `tk scrape --selector` in future)
- No form submission, login flows, or session management
- No websocket support
- No heavy dependency like headless Chrome — pure HTTP + HTML parsing

## Proposed Design

### `tk fetch <url> [options]`

```
tk fetch https://example.com
tk fetch https://example.com/api/docs --format json
tk fetch https://example.com --output docs.md
```

Fetches raw content and converts to clean markdown (or text via `--mode text`).

JSON shape:
```json
{
  "url": "https://example.com/docs",
  "status": 200,
  "content_type": "text/html",
  "content_length": 45210,
  "fetch_time_ms": 342,
  "cached": false,
  "content": "# Documentation\n\n...markdown output..."
}
```

### `tk scrape <url> [options]`

```
tk scrape https://example.com/blog/post
tk scrape https://en.wikipedia.org/wiki/Rust_(programming_language) --mode text
tk scrape https://docs.aws.amazon.com/lambda/latest/dg/welcome.html --selector ".main-content"
```

Like fetch but passes content through a readability/boilerplate extractor. Uses a heuristic DOM parser to identify the main content region (article tag, largest text block, etc.).

Options:
- `--mode text|markdown|html` — output format
- `--selector CSS_SELECTOR` — extract a specific element
- `--strip-tags IMG,SCRIPT,STYLE` — additional tags to remove

### Connection management

Use `reqwest` (already a well-known Rust HTTP crate). Key behaviors:
- Default 30s timeout (configurable with `--timeout SECS`)
- Max 4 concurrent connections (configurable with `--concurrent N`)
- Connection pool with idle timeout
- `Drop` handler ensures all connections close — no orphaned TCP connections
- Explicit abort on Ctrl+C via tokio signal handling

### Rate limiting

Simple token-bucket: `--delay SECS` between requests (default 0). `--concurrent N` for parallel fetches. Sequential by default for politeness.

### Caching

- `--cache DIR` — enable disk cache to `DIR` (default: `~/.cache/tk/fetch/`)
- `--ttl SECS` — cache TTL (default: 3600 = 1 hour)
- Cache keyed by URL, invalidated by TTL
- `--no-cache` to force re-fetch
- `tk scrape --cache-clear` to wipe cache

## Dependencies

New Rust crate: `reqwest` (HTTP client). Optional: `scraper` (HTML parser + CSS selectors). This is the first network dependency for tk — consider making it an optional feature flag (`tk scrape --features network`).

## File Touchpoints

| File or area | Action | Reason |
| --- | --- | --- |
| `src/commands/mod.rs` | Add `pub mod fetch;` | Register |
| `src/commands/fetch.rs` | Create | Fetch + scrape implementation |
| `src/main.rs` | Add `Commands::Fetch(FetchArgs)` + `Commands::Scrape(ScrapeArgs)` | CLI routing |
| `Cargo.toml` | Add `reqwest` (optional, feature-gated) | HTTP client |
| `Cargo.toml` | Add `scraper` (optional, feature-gated) | HTML parsing |
| `src/mcp.rs` | Add `fetch` + `scrape` tools (behind feature flag) | Agent access |
| `tests/` | Add integration tests against local HTTP test server | Verify fetching |
| `docs/specs/README.md` | Register spec | — |

## Rollout Plan

Phase 1: `tk fetch` with basic HTTP GET, timeout, markdown conversion. No cache, no rate limiting. Feature-gated behind `--features net`.

Phase 2: `tk scrape` with readability extraction, caching, rate limiting.

Phase 3: MCP exposure.

## Acceptance Criteria

- `tk fetch https://example.com` returns markdown with page title and body
- `tk fetch https://example.com --format json` returns valid JSON with status/content
- Timeout works: `tk fetch https://httpbin.org/delay/10 --timeout 2` fails with timeout error
- Connections are cleaned up after command exits (verify with netstat)
- `--output FILE` writes to file
- Feature-gated: `tk fetch` without `--features net` gives clear "not compiled with network support" error

## Verification

```bash
tk fetch https://example.com
tk fetch https://httpbin.org/json --format json | jq '.status'
tk fetch https://httpbin.org/delay/5 --timeout 1  # should fail
```

## Rollback

Revert the PR (or unset the feature flag). Entirely additive behind a cargo feature.

## Open Questions

- Feature flag name: `net`, `network`, `fetch`, or `scrape`? `net` is cleanest.
- Should `tk scrape` be a subcommand of `tk fetch` (`tk fetch --scrape`) or a separate command? Separate command is more discoverable and has different options.
- Is reqwest the right choice or should we use `ureq` (simpler, no async)? ureq is simpler (blocking HTTP, no tokio dep). For a CLI tool, blocking is fine. Use `ureq` to avoid the tokio dependency entirely.
- How to handle JS-rendered pages? Out of scope for v1, but worth noting in docs.
