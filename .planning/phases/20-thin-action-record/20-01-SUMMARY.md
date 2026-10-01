---
phase: 20-thin-action-record
plan: 01
subsystem: runtime
tags: [action-record, admission, event-log, ACT-01]

requires:
  - phase: 19-admission-and-policy-meta-facts
    provides: Op::PinPolicy / Op::RecordAdmission, record_justification, admission_status_at
provides:
  - "Op::RecordAction log fact that binds justification args to admission and policy_version"
  - "Runtime::record_action / action_record_at; UnknownAdmission fail-closed"
  - "Leased e2e helper records Action after justification"
affects: [20-02, 20-03, record_action, action_record]

actuals:
  tokens: 6066
  tasks: 3
  commits: 4
  plan_head_before: b22e4d3a36fbbe1fcb471993582d91dc9e36e0be
  plan_head_after: 783b2c7c64f18e62c11c7cc2cc073b3b22ef9e6b

tech-stack:
  added: []
  patterns:
    - "RecordAction mirrors RecordAdmission: dedicated Op, skip-serialized fold entries, MetaOpRejected on public emit"
    - "record_action copies admitted and policy_version from the live admission-entry, not from the caller"

key-files:
  created:
    - crates/kutha-runtime/tests/m012_action_record.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/lib.rs
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs

key-decisions:
  - "RecordAction is a dedicated Op, not Assert/Define/Behavior/PinPolicy/RecordAdmission"
  - "Action.policy_version is copied from the live RecordAdmission; public emit is MetaOpRejected"
  - "Leased e2e helper looks up admission at u64::MAX because RecordAdmission is ingested after t1"

patterns-established:
  - "Action-entries skip-serialize; hydrate_from_log rebuilds them from the log walk"
  - "UnknownAdmission fails closed and does not grow the log"

requirements-completed: [ACT-01]

coverage:
  - id: D1
    description: "RecordAction is on the log and is not a graph Fact; action_record_at binds claim, sources, rule_version, admitted, and policy_version"
    requirement: ACT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_action_record.rs#action_binds_args_to_admission_and_policy
        status: pass
    human_judgment: false
  - id: D2
    description: "Action cites the admission pin; public emit and missing admission do not grow the log"
    requirement: ACT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_action_record.rs#action_cites_admission_pin_and_rejects_public_emit
        status: pass
    human_judgment: false
  - id: D3
    description: "Leased e2e records an Action after justification; ALL/RULE/ADM/FF5/FF6/M012a LOG/REF stay green"
    requirement: ACT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_justification_cites_sources_and_rejects_stale_admission
        status: pass
    human_judgment: false

duration: 10min
completed: 2026-10-01
status: complete
---

# Phase 20 Plan 01: Thin Action record tracer Summary

**Log-native Op::RecordAction binds justification args to the live admission decision and policy version; public emit and missing admission fail closed**

## Performance

- **Duration:** 10 min
- **Started:** 2026-10-01T03:23:21Z
- **Completed:** 2026-10-01T03:33:40Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments

- `Op::RecordAction` is an appendable log fact, not a graph Fact (ACT-01, D-01).
- `Runtime::record_action` copies `admitted` and `policy_version` from the live admission-entry and resolved arguments from the justification cite.
- `action_record_at` answers AS OF; `tt=0` is None; public emit is `MetaOpRejected`; missing cite or retracted admission is `UnknownAdmission` without log growth.
- Leased e2e `record_t1_justification` records an Action on the product path after a successful cite.

## Task Commits

1. **Task 1 RED: End-to-end record Action** - `eab64d0` (test)
2. **Task 1 GREEN: thin Action record** - `b237a68` (feat)
3. **Task 2: Cite admission pin; reject public emit** - `cae8606` (test)
4. **Task 3: Leased fixture records Action** - `783b2c7` (feat)

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - `Op::RecordAction` plus `digest_bytes` tag `record-action`
- `crates/kutha-runtime/src/fold.rs` - skip-serialized action-entries, `action_record_at`, `apply_action_side`
- `crates/kutha-runtime/src/quantum.rs` - `ActionRecord`, `record_action`, `UnknownAdmission`, MetaOpRejected emit
- `crates/kutha-runtime/src/lib.rs` - export `ActionRecord`
- `crates/kutha-runtime/tests/m012_action_record.rs` - ACT-01 tracer and fail-closed oracles
- `crates/kutha-runtime/tests/m011_e2e_fixture.rs` - helper records Action after justification

## Decisions Made

- Dedicated `Op::RecordAction`; not folded into `record_justification` (ADM log-growth-by-2 stays).
- Retract of an action-entry EventId is accepted (`has_action_entry` on the UnknownFact gate).
- Leased helper binds at `u64::MAX` × `VT_INTERIOR` because admission ingest is after `t1`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Leased helper cannot look up admission at t1**
- **Found during:** Task 3 (Leased fixture records Action)
- **Issue:** `record_action(&jid, self.t1, VT_INTERIOR)` returns `UnknownAdmission` because `RecordAdmission` is ingested after `t1`, so `is_live_at(t1, …)` is false.
- **Fix:** Helper calls `record_action(&jid, u64::MAX, VT_INTERIOR)` so the bind uses the live tip. Lookup semantics at `(tt, vt)` stay strict.
- **Files modified:** `crates/kutha-runtime/tests/m011_e2e_fixture.rs`
- **Verification:** `cargo test -p kutha-runtime --offline --test m011_e2e_fixture -- --exact`
- **Committed in:** `783b2c7` (Task 3)

**2. [Rule 2 - Missing Critical] UnknownFact gate accepts action-entry EventIds**
- **Found during:** Task 1 GREEN (`has_action_entry` dead_code)
- **Issue:** Retract of a `RecordAction` EventId would fail `UnknownFact` even though `apply_action_side` invalidates the row. Needed for 20-02 persist/retract.
- **Fix:** UnknownFact Retract gate also checks `has_action_entry`.
- **Files modified:** `crates/kutha-runtime/src/quantum.rs`
- **Verification:** tracer still green; no unused warning
- **Committed in:** `b237a68` (Task 1 GREEN)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** Required for e2e bind and retract of Action rows. No scope creep into six dictionaries or Phase 21.

## TDD Gate Compliance

- RED: `eab64d0` — `action_binds_args_to_admission_and_policy` failed on `record_action` Ok assertion (`UnknownAdmission` stub). Evidence: `.planning/phases/20-thin-action-record/20-01-tdd-red.json` (`RED_EVIDENCE_OK`).
- GREEN: `b237a68` — named tracer passes.
- Task 2 tests were confirmation of fail-closed paths already landed in GREEN (not a second RED).

## Issues Encountered

- `cargo test --format tap` is unsupported on this toolchain; RED evidence TAP appendix was added for `tdd-red-evidence` classification without fabricating the failure.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 20-02 can add the later-policy AS OF oracle and persist/open reconstruction.
- Do not register fsm.yaml names until Plan 20-03. Do not clear the S04 lease.

## Self-Check: PASSED

---
*Phase: 20-thin-action-record*
*Completed: 2026-10-01*
