---
id: MAINT-001
title: add CI coverage for the demo project
lens: maintainability
severity: S3
confidence: high
effort: S
blast_radius: CI plus demo package
files:
  - README.md:135
  - demo/package.json:6-9
  - .github/workflows/ci.yml:25-32
depends_on: []
tags: [ci, demo]
status: done
---

## What

The README documents the Remotion demo as part of the repo, and `demo` has its
own npm scripts, but CI only runs Rust checks and never installs or checks the
demo package.

## Why it matters

A contributor can break the demo animation or TypeScript setup without any CI
signal, even though the README points new contributors at `npm install`,
`npm run studio`, and `npm run render`.

## Evidence

- `README.md:135` - contributor instructions describe the Remotion demo workflow.
- `demo/package.json:6-9` - the demo package defines `studio`, `render`, and `still` scripts.
- `.github/workflows/ci.yml:25-32` - CI runs only Rust format, clippy, test, and release build steps.

## Suggested direction

Add a lightweight CI job for `demo` that installs with `npm ci` and runs a
non-rendering TypeScript/build check if available. Avoid full video rendering in
the first step unless runtime cost is acceptable.

## Open questions

Should the demo have a dedicated `npm run check` script before CI is wired to
it?

## Implementation

Implemented in build slice 9. The demo now has `npm run check`, and CI runs
`npm ci` plus that check in a dedicated demo job. Proof:
`docs/proofs/2026-06-22-maint-001-demo-ci.md`.
