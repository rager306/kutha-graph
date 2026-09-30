---
phase: 04-partial-correction-with-residual-intervals
plan: 01
subsystem: runtime
tags: [CorrectInterval, bitemporal, residuals, M011, S04, CORR-01]

requires:
  - phase: 03-lease-gated-next-slice
    provides: "S04 leased Active Slice; D-G1/D-10/D-15 wave-close ci pattern"

provides:
  - "Op::CorrectInterval { fact_seq, object, patch_from, patch_to } on log/fold"
  - "Named CORR-01 oracle interval_patch_leaves_vt_2012_and_2021_residuals"
  - "Fail-closed emit: UnknownFact / IntervalPatchRejected without append"

affects:
  - 04-02 whole-version Correct regression (Correct arm unchanged)
  - 04-03 GATE-01 needle registration

actuals:
  tokens: 4985
  tasks: 3
  commits: 3

plan_head_before: b854b22c7e4add764ec2f7ff7642781d3ede51cc
plan_head_after: 2dd5190e40e54daa59eb4cfddbd4e36d78e03a8b

tech-stack:
  added: []
  patterns:
    - "New Op variant for interval patch; whole-version Correct fold arm frozen (D-C1)"
    - "Invalidate original Fact VT-unclipped; push prefix, clipped replacement, suffix sharing claim_id (D-C2)"
    - "Emit reject before Event::new; fold empty-intersection no-op for replay (D-C7)"

key-files:
  created:
    - crates/kutha-runtime/tests/m011_partial_correction.rs
    - .planning/phases/04-partial-correction-with-residual-intervals/04-01-SUMMARY.md
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - CHANGELOG.md

key-decisions:
  - "CorrectInterval match arm sits immediately after Correct and before Define (04-02 Correct-arm push count)"
  - "CHANGELOG Product line shipped in the crate commit (docs-coupling HIGH); fail-closed bullet in a follow-up docs commit"
  - "Task 2 tdd=true tests lock emit rejects already shipped by the tracer — no RED commit (feature existed)"

patterns-established:
  - "Half-open leftover helper in fold.rs; emit uses pub(crate) vt_intersect"
  - "GATE-01 fsm.yaml needles stay off this wave (D-C6 / 04-03)"

requirements-completed: [CORR-01]

coverage:
  - id: D1
    description: "Interval patch leaves VT 2012 and 2021 residuals of source a; interior is P-prime; same claim_id (CORR-01)"
    requirement: CORR-01
    verification:
      - kind: integration
        ref: "crates/kutha-runtime/tests/m011_partial_correction.rs#interval_patch_leaves_vt_2012_and_2021_residuals"
        status: pass
    human_judgment: false
  - id: D2
    description: "Unknown, non-intersecting, inverted, and not-live interval patches do not append (D-C7)"
    requirement: CORR-01
    verification:
      - kind: integration
        ref: "crates/kutha-runtime/tests/m011_partial_correction.rs (fail-closed fns)"
        status: pass
    human_judgment: false
  - id: D3
    description: "Product changelog + workspace cargo + wave-close ci HIGH-free + D-10 Trajectory"
    requirement: CORR-01
    verification:
      - kind: other
        ref: "cargo test --workspace --offline"
        status: pass
      - kind: other
        ref: "uv run kutha-gov ci"
        status: pass
      - kind: other
        ref: "uv run kutha-gov explain trajectory"
        status: pass
    human_judgment: false

duration: 7min
completed: 2026-09-29
status: complete
---

# Phase 4 Plan 01: Partial correction with residual intervals Summary

**Explicit `Op::CorrectInterval` patch on a wide-VT fact leaves residual `P` at VT 2012 and 2021 and replacement `P-prime` at 2017, sharing `claim_id`, with fail-closed emit and an unchanged whole-version `Correct` fold arm.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-09-29T14:58:36Z
- **Completed:** 2026-09-29T15:05:29Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- Added `Op::CorrectInterval` with digest tag `correct-interval`, leftover fold arm after `Correct` and before `Define`, and `RuntimeError::IntervalPatchRejected` (D-C1, D-C2, D-C7).
- Named CORR-01 test `interval_patch_leaves_vt_2012_and_2021_residuals` is green; serde JSON round-trip of the new op; `replay_check` Ok (D-C3).
- Fail-closed writes do not append; Product changelog dated 2026-09-29 records the write. GATE-01 needles not registered (D-C6).

## Task Commits

1. **Task 1: End-to-end interval-patch residuals at VT 2012 and 2021** - `cfa3e3e` (feat)
2. **Task 2: Fail-closed interval-patch writes do not append** - `a65f583` (test)
3. **Task 3: Product changelog fail-closed line (wave-close cargo/ci already green)** - `2dd5190` (docs)

## Trajectory

**(1) Commands:**
- `uv run kutha-gov ci`
- `uv run kutha-gov explain trajectory`
- `cargo test --workspace --offline` (D-15; this wave touched `crates/`)

**(2) Outcomes:** `kutha-gov ci` exit 0; HIGH 0; LOW/WARN 0 (27 checks, H4 dogfood). Cargo workspace exit 0.

**(3) Excerpt from `explain trajectory` (≤8 lines):**

```text
check: trajectory
purpose: Active Milestone/Slice are None or exist on ROADMAP
authority: none — harness does not accept ADRs or claim product readiness
source: .kutha/dictionaries/checks.yaml
steps:
  - file_exists  .kutha/STATE.md
  - file_exists  .kutha/ROADMAP.md
  - pointer_in_other_file  .kutha/STATE.md
```

**(4)** Green governor is not ADR Accepted and not `L_capability`.

Locks cited: D-C1 (new variant; Correct arm untouched), D-C2 (residuals share `claim_id`), D-C3 (minimal residual oracle only), D-C4 (no admission/conflict/allowed-action), D-C7 (half-open reject/no-op). D-C5/D-10/D-15 wave close as above. D-C6 GATE-01 needles remain plan 04-03.

## Files Created/Modified

- `crates/kutha-common/src/event.rs` — `CorrectInterval` variant, `Event::new` object_ids, digest tag `correct-interval`
- `crates/kutha-runtime/src/fold.rs` — leftover helper; CorrectInterval apply arm; Correct arm unchanged
- `crates/kutha-runtime/src/quantum.rs` — emit gates, `IntervalPatchRejected`, `op_relation` / `derivation_eligible_at`
- `crates/kutha-runtime/tests/m011_partial_correction.rs` — CORR-01 oracle + fail-closed tests
- `CHANGELOG.md` — 2026-09-29 Product entry

## Decisions Made

- Interval-patch is a new `Op` variant; `Op::Correct` fold meaning is unchanged (D-C1).
- Emit rejects non-intersect / inverted / not-live; fold no-ops empty intersection for replay (D-C7).
- Product changelog in the crate commit to satisfy `docs-coupling` (project rule over per-task file split).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] CHANGELOG in the tracer crate commit**
- **Found during:** Task 1 commit (`docs-coupling` HIGH if `crates/**/*.rs` staged without `CHANGELOG.md`)
- **Issue:** Plan listed CHANGELOG only on task 3; precommit/git_path_implies requires it with crate diffs.
- **Fix:** Dated Product H2 in the feat commit; task 3 added the fail-closed Product bullet.
- **Files modified:** `CHANGELOG.md`
- **Verification:** `uv run kutha-gov ci` docs-coupling OK
- **Committed in:** `cfa3e3e` (Task 1), `2dd5190` (Task 3)

**2. [Rule 2 - Missing Critical] Task 2 tests passed on first run (no RED)**
- **Found during:** Task 2 (`tdd="true"`)
- **Issue:** Tracer already implemented `UnknownFact` / `IntervalPatchRejected` before Event::new. Writing the planned tests could not fail on an assertion for missing gates (tdd.md unexpected GREEN / feature exists).
- **Fix:** Added fail-closed tests as a `test(04-01)` commit without a fake RED or revert of the tracer.
- **Files modified:** `crates/kutha-runtime/tests/m011_partial_correction.rs`
- **Verification:** `cargo test -p kutha-runtime --offline --test m011_partial_correction -- --test-threads=1` — 5 passed
- **Committed in:** `a65f583`

---

**Total deviations:** 2 auto-handled (1 blocking docs-coupling, 1 TDD RED skipped because feature existed)
**Impact on plan:** No scope creep. Correct arm untouched. `.kutha/STATE.md` not edited.

## TDD Gate Compliance

Plan frontmatter is `type: execute` (not `type: tdd`). `workflow.tdd_mode` is false. Task 2 carried `tdd="true"`; RED evidence was not recorded because the reject path already existed after the tracer. GREEN lock is the `test(04-01)` commit. No `refactor(04-01)` commit.

## Authentication Gates

None.

## Issues Encountered

None beyond the documented deviations. Pre-commit hook file was not installed in this checkout; `uv run kutha-gov precommit` was run on later commits. Task 2 briefly staged a crate test file without CHANGELOG (HIGH on a manual precommit run) before the Task 3 changelog bullet.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `04-02-PLAN.md` (CORR-02 whole-version Correct does not invent residuals). Do not register GATE-01 needles until 04-03. Active Slice remains S04; do not edit `.kutha/STATE.md`.

## Self-Check: PASSED
