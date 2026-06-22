---
name: spec0
description: Use for Spec0 planning, execution, review, or handoff workflows. Trigger when the user asks to turn a rough development request into a bounded spec, implement from a spec, review work against a spec, or prepare a cross-agent handoff.
---

# Spec0

Spec0 is a compact meta-framework for turning a rough development intent into a bounded, verifiable implementation loop.

## When To Use

Use this skill when the user asks for Spec0, a meta-framework, a built-in workflow, slash-command style execution, cross-agent handoff, or a disciplined plan-execute-verify loop.

## Loop

1. Orient: identify the repository, active branch, dirty files, relevant agent instructions, and likely source areas.
2. Frame: write a concise spec before editing. Include goal, non-goals, constraints, assumptions, target files, and verification.
3. Plan: split the work into the smallest safe sequence of implementation slices.
4. Execute: make the first useful slice real, following the repo's existing patterns.
5. Verify: run the narrowest meaningful checks first, then broader checks if the blast radius requires them.
6. Handoff: summarize what changed, checks run, unresolved risks, and next actions.

## Modes

- Plan: produce Goal, Context, Non-goals, Assumptions, Plan, Verification, and Risks.
- Exec: implement the next smallest useful slice, then verify it.
- Review: inspect current work against the Spec0 frame and lead with findings.
- Handoff: create a compact factual handoff for the next agent session.

## Operating Rules

- Prefer existing project conventions over new abstractions.
- Keep changes scoped to the spec. If the work expands, update the spec first.
- Do not claim completion without verification or a clear reason verification could not run.
- Preserve user work in the tree. Never revert unrelated changes.
- If the request is ambiguous, make a reasonable low-risk assumption and record it.

If arguments are supplied, treat them as the requested Spec0 goal or mode. If the user asks for planning only, stop after the plan. Otherwise, continue through implementation and verification.
