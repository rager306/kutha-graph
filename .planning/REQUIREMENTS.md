# Requirements: Kutha

**Milestones:** v0.19 / harness M025 shipped 2026-10-09. Next lease: M026 / v0.20. Lease: `.kutha/STATE.md`.

## v0.19 Requirements (M025) — complete

### as_known_on VT (Phase 81, S01)

- [x] **AKN-01**: ADR-013 states `as_known_on` is TT-only by design; pin oracle for not-yet-in-force VT; semantic-contract agrees

### Clock-safe facade (Phase 82, S02)

- [x] **CLK-01**: Public `as_of` / `as_known_on` take `ValidTime` / `WallTime`; bare `u64` and wrong roles fail compile (trybuild); Op storage stays `u64`; bench ≤10%

### Answer envelope (Phase 83, S03)

- [x] **ENV-01**: `norm_view(vt, wall)` with eligibility and typed non-success

### Demo (Phase 84, S04)

- [x] **DEMO-01**: Thin bin + real-text fixtures; no pack thaw

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Cypher / HNSW / ADR-090 pack | Thaw map |
| Five-axis clocks | D013-1 |
| Pi Durable | Parked |
