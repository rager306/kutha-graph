---
id: paper-gradoop-tpgm
source: paper
axes: [Time, Data, Query]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# TPGM bitemporal vertices/edges/collections (GRADOOP)

Paper: [Distributed temporal graph analytics with GRADOOP](https://doi.org/10.1007/s00778-021-00667-4) (Rost et al., 2021). Consensus: https://consensus.app/papers/details/5ecaf179c5b85ecf94bbfbf97ae75e7a/?utm_source=cursor

## 1. Raw idea

Temporal property graph model (TPGM): valid+transaction time not only on vertices/edges but on graph collections. Operators: snapshot, difference, pattern match, grouping. Distributed dataflow, not an OLTP log-SoT.

## 2. STCA applicability

Time: collection-level bitemporal is a pack/materialization shape. Query: analytical workflows (motifs, grouping) from `patterns.md` candidate #12. Must not become Kutha’s write path.

## 3. Quality / cost

Mature VLDB system (to 1.8B edges). Cost: Flink-class cluster vs in-process Rust; analytics pack may starve P0.

## 4. Demand

“What changed between two belief times” at collection scale.

## 5. Niche → effect

`no niche`
