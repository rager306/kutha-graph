# Phase 34 Context: Known-on cut and anchor admission

**Milestone:** v0.08 / harness M015 S01
**Requirements:** KNOWN-01, KNOWN-02, ANK-ADM-01, ANK-ADM-02
**Depends on:** M014 closed (v0.07)
**Source:** post-M014 audit H1/H2; ADR-013; `fold_query.rs` `as_known_on`
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** `as_known_on(D)` must not use `live_at(tt, u64::MAX)` in a way that drops facts with finite `valid_to`. Preferred: evaluate the product picture at the resolved TT with a VT cut that means “all facts live at that TT regardless of open VT end,” or `live_at(tt, wall)` if wall is YearCe and matches fixture scale — pick one in RESEARCH and lock in PLAN; RED oracle uses FF5-shaped `valid_to: Some(2020)`.
- **D-02:** FF5 `as_of` assertions stay byte-stable (or rename-only). No assertion body edits.
- **D-03:** Duplicate wall or backwards wall (new wall < max prior wall) on `TtAnchor` → admit reject. Tie policy: last-wins without admit check is forbidden.
- **D-04 (non-goals):** Rule AST (M016); five clocks; `intern` through admit; WAL vs JSONL open policy (SUMMARY triage only); honeycomb Accepted.
- **D-05:** Verifier ≠ executor model.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.

## Verifier kill-tests

- Closed-interval statute + anchors: `as_known_on` after knowledge date includes the speed fact that `as_of` sees in-window.
- Backwards wall anchor emit returns error; fold unchanged.
- Duplicate wall emit returns error.
- Persist/open: known-on and anchors agree.

## Non-goals

See D-04.

## Harness

Active Milestone M015, Active Slice **S01** while this phase runs.
