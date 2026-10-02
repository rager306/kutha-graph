---
phase: 34-known-on-and-anchor-admission
plan: 01
subsystem: runtime
tags: [as_known_on, is_known_at, closed-VT, KNOWN-01]
requires: [{ phase: 32-tt-anchor-and-as-known-on, provides: as_known_on Option lookup }]
provides: ["KNOWN-01 TT-only as_known_on", "KNOWN-02 RED→GREEN closed-VT oracle"]
affects: [34-02]
actuals: { tokens: 1544, tasks: 3, commits: 3 }
plan_head_before: 3659f466c0b0908ad61100eec6812d95450985e5
plan_head_after: 0985e9f3b4a0a032f7128acb1d04e890ae2032b1
tech-stack: { added: [], patterns: ["Fact::is_known_at TT-only", "GraphFold::known_at Product filter"] }
key-files: { created: [crates/kutha-runtime/tests/m015_known_on.rs], modified: [crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/fold_query.rs, CHANGELOG.md] }
key-decisions: ["D-01: TT-only product at resolved tt; rejected live_at(tt, wall)"]
patterns-established: ["as_known_on never calls live_at; is_live_at unchanged"]
requirements-completed: [KNOWN-01, KNOWN-02]
coverage:
  - id: D1
    description: Closed VT statute visible on as_known_on after knowledge date
    requirement: KNOWN-01
    verification: [{ kind: integration, ref: "crates/kutha-runtime/tests/m015_known_on.rs#closed_vt_interval_survives_as_known_on", status: pass }]
    human_judgment: false
  - id: D2
    description: RED oracle failed on HEAD live_at(tt, MAX); GREEN after known_at; persist/open; FF5 bytes unchanged
    requirement: KNOWN-02
    verification: [{ kind: integration, ref: "tmp/p34-01-red.txt FAILED then tmp/p34-01-green.txt ok", status: pass }]
    human_judgment: false
duration: 2min
completed: 2026-10-02
status: complete
---

# Phase 34 Plan 01: TT-only as_known_on Summary

**as_known_on keeps closed VT intervals via TT-only `known_at`; `as_of`/`is_live_at` unchanged.**

- **Duration:** 2 min · **Started:** 2026-10-02T09:31:47Z · **Completed:** 2026-10-02T09:33:57Z · **Tasks:** 3 · **Files:** 4

- RED: `closed_vt_interval_survives_as_known_on` failed on HEAD (`live_at(tt, MAX)` dropped `valid_to: Some(2020)`); cargo exit 101; `tmp/p34-01-red.txt`
- GREEN: `Fact::is_known_at` + `GraphFold::known_at`; no `live_at(tt, u64::MAX)` in `fold_query.rs`
- Persist/open `tmp/p34-01-store`; FF5/`.kutha/STATE.md` bytes unchanged; no `34-VERIFICATION`; no `kutha-gov ci`

## Task Commits
1. **RED oracle** - `4c79604` (test)
2. **TT-only known_at** - `4170566` (feat)
3. **Persist + CHANGELOG** - `0985e9f` (test)

## Deviations from Plan
None - plan executed as written. `gsd_run check tdd-red-evidence` reports INVALID_RED (`zero_tests_discovered`: cargo libtest ≠ TAP). TDD_MODE is false; cargo named-test assertion failure is the RED evidence.

## Next Phase Readiness
Ready for 34-02 admit fail-closed. Do not write 34-VERIFICATION (D-05). Do not start Phase 35.

## Self-Check: PASSED
FOUND: fold.rs, fold_query.rs, m015_known_on.rs, 34-01-SUMMARY.md; commits 4c79604, 4170566, 0985e9f
