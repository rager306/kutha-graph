---
id: paper-mixed-vector-relational-access
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

# Mixed vector-relational access paths (scan vs probe)

Paper: [Efficient Data Access Paths for Mixed Vector-Relational Search](https://doi.org/10.1145/3662010.3663448) (Sanca et al., DaMoN 2024). Consensus: https://consensus.app/papers/details/f1d80c6d8b725625bef3d49720edcf8b/?utm_source=cursor

## 1. Raw idea

Hybrid queries are selective on *relational* predicates as well as vectors. Exhaustive scan vs ANN probe is not the relational default: at low relational selectivity, scan (with batching / dense-vector formulation) can beat the index; the crossover depends on dimensionality, HNSW parameters, and concurrent queries.

## 2. STCA applicability

Query: HNSW is an access path, not “always probe.” Complements `paper-navix-filtered-hnsw` (how to kNN a prefiltered subset) and `helix-indexes-as-access-paths` / `ruvector-hnsw`. Event log stays SoT; the planner chooses scan vs probe on a projection.

## 3. Quality / cost

High: falsifies “always HNSW.” Cost: need a selectivity/cost model; naive always-ANN will mis-plan legal AS-OF + type filters.

## 4. Demand

Every Cypher+vector path with tenant/time/type predicates. Without this, hybrid retrieve is a heuristic, not a plan.

## 5. Niche → effect

`no niche`
