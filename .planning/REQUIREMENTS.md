# Requirements: Kutha

**Defined:** 2026-10-01
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.05 Dictionaries as facts (harness lease M012)

## v0.05 Requirements

Closing leased product gaps F4/F5 from `docs/architecture/semantic-gap-review.md` under the M012 thawed subset. Cells stay Proposed. Not ADR-050 six dictionaries. Not M002.

### Relation allowlist as facts (F5 ambient → log)

- [x] **ALL-01**: Relation allowlist entries are appendable versioned facts on the event log
- [x] **ALL-02**: `Runtime::admit` (or equivalent) consults the fold cut for allowed relations, not tip YAML/env alone
- [x] **ALL-03**: Unknown relation still fails closed; named FF6 oracle remains green

### Rule registry (F4 free-string → hash)

- [x] **RULE-01**: A rule registry stores definitions; `rule_version` on Behavior equals the definition hash
- [x] **RULE-02**: Unknown or mismatched `rule_version` fails closed on emit/eligibility
- [x] **RULE-03**: Free-string `rule_version` is no longer accepted on the leased path (or is rejected unless it matches a registry hash)

### Admission and policy meta-facts (F5)

- [x] **ADM-01**: Admission status is recorded as a bi-temporal meta-fact (or equivalent log record) queryable AS OF a cut
- [x] **ADM-02**: Policy version is pinned in the log and cited by admission
- [x] **ADM-03**: `check_admission` (or successor) is invoked on the product path for the leased fixture, not only from tests

### Action record (F5 / ADR-051)

- [x] **ACT-01**: A thin Action record (or Op) binds resolved arguments to the admission decision and policy version
- [x] **ACT-02**: Named test shows Action + admission AS OF a prior cut after a later policy change

### Multi-hop derivation (F4)

- [ ] **DER-01**: Derivation eligibility is not limited to one-hop `caused_by` for the leased fixture
- [ ] **DER-02**: A two-hop (or n-ary) named test is green without claiming full provenance polynomials
- [ ] **DER-03**: Hard-coded `inverse_knows`-only cascade is not the sole follow-on mechanism (or is documented as residual spike with a named guard)

## Future Requirements (not v0.05)

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE explicitly thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| ADR-050 six dictionaries | Freeze; M012 lease names a subset only |
| RocksDB / segmented WAL protocol | M002 |
| Cypher / GPML / HNSW | Freeze |
| Legal corpus pack / full ADR-090 | Explicit non-goal |
| Promoting honeycomb to Accepted | L_map stays Proposed |
| Full provenance polynomials / MATCH compiler | ADR-011 open question; not this milestone |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ALL-01 | Phase 17 | Complete |
| ALL-02 | Phase 17 | Complete |
| ALL-03 | Phase 17 | Complete |
| RULE-01 | Phase 18 | Complete |
| RULE-02 | Phase 18 | Complete |
| RULE-03 | Phase 18 | Complete |
| ADM-01 | Phase 19 | Complete |
| ADM-02 | Phase 19 | Complete |
| ADM-03 | Phase 19 | Complete |
| ACT-01 | Phase 20 | Complete |
| ACT-02 | Phase 20 | Complete |
| DER-01 | Phase 21 | Pending |
| DER-02 | Phase 21 | Pending |
| DER-03 | Phase 21 | Pending |

**Coverage:**
- v0.05 requirements: 14 total
- Mapped to phases: 14/14
- Orphans: none
- Duplicates: none

**Phase map (M012 slices):** Phase 17 = S01 (ALL) · Phase 18 = S02 (RULE) · Phase 19 = S03 (ADM) · Phase 20 = S04 (ACT) · Phase 21 = S05 (DER)

---
*Requirements defined: 2026-10-01*
*Last updated: 2026-10-01 after v0.05 roadmap (Phases 17–21)*
