---
phase: 114-lock-before-heal
plan: 01
subsystem: persist
tags: [lock, heal, kutha.lock, TDD]
requires:
  - phase: 113-cascade-atomicity
    provides: M052-S02 cascade atomicity closed
provides:
  - lock before interrupted-seal heal
  - SEM-m052-lock-before-heal killed
affects: [Persister.attach, attach_segmented]
actuals: {tokens: 2627, tasks: 3, commits: 3}
plan_head_before: 75a35788c1604d828819f8ec84c548bec099e454
plan_head_after: a88e7e6961673e098f51e65d2f9485d8b21c74d9
key-files:
  created: [crates/kutha-runtime/tests/m052_lock_before_heal.rs]
  modified: [crates/kutha-runtime/src/persist_append.rs, crates/kutha-runtime/src/segment.rs]
key-decisions:
  - "Read-only pre-lock attach_holding; heal only after kutha.lock (COMMIT-01 InvalidData kept)."
coverage:
  - id: D1
    description: Losing attach while lock held does not heal
    verification: [{kind: integration, ref: "crates/kutha-runtime/tests/m052_lock_before_heal.rs#losing_attach_while_writer_holds_lock_does_not_heal", status: pass}]
    human_judgment: false
  - id: D2
    description: Attach heals interrupted seal after lock released
    verification: [{kind: integration, ref: "crates/kutha-runtime/tests/m052_lock_before_heal.rs#attach_heals_interrupted_seal_after_lock_released", status: pass}]
    human_judgment: false
duration: 17min
completed: 2026-10-11
status: complete
---

# Phase 114 Plan 01: Lock before heal Summary

**Attach heals interrupted seal only after kutha.lock; a losing attach is WouldBlock with unchanged bytes.**

RED `losing_attach_while_writer_holds_lock_does_not_heal` failed creating `seg-00000004.log` then WouldBlock (`198dd2e`). GREEN skips heal when unlocked and re-enters after acquire (`28538aa`). Matrix+benches `a88e7e6`. TDD `tdd_mode` false; RED then GREEN; no refactor. Seed used genesis `relatedTo`. Did not edit `.kutha/STATE.md`. Workspace cargo ok. Benches median ingest_plain.n ≈106.7M / as_of.n ≈303.3k. Live ci HIGH is pre-existing: 113 SUMMARY 143>60 lines; STATE `Active Slice: M052-S03` vs pattern `S\d+|None`.

## Self-Check: PASSED
