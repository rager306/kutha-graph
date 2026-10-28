# Phase 85: Generators and missing oracles — Context

**Milestone:** v0.20 / harness M026 S01  
**Requirements:** GEN-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Extend `ingest_mix` via `MixStyle::Coverage` and add G-C04/C05/C11 oracles. No product behavior change unless an oracle exposes a bug. Not Cypher/HNSW.

## Locked decisions

- **D-COV-STYLE:** Coverage is a third MixStyle; BehaviorFree/WithHorn LCG unchanged.
- **D-GC04:** user Assert EventId equal after persist/open.
- **D-GC05:** facade retract/correct/derive with two live facts → latest only.
- **D-GC11:** empty runtime persist/open keeps empty `as_of`.

## Verifier kill-tests

- `ingest_mix_coverage_reaches_named_gaps` green.
- G-C04/C05/C11 oracles green; BehaviorFree seed 42 still asserts without Coverage flags.
