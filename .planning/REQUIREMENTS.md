# Requirements: Kutha

**Milestones:** v0.32 / harness M038 shipped 2026-10-10. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.32 Requirements (M038) — complete

### Policy delivery-key namespace (Phase 98, S01)

- [x] **DK-NS-01**: `by_delivery_key` namespaced by relation; policy re-pin after admission idempotent

## v0.31 Requirements (M037) — complete

### Claim-lineage eligibility (Phase 97, S01)

- [x] **NV-CLAIM-01**: Correct/CI of ineligible derived stay Abstain; eligible Correct stays Ok

## v0.30 Requirements (M036) — complete

### VT-aware Horn dedup (Phase 96, S01)

- [x] **DEDUP-01**: disjoint VT windows mint separate derived; overlapping still suppress

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| G3 governor oracles / parked residuals | Next controller lease |
| Product fold / Cypher / HNSW | Thaw map |
