# Requirements: Kutha

**Milestones:** v0.40 / harness M046 shipped 2026-10-10. Next lease: G3 #39. Lease: `.kutha/STATE.md`.

## v0.40 Requirements (M046) — complete

### Parallel check-error parity (Phase 106, S01)

- [x] **PAR-ERR-01**: jobs=1 and jobs>1 map check.run exceptions to check-error
- [x] **PAR-ERR-02**: worker-error reserved for transport; budget 71; governor check

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| G3 #39 observe-skipped | Next slice |
| G3 #48 skip_emit/changed_paths | Later |
| G3 #6 | Decision-gate (human) |
| Product fold / Cypher / HNSW | Thaw map |
