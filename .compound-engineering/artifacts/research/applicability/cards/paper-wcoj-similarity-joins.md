---
id: paper-wcoj-similarity-joins
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

# WCOJ + kNN in one LTJ: similarity clauses are not a post-filter

Paper: [Worst-Case-Optimal Similarity Joins on Graph Databases](https://doi.org/10.1145/3639294) (Arroyuelo et al., 2024). Consensus: https://consensus.app/papers/details/01516aa007d05cefaf6435e6b9c0f8c1/?utm_source=cursor

## 1. Raw idea

Some BGP nodes must sit in the kNN of others under a similarity function. Superimpose the data graph with a kNN graph; extend Ring-backed Leapfrog TrieJoin so similarity clauses participate in the *same* multiway join, remaining WCO-optimal in many cases. Beats “LTJ then apply kNN” especially when similarity is densely connected to the pattern.

## 2. STCA applicability

Query/Data: the missing glue between Samyama leapfrog/CSR hops and ruvector/NaviX HNSW. Hybrid Cypher+vector should not be two engines glued by a post-filter. kNN graph is a materialization/lease, not SoT.

## 3. Quality / cost

Highest “one algebra” result this wave. Cost: Ring+kNN assumes a compact in-memory index; Kutha still has log SoT and ANN not exact kNN. Complements Compass (cooperative existing indexes) with a *join-theoretic* integration.

## 4. Demand

“Find drugs similar to X that participate in this pathway as of T” is one query, not retrieve-then-join.

## 5. Niche → effect

`no niche`
