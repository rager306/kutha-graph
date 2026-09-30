---
gsd_state_version: "1.0"
milestone: v0.04
milestone_name: Single-log SoT + stable references
current_phase: 16
current_phase_name: Fold-internal hot indexes
current_plan: Not started
status: planning
stopped_at: Phase 15 complete, ready to plan Phase 16
last_updated: "2026-09-30T19:07:30.359Z"
last_activity: 2026-10-01
last_activity_desc: Phase 15 complete, transitioned to Phase 16
state_head: b7e995cbaabedbb41238a31ef91fdbd5a4850216
progress:
  total_phases: 5
  completed_phases: 15
  total_plans: 12
  completed_plans: 12
  percent: 94
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-09-30 after M012a lease)

**Core value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Current focus:** v0.04 Phase 16 — Fold-internal hot indexes (M012a S04 + S05). Plans 15-01..15-03 written; execute next. Freeze holds for M002. Do not clear harness S04.

**Not this file:** Kutha harness lease is `.kutha/STATE.md`. Do not overwrite it.

**Harness lease (cite only):** Active Milestone **M012a**; Active Slice **S06**; Phase **H5**; `L_map=honeycomb-proposed`; `L_delivery=M012a-S05-done`; `L_capability=ff5-green`. Freeze until explicit **M002** (no RocksDB/Cypher/HNSW/ADR-050 six dicts/legal pack start). Do not overwrite `.kutha/STATE.md`.

## Current Position

Current Plan: Not started
Total Plans in Phase: 3
Phase: 16 of 16 (Fold-internal hot indexes)
Plan: 3 of 3
Status: Ready to plan
Last activity: 2026-10-01 — Phase 15 complete, transitioned to Phase 16

Progress: [█████████░] 94% (v0.04 plans executed)

## Performance Metrics

**Velocity:**
- Total plans completed: 31 (v0.01–v0.03; v0.04 none yet)
- Average duration: ~2–5min typical; Phase 11 outliers 16–93min
- Total execution time: v0.01 ~26min + v0.02/v0.03 as recorded below

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1–3 (v0.01) | 9/9 | ~22min | ~2min |
| 4 | 3 | - | - |
| 5 | 2 | - | - |
| 6 | 2 | - | - |
| 7 | 2 | ~8min | 4min |
| 8 | 2 | ~10min | 5min |
| 9 | 2 | - | - |
| 10 | 2 | - | - |
| 11 | 4 | - | - |
| 12–16 (v0.04) | 0/3 planned | - | - |
| 12 | 3 | - | - |
| 13 | 3 | - | - |
| 14 | 3 | - | - |
| 15 | 3 | - | - |

**Recent Trend:**
- Last 5 plans: Phase 11 P01–P04, Phase 10 P02
- Trend: v0.03 harness/docs complete; Phase 12 plans ready; v0.04 crates not started

**Per-Plan Metrics:**

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 12 P01 | 8min | 3 tasks | 8 files |
| Phase 12 P02 | 12min | 2 tasks | 8 files |
| Phase 12 P03 | 15min | 2 tasks | 7 files |
| Phase 13 P01 | 7 | 2 tasks | 9 files |
| Phase 13 P02 | 4 | 2 tasks | 6 files |
| Phase 13 P03 | 5 | 2 tasks | 6 files |
| Phase 14 P01 | 8 | 2 tasks | 20 files |
| Phase 14 P02 | 4 | 2 tasks | 6 files |
| Phase 14 P03 | 3 | 2 tasks | 5 files |
| Phase 15 P01 | 6min | 2 tasks | 4 files |
| Phase 15 P02 | 1min | 2 tasks | 5 files |
| Phase 15 P03 | 2min | 2 tasks | 5 files |

## Accumulated Context

### Decisions

Full table: `.planning/PROJECT.md`.
- [v0.04]: Five GSD phases 12–16 map M012a S01 | S02 | S03 | S04+S05 | S06; product crates allowed; no M012 dicts-as-facts; freeze until M002
- [v0.04]: TIME (S05) folds into Phase 15 with DUR (S04); S05 `depends:[]` but numeric order is after Phase 14
- [v0.04]: GATE-01/02/03 inherit — named cargo + governor; execute only under matching Active Slice; honeycomb stays Proposed
- [v0.03]: H5 semantic governor + lean AGENTS.md shipped; crates untouched
- [v0.02]: One GSD phase per M011 slice; GATE-01/02/03; M011 S04–S08 closed
- [v0.01]: Phases 1–3 verification overlay; green ≠ Accepted ≠ L_capability ≠ lease grant
- [Phase 12]: OutcomeDisposition lives in kutha-common so Op::QuantumOutcome can carry it
- [Phase 12]: Public emit of Op::QuantumOutcome is MetaOpRejected (T-12-02)
- [Phase 12]: Resume persist/open after sidecar discard stays Plan 12-02
- [Phase 12]: JustificationCite is a distinct fold-noop Op; Resume reuses QuantumOutcome
- [Phase 12]: Provenance domain tag kutha-prov-log-native; I-F1-outcomes stays deferred
- [Phase 13]: Retract/Correct/CorrectInterval share EventId because UnknownFact and IntervalPatchRejected are one type
- [Phase 13]: Lookup prefers the live Fact when several residuals share one CorrectInterval Event.id
- [Phase 13]: CSR TypedEdge.fact_seq and ConflictReport seqs stay lease indexes (ING-03 is Phase 14)
- [Phase 13]: Dropped source_fact_seqs from the durable cite Op so a leaked payload cannot be replayed against a renumbered fold
- [Phase 13]: check_admission still uses tt=MAX and row.vt (FIX-02); unknown EventId is stale_support
- [Phase 13]: REF-03 rebuild drops the first Assert Event only; Defines and QuantumOutcome stay
- [Phase 13]: Governor check lives on bridges.yaml, not invariants.yaml
- [Phase 13]: ADR-061 stays capability none with a non-empty evidence array
- [Phase 14]: Identity for the delivery-key gate is subject, relation, object, valid_from, valid_to, claim, polarity, and delivery_key
- [Phase 14]: Retracted original still returns that EventId; do not mint a replacement support
- [Phase 14]: SupportPolarity lands with default None so Plan 14-02 does not reshape Assert again
- [Phase 14]: Residuals copy polarity; object-changing Correct/CorrectInterval rows flip when old polarity is Some
- [Phase 14]: e2e t1 Asserts store Positive so t2 interior not-P is Negative without an opposite-of dictionary
- [Phase 14]: Governor check lives on bridges.yaml, not invariants.yaml
- [Phase 14]: ADR-011 and ADR-013 evidence append only; map stays Proposed
- [Phase 15]: open maps ReplayDivergence through runtime_err (InvalidData) and does not return Ok Runtime
- [Phase 15]: persist jsonl is same-directory temp, sync_all, rename; abort-before-rename is a pub test seam
- [Phase 15]: Define persist ids are SHA-256 prefix kutha-define-id; intern() still Event::new
- [Phase 15]: ValidTime stays a u64 alias; fixtures treat 2017 as YearCe, not Unix epoch
- [Phase 15]: TransactionTime is log sequence; TT-to-wall mapping is out of M012a S05
- [Phase 15]: Governor checks live on bridges.yaml, not invariants.yaml
- [Phase 15]: ADR-012 and ADR-013 evidence append only; map stays Proposed
- [Phase 15]: Harness Active Slice stays S04 until parent verification closes S04/S05

### Pending Todos

None.

### Blockers/Concerns

- Harness Active Slice is **S02** (GATE-02). Execute Phase 13 under that lease. Do not clear S02 until Phase 13 SUMMARY + verification pass.
- Do not start M012 dictionaries-as-facts, M002 Rocks, legal pack, Cypher, HNSW, or ADR-050 because a GSD phase completed — honeycomb stays Proposed (GATE-03).

## Deferred Items

| Category | Item | Status | Deferred At | Milestone |
|----------|------|--------|-------------|-----------|
| later | Rocks / Cypher / HNSW / ADR-050 / packs | Frozen | 2026-09-29 | until STATE names M002+ |
| later | M012 dictionaries-as-facts (F4/F5) | Frozen | 2026-09-30 | until STATE names M012 |
| later | ADR-061 full GED-class diff API | Out of M012a | 2026-09-30 | v0.04 / REQUIREMENTS |
| verification | Phases 4–8 verification digests marked stale by manager | Acknowledged override | 2026-09-30 | v0.02 closeout |
| audit | Formal `/gsd-audit-milestone` for v0.02 not run before archive | Acknowledged gap | 2026-09-30 | v0.02 closeout |

## Session Continuity

Last session: 2026-09-30T19:06:15.121Z
Stopped at: Phase 15 complete, ready to plan Phase 16
Resume file: None
Next: `/gsd-execute-phase 13`. Harness lease is already S02. Freeze holds. Do not lease M002 without explicit operator intent.

## Operator Next Steps

- Execute: `/gsd-execute-phase 13`
- Do not overwrite `.kutha/STATE.md` from GSD memory; do not clear S02 until verification pass
