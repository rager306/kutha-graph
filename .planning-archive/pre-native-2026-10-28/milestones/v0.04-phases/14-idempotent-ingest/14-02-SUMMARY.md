---
phase: 14-idempotent-ingest
plan: 02
subsystem: runtime-ingest
tags: [kutha, rust, claim-id, polarity, conflict-report, ING-02, ING-03]

requires:
  - phase: 14-idempotent-ingest
    provides: Assert delivery_key and polarity fields; keyed retry gate
provides:
  - Named ING-02 oracle claim_id_distinct_from_support_slot
  - conflict_report_at(claim, tt, vt) partitions by Fact.polarity
  - e2e fixture Asserts store Positive; CorrectInterval flips object-changing rows
affects:
  - 14-03

actuals:
  tokens: 3977
  tasks: 2
  commits: 2
  plan_head_before: 35cb0e94aa334aa77be11b75e45b09e67fce8ade
  plan_head_after: b6f6fd89f01fb5126008dd463329eb23f1411d41

tech-stack:
  added: []
  patterns:
    - claim_id is proposition identity; EventId is the support slot; delivery_key is retry only
    - conflict buckets come from stored polarity; None is unbucketed

key-files:
  created: []
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012a_idempotent_ingest.rs
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs
    - CHANGELOG.md

key-decisions:
  - "Residuals copy polarity; object-changing Correct/CorrectInterval rows flip when old polarity is Some"
  - "e2e t1 Asserts store Positive so t2 interior not-P is Negative without an opposite-of dictionary"

patterns-established:
  - "Pattern: conflict_report_at(claim, tt, vt) — three arguments, stored polarity"
  - "Pattern: None polarity is in neither positive nor negative bucket"

requirements-completed: [ING-02, ING-03]

coverage:
  - id: D1
    description: Two independent keys share claim_id with distinct EventIds; delivery_key is not the claim UUID
    requirement: ING-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_idempotent_ingest.rs#claim_id_distinct_from_support_slot
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_claim_supports.rs#retracting_one_support_leaves_claim_supported
        status: pass
    human_judgment: false
  - id: D2
    description: conflict_report_at partitions by stored polarity; e2e named cuts still hold without caller TermIds
    requirement: ING-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_idempotent_ingest.rs#conflict_report_at_partitions_by_stored_polarity
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_fixture_supports_and_conflict_at_named_cuts
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_incremental_matches_reconstruct_after_discarding_leases
        status: pass
    human_judgment: false

duration: 4min
completed: 2026-10-01
status: complete
---

# Phase 14 Plan 02: Claim vs support and stored polarity Summary

**Claim identity stays distinct from the support EventId; e2e conflict reports partition by stored Fact.polarity without caller TermIds**

## Performance

- **Duration:** 4 min
- **Started:** 2026-09-30T18:35:38Z
- **Completed:** 2026-09-30T18:38:17Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Named ING-02 oracle `claim_id_distinct_from_support_slot` plus rustdoc on `Op::Assert` and `Fact`.
- `conflict_report_at(claim, tt, vt)` buckets Positive vs Negative from stored polarity; `None` is unbucketed.
- Leased e2e fixture t1/t2/t3 cuts still hold. LOG and REF oracles stay green.

## Task Commits

1. **Task 1: Named test and rustdoc: claim is not the support slot** - `f58e796` (test)
2. **Task 2: Conflict report uses stored polarity on the e2e fixture path** - `b6f6fd8` (feat)

**Plan metadata:** pending docs commit after STATE/ROADMAP update.

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - Assert rustdoc names three identities
- `crates/kutha-runtime/src/fold.rs` - Fact rustdoc; Correct/CorrectInterval copy/flip polarity
- `crates/kutha-runtime/src/quantum.rs` - three-arg `conflict_report_at`
- `crates/kutha-runtime/tests/m012a_idempotent_ingest.rs` - ING-02 and ING-03 oracles
- `crates/kutha-runtime/tests/m011_e2e_fixture.rs` - Positive polarity; no TermId args
- `CHANGELOG.md` - Product plane (docs-coupling)

## Decisions Made

- Flip polarity only when the replacement row object differs and old polarity is `Some` — residuals of the e2e patch keep P as Positive.
- No opposite-of dictionary (D-04).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] CHANGELOG in the ING-02 rustdoc commit**
- **Found during:** Task 1 (docs-coupling)
- **Issue:** Plan listed CHANGELOG on Task 2; crate rustdoc is still a product-plane diff.
- **Fix:** Wrote the Product identity entry in Task 1; Task 2 added the polarity Changed bullet.
- **Files modified:** `CHANGELOG.md`
- **Verification:** `uv run kutha-gov precommit --check docs-coupling`
- **Committed in:** `f58e796` (Task 1)

---

**Total deviations:** 1 auto-fixed (docs-coupling)
**Impact on plan:** Hook policy only. No scope creep.

## Issues Encountered

- CBM coverage on edited paths is `metadata_changed` / new test `not_tracked`. Did not reindex. Source + cargo verified.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ING-01..03 named tests exist in one file. Plan 14-03 registers them with the governor. Do not clear S03.

## Self-Check: PASSED

---
*Phase: 14-idempotent-ingest*
*Completed: 2026-10-01*
