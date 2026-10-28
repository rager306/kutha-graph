---
phase: 32-tt-anchor-and-as-known-on
plan: 01
subsystem: runtime
tags: [tt_anchor, as_known_on, WallTime, Op::TtAnchor]
requires: [{ phase: 31-clock-newtypes, provides: CLK-01 WallTime }]
provides: ["ANK-01 Op::TtAnchor log facts", "ANK-02 as_known_on Option lookup"]
affects: [32-02]
actuals: { tokens: 3084, tasks: 3, commits: 3 }
plan_head_before: 7ba41b49eae9009d925984b7a375ad1495c240a5
plan_head_after: fe1be90a27b30dc9ac0e75b916c755989049ce53
tech-stack: { added: [], patterns: ["skip-serialized tt_anchors BTreeMap", "as_known_on Option miss", "ReceiptMode::Dictionary emit_tt_anchor"] }
key-files: { created: [], modified: [crates/kutha-common/src/event.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/hydrate.rs, crates/kutha-runtime/src/fold_query.rs, crates/kutha-runtime/src/provenance.rs, CHANGELOG.md] }
key-decisions: ["Op::TtAnchor { wall: u64 }; mapped tt = Event.ingested_at; no Fact", "as_known_on returns Option (None = miss); not kutha: Assert"]
patterns-established: ["rebuild_tt_anchors beside rebuild_blobs", "as_of stays live_at(MAX, vt)"]
requirements-completed: [ANK-01, ANK-02]
coverage:
  - id: D1
    description: Op::TtAnchor maps ingested_at to wall; no Fact minted
    requirement: ANK-01
    verification: [{ kind: unit, ref: "cargo test -p kutha-runtime --offline --lib as_known_on#as_known_on_after_anchor_is_some_earlier_wall_is_none", status: pass }]
    human_judgment: false
  - id: D2
    description: as_known_on Option; hydrate rebuild; as_of still MAX tt
    requirement: ANK-02
    verification: [{ kind: unit, ref: "cargo test -p kutha-runtime --offline --lib as_known_on#as_known_on_survives_persist_open_and_matches_fold", status: pass }]
    human_judgment: false
duration: 7min
completed: 2026-10-02
status: complete
---

# Phase 32 Plan 01: tt_anchor and as_known_on Summary

**Op::TtAnchor maps log tt to wall; as_known_on is Option of product triples (None = miss).**

- **Duration:** 7 min · **Started:** 2026-10-02T08:02:59Z · **Completed:** 2026-10-02T08:09:39Z · **Tasks:** 3 · **Files:** 8

- `emit_tt_anchor(2015)` then `as_known_on(2015)` is Some; `as_known_on(2014)` is None; no Fact minted
- Hydrate `rebuild_tt_anchors`; `GraphFold::as_known_on` matches Runtime; `as_of` still `live_at(MAX, vt)`
- FF5 and `.kutha/STATE.md` bytes unchanged; no `32-VERIFICATION`; no `kutha-gov ci`

## Task Commits
1. **Op::TtAnchor + as_known_on** - `8f27c0a` (feat)
2. **Hydrate rebuild + GraphFold lookup** - `58d96cd` (feat)
3. **SUMMARY** - `fe1be90` (docs)

## Deviations from Plan
None - plan executed exactly as written.

## Next Phase Readiness
Ready for 32-02 named true-on-D vs known-on-D. Do not write 32-VERIFICATION (D-05). Do not start Phase 33.

## Self-Check: PASSED
FOUND: event.rs, fold.rs, quantum.rs, pipeline.rs, hydrate.rs, fold_query.rs, 32-01-SUMMARY.md; commits 8f27c0a, 58d96cd, fe1be90
