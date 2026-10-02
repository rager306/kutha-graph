# Requirements: Kutha

**Defined:** 2026-10-02
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.07 Typed clocks and as-known-on (harness lease M014)

## v0.07 Requirements

Closes F13 from `docs/architecture/semantic-gap-review.md`. Cells stay Proposed. Not rules (M015), not M002. Execution rules: `docs/process/gsd-auto-contract.md`.

### Clock newtypes (F13)

- [x] **CLK-01**: Public temporal API uses distinct newtypes for valid time (`Vt`), transaction time (`Tt`), and wall time (not bare `u64` aliases that compile interchangeably)
- [x] **CLK-02**: A `compile_fail` doctest or trybuild proves a silent mix of those types is rejected
- [x] **CLK-03**: Named oracles (FF5, FF6, M010–M013) stay green; only mechanical renames, recorded in the phase SUMMARY

### Anchors and as-known-on (F13)

- [x] **ANK-01**: `tt_anchor` facts on the event log map transaction time to wall time
- [x] **ANK-02**: `as_known_on(date)` resolves through the greatest anchor at or before the date
- [x] **ANK-03**: A named test separates “true on D” (`as_of` / valid-time window) from “known on D” (`as_known_on`); FF5 stays green

### Fixture migration

- [ ] **FIX-01**: FF5 and M011/M012/M013 fixtures use the explicit clock types
- [ ] **FIX-02**: Oracle assertions are unchanged (rename-only); SUMMARY carries an old-to-new table

## Future Requirements (not v0.07)

### M015 — executable rules and legal golden fixture

- **RULEX-01**: Canonical rule AST hashed into `rule_version`; evaluator over CSR lease; policy evaluation; golden fixture verdict (F9, F16)

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE explicitly thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| Retract-by-EventId facade widening | Separate follow-up; not F13 |
| Rule evaluation / derive auto-register | M015 |
| WAL / snapshot durability protocol | M002 |
| ADR-050 six dictionaries | Freeze |
| Promoting honeycomb to Accepted | L_map stays Proposed |
| Claiming five legal clocks are solved | ADR-013 open question; two axes + anchors only |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLK-01, CLK-02, CLK-03 | Phase 31 | Complete |
| ANK-01, ANK-02, ANK-03 | Phase 32 | Complete |
| FIX-01, FIX-02 | Phase 33 | Pending |

**Coverage:** v0.07 requirements: 8 total; mapped to phases: 8; unmapped: 0.
