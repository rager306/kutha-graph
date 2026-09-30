# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01, v0.02 and v0.03 are shipped. M011 semantic core is closed. **v0.04** leases M012a product work: log-native SoT, stable references, idempotent ingest, verify-on-open / atomic persist / stable Define, declared time scale, and fold-internal hot indexes. Honeycomb stays Proposed. Not M012 dictionaries-as-facts. Not M002 Rocks. Freeze holds until `.kutha/STATE.md` names otherwise.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- 🚧 **v0.04 Single-log SoT + stable references** — Phases 12–16 (planning) — requirements: [REQUIREMENTS.md](./REQUIREMENTS.md)

## Phases

<details>
<summary>✅ v0.01 GSD foundation (Phases 1–3) — SHIPPED 2026-09-29</summary>

- [x] Phase 1: Legal PIT fitness (3/3 plans) — completed 2026-09-29
- [x] Phase 2: Honest harness and freeze (3/3 plans) — completed 2026-09-29
- [x] Phase 3: Lease-gated next slice (3/3 plans) — completed 2026-09-29

Full detail: [milestones/v0.01-ROADMAP.md](./milestones/v0.01-ROADMAP.md) · requirements: [milestones/v0.01-REQUIREMENTS.md](./milestones/v0.01-REQUIREMENTS.md) · audit: [milestones/v0.01-MILESTONE-AUDIT.md](./milestones/v0.01-MILESTONE-AUDIT.md)

</details>

<details>
<summary>✅ v0.02 Semantic core close (Phases 4–8) — SHIPPED 2026-09-30</summary>

- [x] Phase 4: Partial correction with residual intervals (3/3 plans) — completed 2026-09-29
- [x] Phase 5: Persisted quantum outcome (2/2 plans) — completed 2026-09-30
- [x] Phase 6: Typed CSR lease (2/2 plans) — completed 2026-09-30
- [x] Phase 7: Provenance and rule-version check (2/2 plans) — completed 2026-09-30
- [x] Phase 8: End-to-end candidate fixture (2/2 plans) — completed 2026-09-30

Full detail: [milestones/v0.02-ROADMAP.md](./milestones/v0.02-ROADMAP.md) · requirements: [milestones/v0.02-REQUIREMENTS.md](./milestones/v0.02-REQUIREMENTS.md) · phases: [milestones/v0.02-phases/](./milestones/v0.02-phases/)

</details>

<details>
<summary>✅ v0.03 Lean context + semantic governor (Phases 9–11) — SHIPPED 2026-09-30</summary>

- [x] Phase 9: H5 lease + lean agent context (2/2 plans) — completed 2026-09-30
- [x] Phase 10: ADR and roadmap correction (2/2 plans) — completed 2026-09-30
- [x] Phase 11: Semantic governor (4/4 plans) — completed 2026-09-30

Full detail: [milestones/v0.03-ROADMAP.md](./milestones/v0.03-ROADMAP.md) · requirements: [milestones/v0.03-REQUIREMENTS.md](./milestones/v0.03-REQUIREMENTS.md) · phases: [milestones/v0.03-phases/](./milestones/v0.03-phases/)

</details>

### 🚧 v0.04 Single-log SoT + stable references (Phases 12–16)

**Milestone goal:** The event log is the only durable SoT for outcomes, justifications, and resume; Retract/Correct/cites use stable EventId or proposition ids; identical re-delivery does not mint a second support; `open` verifies snapshot identity; persist is atomic; `Define` ids survive re-persist; VT/TT declare a scale; hot `as_of` / `claim_supported_at` use fold-internal indexes.

**Boundary:** Product crates allowed under harness lease M012a. No M012 dictionaries-as-facts / rule registry / admission meta-facts. No Rocks/Cypher/HNSW/ADR-050 six dictionaries/legal pack. Honeycomb stays Proposed. Durability *protocol* (segmented WAL, hash chain, Rocks behind it) waits for M002. Do not overwrite `.kutha/STATE.md` from GSD memory.

**Harness map:** Phase 12 = S01; Phase 13 = S02; Phase 14 = S03; Phase 15 = S04 + S05; Phase 16 = S06. Execute 12 → 13 → 14 → 15 → 16. Each execute wave needs the matching Active Slice leased in `.kutha/STATE.md` (GATE-02). Named cargo tests register in the governor (GATE-01). Freeze + Proposed map (GATE-03).

**Phase numbering:** Integer phases continue from v0.03 (Phase 12 follows Phase 11).

- [x] **Phase 12: Log-native SoT** - Outcomes, justifications, and resume are log records; sidecars are droppable leases (M012a S01) (completed 2026-10-01)
- [x] **Phase 13: Stable references** - Retract, Correct, and justification cites target EventId / proposition id, not fold-local `fact_seq` (M012a S02) (completed 2026-10-01)
- [x] **Phase 14: Idempotent ingest** - Same delivery key does not mint a second support; claim ≠ support slot (M012a S03) (completed 2026-10-01)
- [ ] **Phase 15: Verify, persist, and time scale** - Verify-on-open, atomic log persist, stable Define ids, declared VT/TT scale (M012a S04 + S05)
- [ ] **Phase 16: Fold-internal hot indexes** - Hot `as_of` / `claim_supported_at` skip full fact scans; CSR stays a lease (M012a S06)

## Phase Details

### Phase 12: Log-native SoT

**Goal**: A developer can discard outcome and justification sidecar files after persist and still reconstruct quantum outcomes, justifications, and resume from the event log; provenance mixes those log-native bytes
**Depends on**: Phase 11 (v0.03 shipped); harness S01 `depends:[]`
**Harness slice**: M012a S01
**Plane**: product crates (`kutha-common`, `kutha-runtime`) + governor registration
**Lease**: execute only while Active Slice is **S01** in `.kutha/STATE.md` (harness lease is S01)
**Requirements**: LOG-01, LOG-02, LOG-03
**Success Criteria** (what must be TRUE):
  1. Quantum outcomes are appendable log records (or equivalent `Op`); discarding `quantum_outcomes.jsonl` after persist does not change reconstructible outcome meaning (LOG-01)
  2. Justifications and resume/continuation evidence are log records; `justifications.jsonl` / outcome sidecar files are droppable leases, not a second SoT (LOG-02)
  3. `provenance_fingerprint` (or successor) mixes those log-native outcome/justification bytes, not only in-log Behavior rows (LOG-03)
  4. Named cargo tests for the three LOG oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; FF5/FF6 stay green; honeycomb stays Proposed; M012 dictionaries-as-facts and M002 Rocks stay unstarted

**Plans**: 3/3 plans complete
Plans:
- [x] 12-01-PLAN.md — LOG-01 tracer: quantum outcomes as log records; discard outcomes sidecar reconstructs disposition
- [x] 12-02-PLAN.md — LOG-02: justifications and resume as log records; sidecars are leases
- [x] 12-03-PLAN.md — LOG-03: provenance mixes log-native bytes; governor observes the three LOG oracles

### Phase 13: Stable references

**Goal**: A developer can retract, interval-patch, and cite justifications by a stable EventId or proposition id that still names the same target after fork or a rebuilt fold that renumbers local seqs
**Depends on**: Phase 12 (harness S02 `depends:[S01]`)
**Harness slice**: M012a S02
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S02** in `.kutha/STATE.md`
**Requirements**: REF-01, REF-02, REF-03
**Success Criteria** (what must be TRUE):
  1. Retract and Correct target a stable EventId (or proposition id), not fold-local `fact_seq` (REF-01)
  2. Justification cites use that same stable identity across fork / rebuild (REF-02)
  3. A rebuilt fold that renumbers local seqs still applies the same retract/cite payloads (REF-03)
  4. Named cargo tests for the three REF oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; freeze and Proposed map hold

**Plans**: 3/3 plans complete
Plans:
- [x] 13-01-PLAN.md — REF-01 tracer: Retract/Correct/CorrectInterval target minting EventId
- [x] 13-02-PLAN.md — REF-02: justification cites and admission use EventId across fork
- [x] 13-03-PLAN.md — REF-03 rebuilt-fold payloads; governor observes REF-01..03

### Phase 14: Idempotent ingest

**Goal**: A developer can re-deliver an identical Assert under the same delivery key without minting a second independent support, and can tell a claim/proposition apart from a support slot
**Depends on**: Phase 13 (harness S03 `depends:[S02]`)
**Harness slice**: M012a S03
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S03** in `.kutha/STATE.md`
**Requirements**: ING-01, ING-02, ING-03
**Success Criteria** (what must be TRUE):
  1. Re-delivering an identical Assert with the same delivery key does not mint a second independent support (ING-01)
  2. Claim/proposition identity is distinct from a support slot — documented and exercised by named tests (ING-02)
  3. Conflict reporting on the leased fixture path does not require caller-only free polarity strings, **or** the remaining gap is documented explicitly as out of M012a (ING-03)
  4. Named cargo tests for the ING oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; freeze and Proposed map hold

**Plans**: 3/3 plans complete
Plans:
- [x] 14-01-PLAN.md — ING-01 tracer: keyed identical Assert retry does not mint a second support
- [x] 14-02-PLAN.md — ING-02 claim vs support slot; ING-03 stored polarity on conflict_report_at
- [x] 14-03-PLAN.md — Governor observes ING-01..03; freeze and Proposed map hold

### Phase 15: Verify, persist, and time scale

**Goal**: A developer can trust `open` against a tampered snapshot, treat persist as atomic for the log file, see stable `Define` ids across persist/open, and read fixture years/cuts against a declared VT/TT scale
**Depends on**: Phase 12 (harness S04 `depends:[S01]`; S05 `depends:[]`). Numeric order still runs this after Phase 14.
**Harness slice**: M012a S04 + S05
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S04** then **S05** (or one leased slice covering both) in `.kutha/STATE.md`. S05 may be planned/executed in the same GSD phase after S04 lands.
**Requirements**: DUR-01, DUR-02, DUR-03, TIME-01, TIME-02
**Success Criteria** (what must be TRUE):
  1. `open` verifies snapshot identity against the log (`replay_check` or equivalent) and rejects a tampered snapshot (DUR-01)
  2. `persist` writes the log file atomically (rename-into-place or equivalent); a crash mid-persist does not leave a truncated SoT as success (DUR-02)
  3. `Define` event ids are stable across persist/open cycles for the same term set (DUR-03)
  4. VT and TT declare an explicit scale (type, constant, or documented contract) used by fixtures (TIME-01)
  5. Named tests document how fixture years/cuts map to that scale (TT↔wall optional; full calendar map not required) (TIME-02); governor observes the DUR and TIME tests; `uv run kutha-gov ci` stays 0 HIGH; durability *protocol* (segmented WAL / Rocks) stays unstarted

**Plans**: 1/3 plans executed
Plans:
- [x] 15-01-PLAN.md — DUR-01..03 tracer: verify-on-open, atomic events.jsonl persist, stable Define ids
- [ ] 15-02-PLAN.md — TIME-01..02: YearCe valid-time and log-sequence transaction-time
- [ ] 15-03-PLAN.md — Governor observes DUR and TIME oracles; freeze and Proposed map hold

### Phase 16: Fold-internal hot indexes

**Goal**: A developer can run hot `as_of` / `claim_supported_at` at the leased N without a full linear scan of all facts, while CSR remains a droppable lease and not a second SoT
**Depends on**: Phase 13 (harness S06 `depends:[S02]`). Numeric order still runs this after Phase 15.
**Harness slice**: M012a S06
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S06** in `.kutha/STATE.md`
**Requirements**: HOT-01, HOT-02, HOT-03
**Success Criteria** (what must be TRUE):
  1. `claim_supported_at` / `as_of` at the leased N do not require a full linear scan of all facts (fold-internal index or equivalent) (HOT-01)
  2. The CSR lease path remains droppable; indexes are not a second SoT (HOT-02)
  3. Materializer vs `from_fold` is either wired on the hot path or explicitly deferred with a named test/guard documenting the spike limit (HOT-03)
  4. Named cargo tests for the HOT oracles are observed by the governor; FF5 stays green; `uv run kutha-gov ci` stays 0 HIGH; ADR-061 full GED-class diff stays out of M012a

**Plans**: TBD

## Progress

**Execution order (v0.04):** Phases execute 12 → 13 → 14 → 15 → 16. Product work needs an Active Slice lease in `.kutha/STATE.md` before execute (GATE-02). GSD STATE is not the harness lease.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Legal PIT fitness | v0.01 | 3/3 | Complete | 2026-09-29 |
| 2. Honest harness and freeze | v0.01 | 3/3 | Complete | 2026-09-29 |
| 3. Lease-gated next slice | v0.01 | 3/3 | Complete | 2026-09-29 |
| 4. Partial correction with residual intervals | v0.02 | 3/3 | Complete | 2026-09-29 |
| 5. Persisted quantum outcome | v0.02 | 2/2 | Complete | 2026-09-30 |
| 6. Typed CSR lease | v0.02 | 2/2 | Complete | 2026-09-30 |
| 7. Provenance and rule-version check | v0.02 | 2/2 | Complete | 2026-09-30 |
| 8. End-to-end candidate fixture | v0.02 | 2/2 | Complete | 2026-09-30 |
| 9. H5 lease + lean agent context | v0.03 | 2/2 | Complete | 2026-09-30 |
| 10. ADR and roadmap correction | v0.03 | 2/2 | Complete | 2026-09-30 |
| 11. Semantic governor | v0.03 | 4/4 | Complete | 2026-09-30 |
| 12. Log-native SoT | v0.04 | 3/3 | Complete    | 2026-10-01 |
| 13. Stable references | v0.04 | 3/3 | Complete    | 2026-10-01 |
| 14. Idempotent ingest | v0.04 | 3/3 | Complete    | 2026-10-01 |
| 15. Verify, persist, and time scale | v0.04 | 1/3 | In Progress|  |
| 16. Fold-internal hot indexes | v0.04 | 0/? | Not started | - |

**Harness citation (not this table):** Active Milestone **M012a**; Active Slice **S03**; Phase **H5**; `L_map=honeycomb-proposed`; `L_delivery=M012a-S02-done`; `L_capability=ff5-green`; freeze until explicit **M002**. See `.kutha/STATE.md`. Do not overwrite that file.

---
*Roadmap updated: 2026-10-01 — Phase 15 plans (15-01..15-03)*
