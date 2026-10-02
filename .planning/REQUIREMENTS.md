# Requirements: Kutha

**Defined:** 2026-10-02
**Milestone:** (none live — v0.09 shipped)

## Future Requirements (not leased)

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes
- **WRITE-01**: `intern` admit path; WAL vs JSONL open preference

## Out of Scope

| Feature | Reason |
|---------|--------|
| Promoting honeycomb to Accepted | L_map stays Proposed |
| Five legal clocks as core axes | FIXG-01 fail-closed 2026-10-02; ADR-013 stays Proposed |

## Traceability

v0.09 (TH/RULE/FIXG) shipped — see `.planning/milestones/v0.09-REQUIREMENTS.md`.
