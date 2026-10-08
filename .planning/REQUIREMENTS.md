# Requirements: Kutha

**Milestones:** v0.18 / harness M024 shipped 2026-10-08. **v0.19 / harness M025** active. Lease: `.kutha/STATE.md`.

## v0.19 Requirements (M025)

### as_known_on VT (Phase 81, S01)

- [x] **AKN-01**: ADR-013 states `as_known_on` is TT-only by design; pin oracle for not-yet-in-force VT; semantic-contract agrees

### Clock-safe facade (Phase 82, S02)

- [ ] **CLK-01**: Public `as_of` / `as_known_on` reject wrong clock roles at compile time; Op storage may stay `u64`; bench ≤10%

### Answer envelope (Phase 83, S03)

- [ ] **ENV-01**: `norm_view(vt, wall)` with eligibility and typed non-success

### Demo (Phase 84, S04)

- [ ] **DEMO-01**: Thin bin + real-text fixtures; no pack thaw

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Cypher / HNSW / ADR-090 pack | Thaw map |
| Five-axis clocks | D013-1 |
| Pi Durable | Parked |
