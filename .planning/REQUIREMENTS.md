# Requirements: Kutha

**Defined:** 2026-10-02
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** (none live — v0.07 shipped)

## Future Requirements (not leased)

### M015 — executable rules and legal golden fixture

- **RULEX-01**: Canonical rule AST hashed into `rule_version`; evaluator over CSR lease; policy evaluation; golden fixture verdict (F9, F16)

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE explicitly thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| Promoting honeycomb to Accepted | L_map stays Proposed |
| Claiming five legal clocks are solved | ADR-013 open question |

## Traceability

v0.07 (CLK/ANK/FIX) shipped — see `.planning/milestones/v0.07-REQUIREMENTS.md`.
