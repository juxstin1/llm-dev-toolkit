# `tree -a` includes `.git` despite git-aware contract

## Summary

`tk tree -a` and `tk ltd -L <n>` show the `.git` directory. This contradicts the README and the shared walker contract that recursive file walks should never descend into `.git`.

## Repro

```powershell
cargo run -- tree -a -L 1
cargo run -- --format json tree -a -L 1
cargo run -- ltd -L 1
```

## Actual

The output includes `.git` as a tree entry.

## Expected

`.git` should be omitted consistently, even when hidden files are shown.

## Likely Cause

`src/commands/tree.rs` builds its own `ignore::WalkBuilder` in `build_node` and `print_tree` instead of using `walk_entries`, so it misses the explicit `.git` component filter in `src/commands/mod.rs`.

## Suggested Fix

Add the same `.git` component filter to the `tree` walkers or refactor `tree` to reuse the shared walker behavior.

## Test Coverage

Add integration tests for:

- `tk tree -a -L 1` excluding `.git`
- `tk tree -a -L 1 --format json` excluding `.git`
- `tk ltd -L 1` excluding `.git`
