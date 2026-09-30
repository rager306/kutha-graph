---
phase: 15-verify-persist-and-time-scale
plan: 02
subsystem: runtime-time
tags: [kutha, rust, ValidTime, TransactionTime, YearCe, TIME-01, TIME-02]

requires:
  - phase: 15-verify-persist-and-time-scale
    provides: Event::stable_define; DUR oracles in m012a_verify_persist.rs
provides:
  - TimeScale plus VALID_TIME_SCALE YearCe and TRANSACTION_TIME_SCALE LogSequence
  - Named oracles fixture_years_map_to_year_ce_scale and transaction_time_is_log_sequence
affects:
  - 15-03

actuals:
  tokens: 1692
  tasks: 2
  commits: 3
  plan_head_before: f247f2347a4d4ecf85ccdc3463dd42b722b0b619
  plan_head_after: 60f046da0d1938e7321239b0bdd988525ec1f603

tech-stack:
  added: []
  patterns:
    - ValidTime and TransactionTime stay u64 aliases; scale is a documented enum plus constants
    - no chrono and no TT-to-wall table

key-files:
  created:
    - crates/kutha-runtime/tests/m012a_time_scale.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-common/src/lib.rs
    - crates/kutha-runtime/tests/ff5_legal_pit.rs
    - CHANGELOG.md

key-decisions:
  - "ValidTime stays a u64 alias; fixtures treat 2017 as YearCe, not Unix epoch"
  - "TransactionTime is log sequence; TT-to-wall mapping is out of M012a S05"

patterns-established:
  - "Pattern: TimeScale constants plus named cargo oracles; do not newtype ValidTime"

requirements-completed: [TIME-01, TIME-02]

coverage:
  - id: D1
    description: VALID_TIME_SCALE is YearCe; fixture 2017 as_of contains the triple and 2016 does not
    requirement: TIME-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_time_scale.rs#fixture_years_map_to_year_ce_scale
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/ff5_legal_pit.rs
        status: pass
    human_judgment: false
  - id: D2
    description: TRANSACTION_TIME_SCALE is LogSequence; ingested_at is a small sequence not Unix epoch
    requirement: TIME-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_time_scale.rs#transaction_time_is_log_sequence
        status: pass
    human_judgment: false
  - id: D3
    description: DUR, LOG, REF, ING, and workspace cargo tests stay green
    requirement: TIME-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_verify_persist.rs
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_log_native.rs
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_stable_refs.rs
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_idempotent_ingest.rs
        status: pass
      - kind: other
        ref: cargo test --workspace --offline
        status: pass
    human_judgment: false

duration: 1min
completed: 2026-10-01
status: complete
---

# Phase 15 Plan 02: YearCe and log-sequence time scale Summary

**Fixtures treat 2015/2017/2021 as Gregorian YearCe valid-time; ingested_at is log sequence, not wall-clock**

## Performance

- **Duration:** 1 min
- **Started:** 2026-09-30T19:01:38Z
- **Completed:** 2026-09-30T19:02:42Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- `TimeScale::{YearCe, LogSequence}` with `VALID_TIME_SCALE` / `TRANSACTION_TIME_SCALE`; `ValidTime` and `TransactionTime` stay `u64` aliases (TIME-01).
- Named tests document fixture years and that `ingested_at` is a small log sequence. No TT-to-wall calendar (TIME-02).
- FF5 `T_OLD`/`T_NEW` remain 2015/2021 with YearCe comments.

## Task Commits

1. **Task 1 RED:** `a7b3798` (test) failing YearCe fixture oracle (TimeScale unresolved)
2. **Task 1 GREEN:** `2ea3eb2` (feat) declare YearCe valid-time scale for fixtures
3. **Task 2:** `60f046d` (test) document transaction-time as log sequence plus Product changelog

## Files Created/Modified

- `crates/kutha-common/src/event.rs` — TimeScale, scale constants, rustdoc
- `crates/kutha-common/src/lib.rs` — re-exports
- `crates/kutha-runtime/tests/m012a_time_scale.rs` — two named TIME oracles
- `crates/kutha-runtime/tests/ff5_legal_pit.rs` — YearCe comments on T_OLD/T_NEW
- `CHANGELOG.md` — Product entry for declared VT/TT scale

## Decisions Made

- ValidTime stays a u64 alias; fixtures treat 2017 as YearCe, not Unix epoch.
- TransactionTime is log sequence; TT-to-wall mapping is out of M012a S05.

## Deviations from Plan

None - plan executed as written. TIME-02's named test was green on first run because `TRANSACTION_TIME_SCALE` already landed in the tracer task; the task added the oracle and changelog only.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 15-03 governor registration. Do not clear harness S04.

---
*Phase: 15-verify-persist-and-time-scale*
*Completed: 2026-10-01*

## Self-Check: PASSED
