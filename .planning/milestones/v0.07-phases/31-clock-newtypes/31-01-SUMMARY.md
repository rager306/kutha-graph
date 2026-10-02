---
phase: 31-clock-newtypes
plan: 01
subsystem: runtime
tags: [clocks, ValidTime, TransactionTime, WallTime, compile_fail]
requires: [{ phase: v0.06, provides: M013-closed fold/query API }]
provides: ["CLK-01 query newtypes", "CLK-02 compile_fail mix"]
affects: [31-02, 32-as-known-on]
actuals: { tokens: 4061, tasks: 3, commits: 3 }
plan_head_before: 8967861951aa1e848535dd2f90f984b250297b4b
plan_head_after: 6898e6d28b9739396c8051db7771c1224441ec43
tech-stack: { added: [], patterns: ["serde-transparent clock newtypes", "impl Into query cuts", "u64 Op/Fact storage"] }
key-files: { created: [], modified: [crates/kutha-common/src/event.rs, crates/kutha-common/src/lib.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/fold_query.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/src/pipeline.rs, CHANGELOG.md] }
key-decisions: ["Op/Event/Fact clocks stay u64 so FF5 literals compile", "query API uses impl Into ValidTime/TransactionTime; no From between clocks", "compile_fail doctest not trybuild"]
patterns-established: ["Vt=ValidTime Tt=TransactionTime aliases", "WallTime exported unused until Phase 32"]
requirements-completed: [CLK-01, CLK-02]
coverage:
  - id: D1
    description: Distinct ValidTime/TransactionTime/WallTime newtypes; Vt/Tt aliases; compile_fail mix
    requirement: CLK-02
    verification: [{ kind: unit, ref: "cargo test -p kutha-common --doc#ValidTime compile_fail", status: pass }]
    human_judgment: false
  - id: D2
    description: GraphFold/Runtime as_of and live_at take Into clocks; write helpers stay u64
    requirement: CLK-01
    verification: [{ kind: integration, ref: "cargo test -p kutha-runtime --test ff5_legal_pit#ff5_as_of_t1_differs_from_as_of_t2_on_statute_log", status: pass }]
    human_judgment: false
duration: 7min
completed: 2026-10-02
status: complete
---

# Phase 31 Plan 01: Clock newtypes Summary

**Distinct serde-transparent clock newtypes on the query API; Op/Fact fields stay u64 so named oracles compile.**

## Performance
- **Duration:** 7 min · **Started:** 2026-10-02T07:10:40Z · **Completed:** 2026-10-02T07:17:12Z · **Tasks:** 3 · **Files:** 7

## Accomplishments
- `ValidTime`/`TransactionTime`/`WallTime` tuple structs; `Vt`/`Tt` aliases; mix `compile_fail` doctest green
- Fold/Runtime `as_of`/`live_at` take `impl Into`; write helpers remain `u64`
- FF5/FF6/m012a_hot_indexes green; `.kutha/STATE.md` and FF5/FF6 files byte-unchanged

## Task Commits
1. **Newtypes + compile_fail** - `804c8b4` (feat)
2. **Fold query Into** - `721b718` (feat)
3. **Runtime as_of Into** - `6898e6d` (feat)

## Decisions Made
Op/Event/Fact clocks stay integers (D-04). Query surface uses `impl Into` so `as_of(2015)` still type-checks. No `as_known_on`.

## Deviations from Plan
None - plan executed exactly as written.

## Next Phase Readiness
Ready for 31-02 (CSR/justification cuts + TIME-01 wrap). Do not start Phase 32. CBM touched paths `metadata_changed` (integrator reindex).

## Self-Check: PASSED
FOUND: event.rs, fold.rs, fold_query.rs, quantum.rs, pipeline.rs; commits 804c8b4, 721b718, 6898e6d
