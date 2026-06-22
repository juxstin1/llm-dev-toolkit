# Spec0 Agent Toolkit

Spec0 packages a repeatable agent workflow as local slash commands for Claude Code and OpenCode, plus a Codex Skill and prompt shims.

The loop is intentionally small:

1. Orient around repo state, instructions, and likely source areas.
2. Frame the goal, non-goals, assumptions, files, and verification.
3. Plan small implementation slices.
4. Execute the next useful slice.
5. Verify before claiming completion.
6. Handoff the state for the next session.

## Commands

- `/spec0` - run the full orient, frame, plan, execute, verify, handoff loop.
- `/spec0-plan` - turn a rough request into a bounded implementation plan.
- `/spec0-exec` - execute the next planned slice and verify it.
- `/spec0-review` - review current work against the Spec0 frame.
- `/spec0-handoff` - summarize status for the next agent session.

## Install With `tk`

```bash
tk spec0 install --agent all --scope user
tk spec0 install --agent all --scope project --dir .
tk spec0 install --agent all --scope user --dry-run
```

Use `--agent claude`, `--agent codex`, or `--agent opencode` to install only one surface. Existing files are skipped unless `--force` is supplied.

## Agent Locations

| Agent | User install | Project install |
|---|---|---|
| Claude Code | `~/.claude/commands/*.md` | `.claude/commands/*.md` |
| OpenCode | `~/.config/opencode/commands/*.md` | `.opencode/commands/*.md` |
| Codex | `~/.agents/skills/spec0/SKILL.md` plus `~/.codex/prompts/*.md` shims | `.agents/skills/spec0/SKILL.md` |

Codex custom prompts are deprecated in favor of Skills, but the prompt shims are still useful when you want slash-style `/prompts:spec0` invocation in Codex CLI or IDE.

## Invocation

- Claude Code: `/spec0`, `/spec0-plan`, `/spec0-exec`, `/spec0-review`, `/spec0-handoff`
- OpenCode: `/spec0`, `/spec0-plan`, `/spec0-exec`, `/spec0-review`, `/spec0-handoff`
- Codex: invoke the `spec0` skill, or use `/prompts:spec0` if you installed the prompt shims
