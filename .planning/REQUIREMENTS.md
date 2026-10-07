# Requirements: Kutha

**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestones:** v0.14 / harness M020 shipped 2026-10-07. **v0.15 / harness M021** active (wedge decisions). Lease: `.kutha/STATE.md`. Prior: archive `.planning/milestones/`.

## v0.15 Requirements (M021)

### Decisions and pins (Phase 68, S01)

- [x] **TR-02-01**: Force = VT; publication = `TtAnchor` / `as_known_on`; case decision clock = `as_of(vt)` argument; named oracle; no fifth axis
- [x] **C01-02**: Pack expresses sunset / application window as Assert (`appliesTo` VT); FIXG-01 still fails as Horn; Phase 46 (a) confirmed
- [x] **MATRIX-01**: SEM-c01-decision, SEM-horn-kc, SEM-asof-elig, SEM-tr02-legal are `killed` with oracles (Phase 46 pins reused where they already exist)

### Fold defect (Phase 69, S02)

- [ ] **RETRACT-01**: `Op::Retract { event_id }` invalidates every live Fact with that event id (CorrectInterval multi-residual); RED first; bench ≤10%

### Close (Phase 70, S03)

- [ ] **CLOSE-01**: M021 closed; Active None; audit note; honeycomb Proposed

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Cypher / HNSW / legal pack crate | Own thaw gates |
| Fifth bi-temporal axis | D013-1 / D-TR-02 |
| Invert Phase 46 HORN-KC / ASOF pins | Locked |
| Pi Durable agent-task resume | Separate harness plane |
| SPIKE-LAW-01 real corpus | After C01-02 pin |
