---
phase: 36-test-hardening-t2-t6
plan: 01
subsystem: testing
tags: [trybuild, clocks, JustificationCite]
requires:
  - phase: 31-clock-newtypes
    provides: ValidTime/TransactionTime/WallTime newtypes
provides:
  - trybuild E0308 Tt and WallTime into ValidTime
  - JustificationCite tt/vt swap oracle
affects: [kutha-common, kutha-runtime]
actuals:
  tokens: 3260
  tasks: 2
  commits: 2
plan_head_before: 82f2aaf6b72b40564839a4af8a6c154c4c46760e
plan_head_after: c008f774327ec0ab47b89672e94ae8b70f9934d5
tech-stack:
  added: [trybuild]
  patterns: [trybuild UI compile_fail for clock mix]
key-files:
  created:
    - crates/kutha-common/tests/ui_clock_mix.rs
    - crates/kutha-common/tests/ui/clock_mix.rs
    - crates/kutha-common/tests/ui/clock_mix.stderr
    - crates/kutha-runtime/tests/m016_cite_tt_vt.rs
  modified:
    - crates/kutha-common/Cargo.toml
    - Cargo.lock
    - CHANGELOG.md
key-decisions:
  - "D-01: dtolnay trybuild 1.x on kutha-common; rustdoc compile_fail kept"
  - "D-02: swap oracle is Justification equality; Op tt/vt stay u64"
requirements-completed: [TH-01, TH-02]
duration: 3min
completed: 2026-10-02
status: complete
---

# Phase 36 Plan 01: trybuild clock mix and cite tt/vt Summary

trybuild E0308 for Tt/WallTime→ValidTime; named JustificationCite swap oracle.

## What shipped

- `cargo test -p kutha-common --test ui_clock_mix`: `clock_mix.stderr` has two `error[E0308]` (TransactionTime and WallTime).
- `swapped_justification_cite_tt_vt_fails_equality` stores `tt=t1`, `vt=0` and `assert_ne!` on swapped clone + log pair.

## Deviations from Plan

**1. [Rule 1 - Bug] EventLog::iter is not DoubleEnded**
- **Found during:** Task 2
- **Fix:** `log().as_slice().iter().rev()`; `tt`/`vt` are Copy u64
- **Commit:** c008f77

## Self-Check: PASSED

- FOUND: crates/kutha-common/tests/ui/clock_mix.stderr, m016_cite_tt_vt.rs
- FOUND: 4931d5b, c008f77
