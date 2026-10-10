# Requirements: Kutha

**Milestones:** v0.44 / harness M050 shipped 2026-10-10. Next lease: G4 #24 or Op backfill. Lease: `.kutha/STATE.md`.

## v0.44 Requirements (M050) — complete

### Op compat policy (Phase 110, S01)

- [x] **OP-COMPAT-01**: documented break — legacy meta-ops fail deserialize
- [x] **OP-COMPAT-02**: modern kutha:* Assert round-trips; policy doc; budget 75

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Serde TermId remap / migrate | Deferred follow-on |
| G4 #24 pin scheme | Next residual |
| G3 #6 | Decision-gate (human) |
| Cypher / HNSW | Thaw map |
