---
phase: 17-allowlist-as-log-facts
verified: 2026-10-01T01:56:15Z
status: passed
score: 5/5 must-haves verified
executor_automated: true
---

# Phase 17: Allowlist as log facts Verification Report

**Phase Goal:** Relation allowlist entries are versioned log facts; admit consults the fold cut — not only tip YAML or env
**Verified:** 2026-10-01T01:56:15Z
**Status:** passed
**Re-verification:** No — executor automated gates after 17-01..17-03. Independent kill-tests remain `/gsd-verify-work`.

`.kutha/STATE.md` Active Slice stays **S01**. This report does not clear the harness lease.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Relation allowlist entries are appendable versioned facts on the event log (ALL-01, D-01) | ✓ VERIFIED | `allow_relation_appends_as_log_fact` and `retract_allow_relation_drops_admit_at_later_cut` pass. AllowRelation is not a graph Fact. Retract of its EventId drops later admit; prefix fork still admits. |
| 2 | `Runtime::admit` consults the fold cut, not tip YAML/env alone (ALL-02, D-02) | ✓ VERIFIED | `admit_consults_fold_allowlist_not_yaml_alone` pass. persist/open reconstructs YAML-absent `leasedRel` from log facts. |
| 3 | Unknown relation still fails closed; FF6 stays green (ALL-03, D-03) | ✓ VERIFIED | `ff6_unknown_relation_does_not_append` pass. Empty AllowRelation name does not append. YAML-seeded `inForceAs` still admits. |
| 4 | Named ALL oracles are observed; `uv run kutha-gov ci` 0 HIGH (phase success criterion 4) | ✓ VERIFIED | `ci` exit 0: 0 HIGH, 0 LOW, 42 checks, `h5_selftest=42/42`. Observe evidence includes the three ALL names. `m012-s01-allowlist-facts` OK. FF6 remains required. |
| 5 | LOG/REF/ING/DUR/TIME/HOT oracles stay required; honeycomb Proposed; no ADR-050 six dictionaries / Rocks / Cypher / HNSW (D-04, D-05) | ✓ VERIFIED | Prior named tests still in `observe_cargo.required` and green. ADR-011/ADR-050 `map: Proposed`. ADR-050 `delivery: frozen`. No frozen product crates added. `.kutha/STATE.md` Active Slice remains S01. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Status |
| -------- | ------ |
| `Op::AllowRelation` + fold allow-entries + admit fold-then-YAML | ✓ |
| `crates/kutha-runtime/tests/m012_allowlist_facts.rs` | ✓ |
| `B-m012-s01` / `m012-s01-allowlist-facts` | ✓ |
| `17-01-SUMMARY.md` `17-02-SUMMARY.md` `17-03-SUMMARY.md` | ✓ |

## Regression

`cargo test --workspace --offline` green including FF5/FF6 and M012a LOG/REF/ING/DUR/TIME/HOT oracles. Honeycomb stays Proposed. `.kutha/STATE.md` Active Slice remains S01; `L_delivery=M012-leased`.

## Gaps

None for ALL-01..03. Do not clear S01 here. Rule registry, admission meta-facts, Action, n-ary derivation, ADR-050 six dictionaries, and M002 Rocks stay unstarted.

## Self-Check: PASSED

---
*Phase: 17-allowlist-as-log-facts*
*Verified: 2026-10-01*
