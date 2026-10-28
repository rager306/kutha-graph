---
phase: 13-stable-references
plan: 02
subsystem: runtime-refs
tags: [kutha, rust, event-id, justification, admission, REF-02]

requires:
  - phase: 13-stable-references
    provides: Fact.event_id and Retract/Correct EventId targeting
provides:
  - JustificationCite source list is EventId-only
  - check_admission matches Fact.event_id at MAX tt and row.vt
  - Named REF-02 oracle justification_cites_source_event_ids_survive_fork
affects:
  - 13-03

actuals:
  tokens: 2868
  tasks: 2
  commits: 1
  plan_head_before: 01849d8975422767e365c8d9bd6f8f105a76a536
  plan_head_after: d4a8e76b2c7ee80855da1c39a06882502535566c

tech-stack:
  added: []
  patterns:
    - admission keys on minting EventId, not fold-local seq
    - fork hydrate restores EventId cites from the log prefix

key-files:
  created: []
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012a_stable_refs.rs
    - crates/kutha-runtime/tests/m012a_log_native.rs
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs
    - CHANGELOG.md

key-decisions:
  - "Dropped source_fact_seqs from the durable cite Op so a leaked payload cannot be replayed against a renumbered fold"
  - "check_admission still uses tt=MAX and row.vt (FIX-02); unknown EventId is stale_support"

patterns-established:
  - "Pattern: record_justification(target_claim, source_claim_ids, source_event_ids, rule_version, tt, vt)"

requirements-completed: [REF-02]

coverage:
  - id: D1
    description: e2e stale-renew after CorrectInterval/Retract still keys on source_event_ids
    requirement: REF-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_justification_cites_sources_and_rejects_stale_admission
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_log_native.rs#discard_justifications_sidecar_keeps_admission_and_resume
        status: pass
    human_judgment: false
  - id: D2
    description: Fork prefix still admits cited EventIds after the parent retracts
    requirement: REF-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_stable_refs.rs#justification_cites_source_event_ids_survive_fork
        status: pass
    human_judgment: false

duration: 4min
completed: 2026-10-01
status: complete
---

# Phase 13 Plan 02: EventId justification cites Summary

**Justification cites and check_admission key on minting EventId; a fork prefix still admits after the parent retracts that support**

## Performance

- **Duration:** 4 min
- **Started:** 2026-09-30T18:07:00Z
- **Completed:** 2026-09-30T18:10:45Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Removed the seq vector from `Op::JustificationCite`, `Justification`, digest, hydrate, and `record_justification`.
- Admission matches live `Fact.event_id` at `(tt=MAX, row.vt)`.
- Named REF-02 fork oracle is green; LOG-02/LOG-03 stay green.

## Task Commits

1. **Task 1+2: EventId cites, admission, and fork oracle** - `d4a8e76` (feat)

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - cite Op is EventId-only
- `crates/kutha-runtime/src/quantum.rs` - admission by Fact.event_id
- `crates/kutha-runtime/tests/m012a_stable_refs.rs` - REF-02 fork oracle
- `crates/kutha-runtime/tests/m012a_log_native.rs` / `m011_e2e_fixture.rs` - call sites
- `CHANGELOG.md` - Product plane

## Decisions Made

- Unknown cited EventId is `stale_support` (do not match `claim_id`).
- ConflictReport still lists fold seqs (ING-03 / Phase 14).

## Deviations from Plan

None - plan executed as written. Both tasks landed in one feat commit because the fork oracle requires the new `record_justification` signature (docs-coupling also wants CHANGELOG with the crate diff).

## TDD Gate Compliance

Plan type `execute`; `workflow.tdd_mode` false. Isolated RED on the signature change is a compile failure. Test + implementation landed together.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- REF-02 holds. Plan 13-03 can rebuild a fold with renumbered seqs and register governor observe names.
- Do not clear S02.

## Self-Check: PASSED

- `justification_cites_source_event_ids_survive_fork` FOUND
- Commit `d4a8e76` FOUND
- e2e stale-admission, LOG-02, LOG-03, workspace cargo tests green

---
*Phase: 13-stable-references*
*Completed: 2026-10-01*
