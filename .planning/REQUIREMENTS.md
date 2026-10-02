# Requirements: Kutha

**Defined:** 2026-10-02
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.09 Test hardening + executable rules (harness lease M016)

## v0.09 Requirements

Closes test-audit T2–T6 and F9/F16. Cells stay Proposed. Not M002. Run: `docs/process/gsd-auto-contract.md`.

### Test hardening (T2–T6)

- [x] **TH-01**: trybuild (or equivalent) proves `TransactionTime`/`WallTime`→`ValidTime` mix is E0308 — not a syntax-error `compile_fail`
- [x] **TH-02**: Named test that swapping `tt`/`vt` on `JustificationCite` (or Op cite path) fails an oracle; fields stay distinguishable at the public cut
- [x] **TH-03**: `as_known_on` agrees across Eager vs Lazy and survives `fork_at` + persist/open
- [x] **TH-04**: Mid-cut snapshot + non-empty tail (Assert + TtAnchor) recovers `as_known_on` equal to full replay; fingerprint or replay includes anchors
- [x] **TH-05**: `GraphFold::replay` applies `TtAnchor` so `replayed.as_known_on ==` live fold

### Rule AST and hash (F9)

- [x] **RULE-01**: Bounded positive rule language with canonical encoding; `rule_version` is its hash; unknown/non-canonical bodies fail closed

### Evaluator (F9)

- [ ] **RULE-02**: Multi-hop / n-ary / `inverse_knows` expressed as rules; premise retract maintains derived supports; hard-coded cascade deleted or reduced to rule eval

### Policy + execution replay (F16)

- [ ] **RULE-03**: Admission runs pinned policy body; changing policy changes a decision
- [ ] **RULE-04**: Replay re-derives derived facts and fails on any it cannot reproduce

### Legal golden fixture (F16 / ADR-013 Q)

- [ ] **FIXG-01**: Real-text (or law-nexus-shaped) transitional rule + `as_known_on` query is green, or fail-closed verdict is dated in ADR-013 Evidence before M002

## Future Requirements (not v0.09)

### M002 — log durability

- **ROCK-01** / **WRITE-01**: segmented log; `intern` admit; WAL vs JSONL

## Out of Scope

| Feature | Reason |
|---------|--------|
| Five clock newtypes as product claim | ADR-013 open Q answered only by FIXG-01 verdict |
| Cypher / HNSW / Rocks / ADR-050 six dicts | Freeze |
| Honeycomb Accepted | L_map stays Proposed |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TH-01..TH-05 | Phase 36 | Complete (verified) |
| RULE-01 | Phase 37 | Complete (verified) |
| RULE-02 | Phase 38 | Pending |
| RULE-03, RULE-04 | Phase 39 | Pending |
| FIXG-01 | Phase 40 | Pending |

**Coverage:** v0.09 requirements: 10 total; mapped: 10; unmapped: 0.
