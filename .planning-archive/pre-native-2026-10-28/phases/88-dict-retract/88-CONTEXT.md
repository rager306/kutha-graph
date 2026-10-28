# Phase 88: Dictionary retract on open — Context

**Milestone:** v0.22 / harness M028 S01  
**Requirements:** REV-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Trailing `emit_retract_dict` must survive persist/open. Reuse `class_with_dict` (no Dictionary QO, no format bump). Honeycomb stays Proposed. Not Cypher/HNSW.

## Locked decisions

- **D-RETRACT-KUTHA:** Retract of a `kutha:*` Assert is SelfCommit, matching the target Assert.
- **D-NO-DICT-QO:** Dictionary mode still writes no `QuantumOutcome`.

## Verifier kill-tests

- `trailing_emit_retract_dict_survives_persist_open`
- `trailing_dict_retract_without_user_qo_survives_open`
