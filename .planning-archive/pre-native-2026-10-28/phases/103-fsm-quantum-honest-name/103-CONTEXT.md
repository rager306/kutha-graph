---
phase: 103-fsm-quantum-honest-name
milestone: v0.37
harness: M043
slice: S01
ce_finding: "P1 #17"
---

# Phase 103 — CONTEXT

## Kill-claim

`test_ci_quantum_reaches_ok_on_this_tree` uses `skip_cargo`/`skip_emit` and still passes by asserting observe/emit state names in the trace, without cargo, tenant, or ff5 observations (CE review P1 #17).

## Goal

Rename the skip variant to scheduling-only; restore cargo/tenant/ff5 observation asserts in a live `KUTHA_LIVE_CARGO=1` variant; governor check forbids the lying name.

## Oracles

- `test_ci_quantum_scheduling_reaches_ok_with_skips` (empty observations)
- `test_ci_quantum_observations_live` (gated)
- check `fsm-quantum-honest-name`

## Out of scope

Cypher/HNSW/legal pack. Honeycomb stays Proposed. Decision-gate #6 untouched.
