---
id: paper-community-detection-lease
source: paper
axes: [Query, Data, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# A community partition is a droppable clustering lease — not a vertex-cut and not a supernode summary

Papers: Louvain modularity heuristic (Blondel); still the default, but can emit **disconnected** communities [2][5]; [Leiden](https://consensus.app/papers/details/26e336dfa3d45900b9c0b5ec45730493/?utm_source=cursor) (Traag et al., 2019) guarantees connectedness and local optimality of subsets; faster than Louvain [2]. Distributed Louvain [1]; hypergraph modularity / h-Louvain [3][4] stay cousins of `paper-hypergraph-higher-order`. Distinct from `paper-graph-partition-vertex-cut` (cut for *scale-out placement*, not “who belongs together”), `paper-graph-summarization-quotient` (merge vertices + correction edges), `paper-graph-olap-cube` (cuboid aggregates), `scirs-graph-scientific` (library that *runs* Louvain — the noun is the lease contract, not the crate). GPU batched Louvain stays queued [12].

## 1. Raw idea

Vertices cluster into densely linked groups [2]. The partition is a **labeling of the fold**, written back as events or a view — not a second graph and not a sharding key.

## 2. STCA applicability

Query: community-id is a derived property for later MATCH/GROUP. Data: the partition is a droppable lease of a snapshot; recompute after vacuum or new events. Composition: resolution/modularity vs runtime is Cui-shaped. Time: communities as-of t, not a third clock. Verify: modularity Q is a quality score, not a receipt. LLM does not name communities as facts.

## 3. Quality / cost

Usefulness high: counsel teams, citation clusters, docket families. Optimality med: modularity NP-complete; Leiden is the corrected default [2]. Cost: P0 = no clustering; honeycomb = Leiden-on-CSR lease. Do not use community-id as the Raft shard. scirs may supply the kernel; Kutha owns when the lease is rebuilt.

## 4. Demand

“Group this graph” is asked before Cypher is known. Engine demand: snapshot → cluster → events, fail-closed to unlabeled vertices.

## 5. Niche → effect

`no niche`
