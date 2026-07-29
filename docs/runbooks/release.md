# Release Runbook

Owner: repository maintainer.

Input required: the intended semantic version and concise release notes.

## One-Time Setup

1. Create the GitHub environment `crates-io`.
2. Add a crates.io API token as the environment secret `CRATES_IO_TOKEN`.
3. Protect the environment if publication should require manual approval.

The crates.io package is `llm-dev-toolkit`; it installs the executable `tk`.

## Release

1. Update `version` in `Cargo.toml` and run `cargo update -w` to refresh the
   root package entry in `Cargo.lock`.
2. Update `README.md` and release-facing docs.
3. Run:

   ```powershell
   cargo fmt --all -- --check
   cargo clippy --all-targets -- -D warnings
   cargo test --all
   cargo build --release --locked
   cargo package --locked
   npm.cmd ci --prefix demo
   npm.cmd run check --prefix demo
   ```

4. Merge the release commit only after CI is green.
5. Create and push an annotated tag matching the manifest exactly:

   ```powershell
   git tag -a v0.5.0 -m "tk v0.5.0"
   git push origin v0.5.0
   ```

6. Confirm the `Release` workflow publishes three archives, three checksum
   files, a GitHub Release, and the crates.io package.

## Recovery

- If validation fails, delete the incorrect remote tag, correct the manifest,
  and create a new tag. Never reuse a tag after any artifact was published.
- If GitHub publication succeeds but crates.io fails, correct the environment
  secret and rerun only the failed job. Crates.io versions are immutable.
- If a published binary is defective, issue a new patch version; do not replace
  existing archives silently.
