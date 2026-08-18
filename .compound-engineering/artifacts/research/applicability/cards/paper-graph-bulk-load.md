---
id: paper-graph-bulk-load
source: paper
axes: [Data, Time, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# First load is parse → intern → append events — not CDC and not a second SoT

Papers: Campero Durand et al. (2018) users name **loading** as the top practical graph-DB bottleneck; server-side bulk load; write batching up to 64× [1]; [Then et al.](https://consensus.app/papers/details/3ba480328ad7522093bba8e789562d82/?utm_source=cursor) (2016) whole pipeline: parse, dense identifier relabel, write in-memory structures; partitioning strategies [16]; [Instant Loading](https://consensus.app/papers/details/6b4da0d9059759c7997c4e969e4a0dba/?utm_source=cursor) (Mühlbauer et al., 2013, VLDB) CSV at wire speed on multicore; load–work–unload [4]; [PGDF](https://consensus.app/papers/details/146cbce9548c5a02b7dfc7d4ae4e1c97/?utm_source=cursor) (Angles et al., 2024) a text interchange for property graphs (schema+data lines) vs GraphML/JSON-Neo4j [7]; GraphOne dual edge-list/adjacency ingest [17]. Distinct from `paper-graph-cdc-ingest` (ongoing foreign WAL → events), `paper-rdf-term-dictionary` (intern map, not the loader), `oxixml-xml-rdf-stack` (document parse, not bulk PG load). G2GML RDF→PG stays queued as mapping dialect.

## 1. Raw idea

Before CDC exists, you have files: CSV, GraphML, PGDF, n-triples. The work is **parse in parallel, intern IDs densely, emit a burst of events** (or build the first CSR lease), not row-by-row MERGE [1][16]. Batching writes dominates [1]. A portable PG text format (PGDF) is the interchange noun [7]. Relabeling to dense IDs is the loader’s job, then the dictionary card owns the map.

## 2. STCA applicability

Time/Data: bulk load **appends a prefix of the log** (or a sealed snapshot + log from offset 0). It is not a second SoT: after load, truth is the events. Composition: loader is a pack (`build` once); CDC is the *incremental* cousin. Query: do not query a half-loaded CSR; admission/SSI apply after the load barrier. Instant Loading’s load–work–unload is a lease cycle, not mutate-in-place.

## 3. Quality / cost

Usefulness high: legal/science dumps arrive as files. Optimality med: batching and parallel parse are real; near-storage JSON accelerators are optional. Cost: P0 = single-thread append of interned events; honeycomb = parallel parse + dense relabel + batched log append + load barrier; do not vendor Neo4j `neo4j-admin import` as the quantum.

## 4. Demand

38B-object dumps cannot wait for per-edge Cypher. Engine demand: a load barrier event, then fold.

## 5. Niche → effect

`no niche`
