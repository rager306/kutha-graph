---
phase: 15-verify-persist-and-time-scale
plan: 01
subsystem: runtime-store
tags: [kutha, rust, persist, snapshot, replay_check, DUR-01, DUR-02, DUR-03]

requires:
  - phase: 12-log-native-sot
    provides: persist/open plus log-native sidecars
provides:
  - open snapshot branch calls replay_check and maps ReplayDivergence to InvalidData
  - persist replaces events.jsonl via same-directory temp, sync, rename
  - Event::stable_define name-stable dictionary Define ids
  - Named oracles open_rejects_tampered_snapshot, persist_replaces_events_jsonl_atomically, define_ids_stable_across_persist_open
affects:
  - 15-02
  - 15-03

actuals:
  tokens: 2689
  tasks: 2
  commits: 4
  plan_head_before: 39efff65f488a407b6377b815b64c36021659bc6
  plan_head_after: da17806c0041655372d92eb04e951cab715a10a3

tech-stack:
  added: []
  patterns:
    - snapshot open fail-closed via replay_check before finish_open
    - events.jsonl replace is temp-in-same-dir then rename; WAL protocol untouched
    - encoded_log Define ids from SHA-256(kutha-define-id || term); intern still UUID v7

key-files:
  created:
    - crates/kutha-runtime/tests/m012a_verify_persist.rs
  modified:
    - crates/kutha-runtime/src/store.rs
    - crates/kutha-common/src/event.rs
    - CHANGELOG.md

key-decisions:
  - "open maps ReplayDivergence through runtime_err (InvalidData) and does not return Ok Runtime"
  - "persist jsonl is same-directory temp, sync_all, rename; abort-before-rename is a pub test seam"
  - "Define persist ids are SHA-256 prefix kutha-define-id; intern() still Event::new"

patterns-established:
  - "Pattern: pub abort_events_jsonl_replace_before_rename for DUR-02 crash-before-rename (integration tests cannot see pub(crate))"
  - "Pattern: Event::stable_define for encoded_log only"

requirements-completed: [DUR-01, DUR-02, DUR-03]

coverage:
  - id: D1
    description: Tampered snapshot is rejected on open (InvalidData, ReplayDivergenceError)
    requirement: DUR-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_verify_persist.rs#open_rejects_tampered_snapshot
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m010_semantic_open.rs
        status: pass
    human_judgment: false
  - id: D2
    description: events.jsonl persist is rename-into-place; WAL-deleted open still matches; abort-before-rename leaves dest
    requirement: DUR-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_verify_persist.rs#persist_replaces_events_jsonl_atomically
        status: pass
    human_judgment: false
  - id: D3
    description: Same term set keeps the same Define ids across persist/open/persist
    requirement: DUR-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_verify_persist.rs#define_ids_stable_across_persist_open
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m010_semantic_open.rs#intern_appends_define_for_new_terms_only
        status: pass
    human_judgment: false
  - id: D4
    description: LOG, REF, ING, FF5/FF6, and workspace cargo tests stay green; wal.rs untouched
    requirement: DUR-01
    verification:
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

duration: 6min
completed: 2026-10-01
status: complete
---

# Phase 15 Plan 01: Verify persist and stable Define Summary

**open rejects a tampered snapshot via replay_check; persist replaces events.jsonl by same-directory rename; Define ids are name-stable across persist/open**

## Performance

- **Duration:** 6 min
- **Started:** 2026-09-30T18:53:47Z
- **Completed:** 2026-09-30T19:00:04Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Snapshot `open` runs `replay_check` after `from_snapshot` and maps `ReplayDivergence` to `io::ErrorKind::InvalidData` without returning a Runtime (DUR-01).
- `persist` writes `events.jsonl` via a same-directory temp file, `sync_all`, then `rename`; WAL cousin protocol is unchanged (DUR-02).
- `Event::stable_define` plus `encoded_log` keep dictionary Define ids stable across persist/open/persist; live intern still mints UUID v7 (DUR-03).

## Task Commits

1. **Task 1 RED:** `4256621` (test) add failing verify-on-open and atomic jsonl oracles
2. **Task 1 GREEN:** `04184b2` (feat) verify snapshot on open and atomic events.jsonl persist
3. **Task 2 RED:** `8106d0d` (test) add failing oracle for stable Define ids
4. **Task 2 GREEN:** `da17806` (feat) name-stable Define ids across persist and open

_Note: TDD tasks produced RED then GREEN commits._

## Files Created/Modified

- `crates/kutha-runtime/src/store.rs` — replay_check on snapshot open; atomic jsonl replace; abort-before-rename seam
- `crates/kutha-common/src/event.rs` — `Event::stable_define`
- `crates/kutha-runtime/tests/m012a_verify_persist.rs` — three named DUR oracles
- `CHANGELOG.md` — Product entry for verify-on-open, atomic jsonl, stable Define

## Decisions Made

- `open` maps `ReplayDivergence` through `runtime_err` (`InvalidData`) and does not return `Ok` Runtime.
- persist jsonl is same-directory temp, `sync_all`, rename; abort-before-rename is a pub test seam because integration tests cannot see `pub(crate)`.
- Define persist ids are SHA-256 prefix `kutha-define-id`; `intern()` still `Event::new`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Tamper oracle cannot use `Result::expect_err`**
- **Found during:** Task 1 RED
- **Issue:** `Runtime` does not implement `Debug`, so `expect_err` would not compile (INVALID_RED).
- **Fix:** `match` on `store::open` and `panic!` if `Ok`.
- **Files modified:** `crates/kutha-runtime/tests/m012a_verify_persist.rs`
- **Verification:** RED then GREEN of `open_rejects_tampered_snapshot`
- **Committed in:** `4256621` (Task 1 RED)

---

**Total deviations:** 1 auto-fixed (Rule 3)
**Impact on plan:** Test authoring only. Persist/open contract matches the plan.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 15-02 (YearCe / LogSequence time scale). Do not register governor names until 15-03. Do not clear harness S04.

---
*Phase: 15-verify-persist-and-time-scale*
*Completed: 2026-10-01*

## Self-Check: PASSED
