---
artifact_contract: "ce-handoff/v1"
created_at: "2026-09-15T07:31:29Z"
title: "H4 next after architecture POV"
summary: "POV accepted lifecycle self-correction and rejected Stages 1-4 as delivery order; authorized next is H4 overlay brainstorm/plan after CE plugin update."
keywords: ["kutha", "H4", "ADR-090", "ce-pov", "honeycomb", "semantic-contracts", "lifecycle"]
cwd: "/root/kutha-graph"
resume_focus: "After CE plugin update: ce-brainstorm then ce-plan for harness H4 (ADR-090 process overlay only). Do not start Stages 1-2 product runtime, legal pack, or frozen cells."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "docs/semantic-contracts-handoff"
head: "b03288cb070ab45555f278273fea6ccf0cf1c59e"
worktree_path: "/root/kutha-graph"
---

# Handoff: architecture POV → H4 CE path

## Objective and user intent

User asked for architecture re-check (brainstorm-shaped audit), then FSM/meta-prompt coverage, then ADR-graph re-evaluation, then `/ce-pov` on that re-evaluation. User then chose: **create this handoff → update compound-engineering plugin → `ce-brainstorm` → `ce-plan`** for the authorized next slice.

User directive for next work (explicit): scope **H4**, not product Stages 1–2 from the rejected priority chain.

## Work completed

- Honeycomb map dictionary and governor wiring landed earlier on this branch (see recent commits / CHANGELOG).
- Architecture audit + requirements matrix produced in chat; then corrected for D3/FSM/meta-prompt omission.
- ADR-graph re-evaluation produced a four-stage priority chain and claimed five “fundamental” gaps.
- `/ce-pov` judged that re-evaluation (chat only; no POV artifact file).

## Decisions (with attribution)

| Decision | Whose | Notes |
|---|---|---|
| Accept methodological corrections: do not collapse L_map/L_delivery/L_capability; H4 ≠ legal pack; domain stays in packs | ce-pov + matches STATE | Keep |
| Reject Stages 1→4 as **delivery order** | ce-pov | Conflicts with STATE next=H4 and `semantic-contract-validation.md` “not a replacement for H4” |
| Treat §3 “gaps” mostly as restatements of 2026-09-13 ADR clarifications + validation note | ce-pov | Not net-new architecture to code now |
| Next CE path: handoff → update CE plugin → brainstorm H4 → plan H4 | **user** | This handoff |

## Rejected / do-not-retry

- Implementing ADR-050 six dictionaries, typed CSR, Cui allocator, Cypher, or legal five-clock mapping **now** (freeze + Error 1 from re-eval).
- Treating semantic recovery / P→Q fixture as the next delivery slice **instead of H4** without a STATE lease change.
- Framing Empty Cypher / N[X] / shared Cui / fingerprint-only replay as newly discovered contradictions (already named in ADR-011/030/060/070/014 and validation note).
- Writing durable CE artifacts under `/tmp` (repo rule: use `docs_root`).

## Current state

| Plane | Status |
|---|---|
| L_map | Honeycomb cells **Proposed**; index in `.kutha/dictionaries/honeycomb.yaml` |
| L_delivery | M001 S01–S03 done; harness **H3**; **H4 open** |
| L_capability | FF5/FF6 named evidence; not “Accepted” product |
| Working tree | Clean at handoff capture (HEAD `b03288c`) unless later edits land |
| Prior handoff | `.compound-engineering/artifacts/handoffs/architecture-contracts-and-codex.md` — older; still useful for semantic-fixture encoding notes, but **does not authorize** starting those over H4 |

## Authoritative references (pointer-first)

- `.kutha/STATE.md` — Phase H3; next thin slice H4; freeze list.
- `.kutha/ROADMAP.md` — H4 checkbox: process dictionaries version like norms (needs ADR-090 overlay).
- `docs/process/kutha-harness.md` — H0–H4 rung table; H4 definition.
- `docs/ADR/ADR-090-legal-reference-pack.md` — overlay semantics for norms; **do not start legal corpus**.
- `docs/architecture/semantic-contract-validation.md` — dependency literacy for semantic recovery/fixture; **not** delivery reorder.
- `.kutha/dictionaries/honeycomb.yaml` — L_map compact index + `depends_on`.
- `AGENTS.md` — durable CE under `.compound-engineering/artifacts/`; Russian chat / English docs.

## Plausible next steps (forks)

1. **Primary (user-chosen):** update compound-engineering plugin, then `/ce-brainstorm` on H4 overlay scope, then `/ce-plan` from that Product Contract.
2. **Alternate:** only if user changes STATE lease — brainstorm/plan semantic recovery or P→Q fixture as a delivery slice.
3. **Not next:** `ce-work` on frozen product cells; M002 Rocks.

## Skills for resume

- `ce-brainstorm` (H4 WHAT) → `ce-plan` (H4 HOW) → later `ce-work` only with plan + authority.
- `ce-setup` if plugin update leaves config/health drift.
- Do **not** load LifeOS unless asked.

## Continuity warnings

- Handoff lives under tracked `docs_root` (survives sessions). Commit only if the user asks.
- POV verdict itself was chat-delivered; this file is the durable carrier of that judgment for the next agent.

## Post-capture: CE plugin update (2026-09-15)

- Marketplace `compound-engineering-plugin` refreshed to commit `0ec66db` / **3.26.2**.
- Project install for `/root/kutha-graph`: **3.25.0 → 3.26.2** (`claude plugin update … -s project`).
- Cursor local symlink retargeted to the same cache path (`~/.cursor/plugins/local/compound-engineering` → `…/3.26.2`).
- Restart agent/session before `/ce-brainstorm` so skills load from 3.26.2, not a stale 3.25.0 path.
