# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.02 closes the M011 semantic core with one GSD phase per remaining harness slice (S04–S08). Phases 4–6 are complete. Phase 7 (S07) is leased and executable. Phase 8 is planned but blocked until `.kutha/STATE.md` names S08 as Active Slice. Stop when one fixture separates preserved history, current evidence, and allowed action. Frozen surfaces (RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, legal/science packs) stay unstarted.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- 🚧 **v0.02 Semantic core close** — Phases 4–8 (Phases 4–6 complete; Phase 7 leased; Phase 8 lease-gated)

## Phases

<details>
<summary>✅ v0.01 GSD foundation (Phases 1–3) — SHIPPED 2026-09-29</summary>

- [x] Phase 1: Legal PIT fitness (3/3 plans) — completed 2026-09-29
- [x] Phase 2: Honest harness and freeze (3/3 plans) — completed 2026-09-29
- [x] Phase 3: Lease-gated next slice (3/3 plans) — completed 2026-09-29

Full detail: [milestones/v0.01-ROADMAP.md](./milestones/v0.01-ROADMAP.md) · requirements: [milestones/v0.01-REQUIREMENTS.md](./milestones/v0.01-REQUIREMENTS.md) · audit: [milestones/v0.01-MILESTONE-AUDIT.md](./milestones/v0.01-MILESTONE-AUDIT.md)

</details>

**Phase Numbering:**
- Integer phases continue from v0.01: Phase 4 follows Phase 3
- Decimal phases (4.1, 4.2): Urgent insertions (marked with INSERTED)

- [x] **Phase 4: Partial correction with residual intervals** - Explicit interval-patch leaves VT 2012/2021 residuals; whole-version Correct unchanged (S04 leased) (completed 2026-09-29)
- [x] **Phase 5: Persisted quantum outcome** - Budgets 0/1/2 and crash-after-prefix are distinguishable; resume is an explicit record (S05 delivered) (completed 2026-09-30)
- [x] **Phase 6: Typed CSR lease** - Same endpoints keep relation labels and support multiplicity; untyped neighbor-set path remains (S06 delivered) (completed 2026-09-30)
- [ ] **Phase 7: Provenance and rule-version check** - Causal-ref or rule-version change is detected without execution replay (S07 leased)
- [ ] **Phase 8: End-to-end candidate fixture** - Semantic-contract observations 1–5 hold; incremental and reconstructed answers agree (lease-gated)

## Phase Details

### Phase 4: Partial correction with residual intervals

**Goal**: A developer can apply an explicit interval-patch correction and observe residual versions on both sides of the corrected VT range, while whole-version `Correct` stays unchanged
**Depends on**: Phase 3 (harness S04 `depends:[S03]`)
**Lease**: executable — Active Slice S04 in `.kutha/STATE.md`
**Requirements**: CORR-01, CORR-02, GATE-01
**Success Criteria** (what must be TRUE):
  1. An explicit interval-patch correction leaves residual versions on both sides of the corrected VT range; at the same TT cut, source `a` keeps VT 2012 and 2021 residuals (CORR-01)
  2. Whole-version `Correct` still behaves as before; no implicit residuals appear (CORR-02)
  3. This slice ships a named cargo test registered in the governor (FSM observe + check needle), and `uv run kutha-gov ci` stays at 0 HIGH (GATE-01; the same named-test pattern applies to every later slice)
  4. This phase executes only while S04 is the Active Slice (GATE-02 applies; S04 is currently leased)
  5. Freeze items stay unstarted and honeycomb cells stay Proposed (GATE-03 applies)

**Plans**: 3/3 plans complete

Plans:
- [x] 04-01-PLAN.md — Explicit interval-patch tracer: residuals at VT 2012/2021 (CORR-01)
- [x] 04-02-PLAN.md — Whole-version Correct leaves no implicit residuals (CORR-02)
- [x] 04-03-PLAN.md — Governor GATE-01 registration, changelog Process, ci HIGH-free

### Phase 5: Persisted quantum outcome

**Goal**: A developer can tell zero, partial, and full quantum progress apart from persisted outcome records, and resume after a crash without treating missing terminal evidence as success
**Depends on**: Phase 3 (harness S05 `depends:[S03]`; not S04)
**Lease**: delivered — Active Slice was S05; now None until S06+ lease in `.kutha/STATE.md`
**Requirements**: OUT-01, OUT-02
**Success Criteria** (what must be TRUE):
  1. Budgets 0/1/2 are distinguishable as zero, partial, and full progress from persisted quantum outcome records (OUT-01)
  2. After a crash following a committed prefix, reopening the log shows no terminal-success record; resume is an explicit record, never inferred (OUT-02)
  3. A named cargo test is registered in the governor and `uv run kutha-gov ci` stays at 0 HIGH (GATE-01 applies)
  4. This phase executes only while S05 is the Active Slice; until then it is planning-only (GATE-02 applies). Freeze items stay unstarted and honeycomb cells stay Proposed (GATE-03 applies)

**Plans**: 2/2 plans complete

Plans:
**Wave 1**
- [x] 05-01-PLAN.md — Persist quantum outcomes; budgets 0/1/2 Zero/Partial/Full + crash resume (OUT-01, OUT-02)

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 05-02-PLAN.md — Governor GATE-01 registration, changelog Process, ci HIGH-free

### Phase 6: Typed CSR lease

**Goal**: A developer can query a typed CSR lease that preserves relation labels and support multiplicity, while the untyped neighbor-set path and the FF5 lease-agrees-with-fold check still hold
**Depends on**: Phase 3 (harness S06 `depends:[S03]`; not S04)
**Lease**: delivered — Active Slice was S06; now None until S07+ lease in `.kutha/STATE.md`
**Requirements**: CSR-01, CSR-02
**Success Criteria** (what must be TRUE):
  1. Same endpoints with different relations or supports survive the lease: labels and support multiplicity are preserved (CSR-01)
  2. The untyped neighbor-set path stays available, and the FF5 lease-agrees-with-fold check stays green (CSR-02)
  3. A named cargo test is registered in the governor and `uv run kutha-gov ci` stays at 0 HIGH (GATE-01 applies)
  4. This phase executes only while S06 is the Active Slice; until then it is planning-only (GATE-02 applies). Freeze items stay unstarted and honeycomb cells stay Proposed (GATE-03 applies)

**Plans**: 2/2 plans complete

Plans:
**Wave 1**
- [x] 06-01-PLAN.md — Typed CSR lease tracer: labels + support multiplicity; untyped path holds (CSR-01, CSR-02)

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 06-02-PLAN.md — Governor GATE-01 registration, changelog Process, ci HIGH-free

### Phase 7: Provenance and rule-version check

**Goal**: A developer can detect a change to only a causal reference or a pinned rule version even when the state fingerprint still matches, without execution replay
**Depends on**: Phase 4 (harness S07 `depends:[S04]`)
**Lease**: executable — Active Slice S07 in `.kutha/STATE.md`
**Requirements**: PROV-01, PROV-02
**Success Criteria** (what must be TRUE):
  1. Verification detects a change to only a Behavior `caused_by` reference even when the state fingerprint still matches (PROV-01)
  2. Verification detects a change to only a pinned rule version the same way; execution replay is not required (PROV-02)
  3. A named cargo test is registered in the governor and `uv run kutha-gov ci` stays at 0 HIGH (GATE-01 applies)
  4. This phase executes only while S07 is the Active Slice (GATE-02 applies; S07 is currently leased). Freeze items stay unstarted and honeycomb cells stay Proposed (GATE-03 applies)

**Plans**: 1/2 plans executed

Plans:
**Wave 1**
- [x] 07-01-PLAN.md — Rule version + provenance digest + two named oracles (PROV-01, PROV-02)

**Wave 2** *(blocked on Wave 1 completion)*
- [ ] 07-02-PLAN.md — Governor GATE-01 registration, changelog Process, ci HIGH-free

### Phase 8: End-to-end candidate fixture

**Goal**: A developer can run one end-to-end fixture that distinguishes preserved history, current evidence, and allowed action at named cuts, with incremental maintenance and clean reconstruction agreeing
**Depends on**: Phase 4, Phase 5, Phase 6, Phase 7 (harness S08 `depends:[S04,S05,S06,S07]`)
**Lease**: gated — planning-only until S08 is the Active Slice in `.kutha/STATE.md`
**Requirements**: FIX-01, FIX-02, FIX-03, GATE-02, GATE-03
**Success Criteria** (what must be TRUE):
  1. Independent supports and last-support withdrawal hold at named (TT, VT) cuts on the candidate fixture; the conflict variant preserves both sides and reports conflict instead of superseding (FIX-01)
  2. Summary and action records cite exact source revisions and rule version; a changed dependency forces re-evaluation, and a stale cached output cannot renew its own admission (FIX-02)
  3. Incremental maintenance and clean reconstruction agree on values, active supports, and completeness; discarding CSR and snapshots changes no answer (FIX-03)
  4. Each slice's GSD phase executed only while that slice was the Active Slice; otherwise it stayed planning-only (GATE-02). Named cargo tests and `uv run kutha-gov ci` at 0 HIGH still apply (GATE-01 applies)
  5. Freeze items stay unstarted and honeycomb cells stay Proposed; any ADR Accepted decision is a separate review at milestone close (GATE-03)

**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 4 → 5 → 6 → 7 → 8. Phase 8 remains planning-only until `.kutha/STATE.md` names S08 as Active Slice. Phase 7 (S07) is leased. Harness slice dependencies: S05 and S06 depend on S03 (not S04); S07 depends on S04; S08 depends on S04–S07.

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 4. Partial correction with residual intervals | 3/3 | Complete    | 2026-09-29 |
| 5. Persisted quantum outcome | 2/2 | Complete    | 2026-09-30 |
| 6. Typed CSR lease | 2/2 | Complete    | 2026-09-30 |
| 7. Provenance and rule-version check | 1/2 | In Progress|  |
| 8. End-to-end candidate fixture | 0/TBD | Not started (lease-gated) | - |

**Harness citation (not this table):** Active Milestone M011; `L_delivery=M011-S06-done`; Phase H4; Active Slice **S07**; freeze until explicit M002. See `.kutha/STATE.md`.
