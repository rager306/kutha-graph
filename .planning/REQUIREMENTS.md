# Requirements: Kutha

**Milestones:** v0.36 / harness M042 shipped 2026-10-10. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.36 Requirements (M042) — complete

### m028×layouts + shrink corpus (Phase 102, S01)

- [x] **RETRACT-LAY-01**: trailing emit_retract_dict revoked after open on every PersistLayout
- [x] **CORPUS-01**: save_shrunk_to / load_shrunk content-addressed roundtrip

## v0.35 Requirements (M041) — complete

### Persist-trace identity + shrink gate (Phase 101, S01)

- [x] **TRACE-ID-01**: persist+open preserves non-Define EventIds across PersistLayout
- [x] **SHRINK-01**: candidate_deps_hold + shrink_events_ddmin; Rollover seam reject ≠ oracle fail

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Remaining G3 (#17, #6, #20…) | Next controller lease |
| Naive×norm_view×facade triad | Later thin slice |
| Product fold / Cypher / HNSW | Thaw map |
