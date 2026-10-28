# Phase 90: Writer lock retention — Context

**Milestone:** v0.24 / harness M030 S01  
**Requirements:** LOCK-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Non-poison `Persister` append/checkpoint errors must keep `kutha.lock`. Reuse poison path for `InvalidData`. Honeycomb stays Proposed. Not Cypher/HNSW.

## Locked decisions

- **D-LOCK-KEEP:** release `kutha.lock` only on fail-closed poison; `Drop` still unlocks.
- **D-CRASH-DROP:** heal/migrate oracles after injected Other must `drop` the crashed writer (process death).

## Verifier kill-tests

- `failpoint_append_keeps_lock_and_writer_usable`
- `failpoint_then_checkpoint_still_holds_lock`
