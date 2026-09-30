---
phase: 14-idempotent-ingest
plan: 01
subsystem: runtime-ingest
tags: [kutha, rust, delivery-key, idempotent-ingest, ING-01]

requires:
  - phase: 13-stable-references
    provides: Fact.event_id; Phase 12 LOG and Phase 13 REF oracles
provides:
  - Op::Assert.delivery_key and SupportPolarity (default None)
  - emit keyed identical retry returns original EventId without append
  - DeliveryKeyConflict fail-closed on key reuse with different payload
  - Named ING-01 oracle identical_assert_same_delivery_key_does_not_mint_second_support
affects:
  - 14-02
  - 14-03

actuals:
  tokens: 8547
  tasks: 2
  commits: 2
  plan_head_before: 5fc31d9e854f9c3e351b9cd70f10f9d5e3a44e70
  plan_head_after: 3a2f45d804dbec6598a83134e641936e72a26726

tech-stack:
  added: []
  patterns:
    - non-empty delivery_key is unique on the log; None/empty always mints
    - retry receipt names the original EventId; events_in_quantum 0; no PersistedQuantumOutcome row

key-files:
  created:
    - crates/kutha-runtime/tests/m012a_idempotent_ingest.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-common/src/lib.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/tenant.rs
    - crates/kutha-runtime/src/leapfrog.rs
    - crates/kutha-runtime/src/materializer.rs
    - crates/kutha-runtime/tests/m011_claim_supports.rs
    - crates/kutha-runtime/tests/m012a_log_native.rs
    - crates/kutha-runtime/tests/m012a_stable_refs.rs
    - CHANGELOG.md

key-decisions:
  - "Identity for the delivery-key gate is subject, relation, object, valid_from, valid_to, claim, polarity, and delivery_key"
  - "Retracted original still returns that EventId; do not mint a replacement support"
  - "SupportPolarity lands with default None so Plan 14-02 does not reshape Assert again"

patterns-established:
  - "Pattern: scan log Op::Assert for a non-empty delivery_key before Event::new"
  - "Pattern: serde default None on delivery_key and polarity so old WAL/JSONL rows stay readable"

requirements-completed: [ING-01]

coverage:
  - id: D1
    description: Identical keyed Assert retry does not mint a second support; unkeyed equal triples still mint
    requirement: ING-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_idempotent_ingest.rs#identical_assert_same_delivery_key_does_not_mint_second_support
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_claim_supports.rs#retracting_one_support_leaves_claim_supported
        status: pass
    human_judgment: false
  - id: D2
    description: Key reuse with a different payload fails closed; persist/open still binds the key
    requirement: ING-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_idempotent_ingest.rs#delivery_key_payload_mismatch_does_not_append
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_idempotent_ingest.rs#keyed_assert_retry_after_persist_open_does_not_mint
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_log_native.rs
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_stable_refs.rs
        status: pass
    human_judgment: false

duration: 8min
completed: 2026-10-01
status: complete
---

# Phase 14 Plan 01: Delivery-key Assert ingest Summary

**Identical Assert retry under a non-empty delivery key returns the original EventId and does not append; mismatch fails closed as DeliveryKeyConflict**

## Performance

- **Duration:** 8 min
- **Started:** 2026-09-30T18:26:26Z
- **Completed:** 2026-09-30T18:34:00Z
- **Tasks:** 2
- **Files modified:** 20

## Accomplishments

- Named ING-01 oracle `identical_assert_same_delivery_key_does_not_mint_second_support`: keyed retry does not grow the log; `live_support_count` stays 1; unkeyed equal triples still mint a second support.
- `RuntimeError::DeliveryKeyConflict` fail-closes when the same key carries a different Assert payload.
- Persist/open still honors the durable key. FF5/FF6, LOG, and REF oracles stay green.

## Task Commits

1. **Task 1: End-to-end identical Assert retry under one delivery key** - `f65b277` (feat)
2. **Task 2: Keyed payload mismatch fails closed; persist/open still binds the key** - `3a2f45d` (test)

**Plan metadata:** pending docs commit after STATE/ROADMAP update.

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - `SupportPolarity`; Assert `delivery_key`/`polarity`; digest mix
- `crates/kutha-common/src/lib.rs` - export `SupportPolarity`
- `crates/kutha-runtime/src/fold.rs` - Fact copies key/polarity; fingerprint mix
- `crates/kutha-runtime/src/quantum.rs` - emit gate before `Event::new`; `DeliveryKeyConflict`
- `crates/kutha-runtime/src/{tenant,leapfrog,materializer}.rs` - Assert literals compile
- `crates/kutha-runtime/tests/m012a_idempotent_ingest.rs` - ING-01 + mismatch + persist/open oracles
- existing `Op::Assert` call sites in tests — `delivery_key: None`, `polarity: None`
- `CHANGELOG.md` - Product plane entry (docs-coupling)

## Decisions Made

- Gate identity includes claim and polarity so a later polarity change under the same key cannot silently alias.
- Retry does not bump `next_tt` and does not push `PersistedQuantumOutcome`.
- Isolated RED was not split: new Assert fields make a tests-only commit a compile failure. `workflow.tdd_mode` is false; plan type is `execute`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] CHANGELOG in the tracer commit**
- **Found during:** Task 1 (docs-coupling pre-commit)
- **Issue:** Plan listed CHANGELOG on Task 2; `git_path_implies` requires CHANGELOG with `crates/**/*.rs`.
- **Fix:** Wrote the Product delivery-key entry in Task 1; Task 2 extended it with mismatch/persist.
- **Files modified:** `CHANGELOG.md`
- **Verification:** `uv run kutha-gov precommit --check docs-coupling` and `changelog-planes`
- **Committed in:** `f65b277` (Task 1)

**2. [Rule 1 - Bug] Persist/open log-length comparison**
- **Found during:** Task 2 (`keyed_assert_retry_after_persist_open_does_not_mint`)
- **Issue:** `open` with snapshot drops live `Define` events from the in-memory log (`encoded_log` synthesizes them at persist). Comparing pre-persist `log.len()` to post-open `log.len()` fails even when retry does not append.
- **Fix:** Capture `n_open` after intern on the opened runtime; assert retry does not grow that length. `live_support_count` and original EventId stay the oracles.
- **Files modified:** `crates/kutha-runtime/tests/m012a_idempotent_ingest.rs`
- **Verification:** named ingest file + workspace cargo tests
- **Committed in:** `3a2f45d` (Task 2)

---

**Total deviations:** 2 auto-fixed (1 blocking hook, 1 test bug)
**Impact on plan:** Hook coupling is existing governor policy. Persist/open oracle still proves the durable key. No scope creep.

## Issues Encountered

- CBM `check_index_coverage` on touched crate paths is `metadata_changed` / new test `not_tracked`. Did not `index_repository` (project present). Source + cargo are the verify plane.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ING-01 keyed ingest holds. Plan 14-02 can document claim vs support and switch `conflict_report_at` to stored polarity.
- Do not clear S03. Do not register fsm.yaml names until Plan 14-03.

## Self-Check: PASSED

---
*Phase: 14-idempotent-ingest*
*Completed: 2026-10-01*
