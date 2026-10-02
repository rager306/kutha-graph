# Requirements: Kutha

**Defined:** 2026-10-02
**Milestone:** (none live — v0.08 shipped)

## Future Requirements (not leased)

### M016 — executable rules and legal golden fixture

- **RULEX-01**: Canonical rule AST hashed into `rule_version`; evaluator over CSR lease; policy evaluation; golden fixture verdict (F9, F16)

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes
- **WRITE-01**: `intern` admit path; WAL vs JSONL open preference

## Out of Scope

| Feature | Reason |
|---------|--------|
| Promoting honeycomb to Accepted | L_map stays Proposed |
| Five legal clocks claimed solved | ADR-013 open question |

## Traceability

v0.08 (KNOWN/ANK-ADM/DOC/GOV) shipped — see `.planning/milestones/v0.08-REQUIREMENTS.md`.
