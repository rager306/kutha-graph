# Matrix rollup

Rows must match `cards/*.md` `id` values. Do not add features that have no card.
Scores: usefulness / optimality / demand = `high|med|low`. Confidence = `code|spec|observed|claim|paper`.
Layer 5 is last and is not an intake filter.

| id | source | axes | usefulness | optimality | demand | confidence | layer5 | status |
|----|--------|------|------------|------------|--------|------------|--------|--------|
| paper-tgms-operators | paper | Time, Agent, Verify, Query | high | high | high | paper | Legal/compliance AS OF + claim gate | closed |
| paper-lftj-wcoj | paper | Query, Data, Composition | high | high | high | paper | no niche | closed |
| paper-llm-compiler-not-executor | paper | Agent, Verify, Time | high | med | high | paper | Scientific claims only from executed views | closed |
| paper-cordon-semantic-tx | paper | Agent, Composition, Verify | med | med | med | paper | no niche | closed |
| paper-temporal-wcoj-bgp | paper | Query, Time, Data | high | high | med | paper | no niche | closed |
| ruvector-hnsw | ruvector | Data, Query | high | high | high | code | no niche | closed |
| ruvector-cypher-empty-success | ruvector | Query | low | low | high | code | no niche | closed |
| ruvector-rvf-cow-seal | ruvector | Packaging, Time, Verify | med | med | med | code | Scientific/legal sealed export | closed |
| ruvector-hybrid-bm25-dense | ruvector | Query, Data | high | med | high | observed | no niche | closed |
| agentdb-hierarchical-memory | agentdb | Agent, Data | med | low | med | spec | no niche | closed |
| agentdb-attestation-mutation-guard | agentdb | Verify, Agent, Security | med | med | med | spec | no niche | closed |
| samyama-csr-frozen-adjacency | samyama | Data, Query | high | high | high | code | no niche | closed |
| samyama-leapfrog-triejoin | samyama | Query, Data | high | high | high | code | no niche | closed |
| samyama-mvcc-version-chains | samyama | Time, Data | high | med | high | code | no niche | closed |
| samyama-late-materialization | samyama | Query, Data | high | high | high | code | no niche | closed |
| samyama-agentic-enrichment-gak | samyama | Agent, Composition | med | med | med | observed | no niche | closed |
| rocksdb-wal-recovery | rocksdb | Time, Data, Verify | high | high | high | code | no niche | closed |
| helix-indexes-as-access-paths | helix | Data, Query | high | med | high | spec | no niche | closed |
| falkor-graphblas-sparse-adj | falkor | Data, Query | high | high | high | spec | no niche | closed |
| raven-in-db-ai-agents | raven | Agent, Composition, Security | high | med | high | spec | Enterprise in-DB agents (weak graph) | closed |
| tarantool-compute-near-data | tarantool | Composition, Data, Agent | med | med | med | spec | no niche | closed |
| paper-toki-contradiction-ops | paper | Time, Composition, Verify | high | high | high | paper | Legal/regulated isolation + audit rows | closed |
| paper-worlddb-worlds-edge-programs | paper | Space, Time, Composition | high | med | high | paper | no niche | closed |
| paper-pact-argument-provenance | paper | Agent, Security, Verify | high | high | high | paper | no niche | closed |
| paper-proof-carrying-llm-envelope | paper | Verify, Agent, Query | high | med | med | paper | Legal/clinical/finance envelope verify | closed |
| paper-user-as-code-log | paper | Time, Agent, Space | med | med | med | paper | no niche | closed |
| paper-dbsp-ivm | paper | Data, Query, Time | high | high | high | paper | no niche | closed |
| paper-ring-wcoj | paper | Query, Data | high | high | med | paper | no niche | closed |
| paper-navix-filtered-hnsw | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-activegraph-log-is-sot | paper | Time, Agent, Composition, Verify | high | high | high | paper | no niche | closed |
| paper-engram-bitemporal-memory | paper | Time, Query, Agent | high | med | high | paper | no niche | closed |
| paper-tgql-intervals | paper | Time, Query | high | med | high | paper | Legal PIT interval paths | closed |
| paper-gradoop-tpgm | paper | Time, Data, Query | med | med | med | paper | no niche | closed |
| paper-rost-bitemporal | paper | Time, Query | high | med | high | paper | no niche | closed |
| paper-compact-ltj | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-free-join | paper | Query | high | high | med | paper | no niche | closed |
| paper-memstrata-stale-fact | paper | Time, Query, Agent | high | high | high | paper | Legal statutory validity constraint | closed |
| paper-gql-rules-materialization | paper | Data, Query, Composition | high | med | med | paper | no niche | closed |
| paper-kumiho-agm-revision | paper | Time, Agent, Verify | med | med | med | paper | no niche | closed |
| paper-hash-wcoj-query-time | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-flowlog-incremental-datalog | paper | Query, Data, Time | high | high | med | paper | no niche | closed |
| paper-memlineage-memory-custody | paper | Verify, Agent, Security | high | med | high | paper | no niche | closed |
| paper-stale-implicit-conflict | paper | Time, Agent, Verify | high | med | high | paper | no niche | closed |
| paper-agent-trace-survey | paper | Verify, Agent | med | med | high | paper | no niche | closed |
| cozo-datalog-hnsw | cozo | Query, Data | high | med | med | code | no niche | closed |
| paper-mixed-vector-relational-access | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-acorn-predicate-subgraph | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-yankin-event-sourced-query | paper | Time, Query, Verify | high | high | high | paper | no niche | closed |
| paper-r3-record-replay-retroaction | paper | Time, Verify, Agent | med | med | med | paper | no niche | closed |
| paper-mas-isolation-lattice | paper | Agent, Composition, Verify | high | high | high | paper | no niche | closed |
| graphiti-bitemporal-fact-edges | graphiti | Time, Agent, Query | med | low | high | code | no niche | closed |
| paper-annotative-indexing | paper | Data, Query | high | med | high | paper | no niche | closed |
| paper-compass-cooperative-hybrid | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-statutory-temporal-qa | paper | Time, Query, Agent | high | med | high | paper | Legal statutory QA: post-cutoff staleness and recency bias; temporal validity as a hard retrieve constraint | closed |
| paper-sat-graph-legal-rag | paper | Time, Query, Verify | high | med | med | paper | Legal/constitutional retrieval: hierarchical Works vs versioned Expressions; point-in-time + provenance | closed |
| pogocache-ttl-kv-cache | pogocache | Data, Composition | low | med | med | code | no niche | closed |
| paper-sieve-index-collection | paper | Query, Data | high | med | high | paper | no niche | closed |
| paper-graphflow-columnar-list | paper | Data, Query | high | high | high | paper | no niche | closed |
| paper-graphflow-delta-generic-join | paper | Query, Time, Composition | high | high | med | paper | no niche | closed |
| paper-bach-lsm-csr-bridge | paper | Data, Time, Query | high | high | high | paper | no niche | closed |
| samyama-raft-ha | samyama | Composition, Time, Verify | med | med | high | code | no niche | closed |
| ruvector-gnn-facade | ruvector | Query, Agent | low | low | med | code | no niche | closed |
| paper-kuzu-factorized-wcoj | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-hybrid-wcoj-intersection-cost | paper | Query | high | high | med | paper | no niche | closed |
| paper-ifvs-instance-codebook | paper | Query, Data | med | high | med | paper | no niche | closed |
| paper-vecbench-fvs | paper | Query, Verify | med | med | high | paper | no niche | closed |
| paper-fvs-postgres-system-costs | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-coagent-mtpo | paper | Agent, Composition, Verify | high | med | high | paper | no niche | closed |
| paper-adopt-adaptive-wcoj-orders | paper | Query | high | med | med | paper | no niche | closed |
| paper-duckdb-adaptive-factorization | paper | Query, Data | high | high | med | paper | no niche | closed |
| paper-wcoj-similarity-joins | paper | Query, Data | high | high | high | paper | no niche | closed |
| paper-graindb-predefined-joins | paper | Query, Data | med | med | med | paper | no niche | closed |
| paper-az-projected-schema-cypher | paper | Query, Agent, Verify | high | med | high | paper | Biomedical/clinical R&D: NL over a KG scientists cannot Cypher; citation-tagged Observations consumed by chat and by patient-safety tools | closed |
| paper-az-research-plan-dag | paper | Agent, Composition, Verify | med | med | high | paper | no niche | closed |
| paper-raphtory-lazy-temporal-views | paper | Time, Data, Query | med | med | med | paper | no niche | closed |
| paper-xacml4g-path-abac | paper | Security, Query, Space | high | med | high | paper | Legal/patent knowledge graphs: authorization as path constraints (subject–task–object), not table ACLs; query rewrite so unauthorized subgraphs never return | closed |
| paper-temporal-complex-entity-resolution | paper | Data, Time, Verify | high | med | high | paper | no niche | closed |
| paper-taris-incremental-icm | paper | Time, Query, Data | high | high | med | paper | no niche | closed |
| paper-temporal-grants-as-facts | paper | Security, Time, Verify | high | med | high | paper | Legal/HIPAA and clinical: permissions have valid-time like norms; AS-OF query must apply the grant that was in force, not today’s role table | closed |
| paper-constant-size-evidence | paper | Verify, Time, Packaging | high | med | high | paper | Clinical/pharma/finance AI workflows: one fixed-size receipt per log event; Merkle/hash-chain anchor; PQ-migrate signatures without changing the tuple layout | closed |
| claim-yotg-context-graph-layers | paper | Agent, Space, Verify, Data | high | low | high | claim | no niche | closed |
| paper-ocpm-multi-object-events | paper | Time, Data, Query | high | med | high | paper | no niche | closed |
| paper-sameas-virtual-identity | paper | Data, Verify, Space | high | med | high | paper | no niche | closed |
| paper-semantic-layer-smq | paper | Query, Agent, Space | high | med | high | paper | no niche | closed |
| dify-workflow-rag-orchestration | dify | Agent, Query, Composition | med | low | high | code | no niche | closed |
| paper-horae-temporal-sketches | paper | Time, Query, Data | med | med | med | paper | no niche | closed |
| paper-pgschema-types-keys | paper | Data, Query, Verify, Space | high | high | high | paper | no niche | closed |
| paper-crdt-graph-eventual | paper | Time, Composition, Space | med | low | high | paper | no niche | closed |
| paper-topic-modeled-tool-routing | paper | Agent, Query, Composition | med | med | high | paper | no niche | closed |
| paper-geo-raft-wan | paper | Space, Composition, Time | med | low | high | paper | no niche | closed |
| paper-blockchain-graph-ads | paper | Verify, Query, Packaging | high | med | med | paper | no niche | closed |
| paper-legalsearch-r1-temporal-agent | paper | Agent, Time, Query | med | low | high | paper | Legal statutory research: RL must bind query time to the governing statute version; agent is a pack over AS-OF views, not Kutha SoT | closed |
| paper-pghive-schema-discovery | paper | Data, Query, Space | med | med | med | paper | no niche | closed |
| paper-pg-constraint-repair | paper | Verify, Data, Agent | high | med | high | paper | no niche | closed |
| ruvector-hnsw-delete-repair | ruvector | Data, Query | high | med | high | code | no niche | closed |
| paper-ontology-temporal-versioning | paper | Time, Space, Data | high | med | high | paper | no niche | closed |
| paper-obda-ontology-compile | paper | Query, Space, Agent | high | high | high | paper | no niche | closed |
| paper-shacl-sparql-compile | paper | Verify, Query, Data | high | med | high | paper | no niche | closed |
| law-nexus-kb-ontology-catalog | law-nexus | Space, Time, Agent, Verify | high | med | high | code | Legal normative AST: YAML meta-ontology + CTV component versions; clocks not collapsed | closed |
| daily-archive-schema-lifecycle | daily-archive | Space, Time, Verify, Data | high | med | high | code | Scientific archive: versioned YAML schema + PaperRevision expressions; Samyama remains a lease | closed |
| reactivegraph-ontology-version-facts | reactivegraph | Time, Space, Query, Verify | med | low | med | code | no niche | closed |
| paper-tvg-journeys-restless | paper | Time, Query, Data | high | med | high | paper | no niche | closed |
| paper-graph-partition-vertex-cut | paper | Space, Data, Composition | high | med | high | paper | no niche | closed |
| paper-why-not-query-provenance | paper | Verify, Query, Agent | high | med | high | paper | Legal AS-OF: missing statute hops explained as why-not on the compiled query, not as an LLM apology | closed |
| hindsight-four-network-tempr | hindsight | Agent, Time, Query, Verify | high | low | high | code | no niche | closed |
| paper-iso-gql-gpml | paper | Query, Space, Data | high | med | high | paper | no niche | closed |
| paper-temporal-motifs | paper | Query, Time, Data | med | med | high | paper | no niche | closed |
| paper-lsm-snapshot-compaction | paper | Time, Data, Packaging | high | med | high | paper | no niche | closed |
| paper-graph-tenant-isolation | paper | Security, Space, Composition | high | med | high | paper | Legal/enterprise: counsel–client graphs as tenant slices; noisy-neighbor isolation is not path-ABAC | closed |
| paper-wasm-udf-sandbox | paper | Security, Agent, Composition | high | med | high | paper | no niche | closed |
| paper-k2tree-succinct-graph | paper | Data, Query, Packaging | high | high | med | paper | no niche | closed |
| paper-named-graphs-rdf-dataset | paper | Space, Data, Query, Composition | high | med | high | paper | Legal/science packs: statute vs commentary vs docket (or paper vs review vs evidence) as named graphs in one fold — not three SoTs | closed |
| paper-subgraph-iso-vs-homomorphism | paper | Query, Data, Composition | high | high | high | paper | no niche | closed |
| paper-graph-branch-fork | paper | Verify, Time, Space, Agent | high | med | high | paper | no niche | closed |
| paper-graph-differential-privacy | paper | Security, Query, Time, Verify | high | med | high | paper | Legal/enterprise: publish degree histograms or AS-OF counts without identifying a party; DP is not path-ABAC and not tenant slices | closed |
| paper-hypergraph-higher-order | paper | Data, Query, Composition | high | med | high | paper | no niche | closed |
| paper-federated-sparql-query | paper | Query, Space, Composition | high | med | high | paper | Legal/science: SERVICE an endpoint you do not ingest (opposing docket, PubMed) — federation is not named-graph scope inside one fold | closed |
| oxixml-xml-rdf-stack | oxixml | Data, Verify, Query, Packaging | high | high | high | code | Legal/science: Akoma Ntoso / statute XML and RDF export; Schematron/XSD validate the document, XMLDSig signs meaning — none of that is the event-log SoT | closed |
| oxify-dag-llm-orchestration | oxify | Agent, Composition, Query | med | low | high | code | no niche | closed |
| oxify-zanzibar-rebac | oxify | Security, Query, Time | high | med | high | code | Legal/enterprise: Zanzibar tuples as a grant-graph overlay (owner⊃editor⊃viewer); not Cypher path-ABAC rewrite and not clock-on-the-role-table | closed |
| scirs-graph-scientific | scirs | Query, Data, Composition | high | med | high | code | no niche | closed |
| paper-graph-structured-encryption | paper | Security, Query, Space | high | med | high | paper | Legal/enterprise: outsource a *lease* (shortest-path / neighbor index) to an untrusted cloud; the event log never leaves the fold; STE is not DP noise and not an ADS receipt | closed |
| paper-graph-cardinality-estimation | paper | Query, Data, Composition | high | high | high | paper | no niche | closed |
| paper-rdf-term-dictionary | paper | Data, Query, Composition | high | high | high | paper | no niche | closed |
| paper-graph-cdc-ingest | paper | Time, Data, Composition | high | med | high | paper | Legal/enterprise: ingest CMS/docket OLTP via log CDC into Kutha events — dual-write is the failure mode; the foreign WAL is not Kutha SoT | closed |
| paper-graph-query-result-cache | paper | Query, Composition, Time | high | med | high | paper | no niche | closed |
| paper-temporal-interval-index | paper | Time, Query, Data | high | high | high | paper | no niche | closed |
| paper-graph-pack-plugin-lifecycle | paper | Composition, Space, Data | high | med | high | paper | no niche | closed |
| paper-online-pg-schema-evolution | paper | Data, Time, Composition, Verify | high | med | high | paper | no niche | closed |
| paper-query-admission-control | paper | Query, Composition, Security | high | med | high | paper | no niche | closed |
| ultra-kg-foundation-reasoner | ultra | Query, Agent, Data | high | med | high | code | no niche | closed |
| crp-spmm-comm-reduced | crp-spmm | Data, Query, Composition | high | high | med | code | no niche | closed |
| typegraph-typed-sql-kg | typegraph | Query, Time, Data, Agent | high | med | high | code | no niche | closed |
| open-ontologies-mcp-govern | open-ontologies | Agent, Verify, Query, Space | high | med | high | code | Legal/science: MCP validate/diff/certify AI-generated OWL; Oxigraph is working memory — not the event-log SoT | closed |
| paper-max-convolution-budgets | paper | Composition, Query, Time | high | high | high | paper | no niche | closed |
| paper-pg-materialized-views | paper | Data, Query, Composition, Time | high | med | high | paper | no niche | closed |
| paper-regular-path-queries | paper | Query, Data, Composition | high | high | high | paper | no niche | closed |
| paper-object-capabilities | paper | Security, Agent, Composition | high | med | high | paper | no niche | closed |
| paper-graph-ssi-isolation | paper | Time, Verify, Agent | high | high | high | paper | no niche | closed |
| paper-graph-bulk-load | paper | Data, Time, Composition | high | med | high | paper | no niche | closed |
| engramx-code-context-spine | engramx | Agent, Query, Time, Data | high | med | high | code | no niche | closed |
| harvey-lab-firm-knowledge | harvey | Query, Agent, Space, Time, Verify | high | low | high | code | Legal practice overlay: cross-matter retrieval over a firm DMS (matters, correspondence, deal files) — not statutory RAG and not Kutha SoT | closed |
| paper-spatial-geosparql-graphs | paper | Space, Query, Data, Time | high | med | high | paper | Legal/science: parcels, indoor graphs, topographic features as geometry on the fold — spatial is a predicate lease, not named-graph Space and not Geo-Raft | closed |
| paper-reachability-2hop-index | paper | Query, Data, Composition | high | high | high | paper | no niche | closed |
| paper-probabilistic-uncertain-graphs | paper | Data, Query, Verify | high | med | high | paper | Legal/science: extracted facts and noisy links as possible worlds — probability is not DP noise and not TOKI invalidation | closed |
| paper-graph-olap-cube | paper | Query, Time, Data, Composition | high | med | high | paper | Legal/science: roll-up court/year/topic as cuboids on the fold — OLAP is a droppable aggregate graph, not MATCH and not Horae sketches | closed |
| paper-keyword-search-graphs | paper | Query, Agent, Data | high | med | high | paper | Legal/journalism: schema-agnostic keywords over the fold return a connecting subtree — not Cypher MATCH and not BM25-on-chunks | closed |
| paper-semi-external-graph | paper | Data, Query, Composition | high | high | high | paper | no niche | closed |
| paper-graph-summarization-quotient | paper | Data, Query, Composition | high | high | high | paper | no niche | closed |
| paper-allen-interval-constraints | paper | Time, Query, Verify | high | med | high | paper | Legal/clinical: during/overlaps/meets on valid-time intervals — qualitative constraints, not Timeline Index storage and not TVG journeys | closed |
| paper-provenance-semirings | paper | Verify, Query, Data | high | high | high | paper | Legal/science: how-polynomials explain a MATCH as +/× of events — not why-not absence, not PACT tool args, not a crypto receipt | closed |
| paper-graph-watermark-fingerprint | paper | Security, Verify, Data | high | med | high | paper | Legal/enterprise: prove a leaked fold is yours — watermark/fingerprint is a perturbation or external signature lease, not SoT and not a crypto receipt | closed |
| paper-sparql-multi-query-opt | paper | Query, Composition | high | high | high | paper | no niche | closed |
| paper-spectral-sparsification | paper | Data, Query, Composition | high | high | high | paper | no niche | closed |
| paper-rdf-stream-processing | paper | Query, Time, Data | high | med | high | paper | no niche | closed |
| paper-pmem-graph-store | paper | Data, Time, Composition | high | med | med | paper | no niche | closed |
| paper-graph-sampling-aqp | paper | Query, Data, Composition | high | med | high | paper | no niche | closed |
| paper-graph-k-anonymity | paper | Security, Data, Verify | high | med | high | paper | Legal/enterprise: k-anonymous publish transform, not DP/STE/watermark | closed |
| paper-stkg-property-streams | paper | Time, Data, Query | high | med | high | paper | no niche | closed |
| paper-frequent-subgraph-mining | paper | Query, Data, Composition | high | med | high | paper | no niche | closed |
| paper-event-log-vacuum-legal-hold | paper | Time, Verify, Packaging, Security | high | med | high | paper | Legal/finance/clinical: retention + litigation hold on the log, not LSM | closed |
| paper-graph-edit-distance | paper | Query, Data, Verify | high | med | high | paper | no niche | closed |
| paper-community-detection-lease | paper | Query, Data, Composition | high | med | high | paper | no niche | closed |
