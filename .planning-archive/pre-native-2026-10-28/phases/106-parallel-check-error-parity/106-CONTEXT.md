---
phase: 106-parallel-check-error-parity
milestone: v0.40
harness: M046
slice: S01
ce_finding: "P2 #23"
---

# Phase 106 — CONTEXT

## Kill-claim

`_eval_check_worker` lets `check.run` exceptions escape to `fut.result()`, becoming `worker-error`, while serial `eval_check` maps them to `check-error` (CE review P2 #23).

## Goal

Worker reuses `eval_check` so jobs=1 and jobs>1 share category/message. Reserve `worker-error` for transport failures. FSM budget 70→71 for the new check.

## Oracles

- `test_mini_tree_check_raise_is_check_error_jobs_1_and_2`
- check `parallel-check-error-parity`

## Out of scope

#39 observe-skipped, #48 skip_emit/changed_paths payload, #6 decision-gate. Honeycomb Proposed.
