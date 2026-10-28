---
phase: 21-multi-hop-derivation
verified: 2026-10-01T04:14:34Z
status: passed
score: 5/5 must-haves verified
executor_automated: true
---

# Phase 21: Multi-hop derivation Verification Report

**Phase Goal:** Derivation eligibility is not limited to one-hop `caused_by` for the leased fixture; a named two-hop hashed test is green; `inverse_knows` remains the residual automatic cascade; governor observes DER-01..03
**Verified:** 2026-10-01T04:14:34Z
**Status:** passed
**Re-verification:** No — executor automated gates after 21-01..21-03. Independent kill-tests remain `/gsd-verify-work`.

`.kutha/STATE.md` Active Slice stays **S05**. This report does not clear the harness lease.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A developer can emit a two-hop hashed Behavior chain and see `derivation_eligible_at` true on the leaf while every hop is live (DER-02, D-01, D-02) | ✓ VERIFIED | `two_hop_derivation_eligible_when_chain_live` pass. hop1 and hop2 eligible at `(u64::MAX, 2017)`. hop2 fact `is_live_at`. Public signature unchanged. No MATCH compiler. |
| 2 | After Retract of the root Assert, the leaf Behavior fact may still be live and `derivation_eligible_at` on that leaf is false (DER-01, D-01, T-21-02) | ✓ VERIFIED | `two_hop_ineligible_when_root_assert_retracted` pass. Parent RegisterRule retract also drops hop2 (`two_hop_ineligible_when_parent_rule_retracted`). persist/open reconstructs both pictures. |
| 3 | `inverse_knows` remains the residual automatic Assert cascade with empty `rule_version`; hashed user Behavior chains are the leased multi-hop follow-on (DER-03, D-03, T-21-03) | ✓ VERIFIED | `inverse_knows_cascade_is_residual_spike` pass. knows Assert mints empty-pin `inverse_knows`; relatedTo Assert does not. |
| 4 | Named DER oracles are observed; `uv run kutha-gov ci` 0 HIGH (phase success criterion 4) | ✓ VERIFIED | `ci` exit 0: 0 HIGH, 0 LOW, 46 checks, `h5_selftest=46/46`. Observe evidence includes the three DER names. `m012-s05-multi-hop` OK. ALL, RULE, ADM, ACT, and FF6 remain required. |
| 5 | Honeycomb Proposed; no ADR-050 six dictionaries / Rocks / Cypher / HNSW / legal pack / MATCH compiler (D-04, D-05) | ✓ VERIFIED | ADR-011 `map: Proposed`. ADR-050 `delivery: frozen`, `freeze_as` unchanged. No frozen product crates added. `.kutha/STATE.md` Active Slice remains S05. `L_delivery=M012-S04-done`. One-hop `derived_q_loses_eligibility_when_last_premise_support_withdrawn` stays green. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Status |
| -------- | ------ |
| `Runtime::derivation_eligible_at` caused_by walk + visited EventId set | ✓ |
| `crates/kutha-runtime/tests/m012_multi_hop.rs` | ✓ |
| `B-m012-s05` / `m012-s05-multi-hop` | ✓ |
| `21-01-SUMMARY.md` `21-02-SUMMARY.md` `21-03-SUMMARY.md` | ✓ |

## Regression

`cargo test --workspace --offline` green including FF5/FF6, Phase 17 ALL, Phase 18 RULE, Phase 19 ADM, Phase 20 ACT, and M012a LOG/REF/ING/DUR/TIME/HOT oracles. Honeycomb stays Proposed. `.kutha/STATE.md` Active Slice remains S05; `L_delivery=M012-S04-done`.

## Gaps

None for DER-01..03. Do not clear S05 here. ADR-050 six dictionaries, MATCH compiler, provenance polynomials, and M002 Rocks stay unstarted.

## Self-Check: PASSED

---
*Phase: 21-multi-hop-derivation*
*Verified: 2026-10-01*
