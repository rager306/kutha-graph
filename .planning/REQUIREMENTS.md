# Requirements: Kutha

**Milestones:** v0.37 / harness M043 shipped 2026-10-10. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.37 Requirements (M043) — complete

### FSM quantum honest name (Phase 103, S01)

- [x] **FSM-OK-01**: skip_cargo quantum renamed scheduling-only; empty observations
- [x] **FSM-LIVE-01**: live variant asserts cargo/tenant/ff5 under KUTHA_LIVE_CARGO=1
- [x] **FSM-GATE-01**: governor `fsm-quantum-honest-name` forbids lying name

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Remaining G3 (#6, #20, #22…) | Next controller lease |
| Naive×norm_view×facade triad | Later thin slice |
| Product fold / Cypher / HNSW | Thaw map |
