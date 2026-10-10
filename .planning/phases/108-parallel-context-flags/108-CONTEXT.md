---
phase: 108-parallel-context-flags
milestone: v0.42
harness: M048
slice: S01
ce_finding: "P2 #48"
---

# Phase 108 — CONTEXT

## Kill-claim

`_eval_check_worker` rebuilds `Context` without `skip_emit` / `changed_paths`, so jobs>1 can diverge from serial when callers set those fields (CE review P2 #48).

## Goal

Extend the worker payload so parallel evaluation honors caller `skip_emit` and `changed_paths`. FSM budget 72→73 for the new check.

## Oracles

- `test_eval_check_worker_propagates_skip_emit_and_changed_paths`
- check `parallel-context-flags`

## Out of scope

#6 decision-gate, #38. Honeycomb Proposed.
