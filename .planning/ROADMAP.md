# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`. Do not plan ADR-010–093 as sequential GSD phases.

v0.01 and v0.02 are shipped. M011 semantic core stop condition is met. **v0.03** (harness/docs only) slims agent context, corrects ADR and proposed-roadmap semantics for verified gaps F1–F8, and adds semantic governor kinds — without `crates/` edits or thawing freeze. Product fixes stay deferred until a separately leased M012a. Phases 9–11 run sequentially after roadmap approval.

## Milestones

- ✅ **v0.01 GSD foundation** — Phases 1–3 (shipped 2026-09-29) — [archive](./milestones/v0.01-ROADMAP.md)
- ✅ **v0.02 Semantic core close** — Phases 4–8 (shipped 2026-09-30) — [archive](./milestones/v0.02-ROADMAP.md)
- 🚧 **v0.03 Lean context + semantic governor** — Phases 9–11 (planning) — requirements: [REQUIREMENTS.md](./REQUIREMENTS.md)

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

### 🚧 v0.03 Lean context + semantic governor (Phases 9–11)

**Milestone goal:** The harness states what is true, not what is present: H5 lease, lean durable agent context, ADR/roadmap corrections for F1–F8 (cells stay Proposed), and a governor that checks meaning — vacuity, assertions, cited lease, evidence links — not needle presence alone.

**Boundary:** harness + docs only; no `crates/` edits; freeze holds; `.kutha/STATE.md` harness lease is not overwritten by GSD delivery.

**Phase numbering:** Integer phases continue from v0.02 (Phase 9 follows Phase 8). Execute 9 → 10 → 11.

- [x] **Phase 9: H5 lease + lean agent context** - Name H5 consistently; slim `AGENTS.md` to durable rules with volatile content relocated; governor CI stays 0 HIGH (completed 2026-09-30)
- [x] **Phase 10: ADR and roadmap correction** - F1–F8 review artifact; dated ADR amendments (Proposed); `.kutha/ROADMAP.md` later-milestone reorder; honeycomb map consistency (completed 2026-09-30)
- [ ] **Phase 11: Semantic governor** - Selftest, non-vacuous tests, cited-state equality, evidence resolution, ADR xref, AGENTS budget, deferred F1–F8 invariants, drift guards

## Phase Details

### Phase 9: H5 lease + lean agent context

**Goal**: An agent loading the repo gets durable operating rules from a lean `AGENTS.md` while trajectory and lease values live only in `.kutha/STATE.md`; harness docs name the **H5** rung (semantic governor + lean context) without changing Active Milestone, lifecycles, or freeze
**Depends on**: Phase 8 (v0.02 complete); harness lease unchanged unless operator edits `.kutha/STATE.md` separately
**Plane**: docs/harness lease citations only — no product crate delivery
**Requirements**: LEASE-01, CTX-01, CTX-02, CTX-03, CTX-04
**Success Criteria** (what must be TRUE):
  1. `README.md`, `docs/process/kutha-harness.md`, `.kutha/STATE.md`, and `.kutha/ROADMAP.md` describe **H5** consistently; Active Milestone, lifecycles, and freeze wording match the harness lease; `uv run kutha-gov ci` reports **0 HIGH** (LEASE-01)
  2. `AGENTS.md` contains no lease or trajectory values (no `L_delivery=` tokens, no “current slice/milestone is …” narrative) and points readers to `.kutha/STATE.md` for live lease (CTX-01)
  3. Content moved out of `AGENTS.md` (P0 inventory, literature/matrix detail, research notes, long subagent workflow, full tree) remains findable via one-line pointers in owning docs — no facts dropped (CTX-02)
  4. After the diet, language policy, two planes, D1–D10 guard, CE routing, commands, freeze, CBM/subagent rules, and working conventions still mean the same as before (CTX-03)
  5. Stale `L_delivery=M011-S03-done` (or equivalent) claims are removed or corrected in `AGENTS.md` and `docs/architecture/semantic-contract-validation.md` (CTX-04)

**Plans:** 2/2 plans complete
- [x] 09-01-PLAN.md — Operator H5 lease, dogfood ladder, in-place governor needles; ci HIGH 0
- [x] 09-02-PLAN.md — AGENTS.md diet, relocations, stale citation fixes; diet ledger

### Phase 10: ADR and roadmap correction

**Goal**: Verified architecture gaps F1–F8 are traced to honeycomb coordinates with explicit verdicts; amended cells carry dated Proposed amendments; the **proposed** harness roadmap order reflects M012a before M012 and M002 as log durability first — without leasing new delivery or promoting any cell to Accepted
**Depends on**: Phase 9
**Plane**: docs + `.kutha/ROADMAP.md` + honeycomb index — no `crates/` edits
**Requirements**: ADR-01, ADR-02, ADR-03, ADR-04
**Success Criteria** (what must be TRUE):
  1. A review artifact maps every finding F1–F8 to affected ADR-000/001/002 and honeycomb cells with a per-cell verdict (`no change` | `amend` | `open question`) and evidence (file + symbol or measurement) (ADR-01)
  2. Cells with `amend` carry a dated amendment or Open Question in the ADR body; every affected cell remains **Proposed**; ADR-000 D1–D10 lock wording is not silently rewritten (ADR-02)
  3. `.kutha/ROADMAP.md` “Later milestones” lists **M012a** (single-log SoT + stable references) before **M012**; M012 names the admission/rule-registry subset it thaws; **M002** is described as log durability first (Rocks for indexes only); benchmarks and a thin legal golden fixture appear earlier in the narrative — no new milestone is leased (ADR-03)
  4. `.kutha/dictionaries/honeycomb.yaml` and `docs/ADR/README.md` stay consistent with those amendments; map/delivery/capability stay orthogonal; no cell is promoted to Accepted (ADR-04)

**Plans:** 2/2 plans complete
- [x] 10-01-PLAN.md — F1–F8 review artifact plus Later-milestones reorder (M012a before M012; M002 log durability first)
- [x] 10-02-PLAN.md — Dated Proposed ADR amendments/open questions, README pointer, honeycomb consistency

### Phase 11: Semantic governor

**Goal**: The governor checks semantic truth — vacuity, test assertions, lease citations, evidence links, ADR references, and context size — and records product gaps F1–F8 as deferred invariants so Phases 9–10 cannot regress silently
**Depends on**: Phase 10
**Plane**: `scripts/kutha_gov`, `.kutha/dictionaries`, process docs — no `crates/` edits
**Requirements**: SEM-01, SEM-02, SEM-03, SEM-04, SEM-05, SEM-06, SEM-07, SEM-08
**Success Criteria** (what must be TRUE):
  1. `uv run kutha-gov selftest` mutates a temporary copy of each mutable YAML check and observes a **HIGH**; checks that cannot be mutated are listed with a reason (SEM-01)
  2. A new check kind proves named Rust tests are non-vacuous (exists, not ignored, body contains an assertion); all existing `fn …` needle checks use it (SEM-02)
  3. A new check kind fails when any doc citation of lease values (`L_delivery`, Active Milestone/Slice, Phase) disagrees with `.kutha/STATE.md` (SEM-03)
  4. `honeycomb.yaml` `evidence` and `capability: named` resolve: each named evidence item points to an existing test or path; named capability cells have at least one resolving evidence item — or CI fails (SEM-04)
  5. Every `ADR-NNN` and `D1`–`D10` reference under `AGENTS.md`, `README.md`, `.kutha/`, and `docs/process/` resolves to an existing ADR or lock — dangling references fail (SEM-05)
  6. `AGENTS.md` has an enforced size budget and forbids lease tokens so CTX-01/02 cannot regress without a HIGH (SEM-06)
  7. Product gaps F1–F8 appear as `disposition: deferred` invariants with `until` naming target milestones; they are recorded, not enforced as product fixes (SEM-07)
  8. Each new kind has a red-path pytest; `.kutha/META.md` and `docs/process/governor-intake.md` document intake; **Process** CHANGELOG records the change; `uv run kutha-gov ci` stays **0 HIGH** (SEM-08)

**Plans:** 1/4 plans executed
- [x] 11-01-PLAN.md — Vacuity selftest CLI, derived/declared mutations, FSM run_selftest, H5 evidence
- [ ] 11-02-PLAN.md — rust_test_asserts + honeycomb evidence resolution
- [ ] 11-03-PLAN.md — cite_equals, refs_resolve, AGENTS.md budget
- [ ] 11-04-PLAN.md — deferred F1–F8, META/intake/CHANGELOG, H5 close

## Progress

**Execution order (v0.03):** Phases execute 9 → 10 → 11. No product slice lease is implied.

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 9. H5 lease + lean agent context | v0.03 | 2/2 | Complete    | 2026-09-30 |
| 10. ADR and roadmap correction | v0.03 | 2/2 | Complete    | 2026-09-30 |
| 11. Semantic governor | v0.03 | 1/4 | In Progress|  |

**Harness citation (not this table):** Active Milestone M011; `L_delivery=M011-S08-done`; Phase H4; Active Slice **None**; freeze until explicit M002. See `.kutha/STATE.md`.
