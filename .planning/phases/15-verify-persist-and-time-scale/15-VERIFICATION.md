---
phase: 15-verify-persist-and-time-scale
verified: 2026-09-30T19:05:42Z
status: passed
score: 5/5 must-haves verified
executor_automated: true
---

# Phase 15: Verify, persist, and time scale Verification Report

**Phase Goal:** open rejects a tampered snapshot; persist replaces events.jsonl atomically; Define ids stay name-stable; VT/TT declare YearCe and LogSequence; governor observes DUR-01..03 and TIME-01..02
**Verified:** 2026-09-30T19:05:42Z
**Status:** passed
**Re-verification:** No — executor automated gates after 15-01..15-03. Independent kill-tests remain `/gsd-verify-work`.

`.kutha/STATE.md` Active Slice stays **S04**. This report does not clear the harness lease.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Tampered snapshot is rejected on open (DUR-01, D-01) | ✓ VERIFIED | `open_rejects_tampered_snapshot` pass. Honest persist/open fingerprints match. `store::open` maps `ReplayDivergence` to `InvalidData`. |
| 2 | events.jsonl persist is rename-into-place; crash-before-rename is not success (DUR-02, D-02) | ✓ VERIFIED | `persist_replaces_events_jsonl_atomically` pass. WAL-deleted open matches fold fingerprint. Abort-before-rename leaves dest bytes. `wal.rs` protocol untouched. |
| 3 | Define event ids are stable across persist/open for the same term set (DUR-03, D-03) | ✓ VERIFIED | `define_ids_stable_across_persist_open` pass. `Event::stable_define` SHA-256 `kutha-define-id`. `intern_appends_define_for_new_terms_only` still green. |
| 4 | VT/TT declare YearCe and LogSequence used by fixtures; named tests document mapping (TIME-01, TIME-02, D-04, D-05) | ✓ VERIFIED | `fixture_years_map_to_year_ce_scale` and `transaction_time_is_log_sequence` pass. FF5 T_OLD/T_NEW remain 2015/2021. No TT-to-wall table. No chrono. |
| 5 | Named DUR/TIME oracles observed; `uv run kutha-gov ci` 0 HIGH; FF5/FF6 green; honeycomb Proposed; LOG/REF/ING remain; M012/M002 unstarted (D-06) | ✓ VERIFIED | `ci` exit 0: 0 HIGH, 0 LOW, 40 checks, `h5_selftest=40/40`. Observe names include five DUR/TIME tests plus LOG/REF/ING. `m012a-s04-verify-persist` and `m012a-s05-time-scale` OK. ADR-012/013 `map: Proposed`. No Rocks/Cypher/HNSW crates added. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Status |
| -------- | ------ |
| `store::open` snapshot `replay_check` | ✓ |
| atomic `events.jsonl` temp+rename | ✓ |
| `Event::stable_define` | ✓ |
| `TimeScale` / `VALID_TIME_SCALE` / `TRANSACTION_TIME_SCALE` | ✓ |
| `crates/kutha-runtime/tests/m012a_verify_persist.rs` | ✓ |
| `crates/kutha-runtime/tests/m012a_time_scale.rs` | ✓ |
| `B-m012a-s04` / `m012a-s04-verify-persist` | ✓ |
| `B-m012a-s05` / `m012a-s05-time-scale` | ✓ |
| `15-01-SUMMARY.md` `15-02-SUMMARY.md` `15-03-SUMMARY.md` | ✓ |

## Regression

`cargo test --workspace --offline` green including FF5/FF6, Phase 12 LOG, Phase 13 REF, and Phase 14 ING oracles. Honeycomb stays Proposed. `.kutha/STATE.md` Active Slice remains S04; `L_delivery=M012a-S03-done`.

## Gaps

None for DUR-01..03 and TIME-01..02. Do not clear S04 here. Fold-internal hot indexes (Phase 16 / S06) are out of this slice.

## Self-Check: PASSED

---
*Phase: 15-verify-persist-and-time-scale*
*Verified: 2026-10-01*
