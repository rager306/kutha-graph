# Requirements: Kutha

**Defined:** 2026-10-31
**Core Value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.

## v1.70 Requirements

- [x] **C1B-01**: A pack install can pin a content-addressed manifest that declares read-only operator names; admission rejects missing, malformed, identity-mismatched, reserved, duplicate, or cross-pack-colliding declarations without executing pack code.
- [x] **C1B-02**: A droppable operator registry is derived deterministically from the log, reports pack lifecycle state, and survives persist/open, fork, and checkpoint without changing graph facts or fold fingerprint.

## Explicit non-goals

- No operator execution, dynamic code loading, dictionary facets, lease rebuild, WASM, marketplace, schema SMO, legal pack, or ADR Accepted status.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| C1B-01 | Phase 261 | Complete |
| C1B-02 | Phase 262 | Complete |

**Coverage:**
- v1.70 requirements: 2 total
- Mapped to phases: 2
- Unmapped: 0
