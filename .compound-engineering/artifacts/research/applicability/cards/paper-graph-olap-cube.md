---
id: paper-graph-olap-cube
source: paper
axes: [Query, Time, Data, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/science: roll-up court/year/topic as cuboids on the fold — OLAP is a droppable aggregate graph, not MATCH and not Horae sketches"
status: closed
channels_failed: []
---

# A cuboid is an aggregated-graph lease — not MATCH and not a sketch

Papers: [Graph OLAP](https://consensus.app/papers/details/1d95fd0b416a572bac6bf421c35b6f19/?utm_source=cursor) (Chen et al., 2008/2009) informational vs topological OLAP; measure = *aggregated graph*, iceberg partial materialization [4][7]; [Graph Cube](https://consensus.app/papers/details/12b2133ea9e55173b78079a035b935d5/?utm_source=cursor) (Zhao et al., 2011) attribute aggregation **and** structure summarization; **crossboid** queries unique to networks [5]; [Temporal Graph Cube](https://consensus.app/papers/details/6b4bfd0622285167afde40da3380ac3c/?utm_source=cursor) (Wang et al., 2023) OLAP in any time range over temporal multidimensional networks; segment-tree index + snapshot similarity as a build/skip signal [1]; graphoids prove classic cube ops on labelled multi-hypergraphs [8]; structure-aware cuboid cache over property graphs (15–30× vs naive Neo4j expand-then-agg) [18]. Distinct from `paper-horae-temporal-sketches` (stream sketches, not cuboids), `paper-pg-materialized-views` (query views, not dimensional roll-up), `paper-temporal-interval-index` (interval access path, not GROUP BY), `paper-gradoop-tpgm` (distributed TPGM operators; grouping is a cousin, not this cube noun). GRADOOP [6] stays on its closed card. Probabilistic cubes and RA-OLAP/LLM stay queued.

## 1. Raw idea

MATCH answers “which nodes.” OLAP answers “summarize the *network* by dimensions”: roll-up year/court/topic into an **aggregated graph** (or a number) [4][5]. Time range is a cube axis, not a journey [1]. Crossboid compares two cuboids [5]. The cube is a **lease**: drop it, rebuild from the log.

## 2. STCA applicability

Query: compile `ROLLUP`/`SLICE` to a cuboid lease, not to WCOJ. Time: valid-time range on the cube [1]; transaction-time still the log. Data: aggregated graphs are reversible materializations (040), iceberg if space-bounded [4]. Composition: which cuboids to keep is a Cui/admission cousin, not the cube itself. Agent: LLM does not invent measures. Verify: a cuboid is explainable as a GROUP BY over events, not a quantum receipt.

## 3. Quality / cost

Usefulness high: counsel asks “filings by court × year,” not a 40-hop MATCH. Optimality med: Graph Cube and Temporal Graph Cube are real; Spark GraphTDC vs DataFrame is an implementation pole [3]. Cost: P0 = scan+group on the fold; honeycomb = selected cuboids as droppable leases. Do not stand up a second warehouse SoT. Do not RAG the cube with an LLM [20].

## 4. Demand

Dashboards over the graph will happen. Engine demand: dimensional aggregation as a first-class lease, fail-closed to “recompute from events.”

## 5. Niche → effect

Legal/science: roll-up court/year/topic as cuboids on the fold — OLAP is a droppable aggregate graph, not MATCH and not Horae sketches
