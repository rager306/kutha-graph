---
artifact_contract: "ce-handoff/v1"
created_at: "2026-10-15T11:13:42Z"
title: "M089 closed — GSD-auto check-surface debt + GSD 1.17.0"
summary: "M088–M089 shipped (AGENTS --check ≠ ci; reject redirect hint); Active None; GSD core updated to 1.17.0 on Cursor global."
keywords: ["M088", "M089", "GSD-auto", "kutha-gov", "--check", "agent-surface", "gsd-update", "handoff"]
cwd: "/root/kutha-graph"
resume_focus: "Controller lease next thin process-debt slice (candidate: --skip-cargo reject hint parity with M089); do not thaw Cypher/HNSW/legal pack; honeycomb stays Proposed."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "chore/m013-roadmap-reset"
head: "74ba858fix(160-01): close M089 with --check reject redirect hint"
worktree_path: "/root/kutha-graph"
---

# Handoff — M089 closed, GSD-auto check debt, GSD 1.17.0

## Objective / user intent

- Repeated **GSD-auto** waves: reindex CBM + GitNexus, close process debt first (discoverability / CLI honesty), thin slices, TDD, changelog + commit, Russian wrap, governor/GSD lessons.
- Then `/gsd-update` → user chose **1** (update now) → GSD **1.16.0 → 1.17.0** global Cursor.
- Then `/ce-handoff` (this artifact).

Attribution: lease choices and “debt first” framing were **user** GSD-auto prompts. Hint text and slice scoping (M088 docs → M089 CLI stderr) were **agent** calls inside that framing. GSD update confirmation was **user**.

## Work completed (this arc)

| Slice | GSD | Commits | What landed |
|-------|-----|---------|-------------|
| M088 | v0.82 / Phase 159 | `3332211` lease, `e6e5df1` close | AGENTS Flags: `--check` is `json\|precommit\|selftest only — not ci`; oracles + `SEM-m088-agents-check-not-ci` |
| M089 | v0.83 / Phase 160 | `e1652f3` lease, `74ba858` close | Invalid `--check` stderr adds `hint: kutha-gov {json\|precommit\|selftest} --check ID[,ID…]`; `SEM-m089-check-reject-hint` |

- Tree clean at HEAD `74ba858`. `L_delivery=M089-closed`. Active Milestone/Slice **None**.
- Full `uv run kutha-gov ci --skip-cargo` green on both closes (`run_high=0`, `tip_high=0`).
- Indexes refreshed after closes (CBM `kutha-graph`; GitNexus `kutha-graph`).

### GSD core update (machine-local)

- Installed: **1.17.0** GLOBAL Cursor → `/root/.cursor` (machine-local).
- Backed up 6 Cursor hook files to `/root/.cursor/gsd-user-files-backup/` (machine-local); restore planner reported all `already_present` (eligible_count=0).
- No `gsd-local-patches`. Restart Cursor advised for skills/hooks.

## Authoritative state pointers

| What | Where / what matters |
|------|----------------------|
| Harness lease | `.kutha/STATE.md` — Active **None**; `L_delivery=M089-closed`; Next = controller chooses lease; thaw map unchanged |
| GSD progress | `.planning/STATE.md` — v0.83 complete, stopped at `160-check-reject-hint` |
| Id map | `docs/process/gsd-auto-contract.md` — rows through v0.83 / Phase 160 |
| ROADMAP closed slices | `.kutha/ROADMAP.md` — M088 + M089 **CLOSED** near top of recent harness milestones |
| CLI reject + hint | `scripts/kutha_gov/__main__.py` (~L413–418) — rule + hint lines together |
| Oracles | `scripts/tests/test_multi_check.py` `test_ci_rejects_check_flag`; `scripts/tests/test_agent_surface.py` `test_agents_md_check_not_for_ci` |
| Agent-surface pins | `.kutha/dictionaries/checks.yaml` `agent-surface` — AGENTS needles + hint string in `__main__.py` |
| Matrix | `docs/architecture/test-matrix.yaml` `SEM-m088-*`, `SEM-m089-check-reject-hint` |
| Phase packets | `.planning/phases/159-agents-check-not-ci/`, `.planning/phases/160-check-reject-hint/` |
| Audits | `.planning/milestones/v0.82-MILESTONE-AUDIT.md`, `v0.83-MILESTONE-AUDIT.md` |
| AGENTS Flags | `AGENTS.md` ~L67 — do not re-group `ci` with `--check` without exclusion |
| Living harness note | `docs/process/kutha-harness.md` — three `--check` surfaces (M087) |

## Constraints (unchanged)

- Honeycomb **Proposed**; not Cypher/HNSW; `I-legal-pack-not-next` stays.
- Do **not** admit `--check` on `ci` (meta-prompt FSM: selective checks stay on json|precommit|selftest).
- Do not bump `0.0.0`, tag, or GitHub Release unless STATE allows.
- CE `docs_root` = `.compound-engineering/artifacts` — handoffs live here, not OS `/tmp`.
- CBM: never `delete_project` / `manage_adr`. GitNexus secondary except M013 tool-pair phases.

## Plausible next steps (controller)

One path (related sequential debt), not exclusive forks unless owner wants product thaw:

1. **Thin harness UX:** `--skip-cargo` reject on non-`ci` currently names the rule only — same redirect-hint pattern as M089 (agent suggestion from M089 wrap; **user has not leased it**).
2. Or scout the next discoverability / governor honesty gap with CBM + GitNexus after reindex, still process-first.
3. Product thaw (Cypher/HNSW/legal) only if STATE explicitly leases it.

## Skills useful on resume

- GSD-auto / lease bundle: `docs/process/gsd-auto-contract.md` + `.cursor/rules/ce-skills-ru.mdc`
- Changelog + commit: `kutha-changelog` then `ce-commit` (Russian verbs)
- Code graph: `.cursor/skills/codebase-memory/SKILL.md` + GitNexus when M013 pair applies
- After GSD update local mods: `/gsd-update --reapply` only if `gsd-local-patches` appears

## Verification snapshot

- Pytest: agent-surface + multi_check green on M088/M089 close.
- `kutha-gov precommit` selective checks green; `ci --skip-cargo` tip_high=0.
- Worktree clean after `74ba858`.
