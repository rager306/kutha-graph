# Requirements: Kutha

**Defined:** 2026-10-31
**Core Value:** Prove legal point-in-time on a named statute-shaped fixture while keeping governor CI honest and advancing one Active Slice at a time.

## v1.77 Requirements

- [ ] **OPS-01**: The runtime keeps an operator index rebuilt from the log on hydrate/open/fork; admission uses it for O(1) collision detection and rejects exactly the collisions replay rejects, with a benchmark showing admission is not super-linear.

## Explicit non-goals

- No new Op, no feature, no operator execution, no legal pack, no grants/capabilities/WASM, no ADR Accepted status.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| OPS-01 | Phase 272 | Pending |

**Coverage:**
- v1.77 requirements: 1 total
- Mapped to phases: 1
- Unmapped: 0
