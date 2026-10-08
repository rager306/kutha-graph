# Requirements: Kutha

**Milestones:** v0.17 / harness M023 shipped 2026-10-08. **v0.18 / harness M024** shipped 2026-10-08. Lease: `.kutha/STATE.md`.

## v0.18 Requirements (M024) — shipped

### Fold identity (Phase 79, S01)

- [x] **FOLD-01**: Retract/Correct/CorrectInterval on a dead or unknown id → `UnknownFact` (fail-closed)
- [x] **FOLD-02**: Correct on multi-residual event_id invalidates all live residuals and mints one replacement
- [x] **FOLD-03**: CorrectInterval on ≠1 live residual → `IntervalPatchRejected`; ADR-013 Clarification recorded

### Write-path (Phase 80, S02)

- [x] **WRITE-01**: intern Define crash window characterized (SelfCommit; Assert-without-QO dropped); D014-1 digest `[u8;32]` pinned; event_ids width leak + GROUP-01 residual

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Typed clocks / answer envelope / demo CLI | M025 |
| Cypher / HNSW / legal pack | Thaw map |
