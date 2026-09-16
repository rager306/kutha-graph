---
artifact_contract: "ce-handoff/v1"
created_at: "2026-09-16T08:08:22Z"
title: "CBM wired as Cursor CE evidence plane; M011 S02 still leased done"
summary: "Repo Cursor adapter makes codebase-memory-mcp primary inside CE; GitNexus AGENTS inject suppressed locally; product next remains leased M011 S03 P→Q."
keywords: ["CBM", "codebase-memory-mcp", "Compound Engineering", "GitNexus", "M011", "Cursor"]
cwd: "/root/kutha-graph"
resume_focus: "Lease M011 S03 for a thin P→Q derivation fixture, or dogfood CBM in a fresh agent chat. Do not start M002."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "feat/m010-s01-semantic-recovery"
head: "0b064b8"
worktree_path: "/root/kutha-graph"
---

# Session handoff

Supersedes orientation from `.compound-engineering/artifacts/handoffs/m011-s02-cbm-mcp.md` for process/tooling; product lease there still holds.

## Objective

User-approved: embed CBM tools/roles into the development process under Compound Engineering constraints, then commit and hand off. Earlier in the same thread: resume M011 S02 handoff, rebuild stale CBM index, suppress GitNexus `AGENTS.md` injection.

## Done

| Area | Where it lives | What matters |
|------|----------------|--------------|
| Product lease | `.kutha/STATE.md` | Active Milestone **M011**, Active Slice **None**, `L_delivery=M011-S02-done`, Phase H4; M002 freeze unchanged |
| CBM index | machine-local CBM cache | Rebuilt `full` this session; project `kutha-graph`; generation ~2026-09-16; do not `delete_project` |
| Cursor rule | `.cursor/rules/code-graph-cbm.mdc` | CBM primary; GitNexus secondary; CE phase map; forbidden tools |
| Cursor skill | `.cursor/skills/codebase-memory/SKILL.md` | Default tool set + Scout/Verify/Auditor; not a CE replacement |
| Project MCP | `.cursor/mcp.json` | `command`: `codebase-memory-mcp` on `PATH` (not `/root/.local/bin/...`) |
| Agent docs | `AGENTS.md` § Code graph (Cursor) | Codex role → Cursor `Task` map; integrator owns `index_repository` |
| Codex bridge | `docs/process/codex-subagents.md` | Pointer to AGENTS map |
| RU routing | `.cursor/rules/ce-skills-ru.mdc` | «переиндексируй CBM» → rule, not a CE skill |
| GitNexus suppress | machine-local `.gitnexusrc` (gitignored) | `skipAgentsMd` + `skipSkills`; listed in `.gitignore` |
| Commit | `0b064b8` | `Wire CBM as Cursor evidence plane inside CE.` |

## Decisions (user-confirmed unless noted)

- CBM is an **evidence plane inside CE**, not a second orchestrator (writer inference, user agreed).
- Default tools only; no mandatory full MCP catalog; no `manage_adr` / `delete_project` / silent reindex.
- GitNexus secondary; analyze must not rewrite `AGENTS.md`/`CLAUDE.md` (user asked for `.gitnexusrc` **and** gitignore — **local only**, not team-shared).
- Codex CBM roles stay Codex; Cursor maps briefs to built-in `Task` types.

## Unfinished / local leftovers

- Product: M011 S03 thin P→Q still needs an explicit Active Slice lease.
- Machine-local: `.gitnexusrc` must be recreated on other machines (gitignored).
- Untracked leftover (not SoT): `.claude/skills/` (GitNexus skill copies), `.compound-engineering/artifacts/handoffs/ce-code-review-h4-overlay/`.
- Fresh Cursor chat recommended so always-on rule + skill reload; CBM binary must remain on `PATH`.

## Do not

- Start M002 / legal pack / Cypher / HNSW.
- Patch CE plugin-cache skills.
- Dual-query GitNexus + CBM “just in case”.
- Treat graph coverage as governor green.

## Next (one path)

1. New agent chat (or continue) with CBM available.
2. If product: lease **M011 S03** P→Q in STATE/ROADMAP → `ce-work` / TDD / `kutha-changelog` / `ce-commit`.
3. Optional cleanup: delete or ignore untracked GitNexus skill trees under `.claude/skills/`.

Skills: `ce-work` after lease; `ce-commit` / `kutha-changelog` for ship; CBM via `.cursor/skills/codebase-memory/SKILL.md`.
