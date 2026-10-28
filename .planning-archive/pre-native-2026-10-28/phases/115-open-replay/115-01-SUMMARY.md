---
phase: 115-open-replay
plan: 01
subsystem: persist
tags: [open, replay_check, snapshot-free, TDD]
requires:
  - phase: 114-lock-before-heal
    provides: M052-S03 lock before heal closed
provides:
  - snapshot-free Define/terms open runs replay_check
  - SEM-m052-open-replay killed
affects: [open_hydrated, finish_open]
actuals:
  tokens: 3500
  tasks: 3
  commits: 3
plan_head_before: 4711388cc75968f5d7944af40f8394d04929df9e
plan_head_after: 35f1f8b978e4a1b1743baed290b6b4676301e7fb
key-files:
  created: [crates/kutha-runtime/tests/m052_open_replay.rs]
  modified: [crates/kutha-runtime/src/store.rs]
key-decisions:
  - "replay_check lives in finish_open so snapshot/Define/terms/empty cannot skip it."
  - "ROADMAP S04 stays [ ] while STATE still leases S04 (lease-drift HIGH otherwise)."
coverage:
  - id: D1
    description: Snapshot-backed open rejects empty-pin inverse
    verification: [{kind: integration, ref: "crates/kutha-runtime/tests/m052_open_replay.rs#snapshot_backed_open_rejects_empty_pin_inverse", status: pass}]
    human_judgment: false
  - id: D2
    description: Define-backed open without snapshot rejects the same log
    verification: [{kind: integration, ref: "crates/kutha-runtime/tests/m052_open_replay.rs#define_backed_open_without_snapshot_rejects_empty_pin_inverse", status: pass}]
    human_judgment: false
  - id: D3
    description: Terms-backed open without snapshot rejects the same log
    verification: [{kind: integration, ref: "crates/kutha-runtime/tests/m052_open_replay.rs#terms_backed_open_without_snapshot_rejects_empty_pin_inverse", status: pass}]
    human_judgment: false
duration: 11min
completed: 2026-10-11
status: complete
---

# Phase 115 Plan 01: Snapshot-free open replay Summary

**Define/terms open without snapshot.json reject an empty-pin inverse that snapshot-backed open already rejects; `finish_open` always `replay_check`s.**

RED `define_backed_open_without_snapshot_rejects_empty_pin_inverse` and `terms_backed_open_without_snapshot_rejects_empty_pin_inverse` failed because open succeeded (`3d3f821`). GREEN moves `replay_check` into `finish_open` (`ca06ee8`). Matrix+benches `35f1f8b`. TDD `tdd_mode` false; RED then GREEN; no refactor. Did not edit `.kutha/STATE.md`. Workspace cargo ok. `kutha-gov ci` 0 HIGH / 1 LOW / 76. Benches median ingest_plain.n ≈103.9M / as_of.n ≈299.2k.

## Deviations from Plan

**1. [Rule 3 - Blocking] ROADMAP S04 left unchecked**
- Ticking S04 while STATE still leases `S04` is trajectory lease-drift HIGH. Checkbox stays open; blurb records landed oracles.

None else — plan executed as written.

## Self-Check: PASSED
