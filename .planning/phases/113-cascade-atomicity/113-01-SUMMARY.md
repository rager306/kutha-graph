---
phase: 113-cascade-atomicity
plan: 01
subsystem: runtime
tags: [cascade, atomicity, horn, QuantumOutcome, TDD]
requires:
  - phase: 112-triage-correction
    provides: advisor demotes whole-repo #1; apply-queue starts at CE #8
provides:
  - commit_user_cascade rolls back on later-hop rule_follow_ons / admit Err
  - m052_cascade_atomic oracles
  - SEM-m052-cascade-atomic killed
affects: [emit, inverse_knows, fork_at]

actuals:
  tokens: 2604
  tasks: 3
  commits: 5
  plan_head_before: 8d31a6a0619cf22a273496234d33f72a3ce75f4d
  plan_head_after: f3cfcab70ebb47bd8bf837e43486fed48f30bd11

tech-stack:
  added: []
  patterns: [error-path fork_at rollback of user cascade]

key-files:
  created:
    - crates/kutha-runtime/tests/m052_cascade_atomic.rs
  modified:
    - crates/kutha-runtime/src/pipeline.rs
    - docs/architecture/test-matrix.yaml
    - docs/architecture/test-matrix.md
    - CHANGELOG.md
    - .kutha/ROADMAP.md

key-decisions:
  - "Rollback via existing Runtime::fork_at on Err instead of cloning Runtime on every emit (relatedTo ingest bench has no inverse cascade)."
  - "No new governor check; KUTHA_GOV_BUDGET stays 76."
  - "Name M052-S02 on .kutha/ROADMAP.md without — ACTIVE and without editing .kutha/STATE.md."

patterns-established:
  - "User cascade: apply hops in place; on any later-hop Err, restore log prefix through fork_at and rewind next_tt."

requirements-completed: []

coverage:
  - id: D1
    description: Later-hop unknown Horn head leaves log, fold, and outcomes unchanged
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m052_cascade_atomic.rs#later_hop_unknown_head_leaves_log_fold_outcomes_unchanged
        status: pass
    human_judgment: false
  - id: D2
    description: Single-hop inverse_knows cascade still writes QuantumOutcome
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m052_cascade_atomic.rs#inverse_knows_cascade_writes_quantum_outcome
        status: pass
    human_judgment: false

duration: 61min
completed: 2026-10-11
status: complete
---

# Phase 113 Plan 01: Cascade atomicity Summary

**Later-hop Horn errors roll back the user cascade; hops and QuantumOutcome commit together on success.**

## Performance

- **Duration:** 61 min
- **Started:** 2026-10-11T05:05:30Z
- **Completed:** 2026-10-11T06:06:00Z
- **Tasks:** 3
- **Files modified:** 6

Quiet bench three `--quick` runs (`KUTHA_BENCH_N=10000`, `taskset -c 0`): median `ingest_plain.n` ≈111.4M / `as_of.n` ≈294.1k vs Phase 111 ≈116.2M / ≈272.7k (−4.1% / +7.8%).

## Accomplishments

- RED oracle `later_hop_unknown_head_leaves_log_fold_outcomes_unchanged` failed on prefix growth (log 13→14, facts 9→10, outcomes 0) before the fix.
- `commit_user_cascade` wraps apply + `persist_outcome_event`; on Err, `fork_at(prefix)` restores log/fold/indexes/outcomes and `next_tt` is rewound.
- Matrix `SEM-m052-cascade-atomic` killed (owner `M052-S02`). Clean-tree `KUTHA_GOV_BUDGET=76 uv run kutha-gov ci`: 0 HIGH / 1 LOW / 76 checks.

## Task Commits

1. **Task 1: RED oracle** - `f8c8ec4` (test)
2. **Task 2: GREEN rollback** - `da5c176` (fix)
3. **Task 3: matrix + ROADMAP owner + bench** - `694ae2e`, `1be9de1`, `f3cfcab` (docs)

## TDD Gate Compliance

`tdd_mode` was false in config; the phase still ran RED then GREEN per the lease.

- **RED:** `later_hop_unknown_head_leaves_log_fold_outcomes_unchanged` panicked on snapshot equality (left 13/9/0, right 14/10/0) after `UnknownRelation { ghostRel }`. `inverse_knows_cascade_writes_quantum_outcome` already passed. Commit `f8c8ec4`.
- **GREEN:** `fork_at` rollback. Both tests `ok. 2 passed`. Commit `da5c176`.
- **REFACTOR:** none.

## Files Created/Modified

- `crates/kutha-runtime/tests/m052_cascade_atomic.rs` — later-hop ghostRel oracle + inverse_knows green path
- `crates/kutha-runtime/src/pipeline.rs` — atomic `commit_user_cascade`
- `docs/architecture/test-matrix.yaml` / `test-matrix.md` — `SEM-m052-cascade-atomic`
- `.kutha/ROADMAP.md` — M052-S02 named without leasing STATE
- `CHANGELOG.md` — Product/Process/Trajectory Phase 113

## Decisions Made

- Reuse `fork_at` for error-path restore rather than `Clone` of `Runtime` on the happy path.
- Skip a new governor check (M050-style policy check does not fit a product kill); budget remains 76.
- Do not edit `.kutha/STATE.md` (controller owns the lease).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Matrix owner missing from committed ROADMAP**
- **Found during:** Task 3 (`kutha-gov ci` on a clean HEAD worktree)
- **Issue:** `SEM-m052-cascade-atomic` owner `M052-S02` is not a ROADMAP slice at HEAD; controller working-tree ROADMAP had it under `— ACTIVE`.
- **Fix:** Commit M052 S01–S04 without the `— ACTIVE` needle so trajectory stays valid with `Active Milestone: None`.
- **Files modified:** `.kutha/ROADMAP.md`
- **Committed in:** `1be9de1`

**Total deviations:** 1 auto-fixed (Rule 3)
**Impact on plan:** Required for `test-matrix` green without leasing STATE.

## Issues Encountered

- Dirty-tree `kutha-gov ci` in the main checkout reports HIGH docs-coupling / trajectory because controller-uncommitted `.kutha/STATE.md` uses `Active Slice: M052-S01` (pattern is `S\d+|None`) plus harness/STATE diffs without an uncommitted CHANGELOG. Product close used a clean worktree at HEAD: 0 HIGH / 1 LOW / 76 checks.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 114 (lock before heal, CE #7) can proceed. Honeycomb stays Proposed. Controller still owns `.kutha/STATE.md`.

## Self-Check: PASSED

Files `m052_cascade_atomic.rs`, `pipeline.rs`, `test-matrix.yaml`, SUMMARY, VERIFICATION present. Commits `f8c8ec4`, `da5c176`, `694ae2e`, `1be9de1`, `f3cfcab` are ancestors of HEAD.
