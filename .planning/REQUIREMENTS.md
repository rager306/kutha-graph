# Requirements: Kutha

**Defined:** 2026-09-29
**Core Value:** Prove legal PIT on a named fixture, keep governor CI honest, and advance one Active Slice at a time — not ship the honeycomb.

No PRDs were ingested (0 PRD classifications). v1 below is derived from `STRATEGY.md` falsifiers, executable fitness (FF5/FF6, H2/H4, M010/M011), freeze/planes, and approved ingest resolutions. Honeycomb ADR-010–093 are **not** v1 requirements.

## v1 Requirements

Requirements for this GSD overlay. Each maps to exactly one roadmap phase.

### Fitness (L_capability)

- [x] **FIT-01**: On the statute-shaped fixture, `as_of(2015)` and `as_of(2021)` yield different live triples (FF5; `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`)
- [x] **FIT-02**: An emit with an unknown product relation does not append (FF6; `ff6_unknown_relation_does_not_append`)
- [x] **FIT-03**: Opening a persisted log without `snapshot.json` restores intern meanings from retained history (M010; including Define-op recovery and live intern `Op::Define`)
- [x] **FIT-04**: Independent supports share `claim_id`; withdrawing one leaves the claim supported; unknown claim does not append; replay rejects Behavior whose `caused_by` is not a prior event; derived Q loses eligibility when the last premise support is withdrawn (M011 S01–S03)
- [x] **FIT-05**: H2 tenant ingest: harness status AS OF two process cuts differs, and same-second statuses use the emitted cut. H4: a prior tenant cut still shows process-relation membership after a later edition drops it. Not a legal pack and not ADR-090 ontology

### Governor honesty

- [ ] **GOV-01**: `uv run kutha-gov ci` is fail-closed on HIGH findings; a green run is not treated as ADR Accepted or as L_capability
- [ ] **GOV-02**: `.kutha/STATE.md` keeps `L_map`, `L_delivery`, and `L_capability` named separately (no collapse into one “green = shipped” status)
- [ ] **GOV-03**: New product-crate slice work starts only when `.kutha/STATE.md` names that Active Slice (today: None)

### Planes and write surface

- [ ] **PLANE-01**: Product truth stays in `crates/kutha-*`; harness stays in `scripts/kutha_gov` + `.kutha/`; Python is not added inside `kutha-runtime`; repo-root `ports/` / `adapters/` / `domain/` are not introduced
- [ ] **PLANE-02**: Product and process writes use typed `Op` (Assert/Retract/Correct/Behavior/Define). STCA-guide §5 JSON merge-patch / `object.created` is not the product write surface
- [ ] **PLANE-03**: Product `kutha-relations/v1` and harness `kutha-harness-relations/v1` remain distinct files and schemas

### Freeze

- [ ] **FREEZE-01**: Until `.kutha/STATE.md` explicitly leases them: no RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, ADR-080/081, full ADR-090/093 packs, legal corpus start, or Consensus Query 103+

### Delivery next

- [ ] **NEXT-01**: Further M011 product slices beyond S03 land only under an explicit Active Slice lease; a legal pack is not started as a substitute
- [ ] **NEXT-02**: After M011 is closed in `.kutha/STATE.md`, the next product milestone is the one STATE names — not an assumed M002 and not “implement honeycomb”

### Map discipline

- [ ] **MAP-01**: Honeycomb cells ADR-010–093 remain Proposed/map. This GSD v1 does not schedule them as a phase-per-cell backlog

## v2 Requirements

Deferred. Acknowledged in ADRs / STRATEGY; not in the current GSD roadmap. Promotion requires a `.kutha/STATE.md` lease (and usually a new GSD milestone), not a silent REQUIREMENTS edit.

### Persistence and query skins

- **M002-01**: Rocks adapter behind the same events (persistence, not a second SoT) — only if STATE names M002
- **QRY-01**: Cypher/GPML skin over the already-correct AS OF cut (ADR-070) — frozen
- **QRY-02**: HNSW as retrieve-not-truth (ADR-042) — frozen

### Agent dictionaries and security

- **DICT-01**: Six ADR-050 dictionary kinds as bi-temporal graph entities — frozen (FF6 stub is not this)
- **SEC-01**: ABAC / multi-tenant query rewrite (ADR-080) — frozen
- **SEC-02**: Agent sandbox / WASM UDFs (ADR-081) — frozen

### Vertical packs

- **LEGAL-01**: Legal Reference pack + corpus (ADR-090 TR-01–TR-10) — not FF5 fixture; freeze
- **SCI-01**: Scientific archive pack (ADR-093) — freeze
- **SEM-01**: Fuller semantic-contract fixture oracles beyond thin P→Q (interval-patch residuals, execution replay) — needs an Active Slice lease, not v1 by default

### Composition / packaging

- **CUI-01**: General Cui max-convolution allocator (ADR-030) — P0/harness budget is not this
- **RVF-01**: RVF capsules as hot SoT — rejected; transport-only remains v2+ packaging (ADR-091)

## Out of Scope

| Feature | Reason |
|---------|--------|
| Phase-per-cell honeycomb delivery | Map ≠ backlog; would collapse L_map into L_delivery |
| STCA-guide §5 JSON merge-patch write API | ADR-010 typed Op wins (approved ingest) |
| TypeScript graph core | D6; GC/event-loop unfit for hot graphs |
| Mandatory Neo4j/FalkorDB/Graphiti/cloud LLM for AS OF | Loses ownership of temporal truth |
| Python inside `kutha-runtime` | Plane split |
| Repo-root hexagonal folders | ADR-022: hexagon inside a slice |
| Promote ADRs to Accepted because CI is green | Governor green ≠ Accepted ≠ capability |
| SemVer / tags / GitHub Releases | Repo changelog policy |
| Second STCA paradigm lock from stca-guide | ADR-002 wins; guide is constraints only |

## Traceability

Which phases cover which requirements. Populated at roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| FIT-01 | Phase 1 | Complete |
| FIT-02 | Phase 1 | Complete |
| FIT-03 | Phase 1 | Complete |
| FIT-04 | Phase 1 | Complete |
| FIT-05 | Phase 1 | Complete |
| GOV-01 | Phase 2 | Pending |
| GOV-02 | Phase 2 | Pending |
| PLANE-01 | Phase 2 | Pending |
| PLANE-02 | Phase 2 | Pending |
| PLANE-03 | Phase 2 | Pending |
| FREEZE-01 | Phase 2 | Pending |
| MAP-01 | Phase 2 | Pending |
| GOV-03 | Phase 3 | Pending |
| NEXT-01 | Phase 3 | Pending |
| NEXT-02 | Phase 3 | Pending |

**Coverage:**
- v1 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0 ✓

---
*Requirements defined: 2026-09-29*
*Last updated: 2026-09-29 after ingest → GSD roadmap*
