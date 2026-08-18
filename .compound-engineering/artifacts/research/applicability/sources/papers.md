# Paper layer (Wave 1)

Consensus batch 2026-08-17: three queries, then **no further Consensus**. Jina follows on selected URLs.

## Query 1 — temporal graph SoT

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| TGMS: An Agent-Native Bi-Temporal Graph Management System | https://arxiv.org/abs/2607.10265 | LLM outside trust boundary; typed temporal operators; claim verifier vs trace | paper-tgms-operators |
| A model and query language for temporal graph databases (T-GQL) | https://doi.org/10.1007/s00778-021-00675-4 | Validity-interval property graphs; temporal paths | paper-tgql-intervals |
| Bitemporal Property Graphs to Organize Evolving Systems | https://arxiv.org/abs/2111.13499 | Bitemporal PG + event detection | paper-rost-bitemporal |
| Less Context, More Accuracy: Engram bi-temporal memory | https://arxiv.org/abs/2606.09900 | Dual-process write; invalidate-not-delete; as-of hybrid read | paper-engram-bitemporal-memory |
| Distributed temporal graph analytics with GRADOOP | https://doi.org/10.1007/s00778-021-00667-4 | TPGM bitemporal vertices/edges/collections | paper-gradoop-tpgm |
| A Scalable Approach to Historical Data Management via Optimized Transaction Logs | https://doi.org/10.1109/access.2025.3599972 | Temporal indexes over transaction logs | (queue) |

## Query 2 — WCOJ / materialization

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Leapfrog Triejoin | https://arxiv.org/abs/1210.0481 | WCOJ on tries; already in AGENTS.md | paper-lftj-wcoj |
| CompactLTJ | https://doi.org/10.1007/s00778-025-00945-5 | Compact tries; dynamism under updates | paper-compact-ltj |
| The Ring: WCOJ almost no extra space | https://doi.org/10.1145/3644824 | Cyclic triple index; wavelet columns | paper-ring-wcoj |
| Worst-Case Optimal BGPs on Temporal Graphs | https://arxiv.org/abs/2607.20356 | WCOJ + edge validity intervals | paper-temporal-wcoj-bgp |
| Free Join | https://doi.org/10.1145/3589295 | Unify WCOJ and binary joins | paper-free-join |
| Adopting WCOJ in RDBMS | https://doi.org/10.14778/3407790.3407797 | Hash WCOJ built at query time | (queue) |

## Query 3 — agents / provenance / trust boundary

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Confining Nondeterminism: AI-Driven Research as DBMS | https://arxiv.org/abs/2607.10508 | LLM as compiler not executor; versioned dataflow | paper-llm-compiler-not-executor |
| From Agent Traces to Trust (survey) | https://arxiv.org/abs/2606.04990 | Execution provenance graph | paper-agent-trace-survey |
| Auditable Agents | https://arxiv.org/abs/2604.05485 | Five auditability dimensions | (queue) |
| PACT: Provenance-Aware Capability Contracts | https://arxiv.org/abs/2605.11039 | Argument-level provenance vs whole-call trust | paper-pact-argument-provenance |
| Cordon: Semantic Transactions for Tool-Using Agents | https://arxiv.org/abs/2606.17573 | Task-scoped commit/rollback of effects | paper-cordon-semantic-tx |
| Proof-Carrying Certificates for LLM Pipelines | https://arxiv.org/abs/2605.16407 | Verify deterministic envelope, not the LLM | (queue) |

## Not Wave-1 vendors

Graphiti/Zep-class memory appears in query-1 papers as contrast, not as a new Wave-1 source. Cozo is indexed in CBM but stays queued unless Wave 1 closes.

Wave 1 closed five distinct paper capabilities (see `cards/paper-*.md`). Remaining seeds in the tables above live in `wave-queue.md` — not incomplete Wave-1 cards.

## Wave 2 — Consensus batch 2 (2026-08-17, user-authorized)

Three queries, no filters. Distinct capabilities closed as cards; rest queued.

### Query 4 — contradiction / isolation

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| TOKI | https://arxiv.org/abs/2606.06240 | Isolation-typed ⊕ operators; audit rows | paper-toki-contradiction-ops |
| WorldDB | https://arxiv.org/abs/2604.18478 | Nested worlds + edge programs (also Q5) | paper-worlddb-worlds-edge-programs |
| User as Code | https://arxiv.org/abs/2606.16707 | Append-only log + typed checkpoint | paper-user-as-code-log |
| Verified multi-agent concurrency anomalies | https://arxiv.org/abs/2606.17182 | Isolation lattice for agent runtimes | paper-mas-isolation-lattice |

### Query 5 — worlds / edge programs / derived graph

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| WorldDB | https://arxiv.org/abs/2604.18478 | Recursive worlds; CA nodes; write-time edges | paper-worlddb-worlds-edge-programs |
| GQL Rules | https://doi.org/10.1109/access.2026.3686122 | Deterministic derived-graph materialization | (queue) |
| Kumiho AGM revision | https://arxiv.org/abs/2603.17244 | AGM postulates on property-graph memory | (queue) |

### Query 6 — provenance / contracts / certificates

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| PACT | https://arxiv.org/abs/2605.11039 | Argument-level trust vs whole-call | paper-pact-argument-provenance |
| Proof-Carrying Certificates for LLM Pipelines | https://arxiv.org/abs/2605.16407 | Verify envelope, not the LLM | paper-proof-carrying-llm-envelope |
| MemLineage | https://arxiv.org/abs/2605.14421 | Chain-of-custody on agent memory | (queue) |
| From Agent Traces to Trust | https://arxiv.org/abs/2606.04990 | Execution provenance survey | (queue) |
| TOKI (again) | https://arxiv.org/abs/2606.06240 | Same as Q4 | paper-toki-contradiction-ops |

## Wave 3 — Consensus batch 3 (2026-08-17)

### Query 7 — Datalog / IVM / WCOJ

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| DBSP | https://doi.org/10.14778/3587136.3587137 | IVM calculus for SQL+Datalog | paper-dbsp-ivm |
| The Ring | https://doi.org/10.1145/3644824 | Compact WCOJ on triples | paper-ring-wcoj |
| FlowLog | https://arxiv.org/abs/2511.00865 | Incremental Datalog on Differential Dataflow | (queue) |
| Free Join | https://doi.org/10.1145/3589295 | Unify WCOJ and binary joins | paper-free-join |
| Hash WCOJ in RDBMS | https://doi.org/10.14778/3407790.3407797 | Build indexes at query time | (queue) |
| GPU WCOJ Datalog | https://arxiv.org/abs/2604.20073 | SIMT WCOJ; GPU temptation | (queue) |

### Query 8 — indexes as access paths

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| NaviX | https://doi.org/10.14778/3749646.3749704 | Native HNSW + predicate-agnostic kNN | paper-navix-filtered-hnsw |
| Mixed vector-relational access paths | https://doi.org/10.1145/3662010.3663448 | Scan vs probe by relational selectivity | paper-mixed-vector-relational-access |
| Annotative indexing | https://doi.org/10.54195/irrj.19910 | Unify inverted/column/object/graph indexes | paper-annotative-indexing |

### Query 9 — event-sourced graph

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| The Log is the Agent (ActiveGraph) | https://arxiv.org/abs/2605.21997 | Log SoT; graph fold; fork/replay | paper-activegraph-log-is-sot |
| Engram | https://arxiv.org/abs/2606.09900 | Bi-temporal KG; invalidate-not-delete | paper-engram-bitemporal-memory |
| T-GQL | https://doi.org/10.1007/s00778-021-00675-4 | Validity-interval PG + temporal paths | paper-tgql-intervals |
| Event-sourced query mechanisms | https://doi.org/10.31891/csit-2026-2-19 | Reconstruction vs temporal vs retroactive replay | paper-yankin-event-sourced-query |
| MemStrata stale-fact | https://arxiv.org/abs/2606.26511 | Deterministic supersession vs embedding RAG | paper-memstrata-stale-fact |

## Wave 4 — Consensus batch 4 (2026-08-17)

### Query 10 — bitemporal analytics

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| GRADOOP / TPGM | https://doi.org/10.1007/s00778-021-00667-4 | Collection-level bitemporal operators | paper-gradoop-tpgm |
| Rost bitemporal PG | https://arxiv.org/abs/2111.13499 | VT×TT + event detection | paper-rost-bitemporal |
| T-GQL (again) | https://doi.org/10.1007/s00778-021-00675-4 | already closed | paper-tgql-intervals |
| TGMS (again) | https://arxiv.org/abs/2607.10265 | already closed | paper-tgms-operators |

### Query 11 — compact / dynamic WCOJ

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| CompactLTJ | https://doi.org/10.1007/s00778-025-00945-5 | Compact tries + dynamism | paper-compact-ltj |
| Free Join | https://doi.org/10.1145/3589295 | Unify WCOJ and binary | paper-free-join |
| Hash WCOJ in RDBMS | https://doi.org/10.14778/3407790.3407797 | indexes at query time | paper-hash-wcoj-query-time |

### Query 12 — stale-fact / temporal validity retrieval

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| MemStrata | https://arxiv.org/abs/2606.26511 | Cosine cannot see contradiction | paper-memstrata-stale-fact |
| Engram (again) | https://arxiv.org/abs/2606.09900 | already closed | paper-engram-bitemporal-memory |
| Statutory temporal QA | https://arxiv.org/abs/2605.23497 | post-cutoff staleness + recency bias | paper-statutory-temporal-qa |
| STALE implicit conflict | https://arxiv.org/abs/2605.06527 | implicit conflict benchmark | paper-stale-implicit-conflict |

## Wave 5 — Consensus batch 5 (2026-08-17) + Cozo CBM

### Query 13 — derived rules / AGM

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| GQL Rules | https://doi.org/10.1109/access.2026.3686122 | stratified MERGE/ENRICH materialization | paper-gql-rules-materialization |
| Kumiho | https://arxiv.org/abs/2603.17244 | AGM postulates on PG memory | paper-kumiho-agm-revision |

### Query 14 — hash WCOJ / incremental Datalog

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Hash WCOJ in RDBMS | https://doi.org/10.14778/3407790.3407797 | query-time hash WCOJ | paper-hash-wcoj-query-time |
| FlowLog | https://arxiv.org/abs/2511.00865 | incremental Datalog / DD | paper-flowlog-incremental-datalog |

### Query 15 — memory custody / implicit conflict / traces

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| MemLineage | https://arxiv.org/abs/2605.14421 | Merkle chain-of-custody | paper-memlineage-memory-custody |
| STALE | https://arxiv.org/abs/2605.06527 | implicit conflict | paper-stale-implicit-conflict |
| Agent traces survey | https://arxiv.org/abs/2606.04990 | execution provenance graph | paper-agent-trace-survey |
| Multi-agent isolation lattice | https://arxiv.org/abs/2606.17182 | TLA+/Verus anomalies | paper-mas-isolation-lattice |

Cozo: `cozo-datalog-hnsw` from CBM `cozo` (`confidence: code`).

## Wave 6 — Consensus batch 6 (2026-08-17) + Graphiti CBM contrast

### Query 16 — mixed vector-relational / hybrid filtered search

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Mixed vector-relational access paths | https://doi.org/10.1145/3662010.3663448 | Scan vs probe by relational selectivity | paper-mixed-vector-relational-access |
| ACORN | https://doi.org/10.1145/3654923 | Predicate-subgraph HNSW traversal | paper-acorn-predicate-subgraph |
| NaviX (again) | https://doi.org/10.14778/3749646.3749704 | already closed | paper-navix-filtered-hnsw |
| Compass | https://arxiv.org/abs/2510.27141 | Cooperative filtered search, no new index | paper-compass-cooperative-hybrid |

### Query 17 — event-sourced query / retroaction

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Yankin event-sourced query mechanisms | https://doi.org/10.31891/csit-2026-2-19 | Four mechanism groups + cost envelopes | paper-yankin-event-sourced-query |
| R3 record-replay-retroaction | https://doi.org/10.14778/3611479.3611510 | SI transaction-granularity time travel | paper-r3-record-replay-retroaction |
| ActiveGraph (again) | https://arxiv.org/abs/2605.21997 | already closed | paper-activegraph-log-is-sot |
| Raphtory | https://doi.org/10.1016/j.future.2019.08.022 | Streaming temporal graph in memory | (queue; keep out of P0) |

### Query 18 — MAS isolation / Graphiti-class memory

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Verified MAS concurrency anomalies | https://arxiv.org/abs/2606.17182 | Isolation lattice L0–L4 | paper-mas-isolation-lattice |
| TOKI (again) | https://arxiv.org/abs/2606.06240 | already closed | paper-toki-contradiction-ops |
| CoAgent / SagaLLM | https://arxiv.org/abs/2606.15376 | Advisory concurrency; sagas | paper-coagent-mtpo |

Graphiti: `graphiti-bitemporal-fact-edges` from CBM `graphiti` (`confidence: code`, contrast — not SoT).

## Wave 7 — Consensus batch 7 (2026-08-17) + Pogocache CBM contrast

### Query 19 — annotative indexing

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Annotative Indexing | https://doi.org/10.54195/irrj.19910 | Unify inverted/column/object/graph | paper-annotative-indexing |
| Graphflow columnar GDBMS | https://doi.org/10.14778/3476249.3476297 | List-based processor; edge property pages | paper-graphflow-columnar-list |
| NaviX / Ring (again) | — | already closed | paper-navix-filtered-hnsw / paper-ring-wcoj |

### Query 20 — statutory temporal QA

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Asking For An Old Friend | https://arxiv.org/abs/2605.23497 | Post-cutoff staleness + recency bias | paper-statutory-temporal-qa |
| SAT-Graph RAG | https://doi.org/10.3233/faia251598 | Hierarchical temporal legal graph RAG | paper-sat-graph-legal-rag |
| Beyond Probabilistic Similarity | https://arxiv.org/abs/2606.09724 | RAG pathologies; bitemporal commitments | paper-sat-graph-legal-rag (companion) |
| MemStrata (again) | https://arxiv.org/abs/2606.26511 | already closed | paper-memstrata-stale-fact |
| LegalSearch-R1 | https://arxiv.org/abs/2605.25920 | RL temporal legal search agent | (queue; not engine) |

### Query 21 — Compass / filtered search poles

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Compass | https://arxiv.org/abs/2510.27141 | Cooperative execution, no new index | paper-compass-cooperative-hybrid |
| NaviX / ACORN (again) | — | already closed | paper-navix-filtered-hnsw / paper-acorn-predicate-subgraph |
| SIEVE | https://doi.org/10.14778/3749646.3749725 | Collection of predicate-form indexes | paper-sieve-index-collection |

Pogocache: `pogocache-ttl-kv-cache` from CBM `root-vendor-source-pogocache` (`confidence: code`, contrast — KV TTL cache, not SoT).

## Wave 8 — Consensus batch 8 (2026-08-17) + Samyama Raft / RuVector GNN CBM

### Query 22 — SIEVE / filtered-search poles

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| SIEVE | https://doi.org/10.14778/3749646.3749725 | Collection of predicate-form indexes | paper-sieve-index-collection |
| Compass / NaviX (again) | — | already closed | paper-compass-cooperative-hybrid / paper-navix-filtered-hnsw |
| iFVS / VecBench | arXiv 2607.22922 / SIGMOD 2026 | instance-optimized FVS; controllable FVS bench | paper-ifvs-instance-codebook / paper-vecbench-fvs |

### Query 23 — columnar GDBMS / Graphflow family

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| GraphflowDB columnar | https://doi.org/10.14778/3476249.3476297 | List-based processor; edge property pages | paper-graphflow-columnar-list |
| Graphflow 2017 active | https://doi.org/10.1145/3035918.3056445 | Delta Generic Join; SCA triggers | paper-graphflow-delta-generic-join |
| BACH LSM adj→CSR | https://doi.org/10.14778/3718057.3718076 | HGTAP aging adjacency into CSR | paper-bach-lsm-csr-bridge |
| Kùzu factorized WCOJ | (Jin et al., 2023) | GDBMS vision; factorization + WCOJ | paper-kuzu-factorized-wcoj |

### Query 24 — Raft HA

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Raft original | (Ongaro & Ousterhout, 2014) | Replicated log; cited on Samyama card | samyama-raft-ha |
| Geo-Raft survey | Computer Life 2026 | WAN single-leader pain | (queue; not P0) |

Samyama: `samyama-raft-ha` (`confidence: code`; `ExecuteQuery` apply is empty). RuVector: `ruvector-gnn-facade` (`confidence: code` on dummy scores).

## Wave 9 — Consensus batch 9 (2026-08-17)

### Query 25 — Kùzu / factorized WCOJ

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Kùzu | https://consensus.app/papers/details/5f24e277251d5b038572b8abdf7342bd/?utm_source=cursor | Factorized processor; binary + multiway WCOJ | paper-kuzu-factorized-wcoj |
| Hybrid binary+WCOJ optimizer | https://doi.org/10.14778/3342263.3342643 | Intersection-cost; Graphflow | paper-hybrid-wcoj-intersection-cost |
| Hash WCOJ (again) | https://doi.org/10.14778/3407790.3407797 | already closed | paper-hash-wcoj-query-time |
| ADOPT RL attribute orders | https://doi.org/10.14778/3611479.3611489 | Adaptive WCOJ order | paper-adopt-adaptive-wcoj-orders |

### Query 26 — iFVS / VecBench / engine FVS

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| iFVS | https://arxiv.org/abs/2607.22922 | Query+predicate-conditioned codebook | paper-ifvs-instance-codebook |
| VecBench | https://doi.org/10.1145/3802125 | Controllable FVS benchmark | paper-vecbench-fvs |
| FVS in PostgreSQL | https://doi.org/10.1145/3802011 | Library FVS ≠ engine I/O | paper-fvs-postgres-system-costs |
| Compass / SIEVE / NaviX (again) | — | already closed | — |

### Query 27 — CoAgent / sagas

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| CoAgent MTPO | https://arxiv.org/abs/2606.15376 | Advisory CC; saga inverses | paper-coagent-mtpo |
| Atomix | https://arxiv.org/abs/2602.14849 | Progress-aware tool txs | paper-coagent-mtpo (companion) |
| SagaLLM | https://consensus.app/papers/details/fb333f7884265bc493c0f90edc691ba5/?utm_source=cursor | Workflow sagas | paper-coagent-mtpo (related) |
| Isolation lattice (again) | https://arxiv.org/abs/2606.17182 | already closed | paper-mas-isolation-lattice |

## Wave 10 — Consensus batch 10 (2026-08-17)

### Query 28 — ADOPT / adaptive WCOJ orders

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| ADOPT | https://doi.org/10.14778/3611479.3611489 | RL episodes over attribute orders | paper-adopt-adaptive-wcoj-orders |
| SkinnerDB | https://doi.org/10.1145/3464389 | Regret-bounded join-order RL in-query | paper-adopt-adaptive-wcoj-orders (related) |
| Simple AQP vs learned QO | https://doi.org/10.1007/s00778-025-00936-6 | Adaptive may beat RL | paper-adopt-adaptive-wcoj-orders (caveat) |

### Query 29 — DuckDB factorization / predefined joins

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Adaptive factorization (DuckDB) | https://consensus.app/papers/details/387a6ee1730c53f9a65e5e917e32f6ff/?utm_source=cursor | Runtime WCOJ/factorization + sketches | paper-duckdb-adaptive-factorization |
| GRainDB predefined joins | https://doi.org/10.14778/3510397.3510400 | RID hash + SIP in DuckDB | paper-graindb-predefined-joins |
| Hash WCOJ / Free Join (again) | — | already closed | paper-hash-wcoj-query-time / paper-free-join |

### Query 30 — WCOJ + kNN

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| WCOJ similarity joins | https://doi.org/10.1145/3639294 | LTJ+Ring with in-join kNN | paper-wcoj-similarity-joins |
| LFTJ / CompactLTJ / Ring (again) | — | already closed | paper-lftj-wcoj / paper-compact-ltj / paper-ring-wcoj |

## User-supplied — arXiv 2608.12395v1 (2026-08-17)

Not a Consensus wave. HTML read via Jina. Three Consensus lookups (arxiv id; BIKG/agentic KG; exact title) did **not** return this preprint; related indexed papers listed below.

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Research Assistant: AstraZeneca’s Agentic System for R&D (Grabowski et al.) | https://arxiv.org/html/2608.12395v1 | Projected schema + deterministic Cypher rewrite; Observation envelopes | paper-az-projected-schema-cypher |
| same paper — Deep Research DAG | https://arxiv.org/html/2608.12395v1 | Judge-accepted DAG of questions; topo rewrite | paper-az-research-plan-dag |
| Biological Insights Knowledge Graph (Geleta et al., 2021) | https://consensus.app/papers/details/485261f969ba5a9ea8b3c3c63326656e/?utm_source=cursor | Underlying AZ KG; multi-projection consumption | (related; not a new Kutha SoT) |
| ChatInvent (He et al., 2026) | https://consensus.app/papers/details/4dc8cd75fd9d59a5943e992c0a05ae28/?utm_source=cursor | Sibling AZ agentic chemistry product | (contrast; not this stack) |

Queued from the same technical note: topic-modeled tool routing vs embedding-NN exemplars; sentence-level RelEx literature index.

## User-supplied — Year of the Graph Vol. 31 (2026-08-17)

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Layers of Meaning: Context Graphs, Graph Memory, Ontologies for AI | https://yearofthegraph.xyz/newsletter/2026/06/layers-of-meaning-context-graphs-graph-memory-and-ontologies-for-ai-the-year-of-the-graph-newsletter-vol-31-summer-2026/ | Context graph as mash-up of KG + traces + ontology + memory | claim-yotg-context-graph-layers |

## Wave 13 — Consensus batch 13 (2026-08-17)

### Query 37 — object-centric process mining / event KG

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| OCPM fabric of real processes (van der Aalst 2023) | https://consensus.app/papers/details/0626e2d220cc5159a6b5524f7c681d82/?utm_source=cursor | OCED; multi-object events | paper-ocpm-multi-object-events |
| Divergence and convergence (2019) | https://consensus.app/papers/details/c2fb4177602c5eaaa951855dd9062592/?utm_source=cursor | Flattening destroys information | paper-ocpm-multi-object-events |
| Cases as graphs (Adams 2022) | https://consensus.app/papers/details/c493b0af1a8c54e0a3d11808d031de1e/?utm_source=cursor | Process executions = graphs | paper-ocpm-multi-object-events |
| EKG → OCEL | https://consensus.app/papers/details/c88f7e929fab510981c30c8a7a27f768/?utm_source=cursor | Two models of multi-object logs | paper-ocpm-multi-object-events |

### Query 38 — no-merge / virtual identity

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| SAGE no-merge KG | https://consensus.app/papers/details/11c30e1788c659b48fbcd505aca3e426/?utm_source=cursor | Validated virtual links; no entity merge | paper-sameas-virtual-identity |
| When owl:sameAs isn't the Same | https://consensus.app/papers/details/b69036544207504088bede88036eb42a/?utm_source=cursor | Identity links weaker than sameAs | paper-sameas-virtual-identity |
| The sameAs Problem survey | https://consensus.app/papers/details/58135a57d66d51898a188381e0f5ab80/?utm_source=cursor | Identity management on the Web | paper-sameas-virtual-identity |
| identiConTo | https://consensus.app/papers/details/c27ac9ee4bb65e93b79fbb0dba074f5d/?utm_source=cursor | Contextual identity | paper-sameas-virtual-identity |
| Kirielle complex ER (again) | — | merge-as-temporal-fold pole | paper-temporal-complex-entity-resolution |

### Query 39 — semantic metrics layer vs KG

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Semantic-layer-mediated NL2SQL (SMQ) | https://consensus.app/papers/details/d8b879deaea55d08a8ec53b4e14c2c1f/?utm_source=cursor | IR + deterministic SQL compiler | paper-semantic-layer-smq |
| Iceberg NL toolkit | https://consensus.app/papers/details/b7da6cd637265ca991d46c1f9dcb624d/?utm_source=cursor | YAML metrics; injection-safe | paper-semantic-layer-smq |
| SemanticQuark | https://consensus.app/papers/details/bb655655b771566fbe04f75a27328db6/?utm_source=cursor | Python metric graph | paper-semantic-layer-smq |
| CoeusBI | https://consensus.app/papers/details/283bd121b81459d697f4eefb3e8b0357/?utm_source=cursor | Dual-agent + SQL compiler | paper-semantic-layer-smq |

## User-supplied — Dify (2026-08-18)

Not a Consensus wave. GitHub + docs + raw Python (Dataset/VDB). No CBM index.

| source | url | capability seed | card id |
|--------|-----|-----------------|---------|
| Dify (langgenius/dify) | https://github.com/langgenius/dify | Visual workflow + RAG knowledge as extra “truth”; pluggable VDB | dify-workflow-rag-orchestration |

## Wave 14 — Consensus batch 14 (2026-08-18)

Three queries, no filters. Distinct capabilities closed as cards; rest queued.

### Query 40 — approximate temporal graph sketches

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Horae (Chen et al., 2022) | https://consensus.app/papers/details/20047a5fd7f956688bcc90e8cc4bbc4c/?utm_source=cursor | Time-prefix multilayer sketch; BRD | paper-horae-temporal-sketches |
| GRIT (Hu et al., 2025) | https://consensus.app/papers/details/14e85b03745d59b28f18875d688f1962/?utm_source=cursor | FlatIndex; lazy update; GBD | paper-horae-temporal-sketches |
| HIGGS (Zhao et al., 2024) | https://consensus.app/papers/details/1df556d901fe52baad75ebc55187be1b/?utm_source=cursor | Bottom-up hierarchy; localize conflicts | paper-horae-temporal-sketches |
| PGSS-MDC (Jia et al., 2023) | https://consensus.app/papers/details/500a30384e935f02abae75dc2d5e0ace/?utm_source=cursor | Persistent past-range counters | paper-horae-temporal-sketches |
| GSS / HourglassSketch / Sliding-ITeM | various | topology sketches; sliding window | paper-horae-temporal-sketches (cousins) |
| Crane neural sketch | https://arxiv.org/abs/2602.15360 | hierarchical neural sketch | (cousin; not SoT) |
| Raphtory / T-GQL / GRADOOP | — | already closed (exact, not sketch) | paper-raphtory-lazy-temporal-views et al. |

### Query 41 — property graph schema / constraints

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| PG-Schema (Bonifati et al., 2023) | https://consensus.app/papers/details/5d19ee4316cd554686dab6e91c458b76/?utm_source=cursor | PG-Types + PG-Keys; strict/loose | paper-pgschema-types-keys |
| PG-Keys (Angles et al., 2021) | https://consensus.app/papers/details/d60f88ee9b02545faa63950a3bee6e22/?utm_source=cursor | exclusive/mandatory/singleton keys | paper-pgschema-types-keys |
| SHACL / ShEx / PG-Schema foundations | https://consensus.app/papers/details/8a660dc171215943beeed8b65fe47ee7/?utm_source=cursor | common constraint core | paper-pgschema-types-keys |
| Schema validation & evolution (2019) | https://consensus.app/papers/details/312978432b0b54bbb3609c1aee0cf42f/?utm_source=cursor | descriptive vs prescriptive; rewriting | paper-pgschema-types-keys (cousin) |
| PG-HIVE | https://consensus.app/papers/details/27ccf5a0457455989c7aad8a9665c169/?utm_source=cursor | incremental schema discovery | (queue: emit dict events) |
| Repairing under PG-Constraints | https://consensus.app/papers/details/6f954c7a709754d3be3c395f7f745e39/?utm_source=cursor | topology repair / deletions | (queue: log violations, don’t silent-delete) |
| GQL Rules / SMQ | — | already closed | paper-gql-rules-materialization / paper-semantic-layer-smq |

### Query 42 — CRDT / conflict-free replicated graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| CRDTs (Shapiro et al., 2011 SSS) | https://consensus.app/papers/details/7f88eff91e1a5133844c2229ab446cd8/?utm_source=cursor | SEC; Graph datatype | paper-crdt-graph-eventual |
| CvRDT comprehensive study | https://consensus.app/papers/details/f1d8ce4587bd5e38b579ed8644bf3a4d/?utm_source=cursor | graphs, monotonic DAGs, sequences | paper-crdt-graph-eventual |
| CRDT overview (Preguiça 2018) | https://consensus.app/papers/details/e46ce1818efd50fc8e5563c154842cb9/?utm_source=cursor | add-wins/LWW; CALM queries unsafe | paper-crdt-graph-eventual |
| DAG CRDTs (Borth et al., 2025) | https://consensus.app/papers/details/03ac4f6ed6075277918ed40a68a6c44f/?utm_source=cursor | cycle compensation | paper-crdt-graph-eventual |
| Hypergraph CRDTs (Bansal 2022) | https://consensus.app/papers/details/ed0d9076d20e530291b07df87d8f41c8/?utm_source=cursor | partial replica hypergraphs | paper-crdt-graph-eventual |
| Log-based CRDT (Saquib 2022) | https://consensus.app/papers/details/1995eb9113ad5a10a8a72c5e1e93243e/?utm_source=cursor | log first; undo; versions | paper-crdt-graph-eventual (cousin) |
| Keep CALM and CRDT On | https://consensus.app/papers/details/1eae8090d3ef5714b03d1c8616cc13f7/?utm_source=cursor | queries over CRDTs unconstrained | paper-crdt-graph-eventual (cousin) |
| Janus reliable CRDTs / ERA | various | BFT/epoch arbitration | (queue: not P0; consensus returns) |

## Wave 15 — Consensus batch 15 (2026-08-18)

Three queries, no filters. Distinct capabilities closed as cards; rest queued.

### Query 43 — topic-modeled / catalog tool routing

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| kNN vs learned LLM routers (Li 2025) | https://consensus.app/papers/details/565e0d2b8e17550589d0df6f3cc1c2ea/?utm_source=cursor | locality; simple kNN wins | paper-topic-modeled-tool-routing |
| Dynamic routing/cascading survey | https://consensus.app/papers/details/51fb2ef93d885ad595c750ecb069622e/?utm_source=cursor | multi-LLM routing taxonomy | paper-topic-modeled-tool-routing |
| Routing strategies survey (JAIR) | https://consensus.app/papers/details/fa77b886b6035006b1b58981ef5ea52c/?utm_source=cursor | pre-generation routing | paper-topic-modeled-tool-routing |
| Multi-agent set-valued routing | https://consensus.app/papers/details/131303c17a36538883edfa1654616c9c/?utm_source=cursor | over-select costs; supervised > NN | paper-topic-modeled-tool-routing |
| AgentRouter KG-GNN | https://consensus.app/papers/details/6b9b81df59e3573a8f2a5932a87a6c93/?utm_source=cursor | query+agent graph router | paper-topic-modeled-tool-routing |
| Tool-to-Agent Retrieval | https://consensus.app/papers/details/f4a0f5370887581cb2619fa5ac15fbdf/?utm_source=cursor | embed tools not just agents | paper-topic-modeled-tool-routing |
| AZ topic map (arXiv 2608.12395v1) | — | dropped cosine-NN exemplars | paper-topic-modeled-tool-routing |

### Query 44 — Geo-Raft / Multi-Raft

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Raft and Beyond (Ding 2026) | https://consensus.app/papers/details/47154398e77451f1b34581c951493805/?utm_source=cursor | WAN RTT; Multi-Raft; Cockroach/TiKV | paper-geo-raft-wan |
| BW-Raft (Du 2023) | https://consensus.app/papers/details/0433151dd1c558f49b2d26e159befe4d/?utm_source=cursor | secretary/observer scale-out | paper-geo-raft-wan |
| Elastic Geo-Raft (Xu 2019) | https://consensus.app/papers/details/2592f8d1739f5e2ba1d6f87a45fe5d94/?utm_source=cursor | same abstractions; spot instances | paper-geo-raft-wan |
| CD-Raft | https://consensus.app/papers/details/39db22d9f25951e089072b690034c247/?utm_source=cursor | cross-domain leader placement | paper-geo-raft-wan |
| GeoCoCo | https://consensus.app/papers/details/c4bf8227cd045c689ff0ee36531bef3d/?utm_source=cursor | WAN sync grouping/pruning | paper-geo-raft-wan |
| RAGraph geo graph processing | https://consensus.app/papers/details/5f3834ce25f85c5086d74c4d75138444/?utm_source=cursor | WAN analytics, not consensus | (queue: query-side geo) |
| samyama-raft-ha / CRDT graph | — | already closed | samyama-raft-ha / paper-crdt-graph-eventual |

### Query 45 — blockchain-assisted graph ADS

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| PAGB (Wu et al., 2023) | https://consensus.app/papers/details/a6b00d73fe9d59a091b3fcaf6a1d14ac/?utm_source=cursor | RSA accumulator; graph queries; privacy | paper-blockchain-graph-ads |
| vChain (Xu et al., 2019) | https://consensus.app/papers/details/1c3931305dd1535ab08ca15fb8ff5e2f/?utm_source=cursor | verifiable Boolean range on chain DBs | paper-blockchain-graph-ads |
| NetChain | https://consensus.app/papers/details/b13057a05426591fa66f821ac7de8b27/?utm_source=cursor | authenticated top-k graph queries | paper-blockchain-graph-ads |
| VGQ | https://consensus.app/papers/details/023fb668ee5056409d69280cb23576bc/?utm_source=cursor | graph queries; chain storage unchanged | paper-blockchain-graph-ads |
| VeriDKG | https://consensus.app/papers/details/e2e2f6c6401156b1810a8a4a71cb72fb/?utm_source=cursor | SPARQL + RGB-Trie ADS | paper-blockchain-graph-ads |
| Authenticated subgraph match | https://consensus.app/papers/details/470e61a23dff5de29bc4c9684773afba/?utm_source=cursor | MELTree; off-chain graphs | paper-blockchain-graph-ads |
| constant-size evidence | — | already closed (local receipts) | paper-constant-size-evidence |

## Wave 16 — Consensus batch 16 (2026-08-18)

Three queries, no filters. Distinct capabilities closed as cards; rest queued.

### Query 46 — RL / agentic temporal legal search

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| LegalSearch-R1 (Fan et al., 2026) | https://consensus.app/papers/details/49ec0da332b9534c94ff389527b33ed8/?utm_source=cursor | RL; local statute RAG + web; temporal consistency | paper-legalsearch-r1-temporal-agent |
| Search-R1 (Jin et al., 2025) | https://consensus.app/papers/details/58192257da745001bdf401595ac7eaf2/?utm_source=cursor | generic RL search-engine agent | paper-legalsearch-r1-temporal-agent |
| Long-horizon legal search RL | https://consensus.app/papers/details/3a2cee02b6a35e57a72d9863cc3c3645/?utm_source=cursor | multi-turn legal document search | paper-legalsearch-r1-temporal-agent |
| LegalMALR | https://consensus.app/papers/details/ada2a27369aa5398a0725bde7b7a9162/?utm_source=cursor | multi-agent rewrite + LLM rerank | paper-legalsearch-r1-temporal-agent |
| LRAS | https://consensus.app/papers/details/310e64c2201053e18827b0ec948137cd/?utm_source=cursor | closed-loop vs active inquiry | paper-legalsearch-r1-temporal-agent |
| statutory QA / SAT-Graph RAG | — | already closed | paper-statutory-temporal-qa / paper-sat-graph-legal-rag |

### Query 47 — incremental PG schema discovery

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| PG-HIVE (Sideri et al., 2025) | https://consensus.app/papers/details/27ccf5a0457455989c7aad8a9665c169/?utm_source=cursor | LSH + incremental type mining | paper-pghive-schema-discovery |
| PG-HIVE EDBT demo | https://consensus.app/papers/details/0b100f91884f517c99fa292555bc01c1/?utm_source=cursor | same capability | paper-pghive-schema-discovery |
| DiscoPG (Bonifati et al., 2022) | https://consensus.app/papers/details/cb56a1d8d6ff5d7a9b293b497be847bc/?utm_source=cursor | first PG schema discovery; GMM | paper-pghive-schema-discovery |
| GMMSchema | https://consensus.app/papers/details/845dec9579485c5195debc53dcc175f8/?utm_source=cursor | hierarchical GMM labels+props | paper-pghive-schema-discovery |
| PG-Schema types/keys | — | already closed (prescriptive) | paper-pgschema-types-keys |

### Query 48 — repairing graphs under constraints

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Repairing under PG-Constraints | https://consensus.app/papers/details/6f954c7a709754d3be3c395f7f745e39/?utm_source=cursor | denial+recursive; ILP/greedy; label delete | paper-pg-constraint-repair |
| User-centric PG repairs | https://consensus.app/papers/details/abfdb34f3426549bb47ba25e61089188/?utm_source=cursor | HITL; independent sets | paper-pg-constraint-repair |
| Grafixer | https://consensus.app/papers/details/96baf3676e7651f49d3e60840f6e3fa1/?utm_source=cursor | Cypher constraints; multi-user UI | paper-pg-constraint-repair |
| Certain fixes (Fan 2019) | https://consensus.app/papers/details/a05134e0c0b65d5f860be72a18f1e519/?utm_source=cursor | Church-Rosser; rules + ground truth | paper-pg-constraint-repair |
| GRR / δ-GRR | https://consensus.app/papers/details/f2f88fbdc2ff5e0cb1d4e08df6eea4f0/?utm_source=cursor | incompleteness/conflict/redundancy | paper-pg-constraint-repair |
| LLM graph repair | https://consensus.app/papers/details/3cea543ff082576abf8a1a0312ad4633/?utm_source=cursor | LLM suggestions; not trusted | (cousin; LLM not SoT) |

## Wave 17 — Consensus batch 17 (2026-08-18)

Three queries, no filters. Distinct capabilities closed as cards; RDF-star / SHACL-DS / counting paths queued as cousins.

### Query 49 — ontology versioning / temporal OWL

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Instance-driven schema versioning (Brahmia et al., 2021) | https://consensus.app/papers/details/3d50ea159bbc5a0b88ad240f4aa01483/?utm_source=cursor | non-conservative instances mint a schema version | paper-ontology-temporal-versioning |
| τOWL temporal schema versioning (Zekri et al., 2016) | https://consensus.app/papers/details/080a93536ea950189ea37d2dfb1e6a77/?utm_source=cursor | conventional schema + annotations; schema and instances | paper-ontology-temporal-versioning |
| τOWL framework (Journal on Data Semantics) | https://consensus.app/papers/details/3d580a4da08f584080bdf544386539ef/?utm_source=cursor | DOI: 10.1007/s13740-016-0066-3 | paper-ontology-temporal-versioning |
| Deep-time KG version graph (Ma 2020) | https://consensus.app/papers/details/b06e7de8ff545754b4196f91790061b5/?utm_source=cursor | version graph + SPARQL | paper-ontology-temporal-versioning |
| Change history ontology (Khattak 2012) | https://consensus.app/papers/details/2d14ba260ef45274b7614f1a584ad0b5/?utm_source=cursor | change as ontology | paper-ontology-temporal-versioning |
| KGCL (Hegde et al., 2024) | https://consensus.app/papers/details/dc032be0a36154b1a0a6a2297be2406b/?utm_source=cursor | CNL diff/patch; DOI: 10.1093/database/baae133 | paper-ontology-temporal-versioning |
| Valid Ontology (Grandi, 2009) | https://consensus.app/papers/details/e2c2613f2e30572f833ef171e901d1d0/?utm_source=cursor | multi-version OWL in one temporal XML | paper-ontology-temporal-versioning |
| Historical KG of ontology versions (Cardoso et al., 2020) | https://consensus.app/papers/details/37b505a497da579394b7a6f1bdd7b5ed/?utm_source=cursor | all versions in one graph | paper-ontology-temporal-versioning |
| Fine-grained OWL component versioning (Fabry 2023) | https://consensus.app/papers/details/9b29a2b9168959ba888ac0d10f902183/?utm_source=cursor | component-level OWL | paper-ontology-temporal-versioning |

### Query 50 — OBDA / compile ontology to SPARQL

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Ontop (Calvanese et al., 2016) | https://consensus.app/papers/details/2ea01ad7a4ba59729ec8b9430f9c307a/?utm_source=cursor | SPARQL over RDB; OWL2QL + mappings; virtual rewrite; DOI: 10.3233/sw-160217 | paper-obda-ontology-compile |
| Quest OWL2QL (Rodriguez-Muro 2012) | https://consensus.app/papers/details/64dfcb1097365c028d7733bef585e820/?utm_source=cursor | same family | paper-obda-ontology-compile |
| UltrawrapOBDA (Sequeda et al., 2014) | https://consensus.app/papers/details/2cb204a574ee5610820c0a55a5d8ea2c/?utm_source=cursor | rewrite and selective materialization | paper-obda-ontology-compile |
| PerfectMap (Di Pinto 2013) | https://consensus.app/papers/details/c1fc9c496bba5de6bb891483bfd4f9e2/?utm_source=cursor | mapping-rewrite explosion | paper-obda-ontology-compile (cost); queue |
| Statoil industrial OBDA (Kharlamov et al., 2017) | https://consensus.app/papers/details/16b163e04f7e5a579322733dd051934d/?utm_source=cursor | industrial mappings | paper-obda-ontology-compile |
| Mapping-translation next-gen OBDA (Corcho 2020) | https://consensus.app/papers/details/1d04333570d55af89b9643281a147772/?utm_source=cursor | mapping translation | paper-obda-ontology-compile |
| Schema-agnostic SPARQL 1.1 rewriting | https://consensus.app/papers/details/1fbc7eeb15305d26b46ba37dbb87a949/?utm_source=cursor | ontology stored with data | paper-obda-ontology-compile |
| AZ Cypher / SMQ | — | already closed (different compile targets) | paper-az-projected-schema-cypher / paper-semantic-layer-smq |

### Query 51 — SPARQL + SHACL compile

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Wikidata constraints as SHACL/SPARQL (Ferranti et al., 2024) | https://consensus.app/papers/details/5eee55207fb258db90e18f6f8c094857/?utm_source=cursor | SHACL-Core insufficient; SPARQL covers 32 types; DOI: 10.3233/sw-243611 | paper-shacl-sparql-compile |
| SHACL over SPARQL endpoints (Corman et al., 2019) | https://consensus.app/papers/details/7220c4725a2c5164bbc84ac492e662c3/?utm_source=cursor | non-recursive = one query; recursive NP-hard + SAT | paper-shacl-sparql-compile |
| SPARQL 1.1 property paths (Kostylev 2015) | https://consensus.app/papers/details/33bd6a44aec357e092f20fef09628c2e/?utm_source=cursor | eval OK; containment harder | paper-shacl-sparql-compile |
| Counting semantics of property paths (Arenas et al., 2012) | https://consensus.app/papers/details/53a72c24c96459418bcb8da1e077d52c/?utm_source=cursor | counting can make paths intractable | paper-shacl-sparql-compile (warning); queue |
| SHACL-DS (Chiem Dao 2025) | https://consensus.app/papers/details/e559eb3a44f8591f980ef6bc4e045433/?utm_source=cursor | RDF datasets / named graphs | paper-shacl-sparql-compile; queue |
| Re-SHACL (Ke 2024) | https://consensus.app/papers/details/aff6b3101aa7571090add65f1e608915/?utm_source=cursor | targeted reasoning before validate | paper-shacl-sparql-compile |
| Magic shapes (Ahmetaj 2022) | https://consensus.app/papers/details/5d5f31f8edec5a59a95db7eb15c26bfb/?utm_source=cursor | full recursive+negation SHACL | paper-shacl-sparql-compile |
| SHACL2SPARQL | https://consensus.app/papers/details/01b947aacda8524092d66b2f2fa52896/?utm_source=cursor | shapes compile to SPARQL | paper-shacl-sparql-compile |
| SHACL→SPARQL any endpoint (Özçep 2024) | https://consensus.app/papers/details/d38f663a4c1151d1aac13331ba9f4c7e/?utm_source=cursor | validator against any SPARQL endpoint | paper-shacl-sparql-compile |
| PG-Schema types/keys | — | already closed (schema language, not SPARQL compile) | paper-pgschema-types-keys |

## Wave 18 — Consensus batch 18 (2026-08-18)

Three queries, no filters. Janus/ERA, RAGraph, AReBAC left queued (cousins of closed cards). Distinct capabilities closed.

### Query 52 — TVG journeys / temporal reachability

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Δ-restless temporal paths (Casteigts et al., 2021) | https://consensus.app/papers/details/b57d5f7e265757e893950cd599ba871e/?utm_source=cursor | unrestricted P; restless W[1]-hard; DOI: 10.1007/s00453-021-00831-w | paper-tvg-journeys-restless |
| Interval vs contact-sequence paths (Jain et al., 2022) | https://consensus.app/papers/details/adfa296b2d5957d1a596aedec1f2ed34/?utm_source=cursor | interval superset; some NP vs P | paper-tvg-journeys-restless |
| Temporal reachability index (Wu et al., 2016) | https://consensus.app/papers/details/5869e75972ec554a896f053a85918657/?utm_source=cursor | ICDE path/reachability index | paper-tvg-journeys-restless |
| Timed transitive closure (Brito et al., 2021) | https://consensus.app/papers/details/d3fd6bea19d45e1db9a617fd44acd80f/?utm_source=cursor | unsorted contact insert | paper-tvg-journeys-restless |
| Temporal reachability graphs (Whitbeck et al., 2012) | https://consensus.app/papers/details/c2874d3dbde153f4af3c8da54cbb3aa9/?utm_source=cursor | (τ,δ)-journeys | paper-tvg-journeys-restless |
| Temporal networks (Kempe et al., 2000) | https://consensus.app/papers/details/ecb5b839681a52549a6c6678a3461d32/?utm_source=cursor | time-respecting paths; Menger fails | paper-tvg-journeys-restless |
| Counting temporal paths (Enright et al.) | https://consensus.app/papers/details/f2280f40c8195171b277cf068016b976/?utm_source=cursor | #P; betweenness | (queue) |
| Span-reachability (Wen et al., 2021) | https://consensus.app/papers/details/6767f4adec535c31bd5a885727a760c7/?utm_source=cursor | order-relaxed window | (queue) |
| T-GQL / TARIS | — | already closed (QL / streaming ICM) | paper-tgql-intervals / paper-taris-incremental-icm |

### Query 53 — graph partitioning / sharding

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| CUTTANA (Rezaei Hajidehi et al., 2023) | https://consensus.app/papers/details/5f8c8288aab958f699f5fd507388c495/?utm_source=cursor | streaming + coarsen; GDBMS throughput; DOI: 10.14778/3696435.3696437 | paper-graph-partition-vertex-cut |
| Property-graph storage-volume cut (Cui et al., 2024) | https://consensus.app/papers/details/474b2bce1b4c5a138905e09518c09648/?utm_source=cursor | bytes ≠ |V|/|E| | paper-graph-partition-vertex-cut |
| MPC minimum property-cut (Peng et al., 2022) | https://consensus.app/papers/details/a542bbf1c49456209c406e1fa8c9a39b/?utm_source=cursor | minimize crossing predicates | paper-graph-partition-vertex-cut |
| PowerLyra hybrid-cut (Chen et al., 2019) | https://consensus.app/papers/details/9c188567ad845efbbb5889d7af7747b7/?utm_source=cursor | edge-cut low-degree + vertex-cut hubs | paper-graph-partition-vertex-cut |
| Gluon/CVC study (Gill et al., 2018) | https://consensus.app/papers/details/501ae148b7fe53a486dffbf8ea326586/?utm_source=cursor | pattern beats volume at scale | paper-graph-partition-vertex-cut |
| Historical-graph partition (Spitalas et al., 2025) | https://consensus.app/papers/details/be6caf65b59557afb1eabfaad6a96ec9/?utm_source=cursor | time-weighted cuts | paper-graph-partition-vertex-cut |
| Local-first GDBMS RDTs (Pandey et al., 2025) | https://consensus.app/papers/details/06fdb1b0988457cc81ac889f78ffbf8b/?utm_source=cursor | availability vs graph constraints | (cousin of CRDT) |
| Geo-Raft / RAGraph | — | already closed / queued | paper-geo-raft-wan |

### Query 54 — why / why-not provenance

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| PUG (Lee et al., 2018) | https://consensus.app/papers/details/500a412f6a4c51a59361bd5c93784415/?utm_source=cursor | why+why-not FO+negation; DOI: 10.1007/s00778-018-0518-5 | paper-why-not-query-provenance |
| Why-not polynomials (Bidoit et al.) | https://consensus.app/papers/details/a46f4a4a19dd5212ad6b10fefeb078e5/?utm_source=cursor | tree-independent explanations | paper-why-not-query-provenance |
| NedExplain (Bidoit et al., 2014) | https://consensus.app/papers/details/6e357c6395e558009608a25721b291f1/?utm_source=cursor | missing-answer query components | paper-why-not-query-provenance |
| SPARQLprov (Hernández et al., 2021) | https://consensus.app/papers/details/e30b7f19ca5a5adb81acc53e0cbf02ff/?utm_source=cursor | how-polynomials via SPARQL rewrite | paper-why-not-query-provenance |
| HUKA (Gaur et al., 2020) | https://consensus.app/papers/details/cfa36c0c9cba52f09a1cd89eda175fca/?utm_source=cursor | maintain polynomials on dynamic KG | paper-why-not-query-provenance |
| Erebus (Palyvos-Giannas et al., 2022) | https://consensus.app/papers/details/1b6a75fe838f5667a393c5495411a8a6/?utm_source=cursor | streaming missing-answer expectations | paper-why-not-query-provenance |
| Causality/responsibility (Meliou et al., 2010) | https://consensus.app/papers/details/868a1a72047650f7ab936f8489ab0efb/?utm_source=cursor | causes ⊆ lineage; CQ PTIME | paper-why-not-query-provenance |
| Why-provenance Datalog (Calautti et al., 2023) | https://consensus.app/papers/details/5621ee37dabf5938870ef3a4a9628b99/?utm_source=cursor | recursive why intractable | paper-why-not-query-provenance |
| PACT / MemLineage / receipts | — | already closed (different provenance nouns) | paper-pact-argument-provenance et al. |

## Wave 19 — Consensus batch 19 (2026-08-18)

Three queries, no filters. User URL Hindsight scouted in code (not a Consensus slot). BACH already closed — Q57 card is snapshot *placement* + time-aware compaction.

### Query 55 — ISO GQL / SQL/PGQ

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| GPML in GQL and SQL/PGQ (Deutsch et al., 2022) | https://consensus.app/papers/details/6146441f8e1d5d068411e0685f54604f/?utm_source=cursor | shared pattern-matching core; DOI: 10.1145/3514221.3526057 | paper-iso-gql-gpml |
| Core PGQ vs Core GQL (Gheerbrant et al., 2024) | https://consensus.app/papers/details/1d2ec7de402354a99c2b1c4dedb538a2/?utm_source=cursor | bottom-up vs pipelined; v1 gaps | paper-iso-gql-gpml |
| GPC (Francis et al., 2023) | https://consensus.app/papers/details/607a2b3e84435df3b4367824ca88c089/?utm_source=cursor | pattern calculus beyond RPQ | paper-iso-gql-gpml |
| Cypher vs all RPQs (Gheerbrant et al., 2025) | https://consensus.app/papers/details/f21b7cc2f07a54c08950310604226872/?utm_source=cursor | Cypher cannot express all RPQs | paper-iso-gql-gpml |
| Path-based algebra (Angles et al., 2024) | https://consensus.app/papers/details/fd2d6ee720cd56d4a72d6e1dc969bee3/?utm_source=cursor | paths first-class in the plan | paper-iso-gql-gpml |
| RPQ walk semantics (Marsault et al., 2026) | https://consensus.app/papers/details/aae9e1eb5092532487fe8eb926922e6b/?utm_source=cursor | trail vs shortest vs selectable | paper-iso-gql-gpml |
| Increasing-edges compiled into graph | https://consensus.app/papers/details/0fb4318c74185122b5c83f6b23a63ceb/?utm_source=cursor | GQL inexpressiveness workaround | (queue) |
| GQL Rules / PG-Keys | — | already closed | paper-gql-rules-materialization / paper-pgschema-types-keys |

### Query 56 — temporal motifs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Motifs in Temporal Networks (Paranjape et al., 2017) | https://consensus.app/papers/details/1f5cbe6a566956c2b05e134cd5eb8d55/?utm_source=cursor | induced subgraphs on edge sequences; DOI: 10.1145/3018661.3018731 | paper-temporal-motifs |
| Temporal motif model survey (Liu et al.) | https://consensus.app/papers/details/c6c75486d1905a5d93673b71a10746e1/?utm_source=cursor | inducedness vs timing windows | paper-temporal-motifs |
| Temporal motifs as lens (Sarıyüce, 2025) | https://consensus.app/papers/details/95de792319e75080ad049627a2f91440/?utm_source=cursor | standard mining primitive | paper-temporal-motifs |
| Timed automata BGP (Aghasadeghi et al.) | https://consensus.app/papers/details/1e3e7c269bd95e91ab427095ed33f91d/?utm_source=cursor | general temporal constraints | paper-temporal-motifs |
| Time-respecting flow patterns (Gao et al., 2021) | https://consensus.app/papers/details/4d562da58dbd5e578c883741ef1bb635/?utm_source=cursor | rooted time-respecting subgraphs | paper-temporal-motifs |
| TIMEST (Pan et al., 2025) | https://consensus.app/papers/details/eb1c6e8dad6b58909add57e63b5f795f/?utm_source=cursor | sampling estimator | paper-temporal-motifs |
| Everest / Mint GPU | various | GPU motif miners | (queue; not P0) |
| TVG journeys / TARIS | — | already closed (paths / streaming ICM) | paper-tvg-journeys-restless / paper-taris-incremental-icm |

### Query 57 — snapshots / compaction / tiered storage

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| LSM compaction design space (Sarkar et al., 2021) | https://consensus.app/papers/details/fe01e75e48205586aa93ab490183b9c1/?utm_source=cursor | trigger/layout/granularity/movement | paper-lsm-snapshot-compaction |
| Query-distribution snapshots (Luo et al., 2020) | https://consensus.app/papers/details/8751803d426d520582799ebb00acb4af/?utm_source=cursor | cluster query timestamps; fewer redo/undo | paper-lsm-snapshot-compaction |
| LSM-Subgraph (Ma et al., 2023) | https://consensus.app/papers/details/6cc4545855da5772b0a9c6057a4366cf/?utm_source=cursor | PMA snapshots + logs; fluctuation shards | paper-lsm-snapshot-compaction |
| Time-tired compaction (Zhang et al., 2024) | https://consensus.app/papers/details/0aeef45fdb945cf894e605ae34edca26/?utm_source=cursor | chronological query load | paper-lsm-snapshot-compaction |
| IoTDB multi-column compaction | https://consensus.app/papers/details/357d688518aa5383905b76c15d0e1874/?utm_source=cursor | out-of-order + multi-column | paper-lsm-snapshot-compaction |
| BACH GR-LSM-Tree | https://consensus.app/papers/details/96ea5ef380255d9981a0c959f4f3b362/?utm_source=cursor | already closed (adj→CSR in levels) | paper-bach-lsm-csr-bridge |
| Rocks WAL | — | already closed (recovery) | rocksdb-wal-recovery |

## Wave 20 — Consensus batch 20 (2026-08-18)

Three queries, no filters. Generic cloud-tenant and “graph of VMs” papers discarded. Distinct capabilities closed.

### Query 58 — multi-tenant graph isolation

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Ddi per-operation graph isolation (Fu et al., 2025) | https://consensus.app/papers/details/2abaddf5f4db5731be063253a5d3c502/?utm_source=cursor | isolation per traversal; graph consistency | paper-graph-tenant-isolation |
| SQLVM (Narasayya et al., 2013) | https://consensus.app/papers/details/a56434cc43435b558f66f584fe83c968/?utm_source=cursor | CPU/I/O/memory reservation | paper-graph-tenant-isolation |
| DRFT fair transactions (Cheng et al., 2025) | https://consensus.app/papers/details/bc4d5667c433592fa9ed0d1ee46cf8c7/?utm_source=cursor | share guarantee; strategy-proof | paper-graph-tenant-isolation |
| ABase cache-aware isolation | https://consensus.app/papers/details/192540b6c4855ca098d7d7252c03b612/?utm_source=cursor | cache hits break traffic accounting | paper-graph-tenant-isolation |
| Silo/Pool/Bridge RAG | https://consensus.app/papers/details/b44cb980b02956d9a3c0a2768da9926e/?utm_source=cursor | vector-plane leakage | paper-graph-tenant-isolation |
| Path-ABAC / MAS lattice / vertex-cut | — | already closed (different isolation nouns) | paper-xacml4g-path-abac et al. |

### Query 59 — WASM / SFI sandbox for UDFs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| WAF WASM UDFs (Huang et al., 2025) | https://consensus.app/papers/details/fc78bba9a56552028dfbd0257c89b55d/?utm_source=cursor | shared memory; compile-time layout; ICDE | paper-wasm-udf-sandbox |
| Zero-cost SFI transitions (Kolosick et al., 2021) | https://consensus.app/papers/details/f75f85f77a65571c8b11083c7d6a5e9b/?utm_source=cursor | Wasm context-switch tax | paper-wasm-udf-sandbox |
| cWAMR CHERI | https://consensus.app/papers/details/4db115b6ac9154879c38309c56d06dc7/?utm_source=cursor | hardware capabilities | paper-wasm-udf-sandbox |
| Twine Wasm-in-TEE | https://consensus.app/papers/details/e30fe7760fc954788f910caec29b538b/?utm_source=cursor | two-way sandbox | paper-wasm-udf-sandbox |
| WaSC WASI decoupling | https://consensus.app/papers/details/b262140e295d584ba24a16f455312f23/?utm_source=cursor | WASI ≠ kernel isolation | paper-wasm-udf-sandbox |
| MCP-SandboxScan | https://consensus.app/papers/details/05d145e82e175d1e96be046770ddfd2b/?utm_source=cursor | agent-tool witnesses | (demand; not a second card) |
| Cordon / PACT | — | already closed (tx / argument provenance) | paper-cordon-semantic-tx / paper-pact-argument-provenance |

### Query 60 — succinct / compressed graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| k²-trees (Brisaboa et al., 2009) | https://consensus.app/papers/details/7eb88d07593b53ed983efb7a9e38036a/?utm_source=cursor | 3.3–5.3 bits/link; pred+succ | paper-k2tree-succinct-graph |
| Compact k²-tree (Brisaboa et al., 2013) | https://consensus.app/papers/details/94df60bbf950543780474a0b9b58cc3d/?utm_source=cursor | 1–3 bits/link; range queries | paper-k2tree-succinct-graph |
| Fast WebGraph / Re-Pair (Claude & Navarro, 2010) | https://consensus.app/papers/details/b41fa00ab8d65f099ed03f2c9e05b33f/?utm_source=cursor | grammar compression | paper-k2tree-succinct-graph |
| Log(Graph) (Besta et al., 2018) | https://consensus.app/papers/details/b3ce20c90de55eb2b11198bbf915257c/?utm_source=cursor | near lower bounds; vs CSR | paper-k2tree-succinct-graph |
| Lossless compression survey (Besta et al., 2018) | https://consensus.app/papers/details/3764b4f48b5858968597432162a66a06/?utm_source=cursor | taxonomy | paper-k2tree-succinct-graph |
| Fan contraction supernodes | https://consensus.app/papers/details/8e1fa06de8f859719842d30f9e27601e/?utm_source=cursor | lossless synopses; decontract | paper-k2tree-succinct-graph |
| Dynamic k²-tree (Coimbra et al.) | https://consensus.app/papers/details/50b2c6b06ff5538fa10e0972216b3820/?utm_source=cursor | updates | paper-k2tree-succinct-graph |
| CSR frozen / BACH | — | already closed (uncompressed / LSM layout) | samyama-csr-frozen-adjacency / paper-bach-lsm-csr-bridge |

## Wave 21 — Consensus batch 21 (2026-08-18)

Three queries, no filters. Queue leftovers stay skip-unless. Distinct capabilities: RDF-dataset Space noun, MATCH injectivity, named branches.

### Query 61 — named graphs / RDF datasets

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Named graphs (Carroll et al., 2005, JWS) | https://consensus.app/papers/details/4857059311aa5ac6a7a1ff36166dddac/?utm_source=cursor | URI-named graphs; SPARQL; provenance | paper-named-graphs-rdf-dataset |
| Named graphs, provenance and trust (WWW 2005) | https://consensus.app/papers/details/f1d4aa68c822570fb1cb85ccdf8d9e38/?utm_source=cursor | sign graphs; task-specific trust | paper-named-graphs-rdf-dataset |
| N3 multiple dataset semantics (Arndt et al., 2019) | https://consensus.app/papers/details/038fdf2a4af451cb810c92816ffdad57/?utm_source=cursor | no unique RDF-dataset semantics | paper-named-graphs-rdf-dataset |
| GRAPH clause federated SPARQL | https://consensus.app/papers/details/844ed19240e257a7b7577d35f920375a/?utm_source=cursor | 5–10%; semantic ambiguity | paper-named-graphs-rdf-dataset |
| RDF→PG mappings (Angles et al., 2020) | https://consensus.app/papers/details/20782504baf05b6ebefb9f5360aa0ef5/?utm_source=cursor | PG subsumes RDF; export not SoT | paper-named-graphs-rdf-dataset |
| RDF-star + Named Graphs (Rupp et al., 2022) | https://consensus.app/papers/details/69bb3a6648c2544c999cd98f575ddbcf/?utm_source=cursor | statement vs bundle meta | paper-named-graphs-rdf-dataset |
| SHACL-DS | https://consensus.app/papers/details/e559eb3a44f8591f980ef6bc4e045433/?utm_source=cursor | dataset *validation* | paper-shacl-sparql-compile (cousin) |
| Tenant isolation | — | already closed (quotas / noisy-neighbor) | paper-graph-tenant-isolation |

### Query 62 — subgraph iso vs homomorphism

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| TurboHOM++ (Kim et al., 2015, VLDB) | https://consensus.app/papers/details/55e34796c4f35072a0de758ba89d96e9/?utm_source=cursor | drop injectivity; SPARQL homomorphism | paper-subgraph-iso-vs-homomorphism |
| TurboISO (Han et al., 2013) | https://consensus.app/papers/details/db281286eb6c591f9d5d20b58ee12517/?utm_source=cursor | candidate regions; NEC | paper-subgraph-iso-vs-homomorphism |
| DAF (Han et al., 2019, SIGMOD) | https://consensus.app/papers/details/c212c795330d5f91aa706a4c74f35183/?utm_source=cursor | DAG DP; failing sets | paper-subgraph-iso-vs-homomorphism |
| Strong simulation (Ma et al., 2011) | https://consensus.app/papers/details/3b09db8dd16e57b1817f2f115228e63f/?utm_source=cursor | cubic relaxation; topology | paper-subgraph-iso-vs-homomorphism |
| VF2++ (Jüttner & Madarasi, 2018) | https://consensus.app/papers/details/160ba900103759e7b8312e5f7e69123a/?utm_source=cursor | matching order; cutting rules | paper-subgraph-iso-vs-homomorphism |
| HFrame homomorphism GNN | https://consensus.app/papers/details/6eec9537909655a096102956ea27ae49/?utm_source=cursor | homo ≠ injective iso | paper-subgraph-iso-vs-homomorphism |
| VF3M ML oracle (Fan et al., 2026) | https://consensus.app/papers/details/2ae3d2a5ef405da4b9770ae90b73fe5c/?utm_source=cursor | EnumP; skip ML matcher | (skip unless) |
| LFTJ / Kuzu WCOJ | — | already closed (AGM joins) | paper-lftj-wcoj / paper-kuzu-factorized-wcoj |
| Temporal motifs | — | already closed (mining) | paper-temporal-motifs |

### Query 63 — graph branching / fork-and-diff

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| ForkBase (Lin et al., 2020, ICDE) | https://consensus.app/papers/details/6dbec732fe4259e3a81c4ab424aa222f/?utm_source=cursor | Git-for-data; immutable; branch/merge | paper-graph-branch-fork |
| GitLake (Sheng et al., 2026) | https://consensus.app/papers/details/7315b8d012f250839dedd57fee99e87c/?utm_source=cursor | agent branches; atomic publish merge | paper-graph-branch-fork |
| Agentic exploration fork semantics | https://consensus.app/papers/details/644ed22c71595cbc93313ffb55495f5d/?utm_source=cursor | CRIU too slow; leak of tentative writes | paper-graph-branch-fork |
| ChronoGraph system-time TinkerPop | https://consensus.app/papers/details/cf3f079f38a957189cce86251d9374cd/?utm_source=cursor | SQL:2011 cousin; not named refs | paper-lsm-snapshot-compaction (cousin) |
| DeltaGraph historical snapshots | https://consensus.app/papers/details/05e3fb75dc3c55659eaa975bee785b9a/?utm_source=cursor | snapshot retrieval | paper-lsm-snapshot-compaction (cousin) |
| GitOfThoughts | https://consensus.app/papers/details/f95b6247b4df5ab99e29af23bcb27055/?utm_source=cursor | git as agent memory | hindsight-four-network-tempr (cousin) |
| CRDT graphs / ActiveGraph log fork | — | already closed (eventual / offset replay) | paper-crdt-graph-eventual / paper-activegraph-log-is-sot |

## Wave 22 — Consensus batch 22 (2026-08-18)

Three queries, no filters. Queue leftovers stay skip-unless. Distinct from tenant isolation, OCPM, named-graph `GRAPH`, Geo-Raft, and RAGraph.

### Query 64 — graph differential privacy

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Node-DP graph analysis (Kasiviswanathan et al., 2013) | https://consensus.app/papers/details/c884306e9967563b8a24590e096ef935/?utm_source=cursor | node vs edge; degree projection | paper-graph-differential-privacy |
| N2E node-to-edge reduction (Hu et al., 2025) | https://consensus.app/papers/details/899a0edd56865521841320ba17ed37f6/?utm_source=cursor | error on true max degree | paper-graph-differential-privacy |
| Fully dynamic edge-DP (Raskhodnikova et al., 2025) | https://consensus.app/papers/details/7dc58aaf6cac54e296d144a891a49414/?utm_source=cursor | event-level vs item-level; inserts+deletes | paper-graph-differential-privacy |
| DP and SPARQL (Buil-Aranda et al., 2023) | https://consensus.app/papers/details/d24b8343202952bb966118b6fe3fa79f/?utm_source=cursor | counting queries; schema metadata | paper-graph-differential-privacy |
| SoK DP on graph-structured data | https://consensus.app/papers/details/5b5aa385dff1537e8335de0495a3d1aa/?utm_source=cursor | taxonomy analysis vs GNN | paper-graph-differential-privacy |
| Path-ABAC / tenant isolation / WASM | — | already closed (authz / slice / process) | paper-xacml4g-path-abac et al. |

### Query 65 — hypergraphs / higher-order

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| From Graphs to Hypergraphs (Wang et al., 2024) | https://consensus.app/papers/details/10348d85263651f4b024ec3f1bc1d8fd/?utm_source=cursor | projection loss; unrecoverable | paper-hypergraph-higher-order |
| HIF interchange format (Coll et al., 2025) | https://consensus.app/papers/details/71366ba219765a119a22898dd1c38db8/?utm_source=cursor | incidences as first-class | paper-hypergraph-higher-order |
| High-order hypergraph walks (Aksoy et al., 2019) | https://consensus.app/papers/details/00b720d012c85978874f81dd78ac84ec/?utm_source=cursor | length and width | paper-hypergraph-higher-order |
| Higher-order motifs (Lotito et al., 2021) | https://consensus.app/papers/details/8056af706c5f58dfac0190a50d539a8d/?utm_source=cursor | not VF2 on cliques | paper-hypergraph-higher-order |
| Hypergraph mining survey (Lee et al., 2024) | https://consensus.app/papers/details/96c3c6c6a72d5f48b8f4cc0d695dc594/?utm_source=cursor | patterns / generators | paper-hypergraph-higher-order |
| OCPM multi-object events | — | already closed (process log encoding) | paper-ocpm-multi-object-events |

### Query 66 — federated SPARQL / graph query

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| FedUP result-aware plans (Aimonier-Davat et al., 2024) | https://consensus.app/papers/details/2f9179016a8a5df891e1807f049705da/?utm_source=cursor | empty combinations; large federations | paper-federated-sparql-query |
| SPARQL 1.1 federation (Buil Aranda et al., 2012) | https://consensus.app/papers/details/cb49500582a8588d933801315e31dbd7/?utm_source=cursor | SERVICE syntax/semantics | paper-federated-sparql-query |
| Heterogeneous LDF federations (Heling & Acosta) | https://consensus.app/papers/details/5c1f72940d0e50ec87acfa1aec9c0052/?utm_source=cursor | mixed endpoint/TPF interfaces | paper-federated-sparql-query |
| Fine-grained federation eval (Saleem et al., 2016) | https://consensus.app/papers/details/20d9cd87c01d52ba96f1db8a101ad134/?utm_source=cursor | source selection / ASK dominate | paper-federated-sparql-query |
| Odyssey federated optimizer | https://consensus.app/papers/details/8768a1fab4af5db0836b062496485f7d/?utm_source=cursor | remote statistics | paper-federated-sparql-query |
| LargeRDFBench | https://consensus.app/papers/details/ace50b037de9591da9fa1999b9593f1a/?utm_source=cursor | simple ≠ hard queries | paper-federated-sparql-query |
| Agentic SPARQL-MCP | https://consensus.app/papers/details/c872cda17c17544c979dcdb4556c3d41/?utm_source=cursor | LLM+MCP demand | (demand; not a second card) |
| Named graphs / Geo-Raft | — | already closed (in-store GRAPH / log replica) | paper-named-graphs-rdf-dataset / paper-geo-raft-wan |

## User URLs — cool-japan (2026-08-18)

GitHub scout, no Consensus. Cloned `/tmp/cool-japan/{oxixml,oxify,scirs}`; not vendored; not CBM-indexed.

| source | url | capability seed | card id |
|--------|-----|-----------------|---------|
| OxiXML XML/RDF stack | https://github.com/cool-japan/oxixml | parse/validate/transform; SPARQL syntax not exec | oxixml-xml-rdf-stack |
| OxiFY DAG engine | https://github.com/cool-japan/oxify | Kahn topo-sort LLM workflows; Qdrant/pgvector | oxify-dag-llm-orchestration |
| OxiFY oxify-authz | https://github.com/cool-japan/oxify | Zanzibar tuples; Leopard index; SQLite | oxify-zanzibar-rebac |
| SciRS2 scirs2-graph | https://github.com/cool-japan/scirs | CSR + NetworkX-class algorithms | scirs-graph-scientific |
| oxirs SPARQL/SHACL engines | oxixml README | execution stays in oxirs | (queue) |
| oxify-authz quantum.rs | oxify | Kyber placeholder | (queue; not a card) |

## User URLs — ULTRA / CRP-SpMM / TypeGraph / Open Ontologies (2026-08-18)

GitHub scout, no Consensus. Cloned `/tmp/user-url-scout/{ultra,crp-spmm,typegraph,open-ontologies}`; not vendored; not CBM-indexed.

| source | url | capability seed | card id |
|--------|-----|-----------------|---------|
| ULTRA + UltraQuery | https://github.com/DeepGraphLearning/ULTRA | inductive relation-graph GNN; fuzzy logical QA | ultra-kg-foundation-reasoner |
| CRP-SpMM | https://github.com/scalable-matrix/CRP-SpMM | MPI 2D comm-reduced SpMM; SC23 | crp-spmm-comm-reduced |
| TypeGraph | https://github.com/nicia-ai/typegraph | Zod PG-in-SQL; app-clock bitemporal; graph-extensions | typegraph-typed-sql-kg |
| Open Ontologies | https://github.com/fabio-rovai/open-ontologies | MCP OWL-RL/SHACL/KGCL; Oxigraph working memory | open-ontologies-mcp-govern |
| TypeGraph vector/hybrid search | typegraph README | pgvector / sqlite-vec | (queue; HNSW cousin) |
| Open Ontologies Dynamics/Planner | README three-layer | PDDL / PyWhy / WASM plugins | (queue) |
| ULTRA rspmm kernel | ultra/rspmm | relational O(V) SpMM | (queue; GNN kernel ≠ MPI CRP) |

## Wave 23 — Consensus batch 23 (2026-08-18)

Three queries, no filters. Distinct from DP, ADOPT/Horae, ontology versioning, and oxixml intern-as-library.

### Query 67 — structured / graph encryption

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Structured encryption (Chase & Kamara, 2010) | https://consensus.app/papers/details/e29a9fb283495ac08ce4716e2787dfc1/?utm_source=cursor | STE generalizes SSE to graphs | paper-graph-structured-encryption |
| GraphShield (Du et al., 2020) | https://consensus.app/papers/details/0cc34d834bb058af981b5e8cb105e6fe/?utm_source=cursor | SP / flow / PageRank; forward privacy | paper-graph-structured-encryption |
| STE for knowledge graphs (Xue et al., 2022) | https://consensus.app/papers/details/c1dd47faed805929a9599cc5cb1b233b/?utm_source=cursor | MG + PG; CQA2 | paper-graph-structured-encryption |
| GES shortest path (Ghosh et al., 2021) | https://consensus.app/papers/details/472523b0fe005eabb174c19f634b041c/?utm_source=cursor | recursive SPSP; optimal overhead | paper-graph-structured-encryption |
| Spidey encrypted property graphs | https://consensus.app/papers/details/33c57d97234a5ae3b8960ab218e0ac2d/?utm_source=cursor | dynamic PG + RBAC | paper-graph-structured-encryption |
| DP / ADS / ReBAC | — | already closed (noise / proofs / tuples) | paper-graph-differential-privacy et al. |

### Query 68 — cardinality estimation

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| OmniSketch joins (Justen et al., 2025) | https://consensus.app/papers/details/fac32d6bdda456b3bf7d9e4bd95f27f4/?utm_source=cursor | count-min + K-minwise; DuckDB | paper-graph-cardinality-estimation |
| Cardinality estimation graphs (Salihoglu et al.) | https://consensus.app/papers/details/56f57f7f60645d4ea81b4683ce5ef252/?utm_source=cursor | optimistic vs pessimistic CEG | paper-graph-cardinality-estimation |
| COLOR (Deeds et al., 2024) | https://consensus.app/papers/details/ba68ec6397c8582aa77c4dd1667d5f61/?utm_source=cursor | coloring summaries | paper-graph-cardinality-estimation |
| Complex graph CardEst (Hu et al., 2024) | https://consensus.app/papers/details/083b515071f25f1e9d167582478283fe/?utm_source=cursor | WanderJoin; nested ops | paper-graph-cardinality-estimation |
| JOB (Leis et al., 2017) | https://consensus.app/papers/details/bc7c711b2aa35c39a94a532274d6a42b/?utm_source=cursor | cardinality ≫ cost model | paper-graph-cardinality-estimation |
| ADOPT / Horae / LFTJ | — | already closed (RL order / temporal sketch / execute) | paper-adopt-adaptive-wcoj-orders et al. |

### Query 69 — term dictionaries

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| RDF dictionaries B+ trees (Singh et al., 2018) | https://consensus.app/papers/details/3539cac6702a539780ec225dc66ff3af/?utm_source=cursor | ensemble B+; SPARQL end-to-end | paper-rdf-term-dictionary |
| Compact in-memory Trie dictionary | https://consensus.app/papers/details/dfef93cd34e053b89e82bdc726d7aefe/?utm_source=cursor | prefix Trie; ID=address | paper-rdf-term-dictionary |
| Dcomp RDF dictionary compression | https://consensus.app/papers/details/ffe7faba3b33550e8fb3e5cc64d45a19/?utm_source=cursor | 22–64% space | paper-rdf-term-dictionary |
| LiteMat RDFS++ encoding | https://consensus.app/papers/details/e40d33f0a9375425b7d08b506b5f5dd7/?utm_source=cursor | hierarchy in the integer | paper-rdf-term-dictionary |
| Ontology versioning / OBDA / named graphs | — | already closed (evolution / mapping / scope) | paper-ontology-temporal-versioning et al. |
| oxixml-model intern | oxixml scout | code cousin | oxixml-xml-rdf-stack |

## Wave 24 — Consensus batch 24 (2026-08-18)

Three queries, no filters. Generic Kafka/BigQuery CDC blogs discarded. Distinct from Raphtory, DBSP/IVM, T-GQL, LSM snapshots.

### Query 70 — graph CDC / changelog ingest

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| DBLog (Andreakis et al., 2020) | https://consensus.app/papers/details/a5098b497ea5575f8193967bfb528214/?utm_source=cursor | watermark CDC; Debezium/Flink | paper-graph-cdc-ingest |
| Certified virtual cuts (Andreakis, 2026) | https://consensus.app/papers/details/5c98978a88a953339d2502c2933fff1b/?utm_source=cursor | snapshot-equivalent certificate | paper-graph-cdc-ingest |
| Neptune CDC (Bebee et al., 2019) | https://consensus.app/papers/details/b2a9ea5f5c595da98f06a3059494e667/?utm_source=cursor | graph→search change stream | paper-graph-cdc-ingest |
| Incremental KG / LDES (Van Assche et al., 2026) | https://consensus.app/papers/details/af5fa2e1bcb757dda7a49bce9b7a06c6/?utm_source=cursor | RML incremental vs regen | paper-graph-cdc-ingest |
| Raphtory / ActiveGraph / TARIS | — | already closed (in-engine log / SoT / ICM) | paper-raphtory-lazy-temporal-views et al. |

### Query 71 — query result cache / memoization

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Graph-aware SPARQL cache (Papailiou et al., 2015, SIGMOD) | https://consensus.app/papers/details/022e132f9bc9543aae7b5614758705ae/?utm_source=cursor | canonical labelling; 100× | paper-graph-query-result-cache |
| eBay application-level result cache | https://consensus.app/papers/details/5b877d91d569540495c67d02338e489f/?utm_source=cursor | final results; write invalidation | paper-graph-query-result-cache |
| One-hop sub-query caches (Nguyen et al., 2024) | https://consensus.app/papers/details/de7586309c805184b1bd9c59ef8f1db7/?utm_source=cursor | vertex-id sets; p95 | paper-graph-query-result-cache |
| SPARQL invalidation by graph patterns | https://consensus.app/papers/details/0daaccb936cb5f549d357d01e2c50408/?utm_source=cursor | which updates touch a cache | paper-graph-query-result-cache |
| KGraph CGQ memoization (Gao et al., 2025) | https://consensus.app/papers/details/dd1177e3a64156a3a2503cdff6ee3908/?utm_source=cursor | partition-local memo | paper-graph-query-result-cache |
| DBSP / Graphflow / PogoCache | — | already closed (IVM / delta join / TTL KV) | paper-dbsp-ivm et al. |

### Query 72 — temporal interval / timeline indexes

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Timeline Index (Kaufmann et al., 2013) | https://consensus.app/papers/details/fd8a9b2d402159ec9512d0d69ff7f196/?utm_source=cursor | HANA; aggregation/travel/join | paper-temporal-interval-index |
| TIDE (Wang et al., 2025) | https://consensus.app/papers/details/c16e3dfce0c15ccb8b12c9e63cd96c43/?utm_source=cursor | duration×endpoint B+ | paper-temporal-interval-index |
| MAP21 B+ ranges (Nascimento & Dunham, 1999) | https://consensus.app/papers/details/55f0e093c367520ba62104beb839113d/?utm_source=cursor | ranges as points | paper-temporal-interval-index |
| IB+tree time-splits | https://consensus.app/papers/details/18b80753a0b15ae495046d145dff80ec/?utm_source=cursor | valid-time intervals | paper-temporal-interval-index |
| Temporal subgraph interval index (Ouyang et al., 2026) | https://consensus.app/papers/details/781761b7f2f854d0af84fc34325ec182/?utm_source=cursor | sub/super-valid windows | paper-temporal-interval-index |
| T-GQL / TVG journeys / LSM snapshots | — | already closed (QL / algorithms / placement) | paper-tgql-intervals et al. |

## Wave 25 — Consensus batch 25 (2026-08-18)

Three queries, no filters. Generic LCA/Neo4j, SBOM, protein-design plugins, and cloud-workflow admission discarded. Distinct from WASM sandbox, PG-HIVE discovery, OWL versioning, tenant isolation, CardEst.

### Query 73 — graph plugin / pack lifecycle

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| DuckPGQ SQL/PGQ extension (ten Wolde et al., 2023) | https://consensus.app/papers/details/0b0861fc7ff05f0cae1579bdebd80068/?utm_source=cursor | inject types/ops/parser into host | paper-graph-pack-plugin-lifecycle |
| RDF KB index/store versioning (Bellini et al., 2015) | https://consensus.app/papers/details/307e19cb16005a7b8a17b99b7e402e1f/?utm_source=cursor | lifecycle tool; not black-box rebuild | paper-graph-pack-plugin-lifecycle |
| Graph DB lifecycle methodology (Nesi et al., 2015) | https://consensus.app/papers/details/5f5024f4b01f5ffea192931a9b64c7ee/?utm_source=cursor | index versioning when ontology changes | paper-graph-pack-plugin-lifecycle |
| Living Databases (Deshpande, 2026) | https://consensus.app/papers/details/c9aca8ad502c53e1977b194091452e2e/?utm_source=cursor | unified evolution/versioning/views | paper-graph-pack-plugin-lifecycle |
| WASM UDF / ISO GQL / named branches | — | already closed (sandbox / surface / overlays) | paper-wasm-udf-sandbox et al. |

### Query 74 — online PG schema evolution

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Tesseract DDaM (Hu et al., 2022) | https://consensus.app/papers/details/35960972b32754d59da99762ae0f2a39/?utm_source=cursor | ALTER as snapshot-isolation write | paper-online-pg-schema-evolution |
| F1 async schema change (Rae et al., 2013) | https://consensus.app/papers/details/50f8df5fb7965381ae918bc805c59b90/?utm_source=cursor | one-version lag; decompose dangerous ALTER | paper-online-pg-schema-evolution |
| PG schema validation/evolution (Bonifati et al., 2019) | https://consensus.app/papers/details/312978432b0b54bbb3609c1aee0cf42f/?utm_source=cursor | homomorphism validate; graph rewrite | paper-online-pg-schema-evolution |
| Orion/U-Schema (Chillón et al., 2024) | https://consensus.app/papers/details/0245cf5f83dd5e5fa2e28a307c980777/?utm_source=cursor | SCO taxonomy incl. graph; Alloy | paper-online-pg-schema-evolution |
| PRISM SMOs (Curino et al., 2008) | https://consensus.app/papers/details/6315b72fad615acc88849e95f73c716a/?utm_source=cursor | schema modification operators + rewrite | paper-online-pg-schema-evolution |
| PG-HIVE / PG-Schema types / OWL versioning / daily-archive | — | already closed (discover / types / OWL / YAML) | paper-pghive-schema-discovery et al. |

### Query 75 — query admission / workload management

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| CASA slot admission (Zeyl et al., 2024) | https://consensus.app/papers/details/6f7ae997037853ce815706ecfeac16fe/?utm_source=cursor | query-text → slots; +48% throughput | paper-query-admission-control |
| SafeLoad MO queries (Wu et al., 2025) | https://consensus.app/papers/details/0bd88af2667d5de08be78b4fef9de0ef/?utm_source=cursor | reject memory-overloading before execute | paper-query-admission-control |
| Bouncer SLO admission (Xu et al., 2024) | https://consensus.app/papers/details/dd2150b4f3525b0a951d128c2b423b9b/?utm_source=cursor | percentile SLO; class-specific; starvation | paper-query-admission-control |
| Workload mgmt taxonomy (Zhang et al., 2018) | https://consensus.app/papers/details/6e4da506e5a55e969b88fe83dfdbb6f3/?utm_source=cursor | admission / schedule / execution control | paper-query-admission-control |
| Banyan scoped dataflow (Su et al., 2022) | https://consensus.app/papers/details/6daaa6c85a6f56b1981409437c5fd6b5/?utm_source=cursor | graph query service isolation at subquery | paper-query-admission-control |
| Tenant isolation / CardEst / WASM | — | already closed (slice / statistic / sandbox) | paper-graph-tenant-isolation et al. |

## Wave 26 — Consensus batch 26 (2026-08-18)

Three queries, no filters. CNN/avocado/mango hits discarded. Distinct from admission, DBSP IVM, pack lifecycle, WCOJ, TVG journeys.

### Query 76 — max-convolution / tropical budgets

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Higher-dim knapsack via (max,+) convolution (Grage et al., 2024) | https://consensus.app/papers/details/9cdca93c1500569ea3e7dbd8d5e27c09/?utm_source=cursor | multi-capacity; concave 1-D reduce | paper-max-convolution-budgets |
| Faster knapsack / min-plus convolution (Bringmann et al., 2024) | https://consensus.app/papers/details/8d5b8caab0b059c5b620e44d43b837a1/?utm_source=cursor | Õ(n+t√p_max); rectangular monotone | paper-max-convolution-budgets |
| Knapsack via convolution+prediction (Bateni et al., 2018) | https://consensus.app/papers/details/0ae30451c7de537b8d7ff0ac62b4a076/?utm_source=cursor | knapsack ≡ (min,+) convolution | paper-max-convolution-budgets |
| Numerical max-convolution (Serang, 2015) | https://consensus.app/papers/details/ea7be5d4edc65e34ab2268e912ae4ab9/?utm_source=cursor | tropical / infimal convolution | paper-max-convolution-budgets |
| Max-convolution via tropical geometry (Brysiewicz et al., 2023) | https://consensus.app/papers/details/d5e5f5bacb605f6386da2e37bfe87746/?utm_source=cursor | quasi-linear integer max-conv | paper-max-convolution-budgets |
| Query admission / ring WCOJ | — | already closed (gate / join semiring) | paper-query-admission-control et al. |

### Query 77 — reversible PG materializations / views

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| PG views rewrite/materialize (Han et al., 2024) | https://consensus.app/papers/details/be70a04652595a7fa3f31a398855daa2/?utm_source=cursor | virtual vs stored; local transforms | paper-pg-materialized-views |
| Implementing PG views (Han et al., 2025 SIGMOD Rec.) | https://consensus.app/papers/details/1827e90380f058339581ef1f1ceb56f5/?utm_source=cursor | same family | paper-pg-materialized-views |
| MV4PG (Xu et al., 2024) | https://consensus.app/papers/details/cd60fd225cbb5c5ebb314275b9299790/?utm_source=cursor | PG MVs; var-length edge templates | paper-pg-materialized-views |
| DRed incremental views (Gupta et al., 1993) | https://consensus.app/papers/details/e02282c955d25050aa8146a626747cdc/?utm_source=cursor | counts; delete/rederive recursive | paper-pg-materialized-views |
| B/F Datalog materialisation (Motik et al., 2015) | https://consensus.app/papers/details/600da72424025bc2b4958b64025b0bc9/?utm_source=cursor | many alternate derivations | paper-pg-materialized-views |
| External RDB2RDF changesets (Vidal et al., 2022) | https://consensus.app/papers/details/f6b51f547c21595b939a3848dc5aceba/?utm_source=cursor | maintain without reading remote view | paper-pg-materialized-views |
| DBSP / pack lifecycle / GQL rules / result cache | — | already closed | paper-dbsp-ivm et al. |

### Query 78 — regular path queries

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Cypher/GQL vs RPQs (Gheerbrant et al., 2025) | https://consensus.app/papers/details/f21b7cc2f07a54c08950310604226872/?utm_source=cursor | original Cypher ⊊ all RPQs | paper-regular-path-queries |
| dl-CRPQ / GQL path modes (Libkin et al., 2025) | https://consensus.app/papers/details/391bd06c00f05d62952787476b1d32b7/?utm_source=cursor | nodes+edges; path vars; automata | paper-regular-path-queries |
| Designing RPQ semantics (Marsault et al., 2026) | https://consensus.app/papers/details/aae9e1eb5092532487fe8eb926922e6b/?utm_source=cursor | walk/trail/simple/shortest | paper-regular-path-queries |
| Transitive-restricted RSPQ (Liang et al., 2024) | https://consensus.app/papers/details/17df08df08a15a58ac077dec25433ab7/?utm_source=cursor | >99% real queries; NP-hard simple | paper-regular-path-queries |
| SPARQL RPQ shapes corpus (Hammerer et al., 2025) | https://consensus.app/papers/details/c12a5bc445e95f928e09ca40aaf76286/?utm_source=cursor | 148.7M RPQs → 572 shapes | paper-regular-path-queries |
| WCOJ / subgraph iso / TVG journeys | — | already closed (conjunctive / temporal paths) | paper-lftj-wcoj et al. |

## Wave 27 — Consensus batch 27 (2026-08-18)

Three queries, no filters. SBOM, fingerprint, TLS-malware, and GNN-loader hits discarded. Distinct from WASM sandbox, ReBAC, path-ABAC, MVCC storage, Cordon, CDC.

### Query 79 — object capabilities / confused deputy

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Capability Myths Demolished (Miller et al., 2003) | https://consensus.app/papers/details/d47742499ea157a087949184f03dbeba/?utm_source=cursor | ACL ≉ capabilities; least privilege; CDA | paper-object-capabilities |
| Access control vs capabilities (Rajani et al., 2016, CSF) | https://consensus.app/papers/details/3653013b63505a73b27877c8af654c87/?utm_source=cursor | fundamentally different; CDA not all prevented | paper-object-capabilities |
| Object capabilities for security (Wagner, 2006) | https://consensus.app/papers/details/11321bc06f525991ba96111ae50d9544/?utm_source=cursor | reference = capability; privilege separation | paper-object-capabilities |
| ScopeGate (Zuvic, 2026) | https://consensus.app/papers/details/7acf73b3461b59eda1c057505e27d261/?utm_source=cursor | agent tool exposure ≠ per-call authorization | paper-object-capabilities |
| WASM / ReBAC / path-ABAC / PACT / CHERI | — | already closed or queued | paper-wasm-udf-sandbox et al. |

### Query 80 — SI / SSI / write skew

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Making SI serializable (Fekete et al., 2005) | https://consensus.app/papers/details/909ad817dbbd5f86a86b56691e485e96/?utm_source=cursor | dangerous structures; write skew | paper-graph-ssi-isolation |
| SSI algorithm (Cahill, 2009) | https://consensus.app/papers/details/992c6cb6360555b98667510cf98f61fa/?utm_source=cursor | runtime anomaly prevention | paper-graph-ssi-isolation |
| PostgreSQL SSI (Ports & Grittner, 2012) | https://consensus.app/papers/details/ddcc144157f2502f870bc20ec31f0fd0/?utm_source=cursor | production SSI | paper-graph-ssi-isolation |
| PSSI cycle detection (Revilak et al., 2011) | https://consensus.app/papers/details/949475a0212a5ba79a0374893ef8da0b/?utm_source=cursor | fewer false aborts than dangerous-structure | paper-graph-ssi-isolation |
| Read-only SI anomaly (Fekete et al., 2004) | https://consensus.app/papers/details/cb0d1aff405854dca0428037749d7163/?utm_source=cursor | RO tx can be non-serializable | paper-graph-ssi-isolation |
| MVCC chains / Cordon / tenant isolation | — | already closed (storage / external effects / quotas) | samyama-mvcc-version-chains et al. |

### Query 81 — bulk / parallel graph load

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Scalable graph loading (Campero Durand et al., 2018) | https://consensus.app/papers/details/f6889cdfeb0c57238e5b48d262a03c74/?utm_source=cursor | loading is top user bottleneck; batch 64× | paper-graph-bulk-load |
| Parallel graph loading techniques (Then et al., 2016) | https://consensus.app/papers/details/3ba480328ad7522093bba8e789562d82/?utm_source=cursor | parse + dense relabel + in-memory write | paper-graph-bulk-load |
| Instant Loading CSV (Mühlbauer et al., 2013) | https://consensus.app/papers/details/6b4da0d9059759c7997c4e969e4a0dba/?utm_source=cursor | wire-speed multicore CSV | paper-graph-bulk-load |
| PGDF (Angles et al., 2024) | https://consensus.app/papers/details/146cbce9548c5a02b7dfc7d4ae4e1c97/?utm_source=cursor | PG interchange text format | paper-graph-bulk-load |
| GraphOne ingest store (Kumar et al., 2020) | https://consensus.app/papers/details/cd8fa8949b715ef1b12d01df0078484c/?utm_source=cursor | dual edge-list/adj; GraphView | paper-graph-bulk-load |
| CDC / dictionary / oxixml parse | — | already closed (WAL ingest / intern / document) | paper-graph-cdc-ingest et al. |

## User URLs — Engramx + Harvey LAB firm-knowledge (2026-08-18)

GitHub scout, no Consensus. Cloned `/tmp/user-url-scout/{engram,harvey-labs}`; not vendored; not CBM-indexed. **Do not** confuse nickcirv/engram with Wang 2026 Engram (`paper-engram-bitemporal-memory`). Do not vendor harvey-labs (~63k files).

| source | url | capability seed | card id |
|--------|-----|-----------------|---------|
| Engramx code KG + Read intercept | https://github.com/nickcirv/engram | SQLite code graph; git-revert mistakes; never-worse Read packets | engramx-code-context-spine |
| Harvey LAB firm-knowledge | https://github.com/harveyai/harvey-labs/tree/main/tasks/firm-knowledge | 250 tasks / 266 synthetic matters; grep harness; precision over matter IDs | harvey-lab-firm-knowledge |
| Engramx mesh/PII | engram `src/mesh` | identity/audit/PII strip | (queue) |
| Harvey LAB other practice areas | `tasks/{antitrust,…}` | per-matter file dumps; skip unless a second eval noun | (queue) |

## Wave 28 — Consensus batch 28 (2026-08-18)

Three queries, no filters. Distinct from Geo-Raft, SPARQL SERVICE, named graphs, RPQ automata, TVG journeys, subgraph iso, DP noise, TOKI, hypergraphs.

### Query 82 — spatial / GIS property graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| GraST PostGIS+Neo4j (Yue et al., 2025) | https://consensus.app/papers/details/2a7c6a249877549fa5d2c3251801d919/?utm_source=cursor | geometry in RDBMS; lightweight graph nodes | paper-spatial-geosparql-graphs |
| GeoSPARQL 1.1 (Car et al., 2022) | https://consensus.app/papers/details/a10a5babbd745ec79b1e3dd7265b391e/?utm_source=cursor | LOD spatial ontology + SPARQL functions | paper-spatial-geosparql-graphs |
| GeoExpand (Sun et al., 2019) | https://consensus.app/papers/details/789d6edce7d8571a8d7a57d78afe78a5/?utm_source=cursor | spatial bitmaps prune graph expansion | paper-spatial-geosparql-graphs |
| FineGeoKG (Wei et al., 2024) | https://consensus.app/papers/details/1f046aa035895944aab4f784873c7bee/?utm_source=cursor | strong geospatial relations as edges | paper-spatial-geosparql-graphs |
| LinkedGeoData VKG / Ontop (Ding et al., 2021) | https://consensus.app/papers/details/8fa79f051837538e8733d3fc60dc3578/?utm_source=cursor | virtual GeoSPARQL over OSM RDB | paper-spatial-geosparql-graphs |
| QSR over YAGO (Mantle et al., 2024) | https://consensus.app/papers/details/a7f3c8ddef7958a7b6c6960481e1ff3b/?utm_source=cursor | qualitative spatial reasoning on large KGs | paper-spatial-geosparql-graphs |
| Federated GeoSPARQL / QGIS | — | cousin of SPARQL SERVICE | (queue) |
| IndoorGML / CityGML-KG | — | indoor/city packs | (queue) |
| Geo-Raft / named graphs | — | already closed (WAN replica / GRAPH name) | paper-geo-raft-wan / paper-named-graphs-rdf-dataset |

### Query 83 — reachability labeling / hop indexes

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| 2-hop labels (Cohen et al., 2002) | https://consensus.app/papers/details/700007f77a5655529d0cf5461ac2b723/?utm_source=cursor | Lout ∩ Lin; cover of paths | paper-reachability-2hop-index |
| GRAIL (Yildirim et al., 2011) | https://consensus.app/papers/details/c1538e3b9bbb510e88755960a8fba77d/?utm_source=cursor | randomized interval labeling | paper-reachability-2hop-index |
| GRAIL PVLDB (Yildirim et al., 2010) | https://consensus.app/papers/details/1f097fc1218259459654de5e55b45738/?utm_source=cursor | linear index; millions of edges | paper-reachability-2hop-index |
| FERRARI (Seufert et al., 2013) | https://consensus.app/papers/details/591b040aff675c05afe54ad961fcd1d1/?utm_source=cursor | bounded approximate ranges | paper-reachability-2hop-index |
| Hierarchical hop labeling (Jin et al., 2013) | https://consensus.app/papers/details/060c2fd267205df8ba6d3d7ab68affcf/?utm_source=cursor | no TC materialization | paper-reachability-2hop-index |
| LCR 2-hop billion-scale (Peng et al., 2020) | https://consensus.app/papers/details/754fbd3dbf9c5bb0bd6fe804e82dd462/?utm_source=cursor | label-constrained; µs queries | paper-reachability-2hop-index |
| LCR/LCKR (Peng et al., 2021) | https://consensus.app/papers/details/0c9b4e0cbcb0530e925caaa15e63fd5f/?utm_source=cursor | hop-bounded labeled reachability | paper-reachability-2hop-index |
| RLC index (Zhang et al., 2022) | https://consensus.app/papers/details/6041f3e1dab95593a959721aabd84818/?utm_source=cursor | Kleene-plus label concatenation | paper-reachability-2hop-index |
| DLCR dynamic LCR (Chen et al., 2022) | https://consensus.app/papers/details/78dedfd6f76e5765af7876d913b97098/?utm_source=cursor | 2-hop under edge updates | paper-reachability-2hop-index |
| Reachability-index tutorial (Zhang et al., 2023) | https://consensus.app/papers/details/57d7955fadb55da9b5d718901aa40e0a/?utm_source=cursor | GDBMS integration of TC indexes | paper-reachability-2hop-index |
| Temporal bipartite reachability | https://consensus.app/papers/details/8429a475e5185514907ed7202848ef8a/?utm_source=cursor | time + bipartite; cousin of TVG | (queue) |
| RPQ / TVG journeys / subgraph iso | — | already closed | paper-regular-path-queries et al. |

### Query 84 — probabilistic / uncertain graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| ProbTree (Maniu et al., 2017) | https://consensus.app/papers/details/356794e882c755c499b034f94f499f24/?utm_source=cursor | succinct possible worlds for ST-queries | paper-probabilistic-uncertain-graphs |
| Uncertain-graph tutorial (Khan & Tu, 2015) | https://consensus.app/papers/details/aef7aaf8136c5bb98d8b33740628589c/?utm_source=cursor | #P-complete classical queries | paper-probabilistic-uncertain-graphs |
| Uncertain subgraph search (Yuan et al., 2011) | https://consensus.app/papers/details/ea296fef575b50b7b5fb8e1a88e3bc21/?utm_source=cursor | #P; PIndex filter-then-verify | paper-probabilistic-uncertain-graphs |
| Monte-Carlo uncertain graphs (Emrich et al., 2012) | https://consensus.app/papers/details/67a7fc1f4cc95d579d85a4fa8db41218/?utm_source=cursor | sample relevant subgraph; significance | paper-probabilistic-uncertain-graphs |
| Representative certain instance (Parchas et al., 2014) | https://consensus.app/papers/details/da65158af97a586089e615db06ff286b/?utm_source=cursor | preserve expected degrees | paper-probabilistic-uncertain-graphs |
| Trigger graphs / PDB (Tsamoura et al., 2023) | https://consensus.app/papers/details/6ff2a85a48d3500b9efbc52614666a28/?utm_source=cursor | lineage without full materialization | paper-probabilistic-uncertain-graphs |
| Uncertain-graph survey (Banerjee, 2021) | https://consensus.app/papers/details/d56c1706594f5bf1bf71ade4b5415c5d/?utm_source=cursor | mining + query landscape | paper-probabilistic-uncertain-graphs |
| Query-answer explanation (Debbi, 2023) | https://consensus.app/papers/details/cfdb1261c1e45429ae45452f65b1e9c4/?utm_source=cursor | causality/blame on uncertain attrs | paper-probabilistic-uncertain-graphs |
| Probabilistic hypergraphs | https://consensus.app/papers/details/ca56776ca1cb5e74a91e0539c3ce4ef1/?utm_source=cursor | nested superedges | (queue) |
| GPU BPGraph path sampling | https://consensus.app/papers/details/bbd1e8671edf5b5c9de6e1de56229bf0/?utm_source=cursor | multi-GPU; not P0 | (queue) |
| DP / TOKI / hypergraph n-ary | — | already closed | paper-graph-differential-privacy et al. |

## Wave 29 — Consensus batch 29 (2026-08-18)

Three queries, no filters. Distinct from Horae sketches, PG views, interval indexes, GRADOOP TPGM, BM25+HNSW, Cypher/GQL, LSM snapshots, bulk load, k².

### Query 85 — graph OLAP / cuboids

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Temporal Graph Cube (Wang et al., 2023) | https://consensus.app/papers/details/6b4bfd0622285167afde40da3380ac3c/?utm_source=cursor | time-range OLAP; segment-tree; snapshot similarity | paper-graph-olap-cube |
| Graph OLAP ICDM (Chen et al., 2008) | https://consensus.app/papers/details/1d95fd0b416a572bac6bf421c35b6f19/?utm_source=cursor | informational vs topological; aggregated graph | paper-graph-olap-cube |
| Graph OLAP KAIS (Chen et al., 2009) | https://consensus.app/papers/details/42ac1c0bd9c3502bb3183f245a20a735/?utm_source=cursor | iceberg cubes; discovery-driven | paper-graph-olap-cube |
| Graph Cube (Zhao et al., 2011) | https://consensus.app/papers/details/12b2133ea9e55173b78079a035b935d5/?utm_source=cursor | structure summarization; crossboid | paper-graph-olap-cube |
| Graphoids (Gómez et al., 2019) | https://consensus.app/papers/details/3bbd45c2964b54188a1d3537fdecc2b0/?utm_source=cursor | cube ops on multi-hypergraphs | paper-graph-olap-cube |
| Structure-aware cuboid cache (Zhang, 2017) | https://consensus.app/papers/details/6b76bfc90cee5f05a5769df2ea9a258e/?utm_source=cursor | 15–30× vs Neo4j expand-then-agg | paper-graph-olap-cube |
| GRADOOP TPGM | https://consensus.app/papers/details/5ecaf179c5b85ecf94bbfbf97ae75e7a/?utm_source=cursor | already closed (grouping cousin) | paper-gradoop-tpgm |
| Probabilistic data cubes | https://consensus.app/papers/details/d20ee95734c154d2b5e6a5ada0514837/?utm_source=cursor | pmf cuboids; cousin of Q84 | (queue) |
| RA-OLAP / LLM | https://consensus.app/papers/details/0bacf27440875ff7a478ca5f99a2bb8f/?utm_source=cursor | RAG+SQL; LLM not SoT | (queue) |

### Query 86 — keyword search on graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| BANKS (Bhalotia et al., 2002) | https://consensus.app/papers/details/163aadd06b755309952343b019e67737/?utm_source=cursor | rooted trees; proximity + prestige | paper-keyword-search-graphs |
| BANKS demo (Aditya et al., 2002) | https://consensus.app/papers/details/4c1df7cc28635e26bf9016b62a92dd1c/?utm_source=cursor | schema-free browse+search | paper-keyword-search-graphs |
| BLINKS (He/Wang et al., 2007) | https://consensus.app/papers/details/0c8d084f3eb45331b5ad25247142b9bd/?utm_source=cursor | bi-level block index; top-k | paper-keyword-search-graphs |
| R-KwS survey (Yu et al., 2010) | https://consensus.app/papers/details/da774d65258858f4ae83e0cc3973fc6b/?utm_source=cursor | schema-SQL vs data-graph Steiner | paper-keyword-search-graphs |
| Full-power graph querying tutorial (Manolescu et al., 2023) | https://consensus.app/papers/details/bce825efc15c5a9ea5fef90e7a3952a7/?utm_source=cursor | SPARQL/GPML ↔ keyword continuum | paper-keyword-search-graphs |
| Elas4RDF (Kadilierakis et al., 2021) | https://consensus.app/papers/details/40de1148a7065f1698ff0bb6aabc4c15/?utm_source=cursor | schema-agnostic triples via ES | paper-keyword-search-graphs |
| ConnectionLens IJ (Anadiotis et al., 2021) | https://consensus.app/papers/details/63972b19c4315cba95099b2c2d94ad21/?utm_source=cursor | heterogeneous sources; journalism | paper-keyword-search-graphs |
| DKWS distributed keyword search | https://consensus.app/papers/details/42888b1b2000598092fd1cabcaad1ada/?utm_source=cursor | cluster keyword; skip unless scale-out | (queue) |
| BM25+HNSW / GQL | — | already closed | ruvector-hybrid-bm25-dense / paper-iso-gql-gpml |

### Query 87 — semi-external / out-of-core graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| FlashGraph (Zheng et al., 2014) | https://consensus.app/papers/details/5856a5be5f2d53b0a0245d594ab33c46/?utm_source=cursor | vertex RAM; edge lists on SSD | paper-semi-external-graph |
| GraphMP (Sun et al., 2017) | https://consensus.app/papers/details/2327f53f45a15254864017f38a378a7e/?utm_source=cursor | VSW; skip shards; edge cache | paper-semi-external-graph |
| Functional external graphs / SEM (Abello et al., 1998) | https://consensus.app/papers/details/6912c0596270573f97f165c742554527/?utm_source=cursor | vertices fit; edges do not | paper-semi-external-graph |
| ROSE read-only SEM (Blelloch et al., 2021) | https://consensus.app/papers/details/f6798fbd87d157cf95f4b6720417ce41/?utm_source=cursor | shared compressed edges; no write wear | paper-semi-external-graph |
| SEM survey (Huang et al., 2019) | https://consensus.app/papers/details/45b81aca605150dbaf7a4dbafccd1444/?utm_source=cursor | out-of-core vs SEM on multicore | paper-semi-external-graph |
| FAM-Graph (Zahka et al., 2022) | https://consensus.app/papers/details/b9bdc32dae2b55a7b007686d67dd02dd/?utm_source=cursor | fabric-attached edge tier | paper-semi-external-graph |
| Clip / Seraph I/O amount | https://consensus.app/papers/details/173afa5c866b59268cdcd04768d0eca9/?utm_source=cursor | less total I/O vs sequential-at-all-costs | paper-semi-external-graph |
| CXL GPU µs-latency | https://consensus.app/papers/details/7a87cd29ef74529dbd78374263aa86bb/?utm_source=cursor | GPU; skip | (queue) |
| I/O core decomposition | https://consensus.app/papers/details/85b393dcba8b57909b46002c9e7a1212/?utm_source=cursor | analytics; skip unless a k-core pack | (queue) |
| LSM snapshots / bulk load / k² | — | already closed | paper-lsm-snapshot-compaction et al. |

## Wave 30 — Consensus batch 30 (2026-08-18)

Three queries, no filters. Distinct from k², OLAP cuboids, Horae, Timeline Index, TVG journeys, timed-automata motifs, why-not, PACT, constant-size evidence, Rocks WAL.

### Query 88 — graph summarization / quotient

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Quotient/bisimulation survey (Scherp et al., 2023) | https://consensus.app/papers/details/2da4168a7db95b8ea3fdfa14dfdec034/?utm_source=cursor | equivalence classes; queries on the quotient | paper-graph-summarization-quotient |
| MoSSo incremental lossless (Ko et al., 2020) | https://consensus.app/papers/details/a6215a2a69a954ac8c6fe5d7db10869b/?utm_source=cursor | supernodes + corrections; dynamic stream | paper-graph-summarization-quotient |
| SWeG web-scale (Shin et al., 2019) | https://consensus.app/papers/details/2050ec761c1b5dc3a0795d5be6128ee3/?utm_source=cursor | parallel; combinable with other compressors | paper-graph-summarization-quotient |
| optGS (Lai et al., 2023) | https://consensus.app/papers/details/fce33495f639528d874f020664452a90/?utm_source=cursor | compactness + speed vs prior lossless | paper-graph-summarization-quotient |
| Quality-guarantee summaries (Riondato et al., 2014) | https://consensus.app/papers/details/3d7dd745b34d5550b9bcffbfcf9afbbe/?utm_source=cursor | lossy; reconstruction/cut-norm error | paper-graph-summarization-quotient |
| SSumM sparse MDL (Lee et al., 2020) | https://consensus.app/papers/details/55c9dac08e8a5653b8bfc28cfeb16cc9/?utm_source=cursor | sparsify summary under bit budget | paper-graph-summarization-quotient |
| IBA-OTC lossless KG (Javed et al., 2024) | https://consensus.app/papers/details/5006ff590a415744af43ffaa06e69d2e/?utm_source=cursor | super-signatures + ±/* corrections | paper-graph-summarization-quotient |
| Personalized / OT supervised summarization | — | skip unless a user-centric pack | (queue) |
| k² / OLAP / Horae | — | already closed | paper-k2tree-succinct-graph et al. |

### Query 89 — Allen interval algebra / TCN

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Processing qualitative temporal constraints (Gerevini, 2005) | https://consensus.app/papers/details/90176079ae7559ccb82572f617bb8a95/?utm_source=cursor | IA/PA algorithms; minimal network | paper-allen-interval-constraints |
| Trends in temporal reasoning (Daykin et al., 2015) | https://consensus.app/papers/details/014b1aac8a55561cb8f697bd845bd269/?utm_source=cursor | tractable IA subclasses; graphs/posets | paper-allen-interval-constraints |
| MinCons PA/IA (Condotta et al., 2016) | https://consensus.app/papers/details/1bf067f9927659b984d9463bdf57725a/?utm_source=cursor | ≤k timeline points; NP vs convex P | paper-allen-interval-constraints |
| Metric + qualitative (Kautz & Ladkin, 1991) | https://consensus.app/papers/details/0b7c65b5ca6f57298cdeefd10a48d5a1/?utm_source=cursor | integrate linear inequalities with Allen | paper-allen-interval-constraints |
| TICSP (Keretho & Loganantharaj, 1991) | https://consensus.app/papers/details/0e336df83fe858b5beae6403b14a2856/?utm_source=cursor | qualitative + quantitative four-ary nets | paper-allen-interval-constraints |
| Graphical Allen phenotyping (Mate et al., 2019) | https://consensus.app/papers/details/fc58f2802839590b873f2ba6f14c0e80/?utm_source=cursor | bars → i2b2 temporal cohort queries | paper-allen-interval-constraints |
| IA^fuz (Badaloni & Giacomin, 2006) | https://consensus.app/papers/details/36734125badd5fb6beaa157e03ee1f33/?utm_source=cursor | fuzzy qualitative interval constraints | (queue) |
| RCC / spatial qualitative | — | cousin of GeoSPARQL | (queue) |
| Timeline Index / TVG / motifs | — | already closed | paper-temporal-interval-index et al. |

### Query 90 — provenance semirings / how-provenance

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Provenance semirings (Green et al., 2007) | https://consensus.app/papers/details/5728ebae04d75009a670bf632c0a7d37/?utm_source=cursor | polynomials unify why/bag/prob/incomplete | paper-provenance-semirings |
| Semiring-annotated data (Karvounarakis & Green, 2012) | https://consensus.app/papers/details/9b50705823945f2898145450c63405bf/?utm_source=cursor | survey; XQuery fragment | paper-provenance-semirings |
| Graph provenance algorithms (Ramusat et al., 2021) | https://consensus.app/papers/details/a262d0de9b6257f1a777c826ba530f44/?utm_source=cursor | rich routing queries; semiring taxonomy | paper-provenance-semirings |
| Semiring provenance over graphs (Ramusat et al., 2018) | https://consensus.app/papers/details/da5cfce62e81591989002f45fbdbfe8f/?utm_source=cursor | RPQ provenance; top-k/security | paper-provenance-semirings |
| SPARQLprov how-polynomials (Hernández et al., 2021) | https://consensus.app/papers/details/e30b7f19ca5a5adb81acc53e0cbf02ff/?utm_source=cursor | rewrite; spm-semirings; non-monotone | paper-provenance-semirings |
| NPCS native SPARQL provenance (Asma et al., 2024) | https://consensus.app/papers/details/dd980ab0fb4058918bc672c14b2529e7/?utm_source=cursor | how-provenance; RDF-star | paper-provenance-semirings |
| Dual-indeterminate FO (Grädel et al., 2024) | https://consensus.app/papers/details/626292e560e259198f9dfb7c581d67b8/?utm_source=cursor | negation; reverse provenance | paper-provenance-semirings |
| DataProv SQL rewrite (Pintor et al., 2025) | https://consensus.app/papers/details/3aa06ad1ba79597fba485c2655c22a7b/?utm_source=cursor | SPJUA polynomials; DBMS-independent | paper-provenance-semirings |
| OBDA / ELHr provenance | — | cousin of OBDA card | (queue) |
| why-not / PACT / constant-size evidence | — | already closed | paper-why-not-query-provenance et al. |

## Wave 31 — Consensus batch 31 (2026-08-18)

Three queries, no filters. Distinct from STE, DP, constant-size receipts, result cache, admission, CardEst, Graphflow delta-join, federation, summarization supernodes, k², Horae. Continuous subgraph matching stays skip (Graphflow cousin).

### Query 91 — graph watermarking / fingerprinting

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Watermarking maps (Khanna et al., 2000) | https://consensus.app/papers/details/f41b367015065ec09b1b3a4ece9008b9/?utm_source=cursor | length perturbations; query-only detection | paper-graph-watermark-fingerprint |
| Graph DB watermark pseudo-nodes (Hristov et al., 2023) | https://consensus.app/papers/details/00b18f01e9ae5cefbc81dfec700cc795/?utm_source=cursor | dummy vertices; randomized vs GA | paper-graph-watermark-fingerprint |
| Fing external fingerprint (Drosis et al., 2025) | https://consensus.app/papers/details/16adcfa51a6b5e9db9fa0764d476bb5b/?utm_source=cursor | no graph mutation; NP-hard extract | paper-graph-watermark-fingerprint |
| KGMark dynamic KG (Peng et al., 2025) | https://consensus.app/papers/details/0607bfd52aba5decba0c3124ee281406/?utm_source=cursor | diffusion fingerprints; temporal robustness | paper-graph-watermark-fingerprint |
| DRGW disentangled (Li et al., 2026) | https://consensus.app/papers/details/5e40e429b8ad5ea2aaa8d5e1d34a9cea/?utm_source=cursor | carrier vs structure; invertible embed | paper-graph-watermark-fingerprint |
| Relational DB watermark surveys | https://consensus.app/papers/details/f011eec95b915c8e96e22f908208db4d/?utm_source=cursor | ownership/tamper/traitor-tracing | paper-graph-watermark-fingerprint |
| Diffusion / DNN image watermarks | — | not graph data | (queue) |
| STE / DP / constant-size evidence | — | already closed | paper-graph-structured-encryption et al. |

### Query 92 — multi-query optimization

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| SPARQL MQO (Le et al., 2012, ICDE) | https://consensus.app/papers/details/510af3e4552a5c6da9ec7bf140484b83/?utm_source=cursor | common sub-structures; engine-agnostic | paper-sparql-multi-query-opt |
| SPARQL MQO evaluation (Chen et al., 2018) | https://consensus.app/papers/details/f254ff62ecad5ed69bcec25ac6f288dc/?utm_source=cursor | share BGP evaluation | paper-sparql-multi-query-opt |
| SwarmGuide RPQ MQO (Abul-Basher et al., 2016/17) | https://consensus.app/papers/details/dd0b78289c435b029e812fcc8b98c5b1/?utm_source=cursor | batched RPQs; visual workloads | paper-sparql-multi-query-opt |
| RPQ MQO ICDE (Abul-Basher, 2017) | https://consensus.app/papers/details/e40aa09b154d5d59856976b5a4cd050c/?utm_source=cursor | globally optimized RPQ batch | paper-sparql-multi-query-opt |
| Federated RDF MQO (Peng et al., 2021) | https://consensus.app/papers/details/129b69f5e30d5432b85b494ed414563b/?utm_source=cursor | SPARQL 1.1 share; shipment cost | paper-sparql-multi-query-opt (share); federation card owns SERVICE |
| Graphflow intersection sharing | https://consensus.app/papers/details/fb85e7286871584bad37fa37aac53834/?utm_source=cursor | already closed (continuous delta) | paper-hybrid-wcoj-intersection-cost |
| Result cache / admission / CardEst | — | already closed | paper-graph-query-result-cache et al. |

### Query 93 — spectral sparsification

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Spectral sparsification (Spielman & Srivastava, 2011) | https://consensus.app/papers/details/3f7db731ea01554b88e5240b65629eab/?utm_source=cursor | nearly-linear size; Laplacian form | paper-spectral-sparsification |
| Spectral sparsification survey (Batson et al., 2013) | https://consensus.app/papers/details/e9d8f1f026ce50f28a79bb65a1ef436a/?utm_source=cursor | cuts + SDD solvers | paper-spectral-sparsification |
| Resistance sampling via RandNLA (Charalambides et al., 2023) | https://consensus.app/papers/details/274eba1d7c5c594491b5bc35d5014650/?utm_source=cursor | sample ∝ weights; unbiased Laplacian | paper-spectral-sparsification |
| Restricted spectral coarsening (Loukas, 2018) | https://consensus.app/papers/details/54ebdd20839d5cbf9f240c11fc2f82c5/?utm_source=cursor | contrast: merge vertices | paper-spectral-sparsification (contrast) |
| Dynamic directed sparsifiers (Zhao, 2025) | https://consensus.app/papers/details/4afb8f44d732568983e832512a31a0e2/?utm_source=cursor | fully dynamic; Eulerian | paper-spectral-sparsification |
| Hypergraph spectral sparsifiers | https://consensus.app/papers/details/6728c641a0d956a08168d7f53d808b5e/?utm_source=cursor | hypergraph; skip unless | (queue) |
| Quantum sparsification | https://consensus.app/papers/details/025ffddf6e095aefb26ed7e968de4259/?utm_source=cursor | GPU/quantum; skip | (queue) |
| Summarization / k² / Horae | — | already closed | paper-graph-summarization-quotient et al. |

## Wave 32 — Consensus batch 32 (2026-08-18)

Three queries, no filters. Distinct from Raphtory, Graphflow, CDC, SEM SSD, LSM, spectral sparsification, summarization, CardEst. Helix already owns secondary-index-as-access-path.

### Query 94 — RDF stream processing / RSP-QL

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| C-SPARQL language (Barbieri et al., 2010) | https://consensus.app/papers/details/55acd995a60f505f86d80f0ad0dfc01d/?utm_source=cursor | continuous SPARQL; windows; aggregation | paper-rdf-stream-processing |
| C-SPARQL SIGMOD Rec (Barbieri et al., 2010) | https://consensus.app/papers/details/d641ad5229465cea82738fb1cef7e7f5/?utm_source=cursor | urban computing; evolving knowledge | paper-rdf-stream-processing |
| RSP-QL unifying semantics (Dell’Aglio et al., 2014) | https://consensus.app/papers/details/7de14b996f32589091de3c4c22c5479a/?utm_source=cursor | why engines disagree; correctness | paper-rdf-stream-processing |
| Unified RSP language (Dell’Aglio et al., 2015) | https://consensus.app/papers/details/0ab4d2c346c75fa79901b4a81836123d/?utm_source=cursor | CQELS vs C-SPARQL vs SPARQL_stream | paper-rdf-stream-processing |
| StreamQR rewrite (Calbimonte et al., 2016) | https://consensus.app/papers/details/876df628e99857129d827c7336e13eaa/?utm_source=cursor | ontology rewrite onto CQELS | paper-rdf-stream-processing |
| C-Sprite hierarchical reasoning (Bonte et al., 2019) | https://consensus.app/papers/details/bb3ae4a931045c7b8aa08c8dcd3cbce3/?utm_source=cursor | constant-time taxonomy stream | paper-rdf-stream-processing |
| IncTreeRDF incremental matching (Zhang et al., 2022) | https://consensus.app/papers/details/1389e4c32bd2591ba560389a9a839547/?utm_source=cursor | window slide without full recompute | paper-rdf-stream-processing |
| RSP4J API (Tommasini et al., 2021) | https://consensus.app/papers/details/9c57f28fe97b5d7e86b098059518477a/?utm_source=cursor | RSP-QL programming abstractions | paper-rdf-stream-processing |
| Bloom stream×store joins | https://consensus.app/papers/details/8c706106ff7957c5a14e7a1fd3795d2d/?utm_source=cursor | skip unless a bloom-join spike | (queue) |
| Raphtory / Graphflow / CDC | — | already closed | paper-raphtory-lazy-temporal-views et al. |

### Query 95 — PMEM / NVRAM graph stores

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Optane graph analytics 6TB (Gill et al., 2019) | https://consensus.app/papers/details/1e20e62fbce15aa499f18a598b1077b1/?utm_source=cursor | PMEM principles; vs cluster | paper-pmem-graph-store |
| Optane evaluation + graph workloads (Peng et al., 2019) | https://consensus.app/papers/details/98567f466cd357528a37081e2b4c98ba/?utm_source=cursor | DRAM+NVM hybrid; write isolation | paper-pmem-graph-store |
| XPGraph (Wang et al., 2022) | https://consensus.app/papers/details/8d7e8144816854239d20a9cbaf056024/?utm_source=cursor | XPLine-friendly evolving graphs | paper-pmem-graph-store |
| XPGraph TOS (Wang et al., 2025) | https://consensus.app/papers/details/e560a2377ed353fb9960a483d1e9bbe9/?utm_source=cursor | scalable dynamic PMEM graphs | paper-pmem-graph-store |
| PerMA-Bench (Benson et al., 2022) | https://consensus.app/papers/details/d74acd778dc55887b821ccd181b60057/?utm_source=cursor | bandwidth/latency across Optane gens | paper-pmem-graph-store |
| HANA PMEM placement cost (Lasch et al., 2022) | https://consensus.app/papers/details/4acab042ad7f5d3f9833d2408968a1a3/?utm_source=cursor | which structures in PMEM vs DRAM | paper-pmem-graph-store |
| Post-Optane CXL research (Desnoyers et al., 2023) | https://consensus.app/papers/details/de9913c187bd52e5a4b62f5a5a148d2b/?utm_source=cursor | pooling; crash-consistency remains | paper-pmem-graph-store |
| CXL GPU µs / SEM SSD | — | queued / already closed | paper-semi-external-graph et al. |

### Query 96 — graph sampling / AQP

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Sampling from large graphs (Leskovec & Faloutsos, 2006) | https://consensus.app/papers/details/db5a297c52ee55fc85c2bc514d6bcb1e/?utm_source=cursor | forest fire / RW; ~15%; scale-up laws | paper-graph-sampling-aqp |
| Rank Degree sampling (Voudigari et al., 2016) | https://consensus.app/papers/details/124ae7f20e3e5c71a4e9eeccdfc40baa/?utm_source=cursor | edge-selection sample; Facebook crawl | paper-graph-sampling-aqp |
| Deterministic exploration (Salamanos et al., 2017) | https://consensus.app/papers/details/448cdff0f0e058909788197e85a8cfce/?utm_source=cursor | Rank Degree vs Forest Fire | paper-graph-sampling-aqp |
| KG aggregate AQP (Wang et al., 2022) | https://consensus.app/papers/details/ca846b060e3a585081b3551be0f64322/?utm_source=cursor | COUNT/SUM/AVG + confidence interval | paper-graph-sampling-aqp |
| WanderJoin-style CardEst (Hu et al., 2024) | https://consensus.app/papers/details/083b515071f25f1e9d167582478283fe/?utm_source=cursor | cousin of planner CardEst | paper-graph-cardinality-estimation (cousin) |
| GPU random walk | — | skip | (queue) |
| Spectral sparsification / summarization / CardEst | — | already closed | paper-spectral-sparsification et al. |

## Wave 33 — Consensus batch 33 (2026-08-18)

Three queries, no filters. Remaining nouns R1–R3 from `research-boundary.md`. Distinct from DP, STE, watermark, TVG journeys, motifs, interval indexes, RSP, MATCH, BANKS, summarization. STGNN stays GNN-skip.

### Query 97 — graph k-anonymity

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| k-degree anonymity (Liu & Terzi, 2008) | https://consensus.app/papers/details/c032ced8cdad5477a5e3a4cd8e574524/?utm_source=cursor | degree sequence; min edits | paper-graph-k-anonymity |
| Randomization vs k-degree (Ying et al., 2009) | https://consensus.app/papers/details/c9cb09b901015b8191c0547cf9ec302e/?utm_source=cursor | identity vs link disclosure | paper-graph-k-anonymity |
| MLDA directed multi-level (Hao et al., 2023) | https://consensus.app/papers/details/f9a5a92bd5b55334b6b9a0b763bc2f35/?utm_source=cursor | fake nodes; community merge | paper-graph-k-anonymity |
| 1HIkDA 1-hop + k-degree (Gong et al., 2026) | https://consensus.app/papers/details/9724d4308a0e5e59b52b3530e8a9f1dd/?utm_source=cursor | neighborhood indistinguishability | paper-graph-k-anonymity |
| Sequential KG kw-tad (Hoang et al., 2021) | https://consensus.app/papers/details/3cc082430d5050b2b27eb4bde51d11b8/?utm_source=cursor | w consecutive releases | paper-graph-k-anonymity |
| Personalized k-ad KGs (Hoang et al., 2024) | https://consensus.app/papers/details/46f567646fdf5570a2301c70bf14ff71/?utm_source=cursor | per-user k | paper-graph-k-anonymity |
| Tabular k-anonymity / hybrid DP | — | not graph structure | (cousin) |
| DP / STE / watermark | — | already closed | paper-graph-differential-privacy et al. |

### Query 98 — STKG / property streams

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| stRDFS spatiotemporal RDF (Zhu et al., 2020) | https://consensus.app/papers/details/28e21a54d6135aca88b5de68c42ff18e/?utm_source=cursor | properties labeled ST; RDF unchanged | paper-stkg-property-streams |
| SSTKG simple STKG (Yang et al., 2024) | https://consensus.app/papers/details/7666a478606757d6943cd291c4f9ff32/?utm_source=cursor | construct+embed dynamic facts | paper-stkg-property-streams |
| Industrial KG + IoT series (Zhou et al., 2022) | https://consensus.app/papers/details/fa1d5a867505539e8d4c9ed9ca81070d/?utm_source=cursor | series semantics on KG nodes | paper-stkg-property-streams |
| STGNN forecasting family | various | GNN; skip | (queue) |
| TVG / motifs / interval index / RSP | — | already closed | paper-tvg-journeys-restless et al. |

### Query 99 — frequent subgraph mining

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| gSpan (Yan & Han, 2002) | https://consensus.app/papers/details/7c0ae3584e265cf7a389e8445ee59826/?utm_source=cursor | DFS code; no candidate gen | paper-frequent-subgraph-mining |
| FFSM (Huan et al., 2003) | https://consensus.app/papers/details/5dd430c02708509ab1da904a134e92ea/?utm_source=cursor | vertical algebraic search | paper-frequent-subgraph-mining |
| GraMi single large graph (Elseidy et al., 2014) | https://consensus.app/papers/details/41416bb508505f4988380b4343417972/?utm_source=cursor | min instances; pattern matching | paper-frequent-subgraph-mining |
| cgSpan closed (Shaul et al., 2021) | https://consensus.app/papers/details/09c3ac1da8335551a184f2df306a7fdc/?utm_source=cursor | closed subgraphs | paper-frequent-subgraph-mining |
| Subdue compression (Ketkar et al., 2005) | https://consensus.app/papers/details/54fa0a270e8a5a81b375540d05c61150/?utm_source=cursor | interesting ≠ all-frequent | paper-frequent-subgraph-mining |
| TipTap streaming k-vertex (Nasir et al., 2021) | https://consensus.app/papers/details/a85efb21dbeb594391b8a47ebae6e3e1/?utm_source=cursor | evolving graphs; approximate | paper-frequent-subgraph-mining |
| DP-gSpan / temporal-quadruple FSM | — | skip unless | (queue) |
| MATCH / BANKS / motifs / summarization | — | already closed | paper-subgraph-iso-vs-homomorphism et al. |

## Wave 34 — Consensus batch 34 (2026-08-18)

Three queries, no filters. Remaining nouns R4–R6 from `research-boundary.md`. Last planned wave. Distinct from LSM snapshots, WAL recovery, SEM, subgraph iso, WCOJ similarity joins, ULTRA, vertex-cut, summarization, OLAP, scirs library.

### Query 100 — event-log vacuum / legal hold

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Foundation for vacuuming TT DBs (Skyt et al., 2002) | https://consensus.app/papers/details/b7a5c5f7ac615549b23540b1f13751d5/?utm_source=cursor | semantics of physical removal; query against vacuumed past | paper-event-log-vacuum-legal-hold |
| Vacuuming Temporal Databases (Skyt et al., 1998) | https://consensus.app/papers/details/a2cbd18ab90b5110907056483d50b94c/?utm_source=cursor | intercept queries affected by vacuum | paper-event-log-vacuum-legal-hold |
| Trustworthy vacuuming and litigation holds (Hasan et al., 2010) | https://consensus.app/papers/details/0c0f0a1f662b500da6af5d1587400554/?utm_source=cursor | hold ≠ vacuum; audit illegal delete | paper-event-log-vacuum-legal-hold |
| Schema vacuuming (Roddick, 2009) | https://consensus.app/papers/details/44cda4399239521682e906dfd6f603b6/?utm_source=cursor | vacuum schemata with versioning | paper-event-log-vacuum-legal-hold |
| MVCC version GC / TVA / agent memory | — | cousins / skip | paper-lsm-snapshot-compaction et al. |

### Query 101 — graph edit distance

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| GED survey (Gao et al., 2010) | https://consensus.app/papers/details/9eb8f457ae595c619963c875a1128e14/?utm_source=cursor | inexact matching; cost models | paper-graph-edit-distance |
| AStar-BMao GED verification (Chang et al., 2023) | https://consensus.app/papers/details/f980cc665db150bca979aaa74569dcde/?utm_source=cursor | tighter bounds; less RAM | paper-graph-edit-distance |
| ILP FORI exact GED (D'Ascenzo et al., 2025) | https://consensus.app/papers/details/f962c7062fa959fa8cd6d0b036e01010/?utm_source=cursor | orientation ILP; IAM solved | paper-graph-edit-distance |
| FGWAlign OT GED (Tang et al., 2025) | https://consensus.app/papers/details/847741cf2a0a55d1ae44fc66d0dd82cb/?utm_source=cursor | Fused GW ≡ GED alignment | paper-graph-edit-distance |
| GEDGNN / SimGNN / GraphSim | various | GNN score, often no path | paper-graph-edit-distance (pack, not SoT) |
| Subgraph iso / WCOJ simjoin / ULTRA | — | already closed | paper-subgraph-iso-vs-homomorphism et al. |

### Query 102 — community detection lease

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Leiden guarantees (Traag et al., 2019) | https://consensus.app/papers/details/26e336dfa3d45900b9c0b5ec45730493/?utm_source=cursor | connected communities; beats Louvain | paper-community-detection-lease |
| Distributed Louvain (Ghosh et al., 2018) | https://consensus.app/papers/details/af1568da366858f49b6ee99358b97a50/?utm_source=cursor | MPI Louvain | paper-community-detection-lease |
| Louvain 15 years later (Blondel et al., 2023) | https://consensus.app/papers/details/c5ab269ca7ed57e3a1ac22ad54c9a7bf/?utm_source=cursor | survey of generalizations | paper-community-detection-lease |
| Hypergraph modularity / h-Louvain | — | cousin of hypergraph card | (queue) |
| GPU batched Louvain | — | skip | (queue) |
| Vertex-cut / summarization / OLAP / scirs | — | already closed | paper-graph-partition-vertex-cut et al. |


## Wave 11 — Consensus batch 11 (2026-08-17)

### Query 31 — streaming temporal graphs

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Raphtory (Steer et al., 2020) | https://consensus.app/papers/details/d854b735d72e572e9b45275da9e9ef40/?utm_source=cursor | Stream insert; full history in memory | paper-raphtory-lazy-temporal-views |
| Raphtory JOSS (Arnold et al.) | https://arxiv.org/html/2306.16309 | Chronological log + lazy views | paper-raphtory-lazy-temporal-views |
| TARIS | https://consensus.app/papers/details/4a65890232335b4ba4542b9352f776bd/?utm_source=cursor | Incremental time-respecting algos on streams | (queue) |
| Streaming graph taxonomy (Besta et al.) | https://consensus.app/papers/details/cd7e8b5ad493535b8198d9bf528bb485/?utm_source=cursor | Dynamic vs temporal vs online | (context) |
| GRADOOP / T-GQL (again) | — | already closed | paper-gradoop-tpgm / paper-tgql-intervals |

### Query 32 — graph ABAC / path authorization

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Comparison of AC for graphs (Mohamed et al., 2024) | https://consensus.app/papers/details/385ebdfcde2053a6ac6840422ed5bb00/?utm_source=cursor | Survey: models, rewrite vs native | paper-xacml4g-path-abac |
| XACML4G (arXiv:2306.12819) | https://arxiv.org/pdf/2306.12819 | Path constraints; datastore-independent enforce | paper-xacml4g-path-abac |
| ABAC Neo4j Cypher rewrite | https://consensus.app/papers/details/5c1f288af18450da8e94e8ed5ff2376f/?utm_source=cursor | Query rewrite to safe Cypher | paper-xacml4g-path-abac |
| Rewriting Graph-DB queries ABAC | https://consensus.app/papers/details/5d21a8521d3f5ecf9494a9c1c99ff796/?utm_source=cursor | ACaaS intercept | paper-xacml4g-path-abac |
| AReBAC Nano-Cypher | https://consensus.app/papers/details/180f1b9f9cd45f2aaed1913b8422bd8b/?utm_source=cursor | Weave policy with query | (related) |

### Query 33 — KG entity resolution / canonical identity

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Complex-entity graph ER (Kirielle et al., 2022) | https://consensus.app/papers/details/97d76d7a5ab053e88a7f05427f348aaf/?utm_source=cursor | Temporal+relational complex entities | paper-temporal-complex-entity-resolution |
| Entity alignment survey (Zeng et al., 2021) | https://consensus.app/papers/details/7e13fbaa4731591f819fbcae0572b92e/?utm_source=cursor | Cross-KG equivalence | paper-temporal-complex-entity-resolution (context) |
| Embedding EA benchmark (Sun et al., 2020) | https://consensus.app/papers/details/42f34d65d17a5fc4936b37efecb86607/?utm_source=cursor | Embedding similarity ≠ SoT | (contrast) |
| SAGE no-merge virtual links | https://consensus.app/papers/details/11c30e1788c659b48fbcd505aca3e426/?utm_source=cursor | Explainable joins without merge | paper-sameas-virtual-identity |

## Wave 12 — Consensus batch 12 (2026-08-17)

### Query 34 — incremental time-respecting streaming

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| TARIS | https://consensus.app/papers/details/4a65890232335b4ba4542b9352f776bd/?utm_source=cursor | Incremental ICM ≡ batch; vs Tink/GRADOOP | paper-taris-incremental-icm |
| ICM / GRAPHITE | https://consensus.app/papers/details/4d96a2d9c57e52d7b3100272a8a34bb4/?utm_source=cursor | Interval as compute grain | paper-taris-incremental-icm |
| Raphtory (again) | — | already closed | paper-raphtory-lazy-temporal-views |
| GPU time-respecting CSR | https://consensus.app/papers/details/6623a3bc82f65bc9b0e304007348e88b/?utm_source=cursor | GPU TRG construction | (queue; not P0) |

### Query 35 — temporal grants / history-aware AC

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Bertino temporal authorizations | https://consensus.app/papers/details/5c2a963f45e0555caf30c28a0a46c5cf/?utm_source=cursor | Valid-time on grants; materialize | paper-temporal-grants-as-facts |
| TRBAC | https://consensus.app/papers/details/9e02682b80245775a6a68071e6eca2ca/?utm_source=cursor | Periodic role enable/disable | paper-temporal-grants-as-facts |
| HO(T)-ReBAC | https://consensus.app/papers/details/fdaa10d51a9b5130b6799ff025f9c85e/?utm_source=cursor | History of relationship changes | paper-temporal-grants-as-facts |
| HIPAA FGTAC survey | https://consensus.app/papers/details/54ff2657409654709bbba38a6c81d382/?utm_source=cursor | SQL temporal AC demand | paper-temporal-grants-as-facts (demand) |
| XACML4G (again) | — | path shape, not grant time | paper-xacml4g-path-abac |

### Query 36 — cryptographic receipts / ADS

| paper | url | capability seed | card id |
|-------|-----|-----------------|---------|
| Constant-size evidence (Kao) | https://consensus.app/papers/details/c7650cf7f6dd5f0dbfa15036584aa3b1/?utm_source=cursor | Fixed-size tuple per event | paper-constant-size-evidence |
| Quantum-adversary-resilient evidence | https://arxiv.org/abs/2512.00110 | PQ games on same layout | paper-constant-size-evidence |
| Crosby/Wallach tamper-evident logs | https://consensus.app/papers/details/ec9fa840998950a18864a6c11bdb5147/?utm_source=cursor | Logarithmic inclusion proofs | paper-constant-size-evidence |
| Goodrich graph ADS | https://consensus.app/papers/details/cec147ed0bfe5fedb0ccf7aa8b582232/?utm_source=cursor | Authenticated path/connectivity | (related; untrusted responder) |
| PAGB / vChain / vProChain | various | blockchain-assisted ADS | (queue: not SoT) |

