---
id: paper-acorn-predicate-subgraph
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

# Predicate-subgraph traversal on HNSW (ACORN)

Paper: [ACORN](https://doi.org/10.1145/3654923) (Patel et al., 2024). Consensus: https://consensus.app/papers/details/7b37ab7d06665ec3b1dad19fc3ac9ca0/?utm_source=cursor

## 1. Raw idea

Predicate-agnostic hybrid search by traversing the *predicate subgraph* of HNSW (emulating an impractical ideal strategy) instead of restricting predicates to small equality sets. Construction is designed so existing HNSW libraries can be extended.

## 2. STCA applicability

Query/Data: another native-index story, complementary to NaviX prefiltering. ACORN = walk a constrained HNSW; NaviX = evaluate S first then kNN. Kutha can treat both as planner alternatives on the same graph-shaped index, not a sidecar vector DB.

## 3. Quality / cost

High empirical claim (throughput at fixed recall vs restricted-predicate methods). Cost: still ANN; construction/search heuristics; does not replace relational selectivity crossover from DaMoN 2024.

## 4. Demand

Ad-hoc filters (range, multi-attribute, time windows) on embeddings — the legal/science default, not “vector-only kNN.”

## 5. Niche → effect

`no niche`
