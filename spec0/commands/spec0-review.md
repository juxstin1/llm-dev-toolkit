---
description: Review current work against the Spec0 frame
argument-hint: [focus area or empty for full working-tree review]
---

# Spec0 Review

Review the current work against Spec0:

`$ARGUMENTS`

## Review Order

1. Reconstruct the intended goal from the spec, plan, prompt, or latest handoff.
2. Inspect the diff and changed files.
3. Check for behavioral regressions, missing tests, unsafe assumptions, and scope creep.
4. Verify that the implementation matches the repo's existing conventions.
5. Run or recommend the most relevant verification if it has not already run.

Lead with findings ordered by severity. Use file and line references for actionable issues. If there are no issues, say so and name any remaining test gaps or residual risk.
