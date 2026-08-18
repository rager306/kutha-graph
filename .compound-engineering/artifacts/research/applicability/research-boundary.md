# Applicability research boundary

Date: 2026-08-18. Status: **literature bound closed** (R1–R6 have cards). Further Consensus waves only on a user URL or an explicit new noun.

This program is **not** “read every graph-database paper.” It is a bounded map of **distinct capability nouns** onto STCA / honeycomb axes, so ADR authors can judge ideas without treating the matrix as a shipping backlog.

Cards remain SoT; `matrix.md` is a rollup. Layer 5 is last, not an intake filter. Legal GTM is a wedge, not a corpus gate.

## Stop conditions (any one is enough)

1. The **in-scope remaining nouns** below are each closed as a five-layer card, queued as skip-unless with a named cousin, or marked “Kutha design, not a Consensus noun.”
2. Two consecutive Consensus waves yield **only cousins** of already-closed cards.
3. The user stops the literature loop or supplies a URL (URL scouts are extra and do not reopen the bound).

After the remaining nouns are closed, **do not invent new Consensus queries** from adjacent ML/systems papers. Graduate surviving cards to honeycomb ADRs only when asked.

Wave 1 of the plan is already done (`WAVE1-ACCEPTANCE.md`). Later waves were authorized continuations of the same card contract, not a new product.

## What is already closed (163 cards)

Coverage is by **noun**, not by paper count. Query/Data/Time are dense; Security and Packaging are thinner but not empty.

| Honeycomb band | Axis | Closed nouns (representative cards) | Do not re-open as a new slice |
|----------------|------|--------------------------------------|-------------------------------|
| 010–019 | Time | Log = SoT; bitemporal facts; invalidation/TOKI; intervals/AS OF; LSM snapshots; SEM/PMEM placement; Horae sketches; TVG journeys; motifs; TARIS ICM; RSP-QL windows; Allen constraints; OCPM; **log vacuum + legal hold** | Another “temporal graph DB” survey; Graphflow CSM; decision-time as a third clock |
| 020–029 | Space | Named graphs; WorldDB worlds; tenant slices; packs/plugins; vertex-cut partition; GeoSPARQL as predicate lease | Cargo layout; IndoorGML/CityGML; federated GeoSPARQL |
| 030–039 | Composition | Cui/max-convolution; pack lifecycle; admission; MQO; Geo-Raft (replication, not SoT); CRDT (losers dropped) | WAN analytics (RAGraph); P2P SPARQL |
| 040–049 | Data | CSR/GraphBLAS; HNSW + repair; hybrid access; CardEst; dictionary; k²; views; bulk load; CDC; summarization; sparsification; sampling AQP; hypergraphs; uncertain graphs | Secondary property indexes (Helix); HDT; learned GNN CardEst |
| 050–059 | Agent | LLM as compiler not executor; topic tool routing; enrichment facades; object capabilities; WASM sandbox; Dify/oxify orchestration (not SoT) | GNN packs; CHERI; ScopeGate-as-authz |
| 060–069 | Verify | Replay/fork; quantum/constant-size receipts; provenance semirings; why-not; PACT; MemLineage; constraint repair; watermark/fingerprint; GED as *diff* | Blockchain as SoT; image/CFG watermarks |
| 070–079 | Query | LFTJ/WCOJ family; Cypher/GQL; RPQ; SPARQL/SHACL/OBDA compile; SMQ; keyword BANKS; OLAP cubes; 2-hop reachability; result cache; FSM; **GED search**; **community lease** | Streaming RPQ (cousin of RPQ+RSP); PathDB algebra |
| 080–089 | Security | Path-ABAC; temporal grants; ReBAC; DP query noise; STE ciphertext lease; tenant isolation; k-anonymity publish; vacuum holds | Local/GNN DP; CMK/air-gap as a GTM checkbox |
| 090–099 | Vertical | Legal/science packs as *overlays* (law-nexus, daily-archive, Harvey, ADR-090/093); RVF seal | Other Harvey practice areas; finance pack as a new engine |

Vendor/code scouts already closed (do not clone again unless the tree changes): RuVector, Samyama, RocksDB, AgentDB, Helix/Falkor/Raven/Tarantool (spec), Graphiti, Cozo, Dify, Hindsight, oxixml/oxify/scirs, ULTRA, CRP-SpMM, TypeGraph, Open Ontologies, Engramx, Harvey LAB.

## In-scope remaining (cap was six nouns — **all closed**)

Each row was a **new honeycomb slice**. R1–R6 now have cards. **Do not mint Consensus Query 103+** unless the user supplies a URL or names a noun that is not a cousin of a closed card.

| # | Noun | Axes | Distinct from | Wave |
|---|------|------|---------------|------|
| R1 | Graph *anonymization* (k-degree / k-neighborhood publish) | Security, Data | DP noise; STE ciphertext; watermark ownership | **closed** Wave 33 `paper-graph-k-anonymity` |
| R2 | Time-series / STKG *property streams* on a graph | Time, Data, Query | TVG journeys (topology); motifs; interval indexes; RSP windows | **closed** Wave 33 `paper-stkg-property-streams` |
| R3 | Frequent subgraph *mining* (gSpan-class discovery) | Query, Data | MATCH a given pattern; BANKS keyword trees; temporal motifs | **closed** Wave 33 `paper-frequent-subgraph-mining` |
| R4 | Event-log *retention / vacuum / legal hold* (policy GC of SoT) | Time, Verify, Packaging | LSM snapshot placement; WAL crash recovery; SEM hot/cold vertices | **closed** Wave 34 `paper-event-log-vacuum-legal-hold` |
| R5 | Graph *similarity / GED* (distance between two graphs) | Query, Data | Subgraph iso yes/no; WCOJ similarity joins on tuples; ULTRA link scores | **closed** Wave 34 `paper-graph-edit-distance` |
| R6 | Community detection / clustering as a **droppable lease** | Query, Data | Vertex-cut partition for scale-out; summarization supernodes; OLAP cuboids | **closed** Wave 34 `paper-community-detection-lease` |

User-supplied URLs remain in-scope at any time and **do not consume** R1–R6.

## Not Consensus nouns (Kutha design)

These honeycomb titles are product/ADR work. Literature already surrounding them is enough; do not fire aggregator queries “for completeness.”

| Honeycomb | Why not another paper card |
|-----------|----------------------------|
| ADR-011 lean event schema | Covered by log=SoT + OCPM; schema is a Kutha encoding choice |
| ADR-020/022 cargo slices + ports | Engineering convention, not a graph capability |
| ADR-050 meta-prompt / agent dictionaries | STRATEGY + LLM-compiler + term dictionary; dictionaries are data, not a new join algorithm |
| ADR-062 regimes gated loop | Optional product control loop |
| ADR-092 naming & license | Packaging, not research |
| CMK / air-gap / enterprise checkbox | STRATEGY buyer constraint; not a matrix feature |

## Permanent skip classes

See `wave-queue.md`. Do not promote unless the noun is **not** a cousin:

- GPU / FPGA / quantum accelerators
- GNN / HGNN / embeddings as MATCH substitutes (ULTRA already owns scored KG lease)
- Continuous subgraph matching (Graphflow)
- Third temporal axis, fuzzy IA, RCC, IndoorGML
- Learned CardEst, HTTP SPARQL cache, GRaCe
- oxirs execution, oxixml HDT, oxify PQC placeholders
- Any second k² / second WCOJ / second path-ABAC / second HNSW-heal

## Out of this program

- Opening honeycomb ADRs (ADR-010+) from cards
- Implementing the Kutha engine or P0 spikes
- Jina full-text of papers already cited on a closed card
- Treating `confidence: paper` rows as a build order
- Re-scoring STRATEGY GTM from the matrix

## Operating rule after the bound

R1–R6 are closed. **Do not fire Consensus Query 103+** unless the user supplies a URL or names a noun that is not a cousin. If a result is a cousin, write it on the existing card or the queue — do not mint a new matrix row for a synonym. Cards stay 1:1 with `matrix.md`.
