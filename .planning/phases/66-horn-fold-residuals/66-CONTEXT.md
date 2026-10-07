# Phase 66: Horn/fold residuals — Context

**Milestone:** v0.14 / harness M020 S04
**Requirements:** HF-01 CorrectInterval residual re-derive, HF-02 co-live `live_triples`, HF-03 Retract `kutha:*` gate, HF-04 fingerprint oracle, HF-05 bench ≤10%
**Depends on:** Phase 65 closed; lease S04 Active
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

Fix Horn/fold residuals from the 2026-10-07 audit. Honeycomb stays **Proposed**. Bench ≤10%. Do not edit `.kutha/STATE.md`. Do not write `66-VERIFICATION.md`.

## Locked decisions

- **D-01 (HF-02):** `live_triples` is `FactIdxList` (multimap); invalidate removes one idx; `has_live_triple` is any remaining live idx.
- **D-02 (HF-01):** CorrectInterval `rule_follow_ons`: retract derived; fire Horn on prefix (old object), patch (new object), suffix (old object) via `leftover_halves`/`vt_intersect`; mint ignores event ids scheduled for retract in the same batch; pending order so retracts apply before fires (LIFO).
- **D-03 (HF-03):** User `Retract` of reserved/`kutha:*` fact → `MetaOpRejected`. Dictionary retract helper for constructor tests (D-07).
- **D-04 (HF-04):** `open_retract_reemit_indexes_match_fingerprint` persist-after-mutation and assert fingerprints.
- **D-05 (HF-05):** Bench gate same host vs pre-change baseline.

## Deferred

Process lease/bench contract → 67. Retract-by-CorrectInterval-id targeting 1-of-3 residual rows — document if not fixed; prefer fixing `fact_index_by_event_id` only if required for green pins.
