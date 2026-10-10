# Requirements: Kutha

**Milestones:** v0.43 / harness M049 shipped 2026-10-10. Next lease: G4 #14 (compat policy). Lease: `.kutha/STATE.md`.

## v0.43 Requirements (M049) — complete

### Skip-cargo log marker (Phase 109, S01)

- [x] **SKIP-LOG-01**: emit_tenant skip records tenant=skipped + fold tip
- [x] **SKIP-LOG-02**: budget 74; governor check `skip-cargo-log-marker`

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| G4 #14 Op compat | Needs policy decision |
| G3 #6 | Decision-gate (human) |
| Product fold / Cypher / HNSW | Thaw map |
