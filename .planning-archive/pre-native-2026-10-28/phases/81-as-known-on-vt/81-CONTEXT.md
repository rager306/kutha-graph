# Phase 81: as_known_on VT decision — Context

**Milestone:** v0.19 / harness M025 S01  
**Requirements:** AKN-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** Controller decision + pin (GSD auto)

## Phase Boundary

Record that `as_known_on` is TT-only **by design** (G-A08 / G-C03). No fold behavior change. Not Cypher/HNSW. Honeycomb stays Proposed.

## Locked decisions

- **D-AKN-TT-ONLY:** `as_known_on` → greatest TtAnchor ≤ wall → `known_at(tt)` with no VT filter. Pin, not fix.
- **D-TR02-TRAP:** not-yet-in-force VT facts may appear on known-on while absent from `as_of`; combining axes is a later facade (`norm_view`).
- Reject collapsing knowledge and force in core.

## Verifier kill-tests

- `m025_as_known_on_vt` two oracles green.
- Prior `closed_vt_interval_survives_as_known_on` / `true_on_d_differs_from_known_on_d` unchanged.
