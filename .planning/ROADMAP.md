# Roadmap: Kutha

## Overview

v0.01–v1.01 shipped. v1.02 stores live behaviors in a set so one removal is one visit. Honeycomb stays Proposed.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. v1.02 closed as M108 (`L_delivery=M108-closed`).

## Milestones

- ✅ **v0.01–v0.98** — shipped (through unindex cause reverse pointer)
- ✅ **v0.99 residual correctness and hot-path honesty** — phases 176–179 closed (M105)
- ✅ **v1.00 open restores fold-index lease** — phase 180 closed (M106)
- ✅ **v1.01 unindex skips the live-behavior vector** — phase 181 closed (M107)
- ✅ **v1.02 live behaviors are a set** — phase 182 closed (M108)

## Phases (v1.02)

- [x] **Phase 182:** one Behavior removal does not walk every live id

## Phases (v1.01)

- [x] **Phase 181:** non-behavior unindex returns before walking `live_behaviors`

## Phases (v1.00)

- [x] **Phase 180:** open calls `set_index_mode` so snapshot lease warmth survives the env

## Phases (v0.99)

- [x] **Phase 176:** newest-first policy pin election (`policy_by_ingest`)
- [x] **Phase 177:** premise-rel reverse pointer on unindex
- [x] **Phase 178:** Op-wire skips a duplicate modern Define
- [x] **Phase 179:** all Horn combinations; replay VT must be covered; open keeps the replay fold; Correct walks every live residual

## Still open

- `FoldIndexMode::Lazy` still full-scans until hot maps rebuild (intentional thin write)
- Cypher, HNSW, legal pack, ADR-050 six kinds stay frozen
- ADR status stays Proposed (`docs/ADR/README.md` engine-evidence table, 2026-10-18)

## Progress

See `.planning/STATE.md` and `.kutha/STATE.md`.
