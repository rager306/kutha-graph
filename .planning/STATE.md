---
gsd_state_version: "1.0"
milestone: v0.05
milestone_name: Dictionaries as facts
current_phase: 19
current_phase_name: Admission and policy meta-facts
current_plan: Not started
status: planning
stopped_at: Phase 18 complete, ready to plan Phase 19
last_updated: "2026-10-01T02:32:20.210Z"
last_activity: 2026-10-01
last_activity_desc: Phase 18 complete, transitioned to Phase 19
state_head: 614f92b664839e98d993fb18067c246fc143d247
progress:
  total_phases: 5
  completed_phases: 18
  total_plans: 6
  completed_plans: 6
  percent: 86
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-10-01 after M012 lease / GSD v0.05)

**Core value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Current focus:** v0.05 Phase 18 — Rule registry (M012 S02). Three execute plans written (18-01..03).

**Not this file:** Kutha harness lease is `.kutha/STATE.md`. Do not overwrite it.

**Harness lease (cite only):** Active Milestone **M012**; Active Slice **None**; Phase **H5**; `L_map=honeycomb-proposed`; `L_delivery=M012-S02-done`; `L_capability=ff5-green`. Freeze until explicit **M002** (no RocksDB/Cypher/HNSW/ADR-050 six dicts/legal pack start).

## Current Position

Phase: 19 of 21 (Admission and policy meta-facts)
Current Plan: Not started
Total Plans in Phase: 3
Status: Ready to plan
Last activity: 2026-10-01 — Phase 18 complete, transitioned to Phase 19

Progress: [█████████░] 86% (v0.05)

## Performance Metrics

**Velocity:**
- Total plans completed: 49 (v0.01–v0.04)
- Average duration: ~2–5min typical; Phase 11 outliers 16–93min
- v0.04: 15 plans (Phases 12–16), all complete 2026-10-01

**By Phase:** v0.01 9/9 · v0.02 11/11 · v0.03 8/8 · v0.04 15/15 · v0.05 0/?

**Recent Trend:** v0.04 crate work closed; v0.05 product crates allowed under M012 thawed subset.
**Per-Plan Metrics:**

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 17-allowlist-as-log-facts P01 | 3min | 2 tasks | 4 files |
| Phase 17-allowlist-as-log-facts P02 | 2min | 2 tasks | 4 files |
| Phase 17-allowlist-as-log-facts P03 | 4min | 2 tasks | 5 files |
| Phase 18 P01 | 7 | 3 tasks | 10 files |
| Phase 18 P02 | 2 | 2 tasks | 4 files |
| Phase 18 P03 | 3 | 2 tasks | 5 files |

## Accumulated Context

### Decisions

Full table: `.planning/PROJECT.md`.
- [v0.05]: Five GSD phases 17–21 map M012 S01 | S02 | S03 | S04 | S05; product crates allowed; no ADR-050 six dictionaries; freeze until M002
- [v0.05]: S03 depends on S01 not S02; S05 depends on S02 not S03/S04; numeric execute order is still 17 → 18 → 19 → 20 → 21
- [v0.05]: GATE-01/02/03 inherit — named cargo + governor; execute only under matching Active Slice; honeycomb stays Proposed
- [v0.04]: M012a S01–S06 shipped (Phases 12–16); F4/F5 wait for M012
- [v0.03]: H5 semantic governor + lean AGENTS.md shipped; crates untouched
- [Phase 17-allowlist-as-log-facts]: AllowRelation is a dedicated Op, not Assert/Define, so admit can bootstrap without gating itself
- [Phase 17-allowlist-as-log-facts]: emit(AllowRelation) appends one log event without a QuantumOutcome sidecar so the tracer log-len contract holds
- [Phase 17-allowlist-as-log-facts]: YAML seed remains the fallback so inForceAs admits on an empty-log Runtime (FF6)
- [Phase 17-allowlist-as-log-facts]: Retract of an allow-entry EventId is not UnknownFact; Correct/CorrectInterval stay Fact-only
- [Phase 17-allowlist-as-log-facts]: Open reconstructs allow-entries from the log walk, not from snapshot JSON keys
- [Phase 17-allowlist-as-log-facts]: ALL file oracles are required observe names; FF6 remains required
- [Phase 17-allowlist-as-log-facts]: ADR-011 evidence append only; honeycomb map stays Proposed; ADR-050 delivery stays frozen
- [Phase 18]: RegisterRule is a dedicated Op, not Assert/Define/Behavior, so the registry is a log fact
- [Phase 18]: rule_definition_hash is SHA-256 of kutha-rule-def plus UTF-8 definition bytes; the Op does not store a precomputed hash
- [Phase 18]: User emit(Behavior) pin check does not run on follow_ons inverse_knows
- [Phase 18]: Retract of RegisterRule is accepted as a live rule-entry EventId, not as a graph Fact
- [Phase 18]: Opened runtimes accept hashed Behavior because hydrate rebuilds rule-entries from the log
- [Phase 18]: RULE file oracles are required observe names; ALL and FF6 remain required
- [Phase 18]: ADR-011 evidence append only; honeycomb map stays Proposed; ADR-050 delivery stays frozen

### Pending Todos

None.

### Blockers/Concerns

- Harness Active Slice is **S01** (cite `.kutha/STATE.md`; do not overwrite it). Execute Phase 17 under that lease (GATE-02).
- Do not start ADR-050 six dictionaries, M002 Rocks, legal pack, Cypher, or HNSW because a GSD phase completed — honeycomb stays Proposed (GATE-03).

## Deferred Items

| Category | Item | Status | Deferred At | Milestone |
|----------|------|--------|-------------|-----------|
| later | Rocks / Cypher / HNSW / ADR-050 / packs | Frozen | 2026-09-29 | until STATE names M002+ |
| later | ADR-061 full GED-class diff API | Out of M012a | 2026-09-30 | v0.04 / REQUIREMENTS |
| verification | Phases 4–8 verification digests marked stale by manager | Acknowledged override | 2026-09-30 | v0.02 closeout |
| audit | Formal `/gsd-audit-milestone` for v0.02 not run before archive | Acknowledged gap | 2026-09-30 | v0.02 closeout |

## Session Continuity

Last session: 2026-10-01T02:30:48.305Z
Stopped at: Phase 18 complete, ready to plan Phase 19
Resume file: None
Next: `/gsd-execute-phase 17`. Active Slice S01 is leased. Freeze holds. Do not lease M002 without explicit operator intent.

## Operator Next Steps

- `/gsd-execute-phase 17`
