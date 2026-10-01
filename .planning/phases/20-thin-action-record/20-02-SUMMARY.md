---
phase: 20-thin-action-record
plan: 02
subsystem: runtime
tags: [action-record, as-of, persist, ACT-01, ACT-02]

requires:
  - phase: 20-thin-action-record
    provides: Op::RecordAction, record_action, action_record_at, UnknownAdmission
provides:
  - "Named ACT-02 oracle: Action + admission AS OF prior cut after later PinPolicy"
  - "persist/open reconstructs Action bind from log facts"
  - "Product changelog for the thin Action record"
affects: [20-03, persist, action_record]

actuals:
  tokens: 1542
  tasks: 2
  commits: 2
  plan_head_before: 2c2630e7b9bae9526bfc37fc3ec197f3811249ce
  plan_head_after: 31bdaeaf0ccbc555e98c56d3d879f5445d1d5482

tech-stack:
  added: []
  patterns:
    - "Later PinPolicy updates live_policy_pin_at at the tip; Action.policy_version stays the stored first hash"
    - "hydrate_from_log rebuild_action_entries so snapshot JSON without action-entries still answers AS OF"

key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012_action_record.rs
    - CHANGELOG.md

key-decisions:
  - "A later pin must not rewrite Action.policy_version or prior-cut admission"
  - "Runtime::live_policy_pin_at delegates to the fold for the ACT-02 tip check"

patterns-established:
  - "Action bind is a log fact: persist/open reconstructs it; public emit stays MetaOpRejected after open"

requirements-completed: [ACT-01, ACT-02]

coverage:
  - id: D1
    description: "After later-policy, AS OF the prior cut still returns the original Action bind and admission Some(true)"
    requirement: ACT-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_action_record.rs#action_and_admission_as_of_prior_cut_after_policy_change
        status: pass
    human_judgment: false
  - id: D2
    description: "persist then open reconstructs Action AS OF and admission; opened emit RecordAction is MetaOpRejected"
    requirement: ACT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_action_record.rs#persist_open_reconstructs_action_record
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 20 Plan 02: Action AS OF after later policy Summary

**Later PinPolicy changes only the live pin at the tip; Action + admission at the prior cut stay bound to the first policy hash, and persist/open reconstructs the bind**

## Performance

- **Duration:** 2 min
- **Started:** 2026-10-01T03:35:29Z
- **Completed:** 2026-10-01T03:37:38Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Named ACT-02 oracle: after `later-policy`, `action_record_at` and `admission_status_at` at `prior_tt` stay the first pin hash and `Some(true)`.
- Tip `live_policy_pin_at` is the later hash; Action.policy_version stays the first hash at the tip as well.
- persist/open reconstructs Action + admission from the log; opened public emit stays `MetaOpRejected`.
- Dated Product changelog; `0.0.0` unchanged.

## Task Commits

1. **Task 1: Action plus admission AS OF prior cut after later policy** - `66caa8c` (test)
2. **Task 2: persist/open reconstructs Action; changelog** - `31bdaea` (feat)

## Files Created/Modified

- `crates/kutha-runtime/src/quantum.rs` - `Runtime::live_policy_pin_at` delegate
- `crates/kutha-runtime/tests/m012_action_record.rs` - ACT-02 and persist/open oracles
- `CHANGELOG.md` - Product entry for the thin Action record

## Decisions Made

- Later pin is additive (latest ingested live pin wins); it does not rewrite stored Action fields.
- UnknownFact Retract gate already accepted action-entry EventIds from 20-01; no further fold change required for persist.

## Deviations from Plan

None - plan executed exactly as written. ACT-02 was already true of the 20-01 liveness model; the named test confirmed it (TDD GREEN without a new RED).

## TDD Gate Compliance

- Task 1 `tdd="true"`: the named test passed on first run because later PinPolicy already left Action entries untouched. Not a false RED.
- Task 2 persist oracle also passed on first run via existing `rebuild_action_entries`.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 20-03 can register the two ACT observe names. Do not clear the S04 lease.

## Self-Check: PASSED

---
*Phase: 20-thin-action-record*
*Completed: 2026-10-01*
