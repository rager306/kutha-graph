# Requirements: Kutha

**Milestones:** v0.25 / harness M031 shipped 2026-10-09. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.25 Requirements (M031) — complete

### WAL-tail heal (Phase 91, S01)

- [x] **HEAL-02**: after Jsonl failpoint (jsonl ahead of WAL, dropped==0), `open_for_append` rewrites and attaches

## v0.24 Requirements (M030) — complete

### Writer lock retention (Phase 90, S01)

- [x] **LOCK-01**: non-poison `Other` append/checkpoint errors keep `kutha.lock`; poison still releases for heal

## v0.23 Requirements (M029) — complete

### Interrupted rollover seal heal (Phase 89, S01)

- [x] **HEAL-01**: after RolloverTrailer / RolloverNewHeader crash, `open_for_append` finishes the seal and can append

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| G2 lineage (#4…) | Next controller lease |
| Product fold / Cypher / HNSW | Thaw map |
| Cached verified tip on open | Deferred (tenant ≪ 100MB) |
