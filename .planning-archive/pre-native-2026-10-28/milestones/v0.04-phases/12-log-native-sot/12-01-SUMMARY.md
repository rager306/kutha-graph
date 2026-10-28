---
phase: 12-log-native-sot
plan: 01
subsystem: runtime-log
tags: [kutha, rust, event-log, quantum-outcome, LOG-01]

requires:
  - phase: 11
    provides: M011 quantum outcomes sidecar + crash oracle
provides:
  - Op::QuantumOutcome fold-noop log records
  - store::open hydrates outcomes from the log; sidecar is a lease
  - Named LOG-01 discard-sidecar oracle
affects:
  - 12-02
  - 12-03

actuals:
  tokens: 5703
  tasks: 3
  commits: 3

plan_head_before: a01edf7652f2511893c862c110013678c9a909fd
plan_head_after: 99688eec30999f8682b0cfb3aa835b3da74b01da

tech-stack:
  added: []
  patterns:
    - fold-noop meta Op on the event log with sidecar lease
    - graph_len / snapshot log_offset count fold-affecting ops only

key-files:
  created:
    - crates/kutha-runtime/tests/m012a_log_native.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-common/src/lib.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/store.rs
    - crates/kutha-runtime/tests/m011_quantum_outcome.rs
    - CHANGELOG.md

key-decisions:
  - "OutcomeDisposition lives in kutha-common so Op::QuantumOutcome can carry it without a runtime cycle"
  - "Public emit of Op::QuantumOutcome returns RuntimeError::MetaOpRejected (T-12-02)"
  - "Resume persist/open after sidecar discard stays Plan 12-02; 12-01 crash oracle checks Resume on the live Runtime"

patterns-established:
  - "Pattern: append_meta after RAM push; hydrate_from_log wins over an empty sidecar"
  - "Pattern: op_affects_fold is false for Define and QuantumOutcome"

requirements-completed: [LOG-01]

coverage:
  - id: D1
    description: Discarding quantum_outcomes.jsonl after persist reconstructs Zero/Partial/Full from log Events
    requirement: LOG-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_log_native.rs#discard_outcomes_sidecar_keeps_reconstructible_disposition
        status: pass
    human_judgment: false
  - id: D2
    description: graph_len and snapshot offset stay fold-fact counts while QuantumOutcome remains on the log
    requirement: LOG-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/src/quantum.rs#cascade_idles_without_inverse_loop
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/ff6_allowlist.rs#ff6_allowlisted_relation_still_appends
        status: pass
    human_judgment: false
  - id: D3
    description: Partial prefix cannot be read as Full after sidecar discard; open does not mint Resume
    requirement: LOG-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_quantum_outcome.rs#crash_after_prefix_has_no_terminal_success_until_explicit_resume
        status: pass
    human_judgment: false

duration: 8min
completed: 2026-10-01
status: complete
---

# Phase 12 Plan 01: Log-native quantum outcomes Summary

**Op::QuantumOutcome fold-noop log records; discarding quantum_outcomes.jsonl still reconstructs Zero/Partial/Full**

## Performance

- **Duration:** 8 min
- **Started:** 2026-09-30T17:16:48Z
- **Completed:** 2026-09-30T17:24:00Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Quantum outcomes append as `Op::QuantumOutcome` after the emit cascade; they are not cascade steps (`events_in_quantum` stays 2 for knows+inverse).
- `store::open` hydrates from log Events; an empty or missing outcomes sidecar cannot wipe hydrated rows.
- `graph_len` / `Snapshot.log_offset` count fold-affecting ops only; FF6 `graph_len == 1` holds.
- Crash oracle reconstructs Partial from the log and still refuses invented Full/Resume.

## TDD Gate Compliance

- Task 1 RED: `discard_outcomes_sidecar_keeps_reconstructible_disposition` failed on empty `outcome_records` after sidecar delete (`gsd_run check tdd-red-evidence` → `RED_EVIDENCE_OK`).
- GREEN: emit-after-append + hydrate + log-wins open. Workspace `cargo test --offline` green.

## Task Commits

1. **Task 1: persist quantum outcomes as log records** - `053af1d` (feat)
2. **Task 2: keep graph_len and snapshot offset on fold facts** - `8e1ee81` (fix)
3. **Task 3: reconstruct Partial from log after sidecar discard** - `99688ee` (test)

## Files Created/Modified

- `crates/kutha-common/src/event.rs` — `OutcomeDisposition` + `Op::QuantumOutcome` + digest tag `quantum-outcome`
- `crates/kutha-runtime/src/quantum.rs` — append-after-emit, hydrate_from_log, MetaOpRejected, op_affects_fold
- `crates/kutha-runtime/src/store.rs` — `finish_open` log-wins for outcomes
- `crates/kutha-runtime/src/fold.rs` — no-op apply arm
- `crates/kutha-runtime/tests/m012a_log_native.rs` — LOG-01 oracle
- `crates/kutha-runtime/tests/m011_quantum_outcome.rs` — Partial-from-log crash oracle
- `CHANGELOG.md` — 2026-10-01 Product entry

## Decisions Made

- Moved `OutcomeDisposition` into `kutha-common` so the event schema owns the enum.
- Reject public `emit(Op::QuantumOutcome)` with `RuntimeError::MetaOpRejected`.
- Resume remains RAM+lease until Plan 12-02; crash oracle checks Resume on the live Runtime after `record_resume`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Resume persist/open would drop sidecar-only Resume under log-wins**
- **Found during:** Task 3 (crash oracle)
- **Issue:** After log-native Partial Events exist, `open` ignores the outcomes sidecar. `record_resume` still only pushes RAM, so persist/open after resume lost Resume.
- **Fix:** Assert Resume and duplicate-resume on the live `opened` Runtime. Log-native resume is Plan 12-02.
- **Files modified:** `crates/kutha-runtime/tests/m011_quantum_outcome.rs`
- **Verification:** `cargo test -p kutha-runtime --offline --test m011_quantum_outcome -- --exact`
- **Committed in:** `99688ee`

---

**Total deviations:** 1 auto-fixed (Rule 1)
**Impact on plan:** Required for log-wins correctness without pulling LOG-02 into this wave.

## Issues Encountered

CBM `check_index_coverage` on edited paths reports `metadata_changed` / `not_tracked` for the new test file. Did not run `index_repository` (parent/integrator only).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- LOG-01 holds. Plan 12-02 can append `Op::JustificationCite` and log-native Resume on the same private `append_meta` path.
- Do not run `kutha-gov ci` until 12-03 registers observe names.
- Do not clear `.kutha/STATE.md` S01 lease.

## Self-Check: PASSED

- `crates/kutha-runtime/tests/m012a_log_native.rs` FOUND
- Commits `053af1d`, `8e1ee81`, `99688ee` FOUND
- Named LOG-01 test green; workspace cargo tests green including FF5/FF6

---
*Phase: 12-log-native-sot*
*Completed: 2026-10-01*
