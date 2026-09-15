---
artifact_contract: "ce-handoff/v1"
created_at: "2026-09-15T09:45:18Z"
title: "H4 process overlay shipped locally"
summary: "H4 ADR-090 process overlay dogfood is implemented, verified, and committed; freeze holds; next agent should not open a legal pack or M002."
keywords: ["H4", "overlay", "processAllows", "membership", "freeze", "ADR-090"]
cwd: "/root/kutha-graph"
resume_focus: "Orient after H4 ship: confirm PR/remote state, keep freeze, decide next steel thread without legal pack or M002."
repository: "kutha-graph"
repo_root_sha: "1ee5942d3f12fafc1be6983f45d0713958047900"
branch: "docs/semantic-contracts-handoff"
head: "b6076e4b289aa8e0a59b70f8b134e5d8e4e294b7"
worktree_path: "/root/kutha-graph"
---

# H4 process overlay shipped

## Objective and user intent

User asked to finish Compound Engineering shipping after H4 work: set `origin/HEAD`, open a PR, write this handoff, and compound the KTD3 membership-snapshot lesson. Chat stays Russian; durable docs stay English.

## Work completed

- H4 process-relation overlay (not a legal pack) implemented and verified.
- Local commit: `b6076e4` — `feat: dogfood H4 process-relation overlay AS OF`.
- Tip YAML remains the admit lease; reserved relation `allows` stays on tip.
- Membership editions are one snapshot assert each (`process.relations` / `allows` / sorted CSV object).
- Tenant maps those editions to product `processAllows` and supports AS OF a prior cut.
- Governor checks `h4-lease` / `h4-membership-as-of`, FSM evidence name for the prior-cut test, lease docs promote Phase H4.
- Plan file committed with that work: `.compound-engineering/artifacts/plans/2026-09-15-1459-feat-h4-adr-090-process-overlay-plan.md`.

## Decisions (attribution)

- **User:** H4 = ADR-090 *process* overlay dogfood only; do not start Stages 1–4 legal pack; do not ask extra clarifying questions when context + ADR suffice; ship path includes local commit then later PR/handoff/compound.
- **Agent (feasibility / plan):** Rejected per-member open chains under `emit_chained` because dropped members would stay live (KTD3 → one snapshot object per edition).
- **Agent:** Lite `ce-code-review` receipt only (no P0/P1); durable under docs_root rather than `/tmp`.

## Current state

| Piece | Status |
|-------|--------|
| H4 encoding + tenant AS OF | Done in `b6076e4` |
| Governor / pytest / cargo verification | Green at ship time (`kutha-gov precommit`, `ci`, `py`; pytest 39; cargo workspace incl. H4) |
| `.kutha/STATE.md` | Phase H4; Next action says overlay is in; freeze listed |
| PR / remote feature branch | Being opened in the same ship turn as this handoff |
| Legal pack / M002 Rocks / Cypher / HNSW | Not started — freeze holds |
| Untracked review receipt | Machine-local: `.compound-engineering/artifacts/handoffs/ce-code-review-h4-overlay/` (optional; not required for product) |

## Authoritative references (pointer-first)

- `.kutha/STATE.md` — Phase H4, Next action, Freeze list (do not start legal pack / M002 until STATE names it).
- `.kutha/ROADMAP.md` — H4 checkbox marked done; wording still mentions legal-pack metaphor — treat as process overlay complete, not Stages 1–4.
- Plan: `.compound-engineering/artifacts/plans/2026-09-15-1459-feat-h4-adr-090-process-overlay-plan.md` — KTD1 tip lease vs logged membership; KTD3 one snapshot per edition.
- Encoding: `scripts/kutha_gov/time_log.py` — `MEMBERSHIP_*`, `encode_membership_object`, `append_membership_edition`.
- Tenant: `crates/kutha-runtime/src/tenant.rs` — `allows` → `processAllows` via `emit_chained`.
- Product allowlist: `crates/kutha-runtime/dictionaries/relations.yaml` — `processAllows`.
- Tests: `scripts/tests/test_kutha_gov.py`, `crates/kutha-runtime/tests/h4_process_allows.rs`.
- Prior POV handoff: `.compound-engineering/artifacts/handoffs/h4-after-architecture-pov.md` — accept lifecycle self-correction; reject Stages 1–4 as delivery.

## Do not touch / wrong paths to retry

- Do **not** start a legal pack, ADR-090 full ontology, ADR-093, ADR-050 six dictionaries.
- Do **not** start M002 Rocks, Cypher/GPML, HNSW, ADR-100+, Consensus Query 103+.
- Do **not** make tip YAML fold-derived; tip admits, log owns membership history.
- Do **not** model membership as N per-member open asserts under `emit_chained` — dropped names stay live. Prefer one snapshot object per edition (see compounded learning once written under `.compound-engineering/artifacts/solutions/`).
- Do **not** strip tip `status` mid-CI to prove AE1; fixture + temp relations path only.
- Do **not** collapse L_map / L_delivery / L_capability because governor is green.

## Plausible next steps

1. Confirm PR is open against `main` and CI is green (or babysit if still running).
2. Short `ce-brainstorm` / `ce-pov` for the **next** steel thread after H4 overlay — STATE does not name one yet.
3. Optional: commit or discard the untracked lite review receipt folder.

## Skills that may help

- `ce-babysit-pr` if PR CI needs watching
- `ce-brainstorm` / `ce-pov` for post-H4 thread selection
- `ce-compound-refresh` only if solutions corpus needs tidy later
