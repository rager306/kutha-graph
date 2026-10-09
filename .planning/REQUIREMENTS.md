# Requirements: Kutha

**Milestones:** v0.27 / harness M033 shipped 2026-10-09. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.27 Requirements (M033) — complete

### CI admission VT (Phase 93, S01)

- [x] **ADM-01**: `check_admission_row` admits when any live CI residual covers `row.vt`; dead CI stays `stale_support`

## v0.26 Requirements (M032) — complete

### Derive live premise (Phase 92, S01)

- [x] **DERIVE-01**: `derive()` premise lookup skips `invalidated_at` facts; sole retract → UnknownFact

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Remaining G2 lineage (#9…) | Next controller lease |
| Product fold / Cypher / HNSW | Thaw map |
