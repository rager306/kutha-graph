# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01–v0.04 are shipped. **v0.05** leases M012 product work (dictionaries as facts): relation allowlist as log facts, rule registry (`rule_version` = definition hash), admission/policy meta-facts, thin action record, and multi-hop derivation eligibility. Honeycomb stays Proposed. Not ADR-050 six dictionaries. Not M002 Rocks. Freeze holds until `.kutha/STATE.md` names otherwise.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- ✅ **v0.03 Lean context + semantic governor** — Phases 9–11 (shipped 2026-09-30) — [archive](./milestones/v0.03-ROADMAP.md)
- ✅ **v0.04 Single-log SoT + stable references** — Phases 12–16 (shipped 2026-10-01) — [archive](./milestones/v0.04-ROADMAP.md)
- 🚧 **v0.05 Dictionaries as facts** — Phases 17–21 (planning) — requirements: [REQUIREMENTS.md](./REQUIREMENTS.md)

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

<details>
<summary>✅ v0.04 Single-log SoT + stable references (Phases 12–16) — SHIPPED 2026-10-01</summary>

- [x] Phase 12: Log-native SoT (3/3 plans) — completed 2026-10-01
- [x] Phase 13: Stable references (3/3 plans) — completed 2026-10-01
- [x] Phase 14: Idempotent ingest (3/3 plans) — completed 2026-10-01
- [x] Phase 15: Verify, persist, and time scale (3/3 plans) — completed 2026-10-01
- [x] Phase 16: Fold-internal hot indexes (3/3 plans) — completed 2026-10-01

Full detail: [milestones/v0.04-ROADMAP.md](./milestones/v0.04-ROADMAP.md) · requirements: [milestones/v0.04-REQUIREMENTS.md](./milestones/v0.04-REQUIREMENTS.md) · phases: [milestones/v0.04-phases/](./milestones/v0.04-phases/)

</details>

### 🚧 v0.05 Dictionaries as facts (Phases 17–21)

**Milestone goal:** Relation allowlist, rule pins, admission, and policy live as log-native facts; a thin action record binds arguments to an admission cut; derivation eligibility is not limited to one-hop + free-string `rule_version`. LLM output remains a proposal.

**Boundary:** Product crates allowed under harness lease M012. Thawed subset only: allowlist as log facts, rule registry (`rule_version` = hash), admission + policy meta-facts, thin action record, multi-hop derivation. No ADR-050 six dictionaries. No Rocks/Cypher/HNSW/legal pack. Honeycomb stays Proposed. Freeze until explicit M002 for Rocks. Do not overwrite `.kutha/STATE.md` from GSD memory.

**Harness map:** Phase 17 = S01; Phase 18 = S02; Phase 19 = S03; Phase 20 = S04; Phase 21 = S05. Execute 17 → 18 → 19 → 20 → 21. Each execute wave needs the matching Active Slice leased in `.kutha/STATE.md` (GATE-02). Named cargo tests register in the governor (GATE-01). Freeze + Proposed map (GATE-03).

**Phase numbering:** Integer phases continue from v0.04 (Phase 17 follows Phase 16). Sequential IDs (`phase_naming`: sequential).

- [x] **Phase 17: Allowlist as log facts** - Relation allowlist entries are versioned log facts; admit consults the fold cut (M012 S01) (completed 2026-10-01)
- [x] **Phase 18: Rule registry** - `rule_version` equals a definition hash; unknown or free-string pins fail closed (M012 S02) (completed 2026-10-01)
- [x] **Phase 19: Admission and policy meta-facts** - Admission status and policy version are bi-temporal log facts; `check_admission` runs on the product path (M012 S03) (completed 2026-10-01)
- [x] **Phase 20: Thin action record** - An Action binds resolved arguments to admission and policy version (M012 S04) (completed 2026-10-01)
- [ ] **Phase 21: Multi-hop derivation** - Derivation eligibility is not one-hop-only; a two-hop fixture is named and green (M012 S05)

## Phase Details

### Phase 17: Allowlist as log facts

**Goal**: A developer can append versioned relation-allowlist facts to the event log and see admit consult that fold cut — not only tip YAML or env
**Depends on**: Phase 16 (v0.04 shipped); harness S01 `depends:[]`
**Harness slice**: M012 S01
**Plane**: product crates (`kutha-common`, `kutha-runtime`) + governor registration
**Lease**: execute only while Active Slice is **S01** in `.kutha/STATE.md`
**Requirements**: ALL-01, ALL-02, ALL-03
**Success Criteria** (what must be TRUE):
  1. Relation allowlist entries are appendable versioned facts on the event log (ALL-01)
  2. `Runtime::admit` (or equivalent) consults the fold cut for allowed relations, not tip YAML/env alone (ALL-02)
  3. An unknown relation still fails closed; the named FF6 oracle remains green (ALL-03)
  4. Named cargo tests for the three ALL oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; FF5 stays green; honeycomb stays Proposed; ADR-050 six dictionaries and M002 Rocks stay unstarted

**Plans**: 3/3 plans complete

Plans:
- [x] 17-01-PLAN.md — Tracer: Op::AllowRelation log fact; admit consults fold for a YAML-absent name; unknown fails closed
- [x] 17-02-PLAN.md — Retract versions the cut; persist/open reconstructs admit from log facts
- [x] 17-03-PLAN.md — Governor observation of ALL oracles; freeze and Proposed map hold

### Phase 18: Rule registry

**Goal**: A developer can pin Behavior to a registry definition hash so unknown, mismatched, or free-string `rule_version` values fail closed on emit/eligibility
**Depends on**: Phase 17 (harness S02 `depends:[S01]`)
**Harness slice**: M012 S02
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S02** in `.kutha/STATE.md`
**Requirements**: RULE-01, RULE-02, RULE-03
**Success Criteria** (what must be TRUE):
  1. A rule registry stores definitions; `rule_version` on Behavior equals the definition hash (RULE-01)
  2. Unknown or mismatched `rule_version` fails closed on emit/eligibility (RULE-02)
  3. Free-string `rule_version` is rejected on the leased path unless it matches a registry hash (RULE-03)
  4. Named cargo tests for the three RULE oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; freeze and Proposed map hold

**Plans**: 3/3 plans complete

Plans:
- [x] 18-01-PLAN.md — Tracer: Op::RegisterRule log fact; Behavior pin equals definition hash; unknown/free-string fail closed
- [x] 18-02-PLAN.md — Retract versions the registry cut; persist/open reconstructs hashed Behavior
- [x] 18-03-PLAN.md — Governor observation of RULE oracles; freeze and Proposed map hold

### Phase 19: Admission and policy meta-facts

**Goal**: A developer can query admission status AS OF a cut, see the policy version pinned in the log and cited by admission, and invoke `check_admission` on the product path — not only from tests
**Depends on**: Phase 17 (harness S03 `depends:[S01]`). Numeric order still runs this after Phase 18.
**Harness slice**: M012 S03
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S03** in `.kutha/STATE.md`
**Requirements**: ADM-01, ADM-02, ADM-03
**Success Criteria** (what must be TRUE):
  1. Admission status is a bi-temporal meta-fact (or equivalent log record) queryable AS OF a cut (ADM-01)
  2. Policy version is pinned in the log and cited by admission (ADM-02)
  3. `check_admission` (or successor) is invoked on the product path for the leased fixture, not only from tests (ADM-03)
  4. Named cargo tests for the three ADM oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; freeze and Proposed map hold

**Plans**: 3/3 plans complete

Plans:
- [x] 19-01-PLAN.md — Tracer: PinPolicy + RecordAdmission AS OF; check_admission on record_justification
- [x] 19-02-PLAN.md — Retract versions admission/policy cuts; persist/open reconstructs
- [x] 19-03-PLAN.md — Governor observation of ADM oracles; freeze and Proposed map hold

### Phase 20: Thin action record

**Goal**: A developer can record a thin Action that binds resolved arguments to the admission decision and policy version, then still see that Action + admission AS OF a prior cut after a later policy change
**Depends on**: Phase 19 (harness S04 `depends:[S03]`)
**Harness slice**: M012 S04
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S04** in `.kutha/STATE.md`
**Requirements**: ACT-01, ACT-02
**Success Criteria** (what must be TRUE):
  1. A thin Action record (or Op) binds resolved arguments to the admission decision and policy version (ACT-01)
  2. A named test shows Action + admission AS OF a prior cut after a later policy change (ACT-02)
  3. Named cargo tests for the ACT oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; ADR-050 six dictionaries stay frozen; honeycomb stays Proposed

**Plans**: 3/3 plans complete

Plans:
- [x] 20-01-PLAN.md — Tracer: Op::RecordAction binds resolved args to admission and policy
- [x] 20-02-PLAN.md — Action + admission AS OF after later policy; persist/open reconstructs
- [x] 20-03-PLAN.md — Governor observation of ACT oracles; freeze and Proposed map hold

### Phase 21: Multi-hop derivation

**Goal**: A developer can exercise derivation eligibility beyond one-hop `caused_by` on a named two-hop (or n-ary) fixture without claiming full provenance polynomials
**Depends on**: Phase 18 (harness S05 `depends:[S02]`). Numeric order still runs this after Phase 20.
**Harness slice**: M012 S05
**Plane**: product crates + governor registration
**Lease**: execute only while Active Slice is **S05** in `.kutha/STATE.md`
**Requirements**: DER-01, DER-02, DER-03
**Success Criteria** (what must be TRUE):
  1. Derivation eligibility is not limited to one-hop `caused_by` for the leased fixture (DER-01)
  2. A two-hop (or n-ary) named test is green without claiming full provenance polynomials (DER-02)
  3. Hard-coded `inverse_knows`-only cascade is not the sole follow-on mechanism, **or** it is documented as residual spike with a named guard (DER-03)
  4. Named cargo tests for the DER oracles are observed by the governor; `uv run kutha-gov ci` stays 0 HIGH; ADR-011 MATCH/polynomial evaluator stays out of this milestone

**Plans**: 1/3 plans executed

Plans:
- [x] 21-01-PLAN.md — Tracer: walk caused_by so two-hop hashed eligibility is not one-hop-only
- [ ] 21-02-PLAN.md — Residual inverse_knows guard; persist/open reconstructs two-hop eligibility
- [ ] 21-03-PLAN.md — Governor observation of DER oracles; freeze and Proposed map hold

## Progress

**Execution order (v0.05):** Phases execute 17 → 18 → 19 → 20 → 21. Product work needs an Active Slice lease in `.kutha/STATE.md` before execute (GATE-02). GSD STATE is not the harness lease. S03 is independent of S02; S05 is independent of S03/S04; numeric order still serializes them.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Legal PIT fitness | v0.01 | 3/3 | Complete | 2026-09-29 |
| 2. Honest harness and freeze | v0.01 | 3/3 | Complete | 2026-09-29 |
| 3. Lease-gated next slice | v0.01 | 3/3 | Complete | 2026-09-29 |
| 4. Partial correction | v0.02 | 3/3 | Complete | 2026-09-29 |
| 5. Persisted quantum outcome | v0.02 | 2/2 | Complete | 2026-09-30 |
| 6. Typed CSR lease | v0.02 | 2/2 | Complete | 2026-09-30 |
| 7. Provenance and rule-version | v0.02 | 2/2 | Complete | 2026-09-30 |
| 8. End-to-end candidate fixture | v0.02 | 2/2 | Complete | 2026-09-30 |
| 9. H5 lease + lean agent context | v0.03 | 2/2 | Complete | 2026-09-30 |
| 10. ADR and roadmap correction | v0.03 | 2/2 | Complete | 2026-09-30 |
| 11. Semantic governor | v0.03 | 4/4 | Complete | 2026-09-30 |
| 12. Log-native SoT | v0.04 | 3/3 | Complete | 2026-10-01 |
| 13. Stable references | v0.04 | 3/3 | Complete | 2026-10-01 |
| 14. Idempotent ingest | v0.04 | 3/3 | Complete | 2026-10-01 |
| 15. Verify, persist, and time scale | v0.04 | 3/3 | Complete | 2026-10-01 |
| 16. Fold-internal hot indexes | v0.04 | 3/3 | Complete | 2026-10-01 |
| 17. Allowlist as log facts | v0.05 | 3/3 | Complete    | 2026-10-01 |
| 18. Rule registry | v0.05 | 3/3 | Complete    | 2026-10-01 |
| 19. Admission and policy meta-facts | v0.05 | 3/3 | Complete    | 2026-10-01 |
| 20. Thin action record | v0.05 | 3/3 | Complete    | 2026-10-01 |
| 21. Multi-hop derivation | v0.05 | 1/3 | In Progress|  |
