# Requirements: Kutha

**Defined:** 2026-09-29
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice at a time — not the honeycomb.
**Milestone:** v0.02 Semantic core close

Derived from `docs/architecture/semantic-contract-validation.md` (observations 1–5 and remaining decisions) and the M011 tail in `.kutha/ROADMAP.md`. No PRDs ingested. Honeycomb ADR-010–093 are **not** requirements. Research skipped by user choice.

## v0.02 Requirements

Each requirement maps to exactly one roadmap phase (one phase per harness slice). A phase executes only while its slice is the Active Slice in `.kutha/STATE.md`.

### Partial correction (M011 S04 — leased)

- [x] **CORR-01**: Developer can apply an explicit interval-patch correction and observe residual versions on both sides of the corrected VT range at the same TT cut (fixture: source `a` keeps VT 2012 and 2021)
- [x] **CORR-02**: Developer can still use whole-version `Correct` and observe unchanged behavior, with no implicit residuals

### Quantum outcome (M011 S05 — blocked until leased)

- [x] **OUT-01**: Developer can tell zero, partial, and full progress apart for budgets 0/1/2 from persisted quantum outcome records
- [x] **OUT-02**: Developer can reopen a log after a crash following a committed prefix and see no terminal-success record; resume is an explicit record, never inferred

### Typed CSR lease (M011 S06 — blocked until leased)

- [ ] **CSR-01**: Developer can query a CSR lease where the same endpoints carry different relations or supports and see labels and support multiplicity preserved
- [ ] **CSR-02**: Developer still gets the untyped neighbor-set path, and the FF5 lease-agrees-with-fold check stays green

### Provenance (M011 S07 — blocked until leased)

- [ ] **PROV-01**: Verification detects a change to only a Behavior `caused_by` reference even when the state fingerprint still matches
- [ ] **PROV-02**: Verification detects a change to only a pinned rule version the same way; execution replay is not required

### End-to-end fixture (M011 S08 — blocked until leased)

- [ ] **FIX-01**: Independent supports and last-support withdrawal hold at named (TT,VT) cuts on the candidate fixture; the conflict variant preserves both sides and reports conflict instead of superseding
- [ ] **FIX-02**: Summary and action records cite exact source revisions and rule version; a changed dependency forces re-evaluation, and a stale cached output cannot renew its own admission
- [ ] **FIX-03**: Incremental maintenance and clean reconstruction agree on values, active supports, and completeness; discarding CSR and snapshots changes no answer

### Gates (every slice)

- [x] **GATE-01**: Each slice ships a named cargo test registered in the governor (FSM observe + check needle), and `uv run kutha-gov ci` stays at 0 HIGH
- [ ] **GATE-02**: A slice's GSD phase executes only while that slice is the Active Slice in `.kutha/STATE.md`; otherwise it is planning-only
- [ ] **GATE-03**: Freeze items stay unstarted and honeycomb cells stay Proposed; any ADR Accepted decision is a separate review at milestone close

## Future Requirements

Deferred to later milestones in `.kutha/ROADMAP.md` "Later milestones": M012 dictionaries as facts (thin ADR-050 subset), M002 Rocks (vacuum, litigation hold), M003 Cypher skin, legal pack MVP, M004 science fixture, pack lifecycle, M005 HNSW, security, packaging and licence.

## Out of Scope

| Feature | Reason |
|---------|--------|
| RocksDB, Cypher/GPML, HNSW | Frozen until explicit M002 lease |
| ADR-050 six dictionaries, ADR-080/081, ADR-100+ | Freeze; thin subset is M012 |
| Legal or science pack | Not started until STATE names it |
| Full provenance polynomials, execution replay | S07 checks references and rule version only |
| Interval-patch beyond the fixture | S04 is scoped to the residual-interval contract |
| Product SemVer, tags, GitHub Releases for the product | Repo policy; GSD overlay tags are a separate user override |

## Traceability

Primary owner is listed below. GATE-01/02/03 still apply to every slice's success criteria (see `.planning/ROADMAP.md`).

| Requirement | Phase | Status |
|-------------|-------|--------|
| CORR-01 | Phase 4 | Complete |
| CORR-02 | Phase 4 | Complete |
| GATE-01 | Phase 4 | Complete |
| OUT-01 | Phase 5 | Complete |
| OUT-02 | Phase 5 | Complete |
| CSR-01 | Phase 6 | Pending |
| CSR-02 | Phase 6 | Pending |
| PROV-01 | Phase 7 | Pending |
| PROV-02 | Phase 7 | Pending |
| FIX-01 | Phase 8 | Pending |
| FIX-02 | Phase 8 | Pending |
| FIX-03 | Phase 8 | Pending |
| GATE-02 | Phase 8 | Pending |
| GATE-03 | Phase 8 | Pending |

**Coverage:**
- v0.02 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0

---
*Requirements defined: 2026-09-29*
*Last updated: 2026-09-29 after v0.02 roadmap*
