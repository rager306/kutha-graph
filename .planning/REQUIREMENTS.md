# Requirements: Kutha

**Milestones:** v0.48 / M054 and v0.49 / M055 shipped 2026-10-11. Source: `ce-review-regression-map.md` + whole-repo findings. Lease: `.kutha/STATE.md`.

## v0.49 Requirements (M055)

### S01 — Append tip rollback (Phase 122) — complete

- [x] **PERS-06**: post-write Other restores durable memory tip; next append rewinds crash-shaped suffix (no duplicate EventIds); poison only after committed rollover

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
- [x] **PERS-06**: closed under M055 / Phase 122

### S03 — Governor honesty (Phase 119) — complete

- [x] **GOV-14**: artifact-budget PLAN gate non-vacuous
- [x] **GOV-22**: matrix schema rejection
- [x] **GOV-23**: matrix row typing rejection
- [x] **GOV-34**: pool at-most-once on submit failure

### S04 — Scaling and bench (Phase 120) — complete

- [x] **SCALE-15**: RSS sample measured before read
- [x] **SCALE-25**: timed as_of result validated
- [x] **SCALE-16**: index derived children before retracts
- [x] **SCALE-17**: Horn premise relation-keyed probe
- [x] **SCALE-19**: incremental append from undisclosed suffix
- [x] **SCALE-27**: rule/policy pin index

### S05 — ID and structure (Phase 121) — complete

- [x] **ID-24**: derived EventId stays in documented UUID contract
- [x] **ID-20**: shrink corpus uses kutha_common::sha256_hex
- [x] **ID-12**: SEM helper module split
- [x] **ID-21**: kinds.py policy family split

## Out of Scope

| Feature | Reason |
|---------|--------|
| Cypher / HNSW | Thaw map |
| Honeycomb Accepted | Proposed only |
