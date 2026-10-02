# Requirements: Kutha

**Defined:** 2026-10-02
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.08 Known-on honesty and M014 residuals (harness lease M015)

## v0.08 Requirements

Audit follow-up after M014 close. Cells stay Proposed. Not F9/F16 rules (M016). Not M002. Run: `docs/process/gsd-auto-contract.md`.

### Known-on semantics (H1)

- [ ] **KNOWN-01**: `as_known_on(D)` returns the product picture at the resolved TT **without** forcing `vt = u64::MAX` in a way that drops facts whose `valid_to` is finite (closed VT intervals remain visible when they were live at the knowledge cut)
- [ ] **KNOWN-02**: A named RED→GREEN oracle uses an FF5-shaped statute (`valid_to: Some(..)`) plus anchors; before the fix the oracle fails; after it passes; FF5 `as_of` stays green

### Anchor admission (H2)

- [ ] **ANK-ADM-01**: Emitting a `TtAnchor` with a wall earlier than a prior anchor, or a duplicate wall, fails closed at admit (or documented fail-closed policy with a named test)
- [ ] **ANK-ADM-02**: Greatest-anchor-≤D still holds for monotone unique walls; persist/open rebuild agrees

### Living docs and governor (DOC / GOV)

- [ ] **DOC-01**: Living docs no longer claim M014 future / F13 fully open / M013 LEASED; PROJECT idle matches STATE; semantic-gap and ADR-013 Evidence reflect M014 ship + known-on residual
- [ ] **GOV-01**: `true_on_d_differs_from_known_on_d` is in `observe_cargo.required`; honeycomb ADR-013 evidence lists it; when live `.planning/phases/` is non-empty, artifact-budget `min_hits` is restored to 1 for PLAN/SUMMARY/VERIFICATION/CONTEXT globs

## Future Requirements (not v0.08)

### M016 — executable rules and legal golden fixture

- **RULEX-01**: Canonical rule AST hashed into `rule_version`; evaluator over CSR lease; policy evaluation; golden fixture verdict (F9, F16)

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes
- **WRITE-01** (triage): `intern` admit path; WAL vs JSONL open preference — backlog note only in v0.08 SUMMARY

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| F9/F16 rule evaluator / golden fixture | M016 |
| Five legal clock newtypes | ADR-013 open Q / M016 verdict |
| Full `intern` through admit + WAL/JSONL SoT policy | triage → M002/WRITE backlog |
| Cypher / HNSW / Rocks | Freeze |
| Promoting honeycomb to Accepted | L_map stays Proposed |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| KNOWN-01, KNOWN-02, ANK-ADM-01, ANK-ADM-02 | Phase 34 | Pending |
| DOC-01, GOV-01 | Phase 35 | Pending |

**Coverage:** v0.08 requirements: 6 total; mapped: 6; unmapped: 0.
