# Phase 89: Interrupted rollover seal heal — Context

**Milestone:** v0.23 / harness M029 S01  
**Requirements:** HEAL-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Crash after seal-trailer sync must not wedge `open_for_append`. Reuse attach path; finish rollover in heal. Honeycomb stays Proposed. Not Cypher/HNSW.

## Locked decisions

- **D-HEAL-SEAL:** detect complete trailing seal on active; promote to sealed + empty next active on attach.
- **D-NO-TRUNCATE:** do not truncate the trailer; finish the interrupted rollover.

## Verifier kill-tests

- `open_for_append_heals_after_rollover_trailer_crash`
- `open_for_append_heals_after_rollover_new_header_crash`
