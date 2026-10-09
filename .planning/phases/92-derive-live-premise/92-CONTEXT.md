# Phase 92: Derive live premise — Context

**Milestone:** v0.26 / harness M032 S01  
**Requirements:** DERIVE-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Facade `derive()` must not cite retracted premises as `caused_by`. Align with retract/correct live filters. Honeycomb stays Proposed. Not Cypher/HNSW.

## Locked decisions

- **D-DERIVE-LIVE:** premise find requires `invalidated_at.is_none()`.
- **D-NO-FORMAT:** no Op/Event shape change; UnknownFact when no live premise.

## Verifier kill-tests

- `derive_after_sole_premise_retract_fails_closed`
- `derive_skips_invalidated_latest_uses_prior_live`
