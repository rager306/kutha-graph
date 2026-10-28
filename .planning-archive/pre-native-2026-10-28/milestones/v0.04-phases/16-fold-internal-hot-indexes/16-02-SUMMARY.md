---
phase: 16-fold-internal-hot-indexes
plan: 02
subsystem: runtime-csr
tags: [kutha, rust, csr, materializer, HOT-02, HOT-03]

requires:
  - phase: 16-fold-internal-hot-indexes
    provides: fold-internal live_facts_at and HOT-01 oracles
provides:
  - CsrLease / TypedCsrLease from_fold consume the indexed live iterator
  - csr_lease_at builds through CsrMaterializer then unloads
  - Named HOT-02 / HOT-03 oracles
affects:
  - 16-03

actuals:
  tokens: 2099
  tasks: 2
  commits: 2
  plan_head_before: b78cb8d02b648081ead48755207dee964a48a62f
  plan_head_after: adfbc09dd8a384304bfb57894ec5c2b4a10320be

tech-stack:
  added: []
  patterns:
    - from_fold iterates GraphFold::live_facts_at
    - untyped hot CSR is Materializer build + clone + unload
    - typed CSR stays from_fold as a named spike limit

key-files:
  created: []
  modified:
    - crates/kutha-runtime/src/csr.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012a_hot_indexes.rs
    - CHANGELOG.md

key-decisions:
  - "CSR and hot maps remain droppable leases; persist/open reconstructs as_of from log and fold facts"
  - "csr_lease_at does not store CsrMaterializer on Runtime"
  - "typed_csr_lease_at stays TypedCsrLease::from_fold (HOT-03 spike limit)"

patterns-established:
  - "Pattern: untyped csr_lease_at = CsrMaterializer::build then unload"
  - "Pattern: rustdoc plus equality vs from_fold names the typed spike limit"

requirements-completed: [HOT-02, HOT-03]

coverage:
  - id: D1
    description: Dropping CSR leases leaves as_of, fingerprint, and claim_supported_at unchanged; persist/open reconstructs without a CSR sidecar
    requirement: HOT-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#discard_csr_lease_and_snapshot_rebuild_keeps_as_of
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_e2e_fixture.rs#e2e_incremental_matches_reconstruct_after_discarding_leases
        status: pass
    human_judgment: false
  - id: D2
    description: csr_lease_at neighbors match CsrMaterializer::build and CsrLease::from_fold; typed path remains from_fold
    requirement: HOT-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012a_hot_indexes.rs#csr_lease_at_builds_via_materializer
        status: pass
    human_judgment: false

duration: 2min
completed: 2026-10-01
status: complete
---

# Phase 16 Plan 02: Droppable CSR and Materializer-wired csr_lease_at Summary

**Untyped CSR lease is built through CsrMaterializer and dropped; from_fold reads the fold live iterator; typed from_fold stays the named spike limit**

## Performance

- **Duration:** 2 min
- **Started:** 2026-09-30T19:30:45Z
- **Completed:** 2026-09-30T19:32:18Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- `CsrLease::from_fold` and `TypedCsrLease::from_fold` iterate `live_facts_at` instead of every fact.
- `Runtime::csr_lease_at` constructs `CsrMaterializer`, builds, clones the lease, unloads.
- Discard-lease + persist/open oracle and Materializer-equivalence oracle are green.

## Task Commits

1. **Task 1:** `262a122` (feat) CSR from_fold uses indexed live facts
2. **Task 2:** `adfbc09` (feat) untyped csr_lease_at through Materializer plus Product changelog

## TDD Gate Compliance

- Plan tasks were `tdd="true"`; `workflow.tdd_mode` is false.
- Discard and Materializer oracles were green on existing neighbor/as_of semantics; RED commits were not produced (unexpected GREEN of the specified assertions). Implementation still landed the live iterator and Materializer wiring.

## Files Created/Modified

- `crates/kutha-runtime/src/csr.rs` — from_fold uses live_facts_at
- `crates/kutha-runtime/src/quantum.rs` — csr_lease_at via CsrMaterializer
- `crates/kutha-runtime/tests/m012a_hot_indexes.rs` — HOT-02 and HOT-03 named tests
- `CHANGELOG.md` — Product entry for droppable CSR via Materializer

## Decisions Made

- Runtime does not retain a mounted CSR.
- Typed CSR remains from_fold; documented in rustdoc and an equality assertion.
- No GED-class comparison API.

## Deviations from Plan

None - plan executed as written. Discard/persist behavior already held; from_fold and Materializer wiring were still applied.

**Total deviations:** 0 auto-fixed
**Impact on plan:** None

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Three named HOT oracles exist. Plan 16-03 registers them with the governor.
- Do not clear harness S06.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/src/csr.rs
- FOUND: crates/kutha-runtime/src/quantum.rs
- FOUND: crates/kutha-runtime/tests/m012a_hot_indexes.rs
- FOUND: 262a122, adfbc09

---
*Phase: 16-fold-internal-hot-indexes*
*Completed: 2026-10-01*
