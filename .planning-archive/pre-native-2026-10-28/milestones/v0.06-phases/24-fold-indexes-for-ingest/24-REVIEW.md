---
phase: 24-fold-indexes-for-ingest
reviewed: 2026-10-01T15:03:00Z
depth: quick
files_reviewed: 5
files_reviewed_list:
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/tests/m013_fold_index_mode.rs
  - crates/kutha-runtime/benches/baseline.rs
findings:
  critical: 0
  warning: 0
  info: 1
  total: 1
status: clean
---

# Phase 24: Code Review Report

**Reviewed:** 2026-10-01T15:03:00Z  
**Depth:** quick  
**Files Reviewed:** 5  
**Status:** clean

## Summary

Quick pattern scan (secrets, dangerous calls, empty catches, debug/TODO artifacts) found nothing in scope. Targeted read of `FoldIndexMode::Lazy` wiring: `hot_maps_warm`, `ensure_hot_maps` on indexed ops (`Retract`, `Correct`, `CorrectInterval`, `Behavior`), and cold-scan fallbacks in `live_facts_at`, `fact_index_by_event_id`, `claim_known`, `has_live_triple`, and `live_supports`.

Lazy correctness invariant holds: reads branch on `hot_maps_warm` (not on `index_mode` alone); cold paths scan `facts` with the same liveness predicates as the warm lease paths. Indexed mutating ops call `ensure_hot_maps()` before lookup/mutation, so Retract/Correct paths rebuild leases when maps are cold. Assert-only ingest sets `hot_maps_warm = false` without polluting warm-path results because subsequent reads use cold scan until the next `ensure`. `Runtime::from_dict_and_events`, `fork_at`, and bench `ensure_fold_indexes` optionally warm maps for fair benchmarks without changing fold semantics.

No Critical or Warning defects proven in the reviewed files.

## Info

### IN-01: Lazy Retract/Correct not covered by integration tests

**File:** `crates/kutha-runtime/tests/m013_fold_index_mode.rs:31-63`  
**Issue:** Tests assert Lazy vs Eager parity for `as_of` after Assert-only ingest and mode round-trips. There is no dedicated test that Lazy mode matches Eager after `Retract`, `Correct`, or `CorrectInterval` while maps stay cold (without calling `ensure_fold_indexes`). Bench `retract_late` runs after `ensure_fold_indexes` for Lazy, so cold-map Retract behavior is not exercised in tests.  
**Fix:** Add tests mirroring `quantum.rs` retract/correct scenarios under `Runtime::with_index_mode(..., FoldIndexMode::Lazy)` without warming, comparing fingerprints or live counts to Eager.

---

_Reviewed: 2026-10-01T15:03:00Z_  
_Reviewer: Claude (gsd-code-reviewer)_  
_Depth: quick_
