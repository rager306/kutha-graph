# Requirements: Kutha

**Defined:** 2026-10-01
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.05 Dictionaries as facts (harness lease M012)

## v0.05 Requirements

Closing leased product gaps F4/F5 from `docs/architecture/semantic-gap-review.md` under the M012 thawed subset. Cells stay Proposed. Not ADR-050 six dictionaries. Not M002.

### Relation allowlist as facts (F5 ambient → log)

- [ ] **ALL-01**: Relation allowlist entries are appendable versioned facts on the event log
- [ ] **ALL-02**: `Runtime::admit` (or equivalent) consults the fold cut for allowed relations, not tip YAML/env alone
- [ ] **ALL-03**: Unknown relation still fails closed; named FF6 oracle remains green

### Rule registry (F4 free-string → hash)

- [ ] **RULE-01**: A rule registry stores definitions; `rule_version` on Behavior equals the definition hash
- [ ] **RULE-02**: Unknown or mismatched `rule_version` fails closed on emit/eligibility
- [ ] **RULE-03**: Free-string `rule_version` is no longer accepted on the leased path (or is rejected unless it matches a registry hash)

### Admission and policy meta-facts (F5)

- [ ] **ADM-01**: Admission status is recorded as a bi-temporal meta-fact (or equivalent log record) queryable AS OF a cut
- [ ] **ADM-02**: Policy version is pinned in the log and cited by admission
- [ ] **ADM-03**: `check_admission` (or successor) is invoked on the product path for the leased fixture, not only from tests

### Action record (F5 / ADR-051)

- [ ] **ACT-01**: A thin Action record (or Op) binds resolved arguments to the admission decision and policy version
- [ ] **ACT-02**: Named test shows Action + admission AS OF a prior cut after a later policy change

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

Filled by roadmapper.

| Requirement | Phase | Status |
|-------------|-------|--------|
| ALL-01 | — | Pending |
| ALL-02 | — | Pending |
| ALL-03 | — | Pending |
| RULE-01 | — | Pending |
| RULE-02 | — | Pending |
| RULE-03 | — | Pending |
| ADM-01 | — | Pending |
| ADM-02 | — | Pending |
| ADM-03 | — | Pending |
| ACT-01 | — | Pending |
| ACT-02 | — | Pending |
| DER-01 | — | Pending |
| DER-02 | — | Pending |
| DER-03 | — | Pending |

**Coverage:**
- v0.05 requirements: 14 total
- Mapped to phases: 0 (pending roadmap)

---
*Requirements defined: 2026-10-01*
*Last updated: 2026-10-01 after M012 lease / gsd-new-milestone --auto*
