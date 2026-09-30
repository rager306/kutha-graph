# Requirements: Kutha

**Defined:** 2026-09-30
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.04 Single-log SoT + stable references (harness lease M012a)

## v0.04 Requirements

Closing leased product gaps F1–F3, F6 (verify/atomic/Define), F7, F8 from `docs/architecture/semantic-gap-review.md`. Cells stay Proposed. Not M012. Not M002.

### Log-native SoT (F1)

- [x] **LOG-01**: Quantum outcomes are appendable log records (or equivalent Op); discarding outcome sidecars does not change reconstructible meaning
- [x] **LOG-02**: Justifications and resume/continuation evidence are log records; `justifications.jsonl` / outcome files are leases only
- [x] **LOG-03**: `provenance_fingerprint` (or successor) mixes those log-native outcome/justification bytes, not only Behavior rows

### Stable references (F2)

- [ ] **REF-01**: Retract and Correct target a stable EventId (or proposition id), not fold-local `fact_seq`
- [ ] **REF-02**: Justification cites use the same stable identity across fork / rebuild
- [ ] **REF-03**: A rebuilt fold that renumbers local seqs can still apply the same retract/cite payloads

### Idempotent ingest (F3)

- [ ] **ING-01**: Re-delivering an identical Assert with the same delivery key does not mint a second independent support
- [ ] **ING-02**: Claim/proposition identity is distinct from a support slot (documented + exercised by named tests)
- [ ] **ING-03**: Conflict reporting does not require caller-only free polarity strings for the leased fixture path (or documents the remaining gap explicitly as out of M012a)

### Verify and persist (F6 subset)

- [ ] **DUR-01**: `open` verifies snapshot identity against the log (replay_check or equivalent) and rejects a tampered snapshot
- [ ] **DUR-02**: `persist` writes the log file atomically (rename-into-place or equivalent); crash mid-persist does not leave a truncated SoT as success
- [ ] **DUR-03**: `Define` event ids are stable across persist/open cycles for the same term set

### Time scale (F7)

- [ ] **TIME-01**: VT and TT declare an explicit scale (type, constant, or documented contract) used by fixtures
- [ ] **TIME-02**: Named tests document how fixture years/cuts map to that scale (TT↔wall optional; full calendar map not required)

### Hot reads (F8)

- [ ] **HOT-01**: `claim_supported_at` / `as_of` at the leased N do not require a full linear scan of all facts (fold-internal index or equivalent)
- [ ] **HOT-02**: CSR lease path remains a droppable lease; indexes are not a second SoT
- [ ] **HOT-03**: Materializer vs `from_fold` relationship is either wired on the hot path or explicitly deferred with a named test/guard documenting the spike limit

## Future Requirements (not v0.04)

### M012 — dictionaries as facts (F4/F5)

- **DICT-01**: Rule registry with `rule_version` = definition hash; n-ary / multi-hop derivation eligibility
- **DICT-02**: Admission status and policy version as bi-temporal facts; relation allowlist entries as log facts
- **DICT-03**: Action entity / record binding resolved arguments to admission decision

### M002 — log durability protocol

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes behind that protocol

## Out of Scope

| Feature | Reason |
|---------|--------|
| M012 dictionaries-as-facts / rule registry / admission meta-facts | Separate lease; F4/F5 until STATE names M012 |
| RocksDB / segmented WAL durability protocol | M002; freeze holds |
| Cypher / GPML / HNSW / ADR-050 six dictionaries | Freeze until explicit lease |
| Legal corpus pack / ADR-090 full ontology | Explicit non-goal until leased |
| Promoting honeycomb cells to Accepted | L_map stays Proposed |
| ADR-061 full GED-class diff API | Open question; fork exists; not M012a success |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| LOG-01 | Phase 12 | Complete |
| LOG-02 | Phase 12 | Complete |
| LOG-03 | Phase 12 | Complete |
| REF-01 | Phase 13 | Pending |
| REF-02 | Phase 13 | Pending |
| REF-03 | Phase 13 | Pending |
| ING-01 | Phase 14 | Pending |
| ING-02 | Phase 14 | Pending |
| ING-03 | Phase 14 | Pending |
| DUR-01 | Phase 15 | Pending |
| DUR-02 | Phase 15 | Pending |
| DUR-03 | Phase 15 | Pending |
| TIME-01 | Phase 15 | Pending |
| TIME-02 | Phase 15 | Pending |
| HOT-01 | Phase 16 | Pending |
| HOT-02 | Phase 16 | Pending |
| HOT-03 | Phase 16 | Pending |

**Coverage:**
- v0.04 requirements: 17 total
- Mapped to phases: 17/17
- Unmapped: 0
- Duplicates: 0

**Phase map (harness S01–S06):**
- Phase 12 (S01): LOG-01, LOG-02, LOG-03
- Phase 13 (S02): REF-01, REF-02, REF-03
- Phase 14 (S03): ING-01, ING-02, ING-03
- Phase 15 (S04 + S05): DUR-01, DUR-02, DUR-03, TIME-01, TIME-02
- Phase 16 (S06): HOT-01, HOT-02, HOT-03

---
*Requirements defined: 2026-09-30*
*Last updated: 2026-09-30 after v0.04 roadmap (Phases 12–16)*
