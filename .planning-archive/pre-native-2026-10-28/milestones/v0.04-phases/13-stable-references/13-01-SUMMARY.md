---
phase: 13-stable-references
plan: 01
subsystem: runtime-refs
tags: [kutha, rust, event-id, retract, correct, REF-01]

requires:
  - phase: 12-log-native-sot
    provides: log-native QuantumOutcome/JustificationCite; Fact.claim_id; M011 interval patch
provides:
  - Op::Retract/Correct/CorrectInterval take minting EventId
  - Fact.event_id set from Event.id on mint and on correction residuals
  - emit UnknownFact/IntervalPatchRejected keyed by EventId
  - Named REF-01 oracle retract_by_event_id_invalidates_support
affects:
  - 13-02
  - 13-03

actuals:
  tokens: 7277
  tasks: 2
  commits: 1
  plan_head_before: 6d90ebb402d445a887b4dd3d8f0155289cbfd67c
  plan_head_after: 4a5d361d621e71ca238e4af1126f0824665ca653

tech-stack:
  added: []
  patterns:
    - operator target is Fact.event_id (minting Event.id), not fold-local seq
    - emit fail-closed UnknownFact before append

key-files:
  created:
    - crates/kutha-runtime/tests/m012a_stable_refs.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/wal.rs
    - crates/kutha-runtime/tests/m011_claim_supports.rs
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs
    - crates/kutha-runtime/tests/m011_partial_correction.rs
    - CHANGELOG.md

key-decisions:
  - "Retract/Correct/CorrectInterval share EventId because UnknownFact and IntervalPatchRejected are one type"
  - "Lookup prefers the live Fact when several residuals share one CorrectInterval Event.id"
  - "CSR TypedEdge.fact_seq and ConflictReport seqs stay lease indexes (ING-03 is Phase 14)"

patterns-established:
  - "Pattern: Fact.event_id = Event.id on Assert/Behavior; replacement rows use the correction Event.id"
  - "Pattern: digest hashes EventId bytes, not u64 seq"

requirements-completed: [REF-01]

coverage:
  - id: D1
    description: Retract by minting EventId invalidates that support; unknown EventId does not append
    requirement: REF-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_stable_refs.rs#retract_by_event_id_invalidates_support
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/src/quantum.rs#retract_keeps_loser
        status: pass
    human_judgment: false
  - id: D2
    description: Correct and CorrectInterval apply by minting EventId; M011 residuals stay green
    requirement: REF-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_partial_correction.rs#interval_patch_leaves_vt_2012_and_2021_residuals
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_partial_correction.rs#whole_version_correct_does_not_invent_residuals
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_partial_correction.rs#interval_patch_unknown_fact_does_not_append
        status: pass
    human_judgment: false

duration: 7min
completed: 2026-10-01
status: complete
---

# Phase 13 Plan 01: Minting EventId retract/correct Summary

**Retract, Correct, and CorrectInterval target Fact.event_id (minting Event.id); fold-local seq is no longer the operator key**

## Performance

- **Duration:** 7 min
- **Started:** 2026-09-30T17:59:21Z
- **Completed:** 2026-09-30T18:06:47Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Named REF-01 oracle `retract_by_event_id_invalidates_support` retracts by minting EventId and fail-closes on `EventId::nil`.
- `Fact.event_id` is mixed into the fold fingerprint; snapshot/replay still match.
- M011 interval residuals, whole-version Correct, claim-support retract, and Phase 12 LOG tests stay green.

## Task Commits

1. **Task 1+2: Retract and Correct by minting EventId** - `4a5d361` (feat)

**Plan metadata:** pending docs commit after STATE/ROADMAP update.

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - Retract/Correct/CorrectInterval take `event_id`; digest hashes those bytes
- `crates/kutha-runtime/src/fold.rs` - `Fact.event_id`; apply lookup prefers live row
- `crates/kutha-runtime/src/quantum.rs` - emit `UnknownFact` / `IntervalPatchRejected` keyed by EventId
- `crates/kutha-runtime/src/wal.rs` - WAL fixture Retracts use EventId
- `crates/kutha-runtime/tests/m012a_stable_refs.rs` - REF-01 oracle
- `crates/kutha-runtime/tests/m011_*.rs` - call sites use receipt/Fact.event_id
- `CHANGELOG.md` - Product plane entry (docs-coupling)

## Decisions Made

- Finished Correct/CorrectInterval in the same wave as Retract because `UnknownFact` / `IntervalPatchRejected` are shared (plan allowed this).
- Did not change `JustificationCite` (D-02) or TypedEdge seq (lease).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Shared error type forced Correct EventId in the same commit as Retract**
- **Found during:** Task 1
- **Issue:** Changing `UnknownFact` to `event_id` left Correct/CorrectInterval unable to compile against `fact_seq`.
- **Fix:** Both operators take `EventId` in `4a5d361`.
- **Files modified:** event.rs, fold.rs, quantum.rs, m011_partial_correction.rs, m011_e2e_fixture.rs
- **Verification:** `m011_partial_correction` and workspace cargo tests
- **Committed in:** 4a5d361

**2. [Rule 2 - Missing Critical] CHANGELOG in the product commit**
- **Found during:** Task 1
- **Issue:** `docs-coupling` requires CHANGELOG.md with `crates/**/*.rs`.
- **Fix:** Dated Product entry landed with the feat commit instead of waiting for a second task commit.
- **Files modified:** CHANGELOG.md
- **Committed in:** 4a5d361

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 coupling)
**Impact on plan:** Correct work planned for task 2 shipped in the same commit. No scope creep.

## TDD Gate Compliance

Plan type is `execute` (not `type: tdd`); `workflow.tdd_mode` is false. Isolated RED on the new `Op::Retract { event_id }` API is a compile failure (INVALID_RED). Test + implementation landed together in `4a5d361`.

## Issues Encountered

- `EventLog::iter` is not `DoubleEndedIterator`; the oracle finds the last Retract with `filter(...).last()`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- REF-01 operators are EventId-keyed. Plan 13-02 can drop seq from `JustificationCite`.
- Do not clear S02. Do not register fsm.yaml names until 13-03.

## Self-Check: PASSED

- `crates/kutha-runtime/tests/m012a_stable_refs.rs` FOUND
- Commit `4a5d361` FOUND
- Named REF-01 test green; workspace cargo tests green including FF5/FF6 and Phase 12 LOG

---
*Phase: 13-stable-references*
*Completed: 2026-10-01*
