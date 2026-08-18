---
id: paper-navix-filtered-hnsw
source: paper
axes: [Query, Data]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Native HNSW in a graph DBMS with predicate-agnostic kNN

Paper: [NaviX](https://doi.org/10.14778/3749646.3749704) (Sehgal et al., 2025). Consensus: https://consensus.app/papers/details/9ce19a5e4813541d8d48c6db585d99af/?utm_source=cursor

## 1. Raw idea

HNSW as a graph stored by the GDBMS, not a sidecar. Prefilter an arbitrary subset S (ad-hoc subquery), then kNN that is robust to selectivity and correlation via adaptive heuristics.

## 2. STCA applicability

Data/Query: Helix “index = access path” with evidence. Hybrid Cypher+vector without a second SoT. Complements ruvector-hnsw / samyama HNSW.

## 3. Quality / cost

High: disk-based, uses host storage/query. Cost: adaptive search complexity; still ANN not exact.

## 4. Demand

Legal/science queries are never “vector only”: always filter on type, time, tenant.

## 5. Niche → effect

`no niche`
