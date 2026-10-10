# Requirements: Kutha

**Milestones:** v0.31 / harness M037 shipped 2026-10-10. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.31 Requirements (M037) — complete

### Claim-lineage eligibility (Phase 97, S01)

- [x] **NV-CLAIM-01**: Correct/CI of ineligible derived stay Abstain; eligible Correct stays Ok

## v0.30 Requirements (M036) — complete

### VT-aware Horn dedup (Phase 96, S01)

- [x] **DEDUP-01**: disjoint VT windows mint separate derived; overlapping still suppress

## v0.29 Requirements (M035) — complete

### Replay dict bind (Phase 95, S01)

- [x] **REPLAY-01**: `replay_with_dict` / `replay_check` bind reserved; bare `replay` stays dictionary-blind

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Remaining G2 (#29) | Next controller lease |
| Product fold / Cypher / HNSW | Thaw map |
