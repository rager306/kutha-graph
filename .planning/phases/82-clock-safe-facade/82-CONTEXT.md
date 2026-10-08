# Phase 82: Clock-safe facade — Context

**Milestone:** v0.19 / harness M025 S02  
**Requirements:** CLK-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Public `as_of` / `as_known_on` take concrete clock newtypes so wrong roles and bare `u64` fail at compile time. Op storage stays `u64`. No fold semantic change. Not Cypher/HNSW.

## Locked decisions

- **D-ASOF-VT:** `as_of(ValidTime)` only.
- **D-AKN-WALL:** `as_known_on(WallTime)` only.
- **D-TRYBUILD:** trybuild UI oracles for wrong-role and bare `u64`.
- Other cuts may keep `impl Into` this slice.

## Verifier kill-tests

- `ui_clock_facade` four compile_fail cases green.
- Named pin oracles still green after call-site wraps.
