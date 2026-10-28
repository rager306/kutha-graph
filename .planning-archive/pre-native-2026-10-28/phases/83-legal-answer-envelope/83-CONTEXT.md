# Phase 83: Legal answer envelope — Context

**Milestone:** v0.19 / harness M025 S03  
**Requirements:** ENV-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

`Runtime::norm_view(vt, wall)` returns force ∩ knowledge ∩ eligibility with typed non-success. Does not change `as_of` or ASOF-ELIG-01. Not Cypher/HNSW; ADR-090 ontology stays frozen.

## Locked decisions

- **D-NV-CUT:** knowledge TT from greatest `TtAnchor` ≤ wall; force via `live_at(tt, vt)`.
- **D-NV-ELIG:** Behavior requires `derivation_eligible_at`; Assert passes (R-22).
- **D-NV-OUT:** `Ok` / `Conflict` / `Abstain` / `Unknown` / `MissingAnchor` (no Incomplete this slice).

## Verifier kill-tests

- Retracted-premise → Abstain while `as_of` still lists derived.
- Missing anchor → `MissingAnchor`.
