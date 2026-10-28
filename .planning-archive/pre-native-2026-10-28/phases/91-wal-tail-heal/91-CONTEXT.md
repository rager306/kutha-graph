# Phase 91: WAL-tail heal — Context

**Milestone:** v0.25 / harness M031 S01  
**Requirements:** HEAL-02  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Legacy jsonl-ahead-of-WAL after Jsonl failpoint must not wedge `open_for_append`. Widen heal beyond `dropped_uncommitted>0`. Honeycomb stays Proposed. Not Cypher/HNSW.

## Locked decisions

- **D-HEAL-WAL:** rewrite when `legacy_attach_needs_rewrite` (attach tip checks would fail).
- **D-REUSE-PERSIST:** heal via existing `persist_unlocked`; no new physical format.

## Verifier kill-tests

- `open_for_append_heals_jsonl_ahead_of_wal`
