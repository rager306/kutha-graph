---
phase: 13-stable-references
plan: 03
subsystem: runtime-refs
tags: [kutha, rust, event-id, rebuild, governor, REF-03]

requires:
  - phase: 13-stable-references
    provides: EventId retract/correct and EventId justification cites
provides:
  - Named REF-03 rebuild oracle
  - fsm/checks/bridges observation of REF-01..03
  - honeycomb ADR-011/ADR-061 evidence names (map stays Proposed)
affects:
  - 14-idempotent-ingest

actuals:
  tokens: 2566
  tasks: 2
  commits: 1
  plan_head_before: 5585bf50af84fb4755193de482e71326c15667bc
  plan_head_after: d2d98122e861da20257e53e35e420f5e4472b1f4

tech-stack:
  added: []
  patterns:
    - from_dict_and_events replay binds retract/cite by EventId after seq renumber
    - governor bridge cites product tests; does not copy L_capability

key-files:
  created: []
  modified:
    - crates/kutha-runtime/tests/m012a_stable_refs.rs
    - .kutha/dictionaries/fsm.yaml
    - .kutha/dictionaries/checks.yaml
    - .kutha/dictionaries/bridges.yaml
    - .kutha/dictionaries/honeycomb.yaml
    - CHANGELOG.md

key-decisions:
  - "REF-03 rebuild drops the first Assert Event only; Defines and QuantumOutcome stay"
  - "Governor check lives on bridges.yaml, not invariants.yaml"
  - "ADR-061 stays capability none with a non-empty evidence array"

patterns-established:
  - "Pattern: B-m012a-s02 / m012a-s02-stable-refs rust_test_asserts over three named tests"

requirements-completed: [REF-03]

coverage:
  - id: D1
    description: Rebuilt fold with renumbered seqs still applies the same Retract and JustificationCite Events
    requirement: REF-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_stable_refs.rs#rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads
        status: pass
    human_judgment: false
  - id: D2
    description: Named REF oracles are required observe names; ci HIGH 0
    requirement: REF-03
    verification:
      - kind: other
        ref: uv run kutha-gov ci
        status: pass
      - kind: other
        ref: uv run kutha-gov precommit --check observe-required-fn
        status: pass
    human_judgment: false

duration: 5min
completed: 2026-10-01
status: complete
---

# Phase 13 Plan 03: Rebuild oracle and governor REF names Summary

**A rebuilt fold that renumbers local seqs still applies the same Retract and JustificationCite Events; governor observes REF-01..03 with ci HIGH 0**

## Performance

- **Duration:** 5 min
- **Started:** 2026-09-30T18:11:00Z
- **Completed:** 2026-09-30T18:16:00Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Named REF-03 oracle proves target `Fact.seq` 1 → 0 after dropping the filler Assert, with the same EventId invalidated.
- Cite-only rebuild (no Retract) still `check_admission` Ok.
- `B-m012a-s02` / `m012a-s02-stable-refs` plus three `observe_cargo.required` names. LOG names remain. Honeycomb `map: Proposed`.

## Task Commits

1. **Task 1+2: rebuild oracle and governor registration** - `d2d9812` (feat)

## Files Created/Modified

- `crates/kutha-runtime/tests/m012a_stable_refs.rs` - REF-03 oracle
- `.kutha/dictionaries/fsm.yaml` - three REF observe names
- `.kutha/dictionaries/checks.yaml` - `m012a-s02-stable-refs`
- `.kutha/dictionaries/bridges.yaml` - `B-m012a-s02`
- `.kutha/dictionaries/honeycomb.yaml` - ADR-011 / ADR-061 evidence
- `CHANGELOG.md` - Product + Process

## Decisions Made

- Did not put the check id on `invariants.yaml` (one ledger).
- Did not edit `.kutha/STATE.md` (S02 stays leased).

## Deviations from Plan

None - plan executed as written. Both tasks landed in one feat commit (docs-coupling: dictionaries + crates + CHANGELOG).

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- REF-01..03 hold. Do not clear S02 until the parent closes the slice after verification.
- Phase 14 (idempotent ingest) is next; do not start M012/M002.

## Self-Check: PASSED

- `rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads` FOUND
- Commit `d2d9812` FOUND
- `uv run kutha-gov ci` 0 HIGH, 0 LOW, 37 checks; three REF observe names ok

---
*Phase: 13-stable-references*
*Completed: 2026-10-01*
