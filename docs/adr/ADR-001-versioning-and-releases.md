---
title: "ADR-001: Semantic Versioning And Tagged Releases"
status: accepted
date: 2026-07-29
owners:
  - repository maintainer
---

# ADR-001: Semantic Versioning And Tagged Releases

## Decision

The project uses semantic versions in `Cargo.toml` and matching annotated tags
named `vMAJOR.MINOR.PATCH`. The executable remains `tk`; the crates.io package
is `llm-dev-toolkit` because the package name `tk` is owned by an unrelated
project.

Every release publishes:

- `x86_64-unknown-linux-gnu` as `.tar.gz`;
- `x86_64-pc-windows-msvc` as `.zip`;
- `aarch64-apple-darwin` as `.tar.gz`;
- a SHA-256 file beside every archive;
- the `llm-dev-toolkit` crate, which installs the `tk` binary.

Patch releases contain compatible fixes. Minor releases add compatible
commands, flags, or output fields. Major releases may remove or change existing
CLI, JSON, configuration, or MCP contracts.

## Verification

The release workflow rejects a tag that does not exactly match the manifest
version. CI runs `cargo package --locked`; tagged builds compile locked release
binaries on each target before either GitHub or crates.io publication begins.

## Consequences

The repository environment named `crates-io` must contain a
`CRATES_IO_TOKEN` secret. A tag is not created until the normal CI workflow is
green on the release commit.
