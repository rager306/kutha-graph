# Phase 101 CONTEXT — persist-trace identity + shrink gate

**Source:** Helix-informed matrix extension (corrected: Naive/shrink/replay exist).  
**Lease:** M041 S01 / GSD v0.35. ADR-023 Clarification.

## Goal

Close the honesty gap on `SEM-seeded-det` (triples ≠ EventIds) with one Op-trace × `PersistLayout` × `expected_persist_event_ids`. Harden shrink: `candidate_deps_hold` + chunk-ddmin; Rollover seam reject ≠ oracle fail (`persist_stamps_disagree` must not panic on tiny candidates). Optional `KUTHA_SAVE_SHRUNK`. Do not invent a parallel ReplayEngine.

## Oracles

- `persist_roundtrip_trace_identity_across_layouts`
- `candidate_deps_hold_rejects_orphan_retract`
- `shrink_events_ddmin_drops_middle_when_prefix_biased`
