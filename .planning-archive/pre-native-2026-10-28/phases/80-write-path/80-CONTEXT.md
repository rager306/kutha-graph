# Phase 80: Write-path atomicity — Context

**Milestone:** v0.18 / harness M024 S02  
**Requirements:** WRITE-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** Controller + owner (GSD auto)

## Phase Boundary

Characterize intern Define crash window and pin D014-1 receipt digest size. No Cypher/HNSW. GROUP-01 stays residual. Honeycomb stays Proposed.

## Locked decisions

- **D-DEFINE-SELFCOMMIT:** Keep Define as SelfCommit (no quantum wrap). Crash after Assert (NeedsOutcome, no QO) drops Assert; prior Defines remain via `committed_prefix`.
- **D-DIGEST-32:** D014-1 constant commitment is `QuantumReceipt::digest` `[u8; 32]` (hex 64). Independent of cascade width.
- **D-WIDTH-LEASE:** `event_ids` / `events_in_quantum` remain resume leases (width leak residual until a format bump).
- **D-GROUP-01:** Typed/Dictionary mid-group partial prefix remains residual (c); no format change this slice.
- RED first; bench ≤10%.

## Verifier kill-tests

- `m024_write_path` four oracles green.
- Named M002/M019 durability oracles unchanged in intent.
