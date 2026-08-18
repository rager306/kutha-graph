---
id: paper-pg-materialized-views
source: paper
axes: [Data, Query, Composition, Time]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A materialization is a droppable view over the log — not a pack install and not DBSP

Papers: [views over property graphs](https://consensus.app/papers/details/be70a04652595a7fa3f31a398855daa2/?utm_source=cursor) (Han et al., 2024, PACMMOD) virtual vs materialized PG views, query rewriting, indexes; a common case is “same graph except local transforms” [8][9]; [MV4PG](https://consensus.app/papers/details/cd60fd225cbb5c5ebb314275b9299790/?utm_source=cursor) (Xu et al., 2024) materialized views for property graphs on TuGraph/Neo4j; templated maintenance for variable-length edges; up to 28.71× workload / ~100× single query [5]; Gupta/Mumick **DRed** incremental view maintenance (counts + delete/rederive) [6]; Motik **B/F** Datalog materialisation when facts have many derivations [15]; external RDB2RDF changesets without reading the remote view [1]. Distinct from `paper-dbsp-ivm` (stream algebra that *can* maintain any view), `paper-graph-pack-plugin-lifecycle` (install/activate a *module*), `paper-gql-rules-materialization` (inflationary MERGE of derived *edges*), `paper-graph-query-result-cache` (memo of one compiled query).

## 1. Raw idea

CSR, HNSW, named-graph slices, and Cypher views are the same noun: a **derived graph** you can *rebuild from the log* or *maintain incrementally*. Han: implement as rewrite (virtual) or store (materialized); local-transform views are the cheap case [8]. MV4PG: keep the duplicate MATCH pattern as a view, maintain it when the PG mutates [5]. DRed/B/F: when the view is recursive, do not recompute from scratch [6][15]. Reversible = drop the view and fold the log again; incremental = apply a changeset. Both must be valid.

## 2. STCA applicability

Data 040: every hot projection is a **plugin with `build` / `apply(delta)` / `drop`**. Time: the view is a lease at a log offset (cousin of result-cache keys). Composition: pack lifecycle (021) *loads* the plugin; this card is the *view contract*. Query: planner may rewrite MATCH through a view [8]. Do not make Neo4j/TuGraph the SoT; do not treat IVM as a second log.

## 3. Quality / cost

Usefulness high: honeycomb 040 is the reversible-materialization slogan. Optimality med: Han/MV4PG are PG-shaped; DRed is classic; DBSP remains the general algebra (already closed). Cost: P0 = rebuild-from-log; honeycomb = view catalog + delta apply for CSR/HNSW; skip GPU ANN rebuild papers.

## 4. Demand

Without drop/rebuild, a bad HNSW becomes SoT. Engine demand: `drop materialization X` is always legal.

## 5. Niche → effect

`no niche`
