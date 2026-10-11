# Requirements: Kutha

**Milestones:** v0.48 / harness M054 active. Source: `ce-review-regression-map.md` + whole-repo findings. Lease: `.kutha/STATE.md`.

## v0.48 Requirements (M054)

### S01 — Horn soundness (Phase 117) — complete

- [x] **HORN-04**: Correct / CorrectInterval re-fire remains derivation_eligible when replacement premise + rule pin live
- [x] **HORN-03**: multi-premise witness; retract non-trigger premise drops derived
- [x] **HORN-02**: derived Behavior VT = intersection of premises (+ rule window)
- [x] **HORN-05**: dedup by covered VT / proof identity; uncovered residuals emit
- [x] **HORN-DUP**: Correct cascade does not duplicate derived Retract EventId

### S02 — Persist durability (Phase 118) — complete

- [x] **PERS-09**: torn empty active heal before append
- [x] **PERS-10**: generation-unique segment names before manifest swap
- [ ] **PERS-06**: blocked until human decision-gate lifts

### S03 — Governor honesty (Phase 119) — complete

- [x] **GOV-14**: artifact-budget PLAN gate non-vacuous
- [x] **GOV-22**: matrix schema rejection
- [x] **GOV-23**: matrix row typing rejection
- [x] **GOV-34**: pool at-most-once on submit failure

### S04 — Scaling and bench (Phase 120) — leased

- [ ] **SCALE-15**: RSS sample measured before read
- [ ] **SCALE-25**: timed as_of result validated
- [ ] **SCALE-16**: index derived children before retracts
- [ ] **SCALE-17**: Horn premise relation-keyed probe
- [ ] **SCALE-19**: incremental append from undisclosed suffix
- [ ] **SCALE-27**: rule/policy pin index

### S05

See ROADMAP; filled when leased.

## Out of Scope

| Feature | Reason |
|---------|--------|
| G3 #6 poison without decision | Human decision-gate |
| Cypher / HNSW | Thaw map |
| Honeycomb Accepted | Proposed only |
