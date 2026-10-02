---
phase: 32-tt-anchor-and-as-known-on
plan: 02
subsystem: testing
tags: [as_known_on, as_of, ANK-03, FF5]
requires: [{ phase: 32-tt-anchor-and-as-known-on, provides: ANK-01 Op::TtAnchor log facts }]
provides: ["ANK-03 true-on-D vs known-on-D named test", "ANK-02 miss and greatest-anchor kill-tests"]
affects: [33-fixture-migration]
actuals: { tokens: 1188, tasks: 3, commits: 3 }
plan_head_before: a0a3c3704717ee78ad20ce5524db2fda0618ab9f
plan_head_after: 90fb5bf15cbda838888c9b2cf6c5a168a8339704
tech-stack: { added: [], patterns: ["named as_of vs as_known_on oracle", "gitignored tmp persist store"] }
key-files: { created: [crates/kutha-runtime/tests/m014_as_known_on.rs], modified: [CHANGELOG.md] }
key-decisions: ["Named test seeds statute between anchors 2014 and 2020", "FF5/FF6 assertion bodies and file bytes unchanged"]
patterns-established: ["as_known_on None miss; W1 < D ≤ W2 uses A1 tt"]
requirements-completed: [ANK-02, ANK-03]
coverage:
  - id: D1
    description: as_of(2015) sees statute; as_known_on(2015) does not; as_known_on(2020) does
    requirement: ANK-03
    verification: [{ kind: integration, ref: "crates/kutha-runtime/tests/m014_as_known_on.rs#true_on_d_differs_from_known_on_d", status: pass }]
    human_judgment: false
  - id: D2
    description: miss without anchor; greatest-anchor; persist/open; FF5/workspace green
    requirement: ANK-02
    verification: [{ kind: integration, ref: "cargo test --workspace --offline", status: pass }]
    human_judgment: false
duration: 3min
completed: 2026-10-02
status: complete
---

# Phase 32 Plan 02: true-on-D vs known-on-D Summary

**Named oracle proves as_of(2015) sees a statute that as_known_on(2015) does not; FF5/FF6 bytes unchanged.**

- **Duration:** 3 min · **Started:** 2026-10-02T08:12:34Z · **Completed:** 2026-10-02T08:15:06Z · **Tasks:** 3 · **Files:** 2

- `true_on_d_differs_from_known_on_d`: anchor@2014, statute vf=2010, anchor@2020
- Kill-tests: None without anchor; W1 < D ≤ W2 uses A1; persist/open in `tmp/p32-02-store`
- `cargo test --workspace --offline` ok; FF5/FF6/`.kutha/STATE.md` byte-unchanged

## Task Commits
1. **Named true-on-D vs known-on-D** - `beff81c` (test)
2. **Miss and greatest-anchor** - `6de4176` (test)
3. **FF5 green + SUMMARY** - `90fb5bf` (test)

## Deviations from Plan
None - plan executed exactly as written.

## Next Phase Readiness
Phase 32 code+oracles complete. Independent verifier writes `32-VERIFICATION.md` (D-05). Do not start Phase 33.

## Self-Check: PASSED
FOUND: m014_as_known_on.rs, 32-01-SUMMARY.md, 32-02-SUMMARY.md; commits beff81c, 6de4176, 90fb5bf
