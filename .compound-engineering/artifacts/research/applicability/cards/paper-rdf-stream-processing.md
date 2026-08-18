---
id: paper-rdf-stream-processing
source: paper
axes: [Query, Time, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A standing SPARQL is a window over a triple stream — not Raphtory and not Graphflow

Papers: [C-SPARQL](https://consensus.app/papers/details/55acd995a60f505f86d80f0ad0dfc01d/?utm_source=cursor) (Barbieri et al., 2010) registers continuous SPARQL over RDF streams; **windows** of recent triples; aggregation primitives [1][2][6]; [RSP-QL](https://consensus.app/papers/details/7de14b996f32589091de3c4c22c5479a/?utm_source=cursor) (Dell’Aglio et al., 2014) unifying operational semantics so C-SPARQL/CQELS/SPARQL_stream answers become comparable [3][7]; StreamQR: rewrite continuous queries over ontologies onto CQELS [5]; C-Sprite constant-time hierarchical stream reasoning [10]; IncTreeRDF incremental subgraph matching as the window slides, not recompute-from-scratch [11]; RSP4J API under RSP-QL [12]. Distinct from `paper-raphtory-lazy-temporal-views` (in-memory PG history + lazy views, not SPARQL windows), `paper-graphflow-delta-generic-join` (Cypher continuous MATCH, not RDF window semantics), `paper-graph-cdc-ingest` (foreign WAL → events, not standing query language), `paper-taris-incremental-icm` (time-respecting algorithms). Bloom stream×store joins and RMLStreamer stay queued.

## 1. Raw idea

Triples arrive as a **stream**. You register a query once; it runs on a sliding/tumbling window of the latest facts [1][3]. That is a standing SPARQL with time, not “replay the whole log” and not “delta-join a Cypher pattern.”

## 2. STCA applicability

Query: compile RSP-QL/C-SPARQL into a standing plan over the *tail* of the event log. Time: the window is valid-time or ingestion-time on recent events — not a third clock; older truth stays in the fold. Data: the window is a **lease** of recent triples, droppable. Composition: many standing queries share the window (MQO cousin). Verify: RSP-QL correctness is “same answers at the same ticks,” not a quantum receipt. LLM does not slide the window.

## 3. Quality / cost

Usefulness high: sensors, dockets, live packs. Optimality med: engines disagree until RSP-QL [3]; incremental matching is the right pole [11]. Cost: P0 = periodic AS-OF on the tail; honeycomb = registered window queries as a pack. Do not make Esper/CQELS the SoT.

## 4. Demand

“Alert when a filing *in the last hour* matches this pattern” is a window, not a dashboard refresh. Engine demand: standing Cypher/SPARQL over a log tail, fail-closed to one-shot AS-OF.

## 5. Niche → effect

`no niche`
