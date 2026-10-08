# Requirements: Kutha

**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestones:** v0.15 / M021 shipped 2026-10-07. **v0.16 / harness M022** active (audit residual honesty). Lease: `.kutha/STATE.md`.

## v0.16 Requirements (M022)

### Audit honesty (Phase 71, S01)

- [ ] **DOC-AUDIT-01**: v0.14 audit no longer lists Retract CI 1-of-3 as open; M022 plan triage records keyed-retry → Fix and deferred items

### Keyed retry (Phase 72, S02)

- [ ] **KEY-RETRY-01**: after Retract of a keyed Assert, identical keyed Assert remints a live support; short-circuit only while live; bench ≤10%
- [ ] **ADM-PIN-01**: `admission_status_at` via admission EventId pin; delivery_key = policy (characterization)

### Harness isolate (Phase 73, S03)

- [ ] **SKIP-EMIT-01**: `Context.skip_emit` skips emit_log/emit_tenant writes; FSM ci unit test uses it
- [ ] **OVERLAP-01**: jobs>1 + observe_cargo without overlap_with → HIGH

### Close (Phase 74, S04)

- [ ] **CLOSE-01**: M022 closed; Active None; audit note; honeycomb Proposed

## Out of Scope

| Feature | Reason |
|---------|--------|
| Rollover seal/manifest heal redesign | Own high-risk slice |
| Cypher / HNSW / legal pack / SPIKE-LAW | Thaw gates |
| Pi Durable harness task resume | Separate plane |
