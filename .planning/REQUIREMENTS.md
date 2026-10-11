# Requirements: Kutha

**Milestones:** v0.47 / harness M053 shipped 2026-10-11. Next lease: M054 gap slice. Lease: `.kutha/STATE.md`.

## v0.47 Requirements (M053) — complete

### CE review regression enrolment (Phase 116, S01)

- [x] **REG-01**: advisor withdrawal oracles (hydrate outcomes, persist_dir anchor, rollover seam)
- [x] **REG-02**: all five legacy meta-ops fail-open regression
- [x] **REG-03**: matrix killed rows + gap classes; `ce-review-regression-map.md`

## Out of Scope

| Feature | Reason |
|---------|--------|
| M054 apply-queue fixes | Gap owners only until leased |
| G3 #6 | Human decision-gate |
| Cypher / HNSW | Thaw map |
