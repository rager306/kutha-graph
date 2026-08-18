# Axis × layer synthesis (163 cards)

Date: 2026-08-18. Rollup of `cards/` after Wave 34. Not a roadmap. Not a Consensus wave.

Counts below are **card tags**, not uniqueness of nouns. Query is dense because many join/access-path cousins share the Query tag. Saturation ≠ “this axis is finished as Kutha design.”

## Snapshot

| | n | share |
|--|---|--------|
| Closed cards (= matrix rows) | 163 | 100% |
| `confidence: paper` | 125 | 77% |
| `confidence: code` | 29 | 18% |
| `spec` / `observed` / `claim` | 6 / 2 / 1 | 5% |
| Layer 5 = `no niche` | 126 | 77% |
| Layer 5 named (mostly legal/science/enterprise) | 37 | 23% |
| Typical score `high / med / high` | 70 | 43% |
| Spine score `high / high / high` | 42 | 26% |
| Demand high ∧ optimality low (trap cluster) | 10 | 6% |

Axis occupancy (a card may count on several axes):

| Axis | cards | paper / code / other | First-listed axis |
|------|------:|----------------------|------------------:|
| Query | 112 | 87 / 22 / 3 | 52 |
| Data | 92 | 70 / 16 / 6 | 28 |
| Time | 66 | 53 / 13 / 0 | 33 |
| Verify | 51 | 39 / 10 / 2 | 11 |
| Composition | 51 | 42 / 6 / 3 | 4 |
| Agent | 47 | 30 / 11 / 6 | 17 |
| Space | 26 | 20 / 5 / 1 | 7 |
| Security | 16 | 13 / 1 / 2 | 10 |
| Packaging | 7 | 5 / 2 / 0 | 1 |

Dominant pair: **Data × Query (72 cards)**. Time × Query (38), Time × Verify (26), Agent × Verify (25). Security almost always rides Verify or Query; Packaging is a sidecar of Time/Verify.

---

## Synthesis by axis

### Query — saturated literature, thin *Kutha planner*

Covered nouns: LFTJ/WCOJ family, factorized/hash/adaptive orders, Cypher/GQL, RPQ, SPARQL/SHACL/OBDA compile, SMQ, BANKS, OLAP, 2-hop, result cache, FSM, GED, community lease, RSP-QL, sampling AQP, hybrid vector-relational.

**Well researched as papers. Poorly researched as one engine:** ADR-043 “hybrid query planner” is still a *pile of access paths*, not a compiled cost model that chooses among them on a log offset. Mono-axis Query cards are almost all WCOJ-order papers.

Do not add another join algorithm.

### Data — strong hot path, weak *placement policy*

Covered: CSR/GraphBLAS, HNSW+repair, dictionary, k², views, bulk load, CDC, summarization, sparsification, sampling, hypergraphs, uncertain graphs, SEM, PMEM, columnar Graphflow, BACH LSM-CSR.

**Hole:** an explicit DRAM | CXL/PMEM | NVMe *policy object* (HANA-style cost model exists on a card; Kutha has no placement ADR). Secondary indexes stay on Helix (`spec`). Learned CardEst stays queued.

### Time — strongest product fit, P0-complete as nouns

Covered: log=SoT, bitemporal, TOKI, intervals/AS OF, LSM snapshots, journeys, motifs, TARIS, RSP windows, Allen, OCPM, property streams, **vacuum + legal hold**.

**Hole is design, not papers:** ADR-011 lean event schema (encoding); decision-time as a third clock (explicitly skipped). Graphiti/Hindsight/Engram are *demand-high / optimality-low* contrasts, not missing physics.

### Verify — good receipts and provenance, weak *operational* replay harness

Covered: constant-size evidence, provenance semirings, why-not, PACT, MemLineage, fork, SSI, watermark, constraint repair, GED-as-diff.

**Hole:** ADR-060/061 are not opened; no `confidence: code` Kutha replay suite (Rocks WAL is the closest kernel). Blockchain ADS is closed as *anchor not SoT*.

### Composition — Cui named once, scheduling under V thin

Covered: max-convolution budgets, pack lifecycle, admission, MQO, Geo-Raft (replication ≠ SoT), CRDT (losers dropped).

**Hole:** ADR-031 “pack scheduling under V” is not a card. Admission (CASA-class) ≠ competition among materializers/agents for the same cascade budget. Composition is the *least often first-listed axis* (4 cards) despite 51 tags — it is a passenger on Query/Data.

### Agent — high demand, systematically *low optimality*

Covered as contrast: LLM-compiler, topic routing, Dify/oxify DAGs, Graphiti, Hindsight, Harvey, YOTG (`claim`), Raven in-DB agents (`spec`), enrichment facades.

**Hole:** ADR-050 meta-prompt + dictionaries has **no dedicated card**. Agent literature is either (a) “LLM outside trust boundary” (good) or (b) memory/RAG products that fail D1/D2 (traps). Capability security exists (object caps, WASM) but is not wired to dictionaries.

### Space — nouns exist, *module physics* does not

Covered: named graphs, WorldDB, tenants, packs, vertex-cut, GeoSPARQL, federation, identity (owl:sameAs virtual).

**Hole:** ADR-020/022 cargo slices and ports are engineering. Space in the matrix is “named graph / tenant / pack,” not vertical-slice cohesion. IndoorGML skipped on purpose.

### Security — enough *nouns*, almost no *kernel*

16 cards, **one `code` (oxify ReBAC)**, two `spec`. Path-ABAC, temporal grants, ReBAC, DP, STE, k-anonymity, tenant, vacuum holds, object caps, WASM, admission.

**Hole:** ADR-080/081 not opened. No CMK/air-gap research (STRATEGY buyer constraint, not a graph noun). No Kutha rewrite-enforcer. Security is the thinnest *implemented* axis, not the thinnest *idea* axis.

### Packaging / Vertical — overlays exist, portable capsule is med/med/med

Legal ADR-090 and science ADR-093 are open; law-nexus / daily-archive / Harvey / oxixml are overlays. RVF seal is **med/med/med**. k² and vacuum sit on Packaging as sidecars.

**Hole:** ADR-091 (RVF capsules) and ADR-092 (naming/license) unopened. Finance is a *tag on receipts/vacuum*, not a pack. Layer 5 never made finance a primary wedge.

---

## Synthesis by layer

### Layer 1 — raw idea

The corpus is **idea-complete for honeycomb nouns**. Remaining “ideas” in `wave-queue.md` are cousins, GPU, or GNN. Collecting more Layer-1 papers will not move Kutha.

Under-collected *as ideas* (by choice): HLC/causality clocks, regimes loop, agent dictionaries, CMK.

### Layer 2 — STCA applicability

The repeated verdict is one sentence: **log = SoT; everything else is a droppable lease; LLM compiles or proposes, never writes truth.**

That mapping is **well researched**. What is poorly researched is the *mechanical* Layer 2: which Port owns vacuum, which Port owns Leiden rebuild, how Cui scores compete. Those are ADR cells, not Consensus queries.

### Layer 3 — quality / cost

| Pattern | n | Meaning |
|---------|---|--------|
| H / M / H | 70 | Honeycomb packs: useful, not yet optimal, wanted |
| H / H / H | 42 | P0 physics: log, WCOJ/CSR, HNSW, IVM, TOKI, intervals, dictionaries, RPQ, isolation |
| D=high ∧ O=low | 10 | **Do not build these as core** (Graphiti, Dify, Hindsight, Harvey-as-SoT, CRDT, Geo-Raft-as-SoT, YOTG, empty Cypher, LegalSearch-R1 as engine) |

Optimality is **med on 97 cards** — the matrix says “we know the noun, we do not have a Kutha kernel.” That is the real research debt.

### Layer 4 — demand

Demand is **high on 128 cards** because intake selected demanded graph capabilities. It is not an independent market study. Treat Layer 4 as “engineers will ask for this,” not as TAM.

### Layer 5 — niche → effect

37 named niches, 126 `no niche`. Legal is **over-mapped** (statutory QA, path-ABAC, grants, vacuum/hold, k-anonymity, named graphs, MemStrata). Science is **present** (daily-archive, oxixml, AZ Cypher, RVF). Finance/clinical appear only as *receipt and vacuum* riders.

**Poorly researched at Layer 5:** finance pack as overlay; clinical beyond grants/receipts; journalism only on BANKS. That is GTM depth, not missing graph physics — and Layer 5 must not reopen intake.

---

## Three kinds of “unresearched”

Do not mix them.

### A. Literature-thin (few cards, still a honeycomb noun)

Already closed on purpose at R1–R6. **No further aggregator waves.** Residual thinness:

- Packaging (7) — capsules, vacuum, k², receipts; not a missing algorithm
- Security (16) — nouns complete; **implementation** missing
- Space (26) vs Query (112) — named-graph/tenant/pack vs join zoo; extra Space papers would be IndoorGML/federation cousins

### B. Kutha-design gaps (papers enough, ADR empty)

These are the actual unresearched *product* cells:

| Cell | Why the matrix does not finish it |
|------|-----------------------------------|
| 011 lean event schema | Encoding choice; OCPM + log=SoT surround it |
| 020/022 ports + slices | Engineering |
| 031 pack scheduling under V | Admission ≠ Cui competition |
| 043 single hybrid planner | Many access-path cards, no compiler |
| 050 dictionaries / meta-prompt | STRATEGY; zero dedicated card |
| 060/061 replay + fork harness | Verify papers exist; no Kutha code |
| 080/081 ABAC rewrite + sandbox wiring | Security nouns exist; one ReBAC crate |
| 091 RVF capsules | Card is med/med/med |

### C. Trap cluster (researched, must not be core)

Demand-high, optimality-low: Graphiti, Dify, oxify-DAG, Hindsight, Harvey-as-SoT, YOTG claim, CRDT graphs, Geo-Raft-as-SoT, RuVector empty Cypher, LegalSearch-R1-as-engine. Literature work here is **done**; further papers would only thicken the warning.

### D. Explicitly out (not holes)

GPU, GNN-as-MATCH, continuous subgraph matching, third temporal axis, CMK as a graph feature, another WCOJ, another path-ABAC paper.

---

## Confidence map (where evidence is weak)

- **`code` is 18%.** Hot path (Samyama CSR/LFTJ, RuVector HNSW, Rocks WAL) is the only kernel-grade cluster.
- Helix / Falkor / Raven / Tarantool remain **`spec`**. Do not raise to `code` without a tree.
- Agent axis is the **weakest evidence mix** (11 code but mostly orchestration/memory products; 1 claim).
- Security is **paper-shaped**. A rewrite-enforcer spike would change the axis more than Query 103.

---

## What to do next (outside this program)

1. **Do not Consensus-search.** Bound holds.
2. Graduate **Time 010–014 + Data 040–043** from H/H/H cards — that is P0.
3. Open **050 dictionaries** as design (not a paper hunt).
4. Open **080** from path-ABAC + temporal grants + vacuum holds as one security cell, not three products.
5. Treat Layer 5 legal density as GTM confirmation, not as a requirement to implement 37 niches.

The matrix is a **noun dictionary with STCA judgments**. Unresearched Kutha is the empty ADR bands and the missing kernels, not the empty arXiv.
