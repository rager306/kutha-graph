# Requirements: Kutha

**Milestones:** v0.35 / harness M041 shipped 2026-10-10. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.35 Requirements (M041) — complete

### Persist-trace identity + shrink gate (Phase 101, S01)

- [x] **TRACE-ID-01**: persist+open preserves non-Define EventIds across PersistLayout
- [x] **SHRINK-01**: candidate_deps_hold + shrink_events_ddmin; Rollover seam reject ≠ oracle fail

## v0.34 Requirements (M040) — complete

### Vacuous planning gates (Phase 100, S01)

- [x] **VAC-GATE-01**: live CONTEXT without VERIFICATION is HIGH; idle without CONTEXT skips

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Remaining G3 (#17, #6, #20…) | Next controller lease |
| Full Helix FileRegressionCorpus / Naive×norm_view×facade triad | Later thin slices |
| Product fold / Cypher / HNSW | Thaw map |
