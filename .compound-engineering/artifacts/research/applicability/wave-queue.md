# Next-wave queue

Bound: `research-boundary.md`. In-scope remaining nouns **R1–R6 are closed** (Waves 33–34). Everything in this table is skip-unless. Do not promote a row into a card unless the user names a distinct noun or supplies a URL.

Sources discovered or deferred.

| queued | found_from | notes |
|--------|------------|-------|
| GPU WCOJ Datalog | Consensus Q7 | do not pull GPU into P0 |
| OpenCypher parser (Samyama) | samyama scout | implied by leapfrog; skip unless distinct |
| POLAR adaptive join routing | Consensus Q29 | DuckDB tuple routing; overlaps ADOPT/AQP |
| OmniSketch join cardinality | Consensus Q29 | closed as planner spike in `paper-graph-cardinality-estimation` |
| AZ sentence-level RelEx literature index | arXiv 2608.12395v1 | 3.8B sentences / Elasticsearch; not engine |
| AReBAC Nano-Cypher weave | Consensus Q32 | closed as code ReBAC in `oxify-zanzibar-rebac`; path-ABAC stays on XACML4G |
| GPU time-respecting CSR | Consensus Q34 | TRG construction on GPU; not P0 |
| Dify knowledge-pipeline custom ingest | user URL langgenius/dify | RAG ETL canvas; skip unless distinct from closed orchestration card |
| Janus reliable CRDTs / ERA epochs | Consensus Q42 | BFT/epoch arbitration; not P0 |
| RAGraph geo-distributed graph analytics | Consensus Q44 | WAN-aware query, not Multi-Raft; skip unless distinct from leases |
| LLM-suggested graph repair | Consensus Q48 | suggestions only; LLM not SoT |
| ruvector-postgres RemediationEngine | ruvector scout (hnsw-repair card) | ops compact/rebalance/throttle; skip unless distinct from closed HNSW rewiring |
| SPARQL 1.1 counting property paths | Consensus Q51 Arenas 2012 | warning already on SHACL card; skip unless a Cypher path-count spike |
| SHACL-DS named-graph datasets | Consensus Q51 | closed as Space noun in `paper-named-graphs-rdf-dataset`; validation stays on SHACL card |
| SPARQL-star / RDF-star manifestation | Consensus Q51 | export dialect, not SoT |
| KGCL as CNL dict patch | Consensus Q49 | folded into `paper-ontology-temporal-versioning`; skip unless a patch-language spike |
| PerfectMap mapping-rewrite explosion | Consensus Q50 | OBDA cost; skip unless Cypher mapping compiler |
| law-nexus event-sourced CTV resolver | law-nexus ADR-0017 | still `[proposed]`; skip until Rust fail-closed resolver ships |
| #Temporal Path / temporal betweenness | Consensus Q52 | counting journeys is #P; skip unless a centrality pack |
| Span-reachability (order-relaxed window) | Consensus Q52 | different predicate than time-respecting journeys; skip unless a pack |
| Local-first GDBMS RDTs | Consensus Q53 Pandey 2025 | cousin of `paper-crdt-graph-eventual`; skip |
| GQL increasing-edges compile-into-graph | Consensus Q55 | rewrite pack; skip unless a Cypher constraint spike |
| GPU temporal motif miners (Everest/Mint) | Consensus Q56 | not P0 |
| Timed-automata BGP as general temporal QL | Consensus Q56 | cousin of motifs; skip unless a constraint language spike |
| CHERI/TEE Wasm runtimes | Consensus Q59 | hardware pole; skip unless a capability-hardware spike |
| RAG Silo/Pool/Bridge vector tenancy | Consensus Q58 | folded into tenant-isolation card; skip unless a vector-silo spike |
| Strong simulation / graph simulation MATCH | Consensus Q62 | polynomial relaxation; skip unless a shape-match pack |
| VF3M ML-oracle subgraph iso | Consensus Q62 | EnumP; LLM off the matcher |
| ChronoGraph / DeltaGraph system-time graphs | Consensus Q63 | cousin of LSM snapshots; skip |
| GitOfThoughts git-as-memory | Consensus Q63 | cousin of Hindsight; skip (code-graph intercept closed as `engramx-code-context-spine`, not this) |
| RDF→PG information-preserving mapping | Consensus Q61 | export dialect; skip unless an RDF ingest pack |
| Local / GNN graph DP | Consensus Q64 | learn-on-noisy-graph; skip unless a GNN pack |
| Hypergraph GNN / foundation models | Consensus Q65 | demand; skip unless an HGNN spike |
| Agentic SPARQL-MCP federation | Consensus Q66 | demand cousin of federation card; skip |
| RAGraph geo-distributed graph analytics | Consensus Q44 | still skip: WAN analytics ≠ SPARQL SERVICE |
| oxirs SPARQL/SHACL execution engines | oxixml README | exec stays in oxirs-arq/oxirs-shacl; skip unless an oxirs scout |
| oxixml HDT compressed RDF | oxixml crate list | cousin of k²; skip unless a second compression card |
| oxify-authz quantum/zkp modules | oxify quantum.rs | Kyber placeholder; skip until real PQC |
| oxify WASM/Rhai Code nodes | oxify-engine | cousin of `paper-wasm-udf-sandbox` |
| scirs2 GNN/HGNN/embeddings | scirs2-graph README | cousin of `ruvector-gnn-facade`; skip unless a GNN spike |
| scirs2 non-graph crates (linalg/stats/…) | scirs workspace | not a graph capability |
| Learned GNN CardEst (LMKG/GNCE/GACE) | Consensus Q68 | skip unless a learned-planner spike; JOB first |
| STE keyword-document SSE / fuzzy adjacency | Consensus Q67 | not structured graph encryption |
| LiteMat hierarchy-in-ID inference | Consensus Q69 | folded into dictionary card; skip unless an RDFS++ pack |
| GRaCe relaxed SPARQL cache semantics | Consensus Q71 | different MATCH contract; skip unless a pack |
| HTTP-layer SPARQL result cache | Consensus Q71 | protocol; not engine |
| Decision-time as third temporal axis | Consensus Q72 | cousin of bitemporal; skip unless a third-time spike |
| ChronoGraph TinkerPop system-time versioning | Consensus Q73 | cousin of LSM snapshots / named branches; skip |
| OrpheusDB bolt-on dataset versioning | Consensus Q73 | relational dataset VCS; skip unless a data-science pack |
| Living Databases dependent-object alerts | Consensus Q73 | folded into pack-lifecycle + schema-evolution cards |
| NoSQL lazy vs eager data migration SLAs | Consensus Q74 Hillenbrand 2021 | skip unless a dual-schema expand/contract spike |
| LLM Schema Transformation Language (GSE) | Consensus Q74 | LLM not SoT for ALTER; skip |
| WiSeDB / QueryBot workload forecasting | Consensus Q75 | cloud placement/forecast; skip unless a pack |
| Hierarchical RL CPU scheduling for mixed DB | Consensus Q75 | cousin of admission; skip unless a scheduler spike |
| ULTRA rspmm relational SpMM | ULTRA scout | GNN O(V) kernel; skip unless a GNN-SpMM spike; not MPI CRP-SpMM |
| TypeGraph vector/hybrid search | TypeGraph README | pgvector / sqlite-vec / libSQL; cousin of `ruvector-hnsw` |
| TypeGraph graph-algorithms API | TypeGraph README | shortest path / WCC / PageRank on SQL; cousin of `scirs-graph-scientific` |
| Open Ontologies Dynamics/Causal/Planner | open-ontologies README | PDDL / PyWhy / action schemas; cousin of `oxify-dag-llm-orchestration` |
| Open Ontologies WASM plugin host | Cargo.toml `plugins` | cousin of `paper-wasm-udf-sandbox` |
| Open Ontologies OWL-DL tableaux | `src/tableaux.rs` | skip unless a DL-reasoner pack; RL fixpoint is the closed card |
| Higher-dim / tropical knapsack NP-completeness | Consensus Q76 | folded into max-convolution card; skip unless an ILP pack |
| Numerical max-conv for HMMs / APSP | Consensus Q76 Serang | skip unless a Viterbi pack |
| RPAI nested-aggregate IVM | Consensus Q77 | SQL nested agg; skip unless a metrics-view spike |
| Elastic index-select for label-hybrid ANN | Consensus Q77 | cousin of HNSW; skip |
| PathDB algebraic RPQ engine | Consensus Q78 | skip unless an RPQ-algebra spike |
| Data-path queries / register automata | Consensus Q78 Libkin 2012 | skip unless a data-filter RPQ pack |
| SPARQL counting property paths | Consensus Q51 | still skip; RPQ card is untimed navigation |
| Type-based split capabilities / membranes | Consensus Q79 Wismüller | skip unless a typed-capability spike |
| Generalized SI / prefix-consistent replicas | Consensus Q80 Elnikety | cousin of Geo-Raft snapshots; skip unless a replica-SI pack |
| Robustness of RC/RU isolation | Consensus Q80 Ketsman | SQL lower levels; skip |
| Near-storage JSON ingest accelerators | Consensus Q81 Kang SmartSSD | skip unless a near-storage spike |
| PGDF as canonical interchange | Consensus Q81 | folded into bulk-load card; skip unless an export-format pack |
| G2GML RDF→PG mapping | Consensus Q81 | still skip: mapping dialect, not loader |
| ParaGrapher compressed-graph loader | Consensus Q81 | cousin of k²/WebGraph; skip unless a compressed-ingest spike |
| Engramx mesh/PII/identity | engramx `src/mesh` | audit envelopes + PII strip; skip unless a mesh-authz spike; not the closed Read intercept |
| Harvey LAB other practice-area tasks | harvey-labs `tasks/{antitrust,…}` | per-matter file dumps + LLM-judge; skip unless a second eval noun distinct from `harvey-lab-firm-knowledge` |
| Federated GeoSPARQL / GIS desktop consumption | Consensus Q82 | cousin of `paper-federated-sparql-query`; skip unless a Geo-SERVICE pack |
| IndoorGML / CityGML / BIM–GIS graphs | Consensus Q82 | indoor/city packs; skip unless a second spatial noun |
| GeoGQL / DE-9IM as GQL DSL | Consensus Q82 Jahn 2025 | compile dialect; folded into spatial card unless a GQL-spatial spike |
| Temporal bipartite reachability | Consensus Q83 Chen et al. 2021 | cousin of TVG journeys; skip unless a bipartite-time pack |
| Partial 2-hop reachability-ratio tuning | Consensus Q83 | planner knob; skip unless an index-budget spike |
| Probabilistic hypergraphs / n-superedges | Consensus Q84 Fujita 2025 | cousin of `paper-hypergraph-higher-order`; skip unless a p-hypergraph spike |
| GPU uncertain-graph path sampling (BPGraph) | Consensus Q84 | still GPU; skip |
| Bayesian PKG / missing-data prediction | Consensus Q84 Freedman / Wu | embeddings + SPARQL DSL; skip unless a KG-embedding pack |
| Probabilistic data cubes | Consensus Q85 Xie | pmf cuboids; cousin of `paper-probabilistic-uncertain-graphs`; skip unless an OLAP×p spike |
| RA-OLAP / LLM-over-cubes | Consensus Q85 Ouafiq | RAG+SQL; LLM not SoT |
| Discovery-driven / cuboid-outlier Graph OLAP | Consensus Q85 | mining cuboids; skip unless an anomaly pack |
| Distributed keyword search (DKWS) / data-lake UnifySea | Consensus Q86 | scale-out IR; skip unless a federation-IR pack |
| CXL/GPU µs external memory graphs | Consensus Q87 | still GPU; skip |
| I/O-efficient core / degeneracy decomposition | Consensus Q87 | analytics; skip unless a k-core pack |
| External graph sketching (semi-streaming) | Consensus Q87 Bender 2025 | cousin of Horae; skip unless a sketch-I/O spike |
| Personalized / OT-supervised graph summarization | Consensus Q88 | user-centric summaries; skip unless a personalization pack |
| Fuzzy Interval Algebra (IA^fuz) | Consensus Q89 Badaloni | skip unless a fuzzy-temporal spike |
| RCC / qualitative spatial constraint networks | Consensus Q89 | cousin of `paper-spatial-geosparql-graphs`; skip |
| Granular / “just before” temporal algebras | Consensus Q89 Cohen-Solal | compile dialect; skip unless a granularity pack |
| OBDA / ELHr provenance polynomials | Consensus Q90 | cousin of `paper-obda-ontology-compile`; skip unless a provenance-OBDA spike |
| Fixed-point / μ-calculus provenance | Consensus Q90 Dannert/Grädel | skip unless an LFP pack |
| Diffusion / DNN image watermarks | Consensus Q91 | not graph data; skip |
| Software CFG watermarks (Collberg) | Consensus Q91 | program IP, not fold IP |
| SHACL-driven SPARQL rewrite | Consensus Q92 Thapa | cousin of `paper-shacl-sparql-compile`; skip |
| Lothbrok P2P SPARQL optimization | Consensus Q92 | cousin of federation; skip unless a P2P pack |
| Hypergraph spectral sparsifiers | Consensus Q93 | skip unless a hypergraph-sparsifier spike |
| Quantum sparsification speedup | Consensus Q93 | still quantum/GPU; skip |
| Spectral coarsening (vertex merge) | Consensus Q93 Loukas | cousin of summarization; contrast only on sparsification card |
| Bloom stream×stored RDF joins | Consensus Q94 Dia | skip unless a bloom-join spike |
| RMLStreamer RDF-from-heterogeneous-streams | Consensus Q94 | cousin of CDC/ingest; skip unless a mapping-stream pack |
| GPU/FPGA random-walk sampling | Consensus Q96 | still GPU; skip |
| Attribute-guided / surprise sampling | Consensus Q96 | skip unless a content-sample pack |
| STGNN / DynaSTy / FourierGNN node-attribute forecast | Consensus Q98 | still GNN; skip; property-stream *model* is closed as stRDFS/SSTKG |
| TKG future-fact GNN reasoners | Consensus Q98 Li STDN | cousin of ULTRA scored lease / TVG; skip |
| DP-gSpan private FSM | Consensus Q99 | skip unless a private-mining spike |
| Temporal-quadruple FSM (TeKGS) | Consensus Q99 | cousin of temporal motifs; skip |
| MVCC tuple GC (Böttcher HTAP) | Consensus Q100 | cousin of `samyama-mvcc-version-chains`; not log vacuum |
| TVA temporal graph versions | Consensus Q100 | cousin of LSM snapshots / named branches; skip |
| LLM-agent memory retention | Consensus Q100 | not event-log SoT; skip |
| GRAIL LLM-generated GED programs | Consensus Q101 | LLM not the metric; skip unless a compile-pack |
| Hypergraph Louvain / h-Louvain | Consensus Q102 | cousin of `paper-hypergraph-higher-order`; skip |
| GPU batched community detection | Consensus Q102 | still GPU; skip |
