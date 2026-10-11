# Requirements: Kutha

**Milestones:** v0.48 / harness M054 active. Source: `ce-review-regression-map.md` + whole-repo findings. Lease: `.kutha/STATE.md`.

## v0.48 Requirements (M054)

### S01 — Horn soundness (Phase 117)

- [ ] **HORN-04**: Correct / CorrectInterval re-fire remains derivation_eligible when replacement premise + rule pin live
- [ ] **HORN-03**: multi-premise witness; retract non-trigger premise drops derived
- [ ] **HORN-02**: derived Behavior VT = intersection of premises (+ rule window)
- [ ] **HORN-05**: dedup by covered VT / proof identity; uncovered residuals emit; alternate proof survives

### S02 — Persist durability (Phase 118)

- [ ] **PERS-09**: torn empty active heal before append
- [ ] **PERS-10**: generation-unique segment names before manifest swap
- [ ] **PERS-06**: blocked until human decision-gate lifts

### S03–S05

See ROADMAP slice After-this lines; requirements filled when each slice is leased.

## Out of Scope

| Feature | Reason |
|---------|--------|
| G3 #6 poison without decision | Human decision-gate |
| Cypher / HNSW | Thaw map |
| Honeycomb Accepted | Proposed only |
