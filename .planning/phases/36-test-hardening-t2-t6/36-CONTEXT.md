# Phase 36 Context: Test hardening T2–T6

**Milestone:** v0.09 / harness M016 S01
**Requirements:** TH-01..TH-05
**Depends on:** M015 closed
**Source:** test-audit T2–T6 (2026-10-02)
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** trybuild crate (dev-dep) or in-tree `tests/trybuild` for E0308 clock mix; weaken rustdoc `compile_fail` only if trybuild covers it.
- **D-02:** TH-02 may add a test without changing Op field types to newtypes (that is larger); assert cite stores distinct tt/vt and a swapped construction fails the oracle.
- **D-03:** TH-04/05 may fix `GraphFold::replay` / hydrate tail to apply `TtAnchor` — that is correctness, not a new feature.
- **D-04 (non-goals):** Rule AST (Phase 37+); five clocks; intern/WAL; honeycomb Accepted.
- **D-05:** Verifier ≠ executor.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.

## Verifier kill-tests

- trybuild fails on Tt→Vt mix with type error.
- Swapped JustificationCite tt/vt fails named assert.
- Eager vs Lazy as_known_on equal; fork_at before later anchor differs.
- Snapshot mid-log + tail with TtAnchor → opened.as_known_on matches full.
- GraphFold::replay applies anchors (oracle).

## Non-goals

See D-04.

## Harness

Active Milestone M016, Active Slice **S01** while this phase runs.
