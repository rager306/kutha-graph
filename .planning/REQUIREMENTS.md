# Requirements: Kutha

**Milestone:** v1.07 selftest mutation static (M113 closed).

## v1.07

- [x] **AUD-18**: `get_checks` LoadError when declared selftest `replace_regex` keeps `pattern` as a substring of `with`

## v1.06

- [x] **AUD-17**: selftest `replace_regex` rejects `with` that still contains `pattern` as a substring

## v1.05

- [x] **AUD-15**: governor `hot-path-lease-honesty` pins `tt_by_ingest` + live-only rebuild needles
- [x] **AUD-16**: `unindex_behavior` does not full-scan when reverse pointers are missing

## v1.04

- [x] **AUD-14**: `rebuild_log_indexes` puts only live Behavior ids into `derived_by_cause`

## v1.03

- [x] **AUD-13**: warm `known_at` examines only facts with `ingested_at <=` cut (`tt_by_ingest`)

## v1.02

- [x] **AUD-12**: unindex of one Behavior removes it from the live set in one visit

## v1.01

- [x] **AUD-11**: `unindex_behavior` does not walk `live_behaviors` for an event that is not a behavior

## v1.00

- [x] **AUD-10**: `finish_open` restores snapshot lease warmth via `set_index_mode`, not only the mode flag

## v0.99

v0.99 residual correctness closed as M105.

- [x] **AUD-01**: `live_policy_pin_at` walks `policy_by_ingest` newest-first and stops at the first live row
- [x] **AUD-02**: `unindex_behavior` drops one premise-rel bucket per indexed relation (`premise_rels_of_behavior`)
- [x] **AUD-03**: Op-wire rewrite omits a modern `Define` whose name is already interned
- [x] **AUD-04**: Horn fire collects every premise combination, not the first DFS hit
- [x] **AUD-05**: Behavior replay requires the premise conjunction to cover the head VT
- [x] **AUD-06**: derivation eligibility of a Behavior also requires `horn_behavior_reproducible`
- [x] **AUD-07**: `finish_open` installs the fold returned by `replay_check`
- [x] **AUD-09**: Correct / CorrectInterval re-fire walks every live residual
- [ ] **AUD-08**: Lazy fold index still scans until `ensure_hot_maps` (kept; thin-write contract)

## Prior

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
