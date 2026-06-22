---
description: Run the full Spec0 loop for a development request
argument-hint: [goal or task]
---

# Spec0

Use Spec0 for this request:

`$ARGUMENTS`

Spec0 is a compact meta-framework for turning a rough development intent into a bounded, verifiable implementation loop.

## Loop

1. Orient: identify the repository, active branch, dirty files, relevant agent instructions, and likely source areas.
2. Frame: write a concise spec before editing. Include goal, non-goals, constraints, assumptions, target files, and verification.
3. Plan: split the work into the smallest safe sequence of implementation slices.
4. Execute: make the first useful slice real, following the repo's existing patterns.
5. Verify: run the narrowest meaningful checks first, then broader checks if the blast radius requires them.
6. Handoff: summarize what changed, checks run, unresolved risks, and next actions.

## Operating Rules

- Prefer existing project conventions over new abstractions.
- Keep changes scoped to the spec. If the work expands, update the spec first.
- Do not claim completion without verification or a clear reason verification could not run.
- Preserve user work in the tree. Never revert unrelated changes.
- If the request is ambiguous, make a reasonable low-risk assumption and record it.

Start by producing the Spec0 frame, then proceed through implementation unless the user explicitly asked for planning only.
