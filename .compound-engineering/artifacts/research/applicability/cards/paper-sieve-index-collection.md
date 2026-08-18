---
id: paper-sieve-index-collection
source: paper
axes: [Query, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# SIEVE: many indexes for predicate forms, not constrained HNSW walks

Paper: [SIEVE: Effective Filtered Vector Search with Collection of Indexes](https://doi.org/10.14778/3749646.3749725) (Li et al., 2025). Consensus: https://consensus.app/papers/details/14f4fb69ec0b5a5786df73de23692d8e/?utm_source=cursor

## 1. Raw idea

Constrained HNSW traversal loses the few-hops property under hard predicates. Opposite pole: pack a **collection** of indexes for different predicate forms, using a 3-D model (size, time, recall) that is both the build budget and the query-time picker. Claims ≤2.15× HNSW memory and up to 8× speedup with modest workload knowledge.

## 2. STCA applicability

Query: planner alternative to Compass (no new index), NaviX (prefilter), ACORN (predicate subgraph). Materializations remain leases on the log; SIEVE is *which leases to keep* under a memory budget. Do not make the collection a second SoT.

## 3. Quality / cost

High: names the Compass dual. Cost: workload-aware packing; ad-hoc legal filters may miss the collection → fall back to Compass/scan.

## 4. Demand

Repeated tenant/type/time filters on embeddings. A single HNSW cannot serve every predicate shape.

## 5. Niche → effect

`no niche`
