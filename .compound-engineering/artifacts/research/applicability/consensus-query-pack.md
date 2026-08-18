# Consensus query pack (Wave 1)

Author before any Consensus MCP `search` call. **At most three searches in one batch.** Then stop aggregator use for this wave (R7, KTD4, AE4).

Do not add year/domain/study_type filters unless a later decision says so (Consensus default).

## Queries (exactly three)

1. **Temporal graph as system of record**  
   `bi-temporal property graph valid time transaction time event log source of truth point-in-time query`

2. **Hot path / join optimality vs materialization**  
   `worst-case optimal join leapfrog triejoin graph adjacency CSR sparse matrix materialization incremental view maintenance`

3. **Agents inside the store, not bolted on**  
   `database native agents tool-using LLM outside trust boundary verified query operators provenance audit`

## After the batch

- Persist titles, URLs, citation links in `sources/papers.md`.
- Select a short URL list for Jina (full text).
- Map distinct capabilities to `cards/` (paper `source`).
- On rate-limit: set `channels_failed` including `consensus`, do not retry the aggregator, continue Jina on URLs already returned plus U4/U5.

## Call log

| when | queries fired | result |
|------|---------------|--------|
| 2026-08-17 | 3 (one batch) | 20+20+20 hits; aggregator stopped; Jina on 5 arXiv HTML URLs |
| 2026-08-17 | Wave 2: 3 (one batch) | 20+20+20 hits; user authorized extra Consensus; no filters |
| 2026-08-17 | Wave 3: 3 (one batch) | Q7 Datalog/IVM; Q8 indexes-as-paths; Q9 event-sourced graph |
| 2026-08-17 | Wave 4: 3 (one batch) | Q10 GRADOOP/Rost; Q11 CompactLTJ; Q12 MemStrata/stale-fact |
| 2026-08-17 | Wave 5: 3 (one batch) | Q13 GQL Rules/Kumiho; Q14 hash WCOJ/FlowLog; Q15 MemLineage/STALE; Cozo CBM scout |
| 2026-08-17 | Wave 6: 3 (one batch) | Q16 mixed vector-relational; Q17 event-sourced query cost; Q18 isolation lattice + Graphiti CBM contrast |
| 2026-08-17 | Wave 7: 3 (one batch) | Q19 annotative indexing; Q20 statutory temporal QA; Q21 Compass; pogocache CBM contrast |
| 2026-08-17 | Wave 8: 3 (one batch) | Q22 SIEVE; Q23 Graphflow columnar; Q24 Raft HA; Samyama Raft + RuVector GNN CBM |
| 2026-08-17 | Wave 9: 3 (one batch) | Q25 Kuzu/factorized WCOJ; Q26 iFVS/VecBench; Q27 CoAgent MTPO |
| 2026-08-17 | Wave 10: 3 (one batch) | Q28 ADOPT; Q29 DuckDB adaptive factorization; Q30 WCOJ similarity joins |
| 2026-08-17 | User URL 2608.12395v1: 3 lookups | arXiv id (miss); BIKG/agentic KG; exact title (preprint not indexed) |
| 2026-08-17 | Wave 11: 3 (one batch) | Q31 Raphtory/streaming temporal; Q32 graph ABAC; Q33 KG entity resolution |
| 2026-08-17 | Wave 12: 3 (one batch) | Q34 TARIS incremental ICM; Q35 temporal ABAC; Q36 hash-chain receipts |
| 2026-08-17 | Wave 13: 3 (one batch) | Q37 OCPM/event KG; Q38 no-merge identity; Q39 semantic metrics layer |
| 2026-08-18 | Wave 14: 3 (one batch) | Q40 Horae/temporal sketches; Q41 PG-Schema/constraints; Q42 CRDT graphs |
| 2026-08-18 | Wave 15: 3 (one batch) | Q43 topic-modeled tool routing; Q44 Geo-Raft/Multi-Raft; Q45 blockchain graph ADS |
| 2026-08-18 | Wave 16: 3 (one batch) | Q46 LegalSearch-R1; Q47 PG-HIVE schema discovery; Q48 graph repair under constraints |
| 2026-08-18 | Wave 17: 3 (one batch) | Q49 ontology versioning; Q50 OBDA/ontology compile; Q51 SPARQL+SHACL compile; 6 cards closed |
| 2026-08-18 | Wave 18: 3 (one batch) | Q52 TVG/journeys; Q53 graph partitioning; Q54 why/why-not provenance; 3 cards closed |
| 2026-08-18 | Wave 19: 3 (one batch) | Q55 ISO GQL/PGQ; Q56 temporal motifs; Q57 snapshot/compaction; Hindsight GitHub scout; 4 cards closed |
| 2026-08-18 | Wave 20: 3 (one batch) | Q58 multi-tenant graph isolation; Q59 sandbox/WASM graph procedures; Q60 succinct graph compression; 3 cards closed |
| 2026-08-18 | Wave 21: 3 (one batch) | Q61 named graphs/RDF datasets; Q62 subgraph matching vs WCOJ; Q63 graph branching/fork-diff; 3 cards closed |
| 2026-08-18 | Wave 22: 3 (one batch) | Q64 graph differential privacy; Q65 hypergraphs; Q66 federated SPARQL/graph query; 3 cards closed |
| 2026-08-18 | User URLs cool-japan (no Consensus) | oxixml + oxify + scirs GitHub scout; 4 cards closed |
| 2026-08-18 | Wave 23: 3 (one batch) | Q67 searchable encryption graphs; Q68 join/graph cardinality estimation; Q69 dictionary encoding identifiers; 3 cards closed |
| 2026-08-18 | Wave 24: 3 (one batch) | Q70 graph CDC/changelog ingest; Q71 query result cache; Q72 temporal interval/timeline indexes; 3 cards closed |
| 2026-08-18 | Wave 25: 3 (one batch) | Q73 graph plugin lifecycle; Q74 online PG schema evolution; Q75 query admission/workload management; 3 cards closed |
| 2026-08-18 | User URLs ULTRA/CRP-SpMM/TypeGraph/Open Ontologies (no Consensus) | 4 GitHub scouts; 4 cards closed |
| 2026-08-18 | Wave 26: 3 (one batch) | Q76 max-convolution/tropical composition; Q77 reversible materialization plugins; Q78 regular path queries; 3 cards closed |
| 2026-08-18 | Wave 27: 3 (one batch) | Q79 object capabilities; Q80 graph tx isolation/write skew; Q81 bulk/parallel graph load; 3 cards closed |
| 2026-08-18 | User URLs engramx + Harvey firm-knowledge (no Consensus) | 2 GitHub scouts; 2 cards closed |
| 2026-08-18 | Wave 28: 3 (one batch) | Q82 spatial/GeoSPARQL graphs; Q83 reachability 2-hop indexes; Q84 probabilistic/uncertain graphs; 3 cards closed |
| 2026-08-18 | Wave 29: 3 (one batch) | Q85 graph OLAP/cuboids; Q86 keyword search on graphs; Q87 semi-external/out-of-core graphs; 3 cards closed |
| 2026-08-18 | Wave 30: 3 (one batch) | Q88 graph summarization/quotient; Q89 Allen interval constraints; Q90 provenance semirings; 3 cards closed |
| 2026-08-18 | Wave 31: 3 (one batch) | Q91 graph watermarking/fingerprinting; Q92 SPARQL/RPQ multi-query optimization; Q93 spectral sparsification; 3 cards closed |
| 2026-08-18 | Wave 32: 3 (one batch) | Q94 RDF stream processing/RSP-QL; Q95 PMEM/CXL graph placement; Q96 graph sampling AQP; 3 cards closed |
| 2026-08-18 | Wave 33: 3 (one batch) | Q97 graph k-anonymity; Q98 STKG property streams; Q99 frequent subgraph mining; 3 cards closed |
| 2026-08-18 | Wave 34: 3 (one batch) | Q100 event-log vacuum/legal hold; Q101 graph edit distance; Q102 community detection lease; 3 cards closed; literature bound R1–R6 complete |

---

# Wave 3 pack

## Queries (exactly three)

7. **Datalog / IVM / WCOJ**  
   `datalog graph database incremental view maintenance differential dataflow worst-case optimal join`

8. **Indexes as access paths / unified hybrid**  
   `indexes as access paths object storage graph database vector full-text same transaction ACID`

9. **Event-sourced graph replay**  
   `event sourced graph replay fork copy-on-write append-only log temporal table as of query`

---

# Wave 2 pack (user-authorized extra Consensus)

Wave 1 R7/KTD4 cap lifted by user. Still **≤3 searches per batch**, no filters, no per-feature Consensus.

## Queries (exactly three)

4. **Write-time contradiction / isolation**  
   `isolation typed contradiction operators bitemporal knowledge graph last-write-wins evidence merge audit row`

5. **Nested worlds / write-time edge programs**  
   `nested worlds recursive property graph content-addressed immutable blobs write-time edge behaviors`

6. **Provenance finer than whole-call trust**  
   `provenance semiring argument-level provenance capability contracts proof-carrying LLM pipeline`

---

# Wave 4 pack

## Queries (exactly three)

10. **Bitemporal analytics / TPGM**  
    `bitemporal property graphs GRADOOP TPGM temporal graph analytics event detection valid time transaction time`

11. **Compact / dynamic WCOJ**  
    `CompactLTJ compact tries dynamism worst-case optimal join updates graph databases`

12. **Stale-fact vs cosine**  
    `temporal validity retrieval stale fact supersession bi-temporal ledger without LLM cosine similarity contradiction`

---

# Wave 5 pack

## Queries (exactly three)

13. **Derived rules / AGM memory**  
    `deterministic rule-based materialization property graphs GQL MERGE provenance stratified fixpoint AGM belief revision graph memory`

14. **Hash WCOJ / incremental Datalog**  
    `hash-based worst-case optimal join query-time indexes HTAP FlowLog incremental Datalog Cozo engine`

15. **Memory custody / implicit conflict**  
    `MemLineage chain of custody agent memory STALE implicit conflict execution provenance graph isolation lattice multi-agent LLM`

---

# Wave 6 pack

## Queries (exactly three)

16. **Mixed vector-relational access paths**  
    `mixed vector relational search access paths prefiltering HNSW graph DBMS annotative indexing ACID hybrid query`

17. **Event-sourced query mechanisms / cost envelopes**  
    `event-sourced query reconstruction temporal retroactive replay cost envelope immutable event history projections snapshots`

18. **Multi-agent isolation + Graphiti-class memory**  
    `multi-agent LLM concurrency isolation anomalies stale-generation phantom-tool Graphiti bitemporal knowledge graph fact invalidation`

---

# Wave 7 pack

## Queries (exactly three)

19. **Annotative indexing**  
    `annotative indexing inverted column store object graph indexes unified access path DBMS`

20. **Statutory temporal QA**  
    `statutory temporal question answering post-cutoff staleness recency bias legal retrieval as-of superseded law`

21. **Compass cooperative hybrid search**  
    `Compass general filtered search vector structured data cooperative query execution HNSW B-tree without new index`

---

# Wave 8 pack

## Queries (exactly three)

22. **SIEVE index collection**  
    `SIEVE filtered vector search collection of indexes predicate forms workload-aware analytical model`

23. **Graphflow columnar / list-based GDBMS**  
    `columnar storage list-based query processing graph database Graphflow edge property pages many-to-many joins`

24. **Raft HA for replicated logs**  
    `Raft consensus graph database high availability replication leader election snapshot MVCC`

---

# Wave 9 pack

## Queries (exactly three)

25. **Kùzu / factorized WCOJ**  
    `Kuzu graph database factorized query processor worst-case optimal join many-to-many sequential scans`

26. **iFVS / VecBench**  
    `iFVS instance-optimized filtered vector search VecBench controllable benchmark filter-agnostic PostgreSQL`

27. **CoAgent MTPO**  
    `CoAgent MTPO monotonic trajectory pre-order multi-agent concurrency control speculative writes saga compensation LLM`

---

# Wave 10 pack

## Queries (exactly three)

28. **ADOPT adaptive WCOJ orders**  
    `ADOPT adaptively optimizing attribute orders worst-case optimal join reinforcement learning episodes`

29. **DuckDB adaptive factorization**  
    `adaptive factorization DuckDB linear-chained hash tables worst-case optimal joins runtime sketches`

30. **WCOJ similarity joins**  
    `worst-case optimal similarity joins k-nearest neighbors graph database Leapfrog TrieJoin Ring`

---

# Wave 11 pack

## Queries (exactly three)

31. **Streaming temporal graphs**  
    `streaming temporal graph Raphtory windowed aggregation event time in-memory property graph`

32. **Graph fine-grained authorization**  
    `property graph access control fine-grained authorization ABAC Cypher security labels row-level`

33. **KG entity resolution / canonical identity**  
    `entity resolution canonical identifiers knowledge graph identity mapping synonym merge`

---

# Wave 12 pack

## Queries (exactly three)

34. **Incremental time-respecting streaming graphs**  
    `TARIS incremental time-respecting algorithms streaming temporal graph interval-centric computing`

35. **Temporal authorization / valid-time of permissions**  
    `temporal access control valid-time authorization history-aware ABAC graph database as-of permission`

36. **Cryptographic log receipts / hash-chain provenance**  
    `cryptographic provenance hash chain event log verifiable query receipts authenticated data structure graph`

---

# Wave 13 pack

## Queries (exactly three)

37. **Object-centric process mining / event knowledge graphs**  
    `object-centric process mining event knowledge graph decision traces case graph multiple objects`

38. **No-merge entity integration / virtual identity links**  
    `knowledge graph entity resolution without merge virtual links same-as owl sameAs no-merge identity`

39. **Semantic metrics layer vs knowledge graph**  
    `semantic layer metrics cube dbt MetricFlow business meaning query rewrite knowledge graph agents`

---

# Wave 14 pack

## Queries (exactly three)

40. **Approximate temporal graph sketches / stream summaries**  
    `Horae graph stream summarization temporal range sketch approximate temporal graph query`

41. **Property graph schema / typed constraints**  
    `property graph schema PG-Schema constraints validation GQL types SHACL`

42. **CRDT / conflict-free replicated graphs**  
    `CRDT graph database conflict-free replicated property graph eventual consistency`

---

# Wave 15 pack

## Queries (exactly three)

43. **Topic-modeled tool routing / agent dispatch**  
    `topic modeling tool routing multi-agent LLM dispatch query classification vs embedding nearest neighbor`

44. **Geo-distributed Raft / Multi-Raft**  
    `geo-distributed Raft Multi-Raft WAN replication graph database cross-datacenter consensus`

45. **Blockchain-assisted graph authenticated structures**  
    `blockchain authenticated data structure graph query PAGB vChain verifiable graph query`

---

# Wave 16 pack

## Queries (exactly three)

46. **RL / agentic temporal legal search**  
    `LegalSearch-R1 reinforcement learning temporal legal search agent statutory retrieval`

47. **Incremental property-graph schema discovery**  
    `PG-HIVE incremental schema discovery property graph clustering locality-sensitive hashing`

48. **Repairing property graphs under constraints**  
    `repairing property graphs PG-Constraints denial constraints graph repair deletions`

---

# Wave 17 pack

## Queries (exactly three)

49. **Ontology versioning / temporal ontologies**  
    `ontology versioning temporal OWL ontology evolution change management knowledge graph`

50. **Ontology compilation / ontology-based data access**  
    `ontology based data access OBDA compile OWL to SPARQL query rewriting ontology mediated`

51. **SPARQL + SHACL as compile surfaces**  
    `SPARQL property paths SHACL shapes compilation SPARQL-star RDF validation`

---

# Wave 18 pack

Janus/ERA, RAGraph, and AReBAC stay queued: they are already cousins of closed CRDT, Geo-Raft, and path-ABAC cards. This wave takes three honeycomb gaps instead.

## Queries (exactly three)

52. **Time-varying graphs / journeys / temporal reachability**  
    `time-varying graphs journeys temporal reachability time-respecting paths foremost path`

53. **Property graph partitioning / sharding**  
    `property graph partitioning sharding distributed graph database vertex-cut edge-cut workload`

54. **Why / why-not provenance for graph queries**  
    `why provenance why-not provenance missing answers graph query lineage polynomial`

---

# Wave 19 pack

Hindsight (user URL) is a GitHub scout, not a Consensus query. Literature queries avoid another agent-memory paper (Engram/Graphiti/YOTG already closed).

## Queries (exactly three)

55. **ISO GQL / SQL PGQ as query surface**  
    `ISO GQL graph query language standard SQL PGQ property graph queries Cypher`

56. **Temporal motifs / time-respecting subgraph patterns**  
    `temporal motifs temporal graphs pattern mining time-respecting subgraphs`

57. **Event-log snapshots, compaction, tiered storage**  
    `event sourced snapshot compaction tiered storage log structured merge graph database`

---

# Wave 20 pack

Queue leftovers stay skip-unless. This wave fills honeycomb 080/081 plus compressed lease layouts, not another WCOJ or agent-memory product.

## Queries (exactly three)

58. **Multi-tenant graph isolation**  
    `multi-tenant graph database isolation named graphs tenant subgraph workload isolation`

59. **Sandbox for untrusted graph procedures**  
    `sandbox untrusted stored procedures WASM graph database user-defined functions capability`

60. **Succinct / compressed graph representations**  
    `succinct graph compression WebGraph k2-tree compact graph representation`

---

# Wave 21 pack

Queue leftovers stay skip-unless (GPU, Janus/ERA, RAGraph, AReBAC, CHERI, Fan-as-second-k²). This wave takes three honeycomb gaps: RDF-dataset space (queued as skip-unless *unless* a dataset pack), static subgraph matching vs WCOJ, and Verify 061 fork-and-diff. Distinct from tenant isolation, leapfrog/WCOJ, TVG journeys, CRDT merge, and LSM snapshots.

## Queries (exactly three)

61. **Named graphs / RDF datasets as first-class space**  
    `named graphs RDF dataset SPARQL GRAPH clause quads default graph property graph subgraphs`

62. **Subgraph matching / homomorphism vs WCOJ**  
    `subgraph isomorphism homomorphism matching VF2 CFL TurboISO property graph pattern matching`

63. **Graph branching / fork-and-diff overlays**  
    `graph database branching fork copy-on-write git-like versioned graph overlay snapshot`

---

# Wave 22 pack

Queue leftovers stay skip-unless. This wave takes three honeycomb gaps that are not RAGraph WAN analytics, not OCPM multi-object events, and not named-graph slices in one store: graph differential privacy, hypergraphs as a data model, and federated query across endpoints.

## Queries (exactly three)

64. **Graph differential privacy / anonymization**  
    `differential privacy graphs edge privacy node privacy anonymization graph database query`

65. **Hypergraphs / higher-order graphs**  
    `hypergraph database higher-order networks hyperedges property hypergraph query`

66. **Federated graph / SPARQL query across endpoints**  
    `federated SPARQL query processing multiple endpoints graph database polystore federation`

---

# Wave 23 pack

Queue leftovers stay skip-unless (GPU, oxirs, GNN, PQC placeholders). This wave takes three honeycomb gaps: encrypted graph query (Security, not DP noise), planner cardinality (043; OmniSketch was skip-unless *unless* a planner spike — now that spike), and identifier dictionaries (050). Distinct from DP, ADOPT/Horae, and ontology versioning.

## Queries (exactly three)

67. **Searchable encryption for graphs**  
    `searchable encryption graph database encrypted graph query structured encryption adjacency`

68. **Graph / join cardinality estimation**  
    `cardinality estimation graph query join Synopses OmniSketch subgraph cardinality estimator`

69. **Dictionary encoding / interned graph identifiers**  
    `dictionary encoding interned strings RDF term dictionary graph database identifier mapping`

---

# Wave 24 pack

Queue leftovers stay skip-unless. This wave takes three honeycomb gaps: ingest CDC from foreign OLTP (not Raphtory’s in-engine change log), result-cache memoization (not DBSP IVM), and interval/timeline indexes (not T-GQL syntax and not TVG journey algorithms).

## Queries (exactly three)

70. **Graph change data capture / changelog ingest**  
    `change data capture graph database CDC Kafka streaming graph updates changelog Debezium`

71. **Graph query result caching / memoization**  
    `query result cache memoization SPARQL Cypher graph database cached query results`

72. **Temporal interval / timeline indexes for graphs**  
    `temporal index interval tree timeline index valid time transaction time graph database`

---

# Wave 25 pack

Queue leftovers stay skip-unless. R3 replay and Rocks WAL already closed. This wave takes honeycomb 021 (pack lifecycle), online PG schema evolution (not PG-HIVE discovery, not OWL versioning), and query admission / cascade budgets (not tenant noisy-neighbor).

## Queries (exactly three)

73. **Graph database plugin / extension lifecycle**  
    `graph database plugin extension lifecycle module system stored procedures pack versioning`

74. **Online property-graph schema evolution**  
    `online schema evolution property graph migration expand contract ALTER schema change`

75. **Query admission control / workload management**  
    `query admission control workload management graph database resource governor cascade budget`

---

# Wave 26 pack

Queue leftovers stay skip-unless (GPU, oxirs, GNN, TypeGraph vectors, Open Ontologies PDDL). This wave takes three honeycomb gaps: Cui max-convolution / tropical composition of budgets (030; not per-query admission), reversible materialization plugin protocol (040; not pack install, not DBSP IVM), and regular path queries (Cypher `*`, not WCOJ and not TVG journeys).

## Queries (exactly three)

76. **Max-convolution / tropical resource composition**  
    `max convolution tropical semiring knapsack resource allocation compositional modules database`

77. **Reversible materialization / view plugins**  
    `reversible materialization incremental view plugin graph database drop rebuild derived index`

78. **Regular path queries / variable-length paths**  
    `regular path queries RPQ graph database Cypher variable length paths automata`

---

# Wave 27 pack

Queue leftovers stay skip-unless. This wave takes three honeycomb gaps: capability security (051; not WASM sandbox, not ReBAC, not CHERI), transactional isolation anomalies on graphs (not MVCC version chains, not tenant quotas, not Cordon semantic tx), and bulk/parallel graph ingest (not CDC from a foreign WAL).

## Queries (exactly three)

79. **Object capabilities / confused-deputy in stores**  
    `object capabilities confused deputy capability based security database graph`

80. **Graph transactional isolation / write skew**  
    `snapshot isolation write skew phantom graph database transactional isolation serializable`

81. **Bulk / parallel graph loading**  
    `bulk load graph database parallel ingest GraphML CSV property graph loading`

---

# Wave 28 pack

Queue leftovers stay skip-unless (GPU, oxirs, GNN, Engramx mesh/PII, Harvey other practice areas). This wave takes three honeycomb gaps: spatial/GIS as a Space *predicate* (not Geo-Raft WAN, not SPARQL SERVICE, not named graphs), reachability *indexes* (not RPQ automata, not TVG journeys, not subgraph iso), and probabilistic/uncertain graphs (not DP noise, not TOKI contradiction).

## Queries (exactly three)

82. **Spatial / GIS property graphs**  
    `spatial property graph GeoSPARQL geographic knowledge graph GIS spatial join graph database`

83. **Reachability labeling / hop indexes**  
    `reachability index 2-hop labeling GRAIL Ferrari transitive closure graph database`

84. **Probabilistic / uncertain graphs**  
    `probabilistic graph uncertain property graph possible worlds incomplete information graph query`

---

# Wave 29 pack

Queue leftovers stay skip-unless (GPU, oxirs, IndoorGML, probabilistic hypergraphs). This wave takes three honeycomb gaps: graph OLAP / cuboids (not Horae sketches, not interval indexes, not PG views), keyword/IR search on graphs (not BM25+HNSW hybrid retrieve, not Cypher MATCH), and out-of-core / external-memory processing (not LSM snapshot compaction, not bulk load, not k² compression).

## Queries (exactly three)

85. **Graph OLAP / cuboids / snapshot aggregation**  
    `graph OLAP cube snapshot aggregation temporal graph analytics cuboid`

86. **Keyword search on graphs**  
    `keyword search graph database BANKS BLINKS schema agnostic IR`

87. **Out-of-core / external-memory graph processing**  
    `external memory out-of-core graph processing semi-external graph algorithms`

---

# Wave 30 pack

Queue leftovers stay skip-unless (GPU, oxirs, IndoorGML, DKWS, CXL, RA-OLAP). This wave takes three honeycomb gaps: structural graph summarization / quotient (not OLAP cuboids, not k² adjacency encoding, not Horae sketches), qualitative Allen-interval constraints (not Timeline Index storage, not TVG journeys, not timed-automata motifs), and how-provenance / semiring polynomials (not why-not boolean, not PACT tool arguments, not constant-size receipts). Rocks WAL recovery stays the storage analog — not this wave.

## Queries (exactly three)

88. **Graph summarization / quotient supergraphs**  
    `graph summarization supergraph quotient graph lossless compression structural`

89. **Allen interval algebra / temporal constraint networks**  
    `Allen interval algebra temporal constraint network graph database qualitative temporal`

90. **Provenance semirings / how-provenance**  
    `provenance semiring polynomial provenance how-provenance graph query`

---

# Wave 31 pack

Queue leftovers stay skip-unless. Continuous subgraph matching stays skip (cousin of Graphflow Delta Generic Join). This wave takes three honeycomb gaps: graph watermarking/fingerprinting (Security; not STE, not DP, not constant-size receipts), multi-query optimization / shared plans (Composition; not result cache, not admission, not CardEst), and spectral/cut sparsification (Data; not supernode summarization, not k², not Horae sketches).

## Queries (exactly three)

91. **Graph watermarking / fingerprinting**  
    `graph watermarking fingerprinting copyright protection graph database`

92. **Multi-query optimization on graphs**  
    `multi-query optimization graph database shared plans SPARQL`

93. **Graph sparsification / spectral sampling**  
    `graph sparsification spectral sampling cut approximation not summarization`

---

# Wave 32 pack

Queue leftovers stay skip-unless. Secondary property indexes stay on Helix (access-path noun already closed). This wave takes three honeycomb gaps: RDF stream processing / standing SPARQL on triple streams (not Raphtory PG views, not Graphflow delta MATCH, not CDC ingest), byte-addressable persistent memory graph stores (not SEM SSD edge lists, not LSM snapshots), and graph sampling for approximate analytics (not spectral sparsification reweighting, not supernode summarization, not CardEst).

## Queries (exactly three)

94. **RDF stream processing / RSP-QL**  
    `RDF stream processing C-SPARQL CQELS RSP-QL continuous SPARQL`

95. **Persistent memory / NVRAM graph stores**  
    `persistent memory NVRAM PMEM graph database byte-addressable`

96. **Graph sampling / approximate analytics**  
    `graph sampling forest fire random walk approximate query processing`

---

# Wave 33 pack

Research bound: `research-boundary.md` remaining nouns R1–R3. Queue leftovers stay skip-unless. This wave does not reopen DP, STE, watermark, TVG journeys, motifs, interval indexes, RSP windows, MATCH, BANKS, or temporal motifs.

## Queries (exactly three)

97. **Graph anonymization / k-anonymity**  
    `graph k-anonymity k-degree anonymity privacy preserving graph publishing`

98. **Time-series / STKG property streams**  
    `spatiotemporal knowledge graph time series attributes on graph nodes`

99. **Frequent subgraph mining / pattern discovery**  
    `frequent subgraph mining gSpan graph pattern discovery`

---

# Wave 34 pack

Research bound: `research-boundary.md` remaining nouns R4–R6. Last planned Consensus wave; then stop unless a user URL. Distinct from LSM snapshots, WAL recovery, SEM, subgraph iso, WCOJ similarity joins, ULTRA, vertex-cut partition, summarization, OLAP, scirs algo library.

## Queries (exactly three)

100. **Event-log retention / vacuum / legal hold**  
    `temporal database vacuum retention policy event log garbage collection legal hold`

101. **Graph edit distance / graph similarity**  
    `graph edit distance GED graph similarity matching two graphs`

102. **Community detection as clustering lease**  
    `graph community detection modularity Louvain clustering not partition`

