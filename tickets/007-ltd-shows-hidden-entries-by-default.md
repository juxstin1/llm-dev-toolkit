# `ltd -L` shows hidden entries by default unlike `tree -L`

## Summary

`tk ltd -L <n>` is documented as a depth-limited tree shortcut, but it shows
hidden entries by default. `tk tree -L <n>` hides them unless `-a` is provided.

## Repro

```powershell
target\release\tk.exe tree -L 1
target\release\tk.exe ltd -L 1
```

## Actual

`tree -L 1` hides entries such as `.git`, `.gitattributes`, `.github`, and
`.gitignore`.

`ltd -L 1` shows those hidden entries.

## Expected

`ltd -L 1` should behave like `tree -L 1`, only adding the shorter command name
for depth-limited tree output. Hidden entries should remain hidden unless an
explicit all/hidden flag exists.

## Likely Cause

`src/commands/tree.rs::run_depth` calls `print_tree` and `build_node` with
`show_all = true`.

## Suggested Fix

Change `run_depth` to use `show_all = false`, or reconsider whether `ltd`
should exist as a separate command instead of an alias-like wrapper around
`tree -L`.

## Test Coverage

Add integration tests that compare `tk tree -L 1` and `tk ltd -L 1` hidden-file
behavior in text and JSON modes.
