# Requirements: Kutha

**Milestones:** v0.46 / harness M052 active. Source: `.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/` + `advisor-correction.md`. Lease: `.kutha/STATE.md`.

## v0.46 Requirements (M052)

### S01 — Triage correction (Phase 112) — complete

- [x] **TRIAGE-01**: Advisor correction demotes whole-repo #1 from P1/fix-first
- [x] **TRIAGE-02**: STATE / ROADMAP / gsd-auto-contract cite corrected fix-first order starting at #8

### S02 — Cascade atomicity (Phase 113) — complete

- [x] **CASC-08-01**: Later-hop error leaves fold + log + outcomes unchanged
- [x] **CASC-08-02**: Successful multi-hop cascade still commits + QuantumOutcome

### S03 — Lock before heal (Phase 114)

- [ ] **LOCK-07-01**: `Persister::attach` acquires writer lock before any heal/write path
- [ ] **LOCK-07-02**: Concurrent second attach denied without mutating store bytes

### S04 — Snapshot-free open replay (Phase 115)

- [ ] **OPEN-11-01**: Define/terms open run replay validation before `finish_open`
- [ ] **OPEN-11-02**: Tampered log without snapshot fails open

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Op serde TermId remap (#1 migrate) | Deferred residual; not a defect |
| Horn VT/lineage #2–#5 | Next wave |
| EventId UUID #24 (whole-repo) | Later |
| G3 #6 | Human decision-gate |
| Cypher / HNSW | Thaw map |
