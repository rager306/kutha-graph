# Phase 84: Demo binary and fixtures — Context

**Milestone:** v0.19 / harness M025 S04  
**Requirements:** DEMO-01  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** GSD auto

## Phase Boundary

Thin `examples/statute_as_of` answers "norm in force on D, as known on W" via `norm_view`. 2–3 real-text statute excerpts under `fixtures/statute/` are test data only. Not Cypher/HNSW; `I-legal-pack-not-next` stays.

## Locked decisions

- **D-DEMO-BIN:** extend `statute_as_of`, not ADR-070.
- **D-FIXTURE-DATA:** `m016` quote pattern; no pack crate.
- **D-ENVELOPE:** demo asserts `norm_view` Ok cuts differ (50 vs 30 km/h).

## Verifier kill-tests

- `m025_demo_fixtures` art12 / punkt / edition oracles green.
- `cargo run --example statute_as_of` prints Ok envelopes.
