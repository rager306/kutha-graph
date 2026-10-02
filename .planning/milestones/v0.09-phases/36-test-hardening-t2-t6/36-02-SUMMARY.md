---
phase: 36-test-hardening-t2-t6
plan: 02
subsystem: testing
tags: [as_known_on, TtAnchor, snapshot, GraphFold]
requires:
  - phase: 32-tt-anchor-and-as-known-on
    provides: Op::TtAnchor and as_known_on
provides:
  - Eager/Lazy and fork_at known-on oracles
  - mid-cut snapshot+tail as_known_on
  - GraphFold::replay TtAnchor known-on
affects: [kutha-runtime]
actuals:
  tokens: 2003
  tasks: 2
  commits: 2
plan_head_before: 0065aa2fc1c91c76779109ae9edc1e562f42d97b
plan_head_after: 635028f34da8dc7a2cb61cb1d0f8823ad50fad9a
tech-stack:
  added: []
  patterns: [as_known_on oracles not fingerprint-only]
key-files:
  created:
    - crates/kutha-runtime/tests/m016_known_on_hardening.rs
  modified:
    - crates/kutha-runtime/src/quantum_invariants.rs
    - CHANGELOG.md
key-decisions:
  - "D-03: no fold.rs/hydrate.rs patch; apply already inserts TtAnchor"
  - "replay as_known_on equality needs bind_reserved (product filter); fingerprint stays facts-only"
requirements-completed: [TH-03, TH-04, TH-05]
duration: 6min
completed: 2026-10-02
status: complete
---

# Phase 36 Plan 02: known-on snapshot replay Summary

Eager==Lazy known-on; fork before later anchor differs; mid-cut snapshot+tail and GraphFold::replay match live as_known_on.

## RED/GREEN

- First `replayed.as_known_on(D)` vs live failed: replay lacked `bind_reserved`, so `kutha:` leaked into product. Not a skipped `TtAnchor` (`as_known_on` was `Some`).
- After `bind_reserved(rt.dictionary())`: 4/4 `m016_known_on_hardening` green. `snapshot_plus_tail_matches_full_replay` green with tail `emit_tt_anchor`.

## Deviations from Plan

**1. [Rule 1 - Bug] Replay oracle compared unfiltered vs product picture**
- **Fix:** `replayed.bind_reserved(rt.dictionary())` then equality
- **Commit:** 217e657

**2. [Rule 3] No production apply/hydrate edit** — RED after oracle fix was green (D-03).

## Self-Check: PASSED

- FOUND: m016_known_on_hardening.rs, quantum_invariants.rs emit_tt_anchor
- FOUND: 217e657, 635028f
