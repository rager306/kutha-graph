---
phase: 13-stable-references
verified: 2026-09-30T18:16:00Z
status: passed
score: 4/4 must-haves verified
executor_automated: true
---

# Phase 13: Stable references Verification Report

**Phase Goal:** Retract, Correct, and justification cites target minting EventId; a rebuilt fold that renumbers local seqs still applies the same payloads; governor observes the three REF oracles
**Verified:** 2026-09-30T18:16:00Z
**Status:** passed
**Re-verification:** No — executor automated gates after 13-01..13-03. Independent kill-tests remain `/gsd-verify-work`.

`.kutha/STATE.md` Active Slice stays **S02**. This report does not clear the harness lease.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Retract and Correct (including CorrectInterval) target minting EventId, not fold-local seq (REF-01, D-01) | ✓ VERIFIED | `retract_by_event_id_invalidates_support` pass. `m011_partial_correction` residuals and whole-version Correct pass. Unknown EventId does not append. |
| 2 | Justification cites and `check_admission` key on minting EventId across fork (REF-02, D-02) | ✓ VERIFIED | `justification_cites_source_event_ids_survive_fork` pass. `e2e_justification_cites_sources_and_rejects_stale_admission` still stales after CorrectInterval/Retract. LOG-02 discard-sidecar still admits. |
| 3 | A rebuilt fold that assigns different local seqs still applies the same Retract and JustificationCite payloads (REF-03, D-03) | ✓ VERIFIED | `rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads` pass: live seq 1 → rebuilt seq 0; retract still binds; cite-only rebuild admits. |
| 4 | Named REF oracles observed; `uv run kutha-gov ci` 0 HIGH; FF5/FF6 green; honeycomb Proposed; M012/M002 unstarted (GATE-01 / D-04) | ✓ VERIFIED | `ci` exit 0: 0 HIGH, 0 LOW, 37 checks, `h5_selftest=37/37`. Observe names include the three REF tests and the three LOG tests. `m012a-s02-stable-refs` OK. ADR-011/061 `map: Proposed`. No Rocks/Cypher/HNSW crates added. |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Status |
| -------- | ------ |
| `Fact.event_id` / EventId Retract/Correct/CorrectInterval | ✓ |
| EventId-only `Op::JustificationCite` | ✓ |
| `crates/kutha-runtime/tests/m012a_stable_refs.rs` (three named tests) | ✓ |
| `B-m012a-s02` / `m012a-s02-stable-refs` | ✓ |
| `13-01-SUMMARY.md` `13-02-SUMMARY.md` `13-03-SUMMARY.md` | ✓ |

## Regression

`cargo test --workspace --offline` green including FF5/FF6 and Phase 12 LOG oracles. Honeycomb stays Proposed.

## Gaps

None for REF-01..03. Do not clear S02 here. Phase 14 (idempotent ingest / delivery keys) is out of this lease.

## Self-Check: PASSED

---
*Phase: 13-stable-references*
*Verified: 2026-10-01*
