# Requirements: Kutha

**Milestones:** v0.45 / harness M051 shipped 2026-10-11. Next lease: G4 #26 or Op backfill. Lease: `.kutha/STATE.md`.

## v0.45 Requirements (M051) — complete

### Pin compat policy (Phase 111, S01)

- [x] **PIN-COMPAT-01**: documented rotation to pin scheme v1; no silent UTF-8 remap
- [x] **PIN-COMPAT-02**: hardcoded ident / legacy / non-ident stability vectors; budget 76

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Behavior rule_version migrate UTF-8→v1 | Deferred follow-on |
| Op serde TermId remap | Deferred (`op-compat.md`) |
| G4 #26 clock facade | Next residual |
| G3 #6 | Decision-gate (human) |
| Cypher / HNSW | Thaw map |
