# Requirements: Kutha

**Defined:** 2026-10-31
**Core Value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.

## v1.71 Requirements

- [ ] **C5-01**: An enrichment proposal is a typed log record pinned to an installed/active pack, source event ids, model/prompt/cache identity, and a content-addressed payload; admission rejects unknown/retired packs, missing sources, missing or mismatched blobs, empty pins, and duplicate identity, and never touches a kernel fact.
- [ ] **C5-02**: A droppable enrichment ledger is derived deterministically from the log, reports pack lifecycle state, and survives persist/open, fork, and checkpoint without changing graph facts or fold fingerprint.

## Explicit non-goals

- No shipped LLM, no model call, no derived lease build, no HNSW, no trust-path write, no dictionary facets, no WASM, no marketplace, no ADR Accepted status.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| C5-01 | Phase 263 | Pending |
| C5-02 | Phase 264 | Pending |

**Coverage:**
- v1.71 requirements: 2 total
- Mapped to phases: 2
- Unmapped: 0
