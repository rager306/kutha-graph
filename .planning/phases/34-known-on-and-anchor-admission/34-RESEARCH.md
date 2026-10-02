# Phase 34 Research: Known-on cut and anchor admission

**Date:** 2026-10-02. **CBM:** `root-kutha-graph` (prefer over `kutha-graph`; indexed 2026-10-02T09:06:03Z; cited paths `metadata_match`). GitNexus unused (D-06, outside M013). `rg` arbiter.

## Bug (HEAD)

`GraphFold::as_known_on` (`fold_query.rs` 91–95) does `live_at(tt, u64::MAX)`. `Fact::is_live_at` drops when `vt >= valid_to`. Closed VT (`valid_to: Some(2020)`) vanishes from knowledge-date queries. `as_of` stays `live_at(u64::MAX, vt)` (correct). `collect_live_facts_at` hot path `vt_by_from.range(..=vt)` is the same trap — do not reuse it for known-on.

`Op::TtAnchor`: `admit` hits `op_relation` None and returns Ok. `apply` / `rebuild_tt_anchors` `BTreeMap::insert` last-wins. Duplicate or backwards wall is silent.

## D-01 lock (PLAN must honor)

**TT-only product at resolved tt.** After greatest-anchor-≤D, collect product triples (`LiveFactFilter::Product`) whose facts pass TT liveness only: `ingested_at <= tt` and not invalidated. Add `Fact::is_known_at(tt)` (no `valid_from` / `valid_to`). `as_known_on` calls `known_at(tt)`, never `live_at`. Do not change `is_live_at` / `as_of` / FF5 assertion bodies.

**Rejected:** `live_at(tt, wall)`. Wall YearCe coinciding with VT is fixture-scale coincidence. `as_known_on(2020)` on FF5 `valid_to: Some(2020)` still drops the closed interval (`vt >= valid_to`). Knowledge date is not valid-time.

## Anchors (D-03)

Admit reject when `wall <= max` existing wall (covers duplicate of max and any earlier/in-between). First wall always Ok. New `RuntimeError::TtAnchorRejected { wall, max_wall }`. Fold unchanged on reject (event not committed). Hydrate rebuild stays last-wins for historical logs; ANK-ADM-02 only requires monotone unique walls to round-trip.

## RED oracle (KNOWN-02)

Named test, FF5-shaped `valid_from: 2010`, `valid_to: Some(2020)`, 50-kmh, then `emit_tt_anchor` after ingest. `as_of(2015)` contains the speed fact; `as_known_on` after that knowledge date must too. Must FAIL on current HEAD (`live_at(tt, MAX)`). Must not edit `ff5_legal_pit.rs` assertion bodies.

## Out of scope (D-04 / D-05)

Rule AST; five clocks; `intern` through admit; WAL vs JSONL; honeycomb Accepted; executor `34-VERIFICATION`; `.kutha/STATE.md`; `kutha-gov ci`; Phase 35 / M016.

## Plan shape

34-01: known-on TT-only (KNOWN-01/02). 34-02: admit fail-closed (ANK-ADM-01/02).
