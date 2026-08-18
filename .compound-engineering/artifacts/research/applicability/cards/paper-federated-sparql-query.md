---
id: paper-federated-sparql-query
source: paper
axes: [Query, Space, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/science: SERVICE an endpoint you do not ingest (opposing docket, PubMed) — federation is not named-graph scope inside one fold"
status: closed
channels_failed: []
---

# Federation is SERVICE across stores — not GRAPH inside one store and not Geo-Raft

Papers: [FedUP](https://consensus.app/papers/details/2f9179016a8a5df891e1807f049705da/?utm_source=cursor) (Aimonier-Davat et al., 2024, WWW) — “minimal sources per triple pattern” still enumerates empty combinations; **result-aware** plans; engines today scale to dozens not hundreds of endpoints [1]; [SPARQL 1.1 federation](https://consensus.app/papers/details/cb49500582a8588d933801315e31dbd7/?utm_source=cursor) (Buil Aranda et al., 2012) `SERVICE` syntax and semantics [2]; [heterogeneous Linked Data Fragments](https://consensus.app/papers/details/5c1f72940d0e50ec87acfa1aec9c0052/?utm_source=cursor) (Heling & Acosta, 2021/2022) endpoints vs TPF vs other LDF — one federation, many interfaces [3][5]; fine-grained FedBench eval: source-selection time and ASK count dominate runtime [4]; Odyssey cost stats [10]; LargeRDFBench: simple-query ranking ≠ hard-query ranking [11]. Distinct from `paper-named-graphs-rdf-dataset` (`GRAPH` scope in *one* dataset), `paper-geo-raft-wan` (replicate the log), RAGraph (WAN *analytics*, queued skip). Agentic SPARQL-MCP [16] is demand, not a second card.

## 1. Raw idea

A federated query contacts **several independent endpoints** and joins remotely. SPARQL 1.1 `SERVICE` names the endpoint [2]. The bottleneck is source selection + empty join combinations, not Cypher parsing [1][4]. Heterogeneous interfaces (full SPARQL vs triple-pattern fragments) need interface-aware operators [3]. Completeness vs utility: you may legally skip low-quality sources [17]. Do not confuse with named graphs: `GRAPH` selects a slice *you already store*; `SERVICE` asks a store *you do not own*.

## 2. STCA applicability

Query 070/071: federation is a **compile plan** (which remote, which join order, which fragment interface), executed against *other people’s* leases. Space: each remote is a foreign fold; Kutha’s log stays SoT for *local* facts only. Composition: results enter as events with provenance of the endpoint + query, not as silent CSR merges. Verify: a federated hop is not replayable unless the remote is snapshotted or the answer is receipted. Do not Geo-Raft someone else’s graph into Kutha.

## 3. Quality / cost

Usefulness high: packs will not copy PubMed or an opposing CMS. Optimality med: FedUP is a step-change on large federations [1]; classic engines die at dozens of endpoints [15]. Cost: P0 = no SERVICE; honeycomb = explicit remote catalog + result-aware plans; MCP-agent SPARQL is an Agent pack on top [16].

## 4. Demand

Legal and scientific questions span graphs you will never ingest. Engine demand: `SERVICE`-class operator in the compiler, with source selection that does not explode.

## 5. Niche → effect

Legal/science: SERVICE an endpoint you do not ingest (opposing docket, PubMed) — federation is not named-graph scope inside one fold.
