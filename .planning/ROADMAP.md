# Roadmap: Kutha

## Overview

This GSD overlay tracks the **steel thread already in crates**, not a honeycomb waterfall. Phase 1 holds legal PIT and related fitness (FF5/FF6, M010, M011 S01–S03, H2/H4). Phase 2 holds governor honesty, two-plane isolation, typed Op, freeze, and map discipline. Phase 3 is the only *new* product work: continue M011 only under an explicit Active Slice lease, then start only the milestone `.kutha/STATE.md` names.

Harness delivery (`M001`/`M010` closed, `M011` S03 done, Active Slice **None**, Phase **H4**) stays in `.kutha/STATE.md` and `.kutha/ROADMAP.md`. This file does not replace them. Do not plan ADR-010–093 as sequential GSD phases.

In-repo tests already witness most Phase 1–2 criteria; GSD plans for those phases are verification-first (do not regress, do not thaw freeze). Phase 3 is blocked until STATE names a slice.

## Milestones

- 🚧 **Research spike (harness M011)** — GSD Phases 1–3 (in progress on the harness plane; GSD plans not yet written)
- 📋 **Next STATE-named milestone** — not assumed to be M002; not honeycomb promotion

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned GSD work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

- [x] **Phase 1: Legal PIT fitness** - Named AS OF cuts, fail-closed relations, recovery, claims, H2/H4 dogfood stay falsifiable (completed 2026-09-29)
- [ ] **Phase 2: Honest harness and freeze** - Governor CI, three lifecycles, planes, typed Op, freeze, honeycomb-as-map
- [ ] **Phase 3: Lease-gated next slice** - One Active Slice; finish M011 only when leased; next milestone only if STATE names it

## Phase Details

### Phase 1: Legal PIT fitness

**Goal**: A developer can run the named product fitness suite and observe legal point-in-time, fail-closed relations, semantic recovery, claim/support identity, and H2/H4 tenant AS OF still hold
**Depends on**: Nothing (first phase; brownfield spike already in crates)
**Requirements**: FIT-01, FIT-02, FIT-03, FIT-04, FIT-05
**Success Criteria** (what must be TRUE):
  1. On the statute-shaped log, `as_of(2015)` and `as_of(2021)` return different live triples (FF5)
  2. An unknown product relation is rejected and does not append (FF6)
  3. Discarding `snapshot.json` still restores intern meanings from retained history (M010)
  4. Two supports for one claim remain distinguishable; unknown claim and dangling `caused_by` fail closed; derived Q loses eligibility when the last premise support is withdrawn (M011 S01–S03)
  5. H2 process-status AS OF cuts differ (including same-second emitted cut); H4 prior membership remains visible after a later edition drops it — without starting a legal pack

**Plans**: 3/3 plans complete
Plans:
**Wave 1**
- [x] 01-01-PLAN.md — Tracer: hard gate + VERIFICATION evidence skeleton

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 01-02-PLAN.md — Complete twelve-row FIT pass/fail map + SUMMARY

**Wave 3** *(blocked on Wave 2 completion)*
- [x] 01-03-PLAN.md — Batch REQUIREMENTS FIT checkboxes + GSD STATE (after VERIFICATION green)

### Phase 2: Honest harness and freeze

**Goal**: Process CI tells the truth about trajectory; product and harness stay on separate planes; frozen surfaces stay unstarted; honeycomb stays a map
**Depends on**: Phase 1
**Requirements**: GOV-01, GOV-02, PLANE-01, PLANE-02, PLANE-03, FREEZE-01, MAP-01
**Success Criteria** (what must be TRUE):
  1. `uv run kutha-gov ci` fails closed on HIGH; a green run is not presented as ADR Accepted or as capability
  2. `.kutha/STATE.md` still lists `L_map`, `L_delivery`, and `L_capability` as separate lifecycles
  3. Product crates remain Rust; harness remains Python 3.13/uv; relation YAML schemas are not mixed; no repo-root `ports/` / `adapters/` / `domain/`
  4. Writes stay typed `Op`; stca-guide §5 JSON merge-patch is not the product write surface
  5. RocksDB, Cypher/GPML, HNSW, ADR-050 six dictionaries, ADR-080/081, and legal/science packs are absent unless `.kutha/STATE.md` has leased them; honeycomb ADRs remain Proposed

**Plans**: 3 plans
Plans:
**Wave 1**
- [ ] 02-01-PLAN.md — Tracer: ci + cargo smoke + VERIFICATION skeleton + VALIDATION wave_0

**Wave 2** *(blocked on Wave 1 completion)*
- [ ] 02-02-PLAN.md — Fill GOV/PLANE/FREEZE/MAP probe map + SUMMARY Trajectory

**Wave 3** *(blocked on Wave 2 completion)*
- [ ] 02-03-PLAN.md — Batch REQUIREMENTS Phase 2 IDs + GSD STATE/ROADMAP closeout

### Phase 3: Lease-gated next slice

**Goal**: New crate work happens only as the Active Slice STATE names; M011 does not sprawl into a legal pack; the following milestone is not implied
**Depends on**: Phase 2
**Requirements**: GOV-03, NEXT-01, NEXT-02
**Success Criteria** (what must be TRUE):
  1. While Active Slice is None, no new M011 (or other) product-slice implementation starts
  2. When STATE names a slice, only that slice is implemented; a legal corpus/pack is not started in its place
  3. When M011 is closed in `.kutha/STATE.md`, the next product milestone is whatever STATE then names — not an assumed M002 and not “implement honeycomb”

**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Legal PIT fitness | 3/3 | Complete    | 2026-09-29 |
| 2. Honest harness and freeze | 0/3 | Planned | - |
| 3. Lease-gated next slice | 0/TBD | Not started | - |

**Harness citation (not this table):** Active Milestone M011; `L_delivery=M011-S03-done`; Phase H4; Active Slice None; freeze until explicit M002. See `.kutha/STATE.md`.
