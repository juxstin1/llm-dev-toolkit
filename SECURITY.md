# Security Policy

## Supported Versions

`tk` is pre-1.0. Security fixes target the current `main` branch until formal
release channels exist.

## Reporting A Vulnerability

Please do not open a public issue with exploit details.

Use GitHub's private vulnerability reporting or security advisory flow if it is
available for this repository. If that path is unavailable, open a minimal issue
asking for a private reporting channel and omit sensitive details.

Useful report details:

- Affected command or MCP tool.
- Operating system.
- Exact version, commit, or branch.
- Minimal reproduction steps.
- Expected impact.
- Any known workaround.

## Scope

Security-sensitive areas include:

- Archive extraction path handling.
- Clipboard fallback storage.
- MCP tool exposure and read-only guarantees.
- File traversal boundaries.
- Handling of untrusted paths, JSON, and binary files.

## Maintainer Response

Maintainers should acknowledge reports, reproduce the issue, prepare a fix, and
publish a short advisory or release note when the risk is confirmed.
