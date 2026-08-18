---
id: paper-graph-partition-vertex-cut
source: paper
axes: [Space, Data, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Partition the fold (vertex-cut / hybrid-cut) — not the event log

Papers: [CUTTANA](https://consensus.app/papers/details/5f8c8288aab958f699f5fd507388c495/?utm_source=cursor) (Rezaei Hajidehi et al., 2023, DOI: 10.14778/3696435.3696437) streaming partitioner + coarsen/refine; +23% GDBMS throughput [1]; [property-graph storage-volume cut](https://consensus.app/papers/details/474b2bce1b4c5a138905e09518c09648/?utm_source=cursor) (Cui et al., 2024) |V|/|E| balance ≠ byte balance when properties are fat [2]; [MPC minimum property-cut](https://consensus.app/papers/details/a542bbf1c49456209c406e1fa8c9a39b/?utm_source=cursor) (Peng et al., 2022, ICDE) minimize *distinct crossing properties* so SPARQL/BGP stays intra-partition [3]; [PowerLyra hybrid-cut](https://consensus.app/papers/details/9c188567ad845efbbb5889d7af7747b7/?utm_source=cursor) (Chen et al., 2019) edge-cut low-degree + vertex-cut high-degree [20]; [Gluon/CVC study](https://consensus.app/papers/details/501ae148b7fe53a486dffbf8ea326586/?utm_source=cursor) (Gill et al., 2018) no single policy wins at every scale; communication *pattern* beats volume [13]; [historical-graph partition](https://consensus.app/papers/details/be6caf65b59557afb1eabfaad6a96ec9/?utm_source=cursor) (Spitalas et al., 2025) time-weighted cuts [14]. Distinct from `paper-geo-raft-wan` (replicate the **log**) and RAGraph (WAN *analytics* schedule).

## 1. Raw idea

Distributed graph engines cut **vertices** (replicate hubs, assign edges) or **edges** (cut edges, assign vertices). Power-law graphs want **hybrid-cut**. Property graphs add a third objective: bytes of properties, not just counts [2]. RDF/MPC: cut few *predicates* so star joins do not hop the network [3]. Streaming cuts (CUTTANA, Fennel, HDRF) are mandatory when the graph does not fit RAM; quality is lower unless you buffer/restream [1]. At 256 machines Cartesian vertex-cut can beat a “better” edge-cut because of sync *shape* [13].

## 2. STCA applicability

Space/Data: partitioning is a **lease layout** of CSR/HNSW/views. The event log stays unsharded-or-log-replicated (Raft cell); do not shard SoT by vertex-cut. Composition: pack placement (legal vs scientific slices) is a *policy* partition, orthogonal to min-cut. Time: historical/weighted cuts [14] are how you keep journeys local — cousin of TVG, not a second log. Local-first GDBMS + RDTs [18] is the CRDT anti-pattern again: availability ≠ PIT.

## 3. Quality / cost

Usefulness high once one machine is not enough. Optimality med: NP-hard; heuristics + restream. Cost: P0 = single node; honeycomb 012/020 = partition *projections* with hybrid-cut + property-volume; never partition-as-SoT.

## 4. Demand

Operators ask “how do we shard Neo4j.” Answer: shard pictures, replicate the log. Engine demand: a partition descriptor on materializations.

## 5. Niche → effect

`no niche`
