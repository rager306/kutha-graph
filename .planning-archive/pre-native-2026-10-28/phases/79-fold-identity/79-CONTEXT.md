# Phase 79: Fold identity — Context

**Milestone:** v0.18 / harness M024 S01  
**Requirements:** FOLD-01, FOLD-02, FOLD-03  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** Controller + owner (GSD auto)

## Phase Boundary

Close G-A02 / G-A05 / G-A07 fold-identity debt from post-M022 audits. No Cypher/HNSW. Honeycomb stays Proposed.

## Locked decisions

- **D-LIVE-GATE:** Retract/Correct/CorrectInterval require ≥1 live residual; else `UnknownFact`.
- **D-CORRECT-MULTI:** Correct on N>1 live residuals → invalidate all, mint one replacement with Correct payload.
- **D-CI-MULTI:** CorrectInterval on ≠1 live residual → `IntervalPatchRejected`.
- **D-G-A07:** Retract(original) after CI is dead → UnknownFact; clear via Retract(ci_id).
- RED first; bench ≤10%.

## Verifier kill-tests

- `m024_fold_identity` four oracles green.
- `m011_partial_correction::interval_patch_not_live` expects UnknownFact.
- Named M010–M022 oracles unchanged in intent.
