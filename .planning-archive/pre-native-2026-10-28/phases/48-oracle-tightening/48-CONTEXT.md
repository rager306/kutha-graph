# Phase 48: Differential tests and oracle tightening — Context

**Milestone:** v0.11 / harness M018 S04
**Requirements:** ORACLE-01, GATE-01
**Depends on:** Phase 47
**Gathered:** 2026-10-06
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

Tighten I-F15/I-F16 so they measure what they claim, and record GATE-01. DIFF-01 is already DIFF-02 in M019 — do not re-implement fold/CSR/rebuild. Do not thaw CLOCK-01, Cypher, HNSW.

## Locked decisions

- **D-ORACLE-I-F15:** Keep `extra_relatedto_horn_visits_constant_in_n` (`m017_ingest_scaling.rs`) as the visit-bound oracle. If GREEN, it already fails an O(N) extra `facts()` walk — do not re-introduce that walk. Document the bound in matrix/ADR-041 Evidence if stale.
- **D-ORACLE-I-F16:** Re-derive is now product behaviour (Phase 47). Pin I-F16 to `horn_correct_derived_not_live_after_premise_correct` (and interval twin). Do not add a second fold path.
- **D-GATE-01 controller step:** Do not invent a second bench harness. Record GATE-01 as the existing `gsd-auto-contract` bench gate plus `BENCH-controller` matrix row; add a selftest/governor cite only if a check already exists. Product close of this slice still needs `kutha-gov ci` and the named cargo pins.
- **D-SCOPE:** honeycomb Proposed. No `.kutha/STATE.md` from executor. Verifier writes `48-VERIFICATION.md`.

## Verifier kill-tests

- I-F15 test exists and would fail if Horn examine grew linearly with extra unrelated facts (or documents why GREEN after M017 repair).
- I-F16 named Correct-rederive tests fail if `rule_follow_ons` skips Correct again.
- GATE-01 named in CHANGELOG and/or matrix `BENCH-controller`.
- FF5 unchanged.

## Harness

Active Milestone M018, Active Slice **S04**.
