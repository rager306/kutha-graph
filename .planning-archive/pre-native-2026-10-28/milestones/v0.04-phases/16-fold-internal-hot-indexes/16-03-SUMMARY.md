---
phase: 16-fold-internal-hot-indexes
plan: 03
subsystem: harness-governor
tags: [kutha, governor, fsm, HOT-01, HOT-02, HOT-03]

requires:
  - phase: 16-fold-internal-hot-indexes
    provides: three named HOT oracles in m012a_hot_indexes.rs
provides:
  - fsm observe_cargo.required lists HOT-01..03
  - bridges B-m012a-s06 and rust_test_asserts m012a-s06-hot-indexes
  - ADR-040 / ADR-041 evidence append; map stays Proposed
affects:
  - verify-work

actuals:
  tokens: 1101
  tasks: 2
  commits: 1
  plan_head_before: 3f1b22b0033660b56007c0ed042658d22312e44c
  plan_head_after: eb562c8d5a5c7e12a69ebd2a0afaddc0aaef01fa

tech-stack:
  added: []
  patterns:
    - GATE-01 named cargo tests on observe_cargo.required
    - Bridge cites product tests; check id stays off invariants.yaml

key-files:
  created: []
  modified:
    - .kutha/dictionaries/fsm.yaml
    - .kutha/dictionaries/checks.yaml
    - .kutha/dictionaries/bridges.yaml
    - .kutha/dictionaries/honeycomb.yaml
    - CHANGELOG.md

key-decisions:
  - "HOT-01..03 are required observe names so ci fail-closes if they vanish"
  - "Honeycomb map stays Proposed; ADR-061 not claimed as GED-class comparison"
  - "Harness S06 lease is not cleared in this plan"

patterns-established:
  - "Pattern: B-m012a-sNN + rust_test_asserts + observe_cargo.required for a slice"

requirements-completed: [HOT-01, HOT-02, HOT-03]

coverage:
  - id: D1
    description: Three HOT oracles exist and pass before and after governor registration
    requirement: HOT-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#as_of_and_claim_supported_at_skip_non_overlapping_facts
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#discard_csr_lease_and_snapshot_rebuild_keeps_as_of
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#csr_lease_at_builds_via_materializer
        status: pass
    human_judgment: false
  - id: D2
    description: kutha-gov ci is 0 HIGH and observes the three HOT names; honeycomb Proposed
    requirement: HOT-03
    verification:
      - kind: other
        ref: uv run kutha-gov ci
        status: pass
    human_judgment: false

duration: 3min
completed: 2026-10-01
status: complete
---

# Phase 16 Plan 03: Governor registration of HOT-01..03 Summary

**Governor observes the three HOT oracles; honeycomb stays Proposed; S06 lease is not cleared**

## Performance

- **Duration:** 3 min
- **Started:** 2026-09-30T19:33:09Z
- **Completed:** 2026-09-30T19:35:56Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Confirmed `as_of_and_claim_supported_at_skip_non_overlapping_facts`, `discard_csr_lease_and_snapshot_rebuild_keeps_as_of`, and `csr_lease_at_builds_via_materializer` pass.
- Registered those names on `observe_cargo.required`, `m012a-s06-hot-indexes`, and `B-m012a-s06`.
- `uv run kutha-gov ci` exited 0 with 0 HIGH, 0 LOW, 41 checks; HOT names in observe evidence.

## Task Commits

1. **Task 1:** no commit (confirm-only; names already present)
2. **Task 2:** `eb562c8` (chore) register HOT-01..03 governor observe names

## Files Created/Modified

- `.kutha/dictionaries/fsm.yaml` — three HOT required observe names
- `.kutha/dictionaries/checks.yaml` — `m012a-s06-hot-indexes`
- `.kutha/dictionaries/bridges.yaml` — `B-m012a-s06`
- `.kutha/dictionaries/honeycomb.yaml` — ADR-040/041 evidence append; map Proposed
- `CHANGELOG.md` — Process entry for HOT observe names

## Decisions Made

- Check lives on bridges, not invariants.
- LOG/REF/ING/DUR/TIME required names remain.
- Do not edit `.kutha/STATE.md` (S06 stays leased until parent verification).

## Deviations from Plan

None - plan executed as written.

**Total deviations:** 0 auto-fixed
**Impact on plan:** None

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 16 product + governor work is in. Independent `/gsd-verify-work` may still kill-test.
- Do not start M012 or M002. Do not clear S06 here.

## Self-Check: PASSED

- FOUND: .kutha/dictionaries/fsm.yaml
- FOUND: .kutha/dictionaries/checks.yaml
- FOUND: .kutha/dictionaries/bridges.yaml
- FOUND: eb562c8

---
*Phase: 16-fold-internal-hot-indexes*
*Completed: 2026-10-01*
