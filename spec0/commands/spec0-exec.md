---
description: Execute the next Spec0 implementation slice and verify it
argument-hint: [slice, goal, or empty for next obvious slice]
---

# Spec0 Exec

Execute the next Spec0 slice:

`$ARGUMENTS`

## Procedure

1. Re-read the current spec, plan, or latest handoff if one exists.
2. Confirm the next smallest useful implementation slice.
3. Inspect the files that own that behavior before editing.
4. Make scoped changes only for that slice.
5. Run the narrowest relevant verification.
6. Update the working notes or final response with changed files, checks, and remaining slices.

If there is no existing Spec0 plan, create a minimal frame first, then execute the first safe slice.
