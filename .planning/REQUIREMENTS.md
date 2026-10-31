# Requirements: Kutha

**Defined:** 2026-10-31
**Core Value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.

## v1.76 Requirements

- [x] **DIG-01**: The manifest and enrichment payload digest checks have direct falsifiers — a blob addressed by a policy/rule-body hash that passes the generic `StoreBlob` gate but fails raw SHA-256 must be rejected, and removing the raw check must fail the test.
- [x] **DIG-02**: Manifest operator collision detection uses an operator→pack index, so replay is linear in packs and operators, while retired packs still release names and admission/replay stay in agreement.

## Explicit non-goals

- No new Op, no feature, no operator execution, no legal pack, no grants/capabilities/WASM, no ADR Accepted status.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| DIG-01 | Phase 270 | Complete |
| DIG-02 | Phase 271 | Complete |

**Coverage:**
- v1.76 requirements: 2 total
- Mapped to phases: 2
- Unmapped: 0
