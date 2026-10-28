---
phase: 16-fold-internal-hot-indexes
verified: 2026-09-30T19:35:56Z
status: passed
score: 5/5 must-haves verified
executor_automated: true
---

# Phase 16: Fold-internal hot indexes Verification Report

**Phase Goal:** hot `as_of` / `claim_supported_at` skip a full facts walk; CSR and indexes stay droppable leases; untyped `csr_lease_at` builds through Materializer; governor observes HOT-01..03
**Verified:** 2026-09-30T19:35:56Z
**Status:** passed
**Re-verification:** No — executor automated gates after 16-01..16-03. Independent kill-tests remain `/gsd-verify-work`.

`.kutha/STATE.md` Active Slice stays **S06**. This report does not clear the harness lease.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `as_of` / `claim_supported_at` match brute-force liveness and examine fewer than all facts on a decoy-heavy log (HOT-01, D-01) | ✓ VERIFIED | `as_of_and_claim_supported_at_skip_non_overlapping_facts` pass. Retract and snapshot rebuild oracles pass. Fingerprint hashes facts only. |
| 2 | Dropping CSR leaves fold answers unchanged; persist/open reconstructs `as_of` without a CSR sidecar SoT (HOT-02, D-02) | ✓ VERIFIED | `discard_csr_lease_and_snapshot_rebuild_keeps_as_of` pass. `e2e_incremental_matches_reconstruct_after_discarding_leases` pass. Snapshot fold JSON is facts + next_seq. |
| 3 | Untyped `csr_lease_at` uses `CsrMaterializer`; typed `from_fold` is a named spike limit (HOT-03, D-03) | ✓ VERIFIED | `csr_lease_at_builds_via_materializer` pass. Neighbors equal Materializer.build and `CsrLease::from_fold`. Typed edges equal `TypedCsrLease::from_fold`. |
| 4 | Named HOT-01..03 cargo tests are observed; `uv run kutha-gov ci` 0 HIGH (phase success criterion 4) | ✓ VERIFIED | `ci` exit 0: 0 HIGH, 0 LOW, 41 checks, `h5_selftest=41/41`. Observe evidence includes the three HOT names. `m012a-s06-hot-indexes` OK. |
| 5 | LOG/REF/ING/DUR/TIME oracles stay required; FF5/FF6 green; honeycomb Proposed; no M012/M002; no GED-class fork-diff API (D-04) | ✓ VERIFIED | Prior named tests still in `observe_cargo.required` and green. ADR-040/041 `map: Proposed`. ADR-061 evidence unchanged. No Rocks/Cypher/HNSW crates added. `.kutha/STATE.md` Active Slice remains S06. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Status |
| -------- | ------ |
| Fold-internal VT/claim maps | ✓ |
| `crates/kutha-runtime/tests/m012a_hot_indexes.rs` | ✓ |
| Materializer-wired `csr_lease_at` | ✓ |
| `B-m012a-s06` / `m012a-s06-hot-indexes` | ✓ |
| `16-01-SUMMARY.md` `16-02-SUMMARY.md` `16-03-SUMMARY.md` | ✓ |

## Regression

`cargo test --workspace --offline` green including FF5/FF6, Phase 12 LOG, Phase 13 REF, Phase 14 ING, and Phase 15 DUR/TIME oracles. Honeycomb stays Proposed. `.kutha/STATE.md` Active Slice remains S06; `L_delivery=M012a-S05-done`.

## Gaps

None for HOT-01..03. Do not clear S06 here. M012 dictionaries-as-facts and M002 Rocks stay frozen.

## Self-Check: PASSED

---
*Phase: 16-fold-internal-hot-indexes*
*Verified: 2026-10-01*
