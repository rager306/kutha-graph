---
phase: 104-nameless-dict-retain
milestone: v0.38
harness: M044
slice: S01
ce_finding: "P2 #20"
---

# Phase 104 — CONTEXT

## Kill-claim

A checks.yaml row without `id` replaces the entire loaded dictionary with one `LoadErrorCheck`, discarding every valid check parsed so far (CE review P2 #20).

## Goal

Keep already-collected checks; add `LoadErrorCheck` alongside them with the row index in the message. Quantum still fails closed.

## Oracles

- `test_nameless_dict_id_is_load_error` (retains `ok-check` + `_dictionary`)
- Matrix `SEM-m044-nameless-dict-retain`

## Out of scope

#22 glob_absent comment policy (M045). #6 decision-gate. Cypher/HNSW. Honeycomb Proposed.
