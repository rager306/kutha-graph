# Requirements: Kutha

**Milestones:** v0.26 / harness M032 shipped 2026-10-09. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.26 Requirements (M032) — complete

### Derive live premise (Phase 92, S01)

- [x] **DERIVE-01**: `derive()` premise lookup skips `invalidated_at` facts; sole retract → UnknownFact

## v0.25 Requirements (M031) — complete

### WAL-tail heal (Phase 91, S01)

- [x] **HEAL-02**: after Jsonl failpoint (jsonl ahead of WAL, dropped==0), `open_for_append` rewrites and attaches

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Remaining G2 lineage (#8…) | Next controller lease |
| Product fold / Cypher / HNSW | Thaw map |
