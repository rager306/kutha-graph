# Requirements: Kutha

**Defined:** 2026-10-31
**Core Value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.

## v1.73 Requirements

- [ ] **RPL-01**: `replay_check` and store open reject inconsistent pack lifecycle ordering and manifest identity (missing blob, digest mismatch, identity mismatch) instead of deriving a partial lifecycle or registry.
- [ ] **RPL-02**: Replay rejects inconsistent enrichment proposals (unknown/retired pack, wrong version, empty pins, missing source, missing or mismatched payload, duplicate cache key) while valid stores open unchanged.

## Explicit non-goals

- No new Op, no new feature, no operator execution, no legal pack, no dictionary facets, no HNSW, no WASM, no ADR Accepted status.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| RPL-01 | Phase 266 | Pending |
| RPL-02 | Phase 267 | Pending |

**Coverage:**
- v1.73 requirements: 2 total
- Mapped to phases: 2
- Unmapped: 0
