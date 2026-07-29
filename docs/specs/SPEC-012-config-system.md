---
title: "SPEC-012: Layered Configuration"
status: implemented
date: 2026-07-29
owners:
  - repository maintainer
---

# SPEC-012: Layered Configuration

## Discovery And Precedence

`tk` reads configuration once at startup from:

1. `$XDG_CONFIG_HOME/tk/config.toml`, or `~/.config/tk/config.toml`;
2. the nearest `.tkconfig.toml` from the current directory upward.

Project values override global values. Explicit CLI flags override both.

## Supported Values

- `[features]`: `git`, `fetch`, `symbols`, `detect`, and `context` booleans.
- `[defaults]`: `format = "text" | "json"` and
  `color = "auto" | "always" | "never"`.
- `[commands.status]`: `porcelain`.
- `[commands.diff]`: `context`, `staged`.
- `[commands.log]`: `count`.
- `[commands.branch]`: `all`.
- `[commands.info]`: `disk-usage`.
- `[commands.context]`: `max-tokens`, `include`, `exclude`, and
  `no-line-numbers`.

Missing configuration preserves the ordinary CLI defaults. Disabled features
parse normally and then fail nonzero with the exact key required to enable
them. `tk config` prints the merged values; `tk config --paths` reports
discovery locations.

## Verification

Integration tests run `tk` from a temporary project containing
`.tkconfig.toml`, prove feature denial, and prove command defaults affect JSON
output. Unit tests cover merging and typed accessors.
