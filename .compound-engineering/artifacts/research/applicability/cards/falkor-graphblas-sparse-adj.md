---
id: falkor-graphblas-sparse-adj
source: falkor
axes: [Data, Query]
usefulness: high
optimality: high
demand: high
confidence: spec
layer5: no niche
status: closed
channels_failed: []
---

# GraphBLAS sparse adjacency (query as matrix ops)

Evidence: https://docs.falkordb.com/design — GraphBLAS API; CSC sparse format. GitHub FalkorDB: sparse matrix adjacency. No kernel in vendor-source → `spec`.

## 1. Raw idea

Graph = sparse matrix; hops and some analytics are linear-algebra kernels, not pointer chasing.

## 2. STCA applicability

Data: GraphBLAS-like hot path (ADR-000 / ADR-041). Query: multi-hop cost model. Not a temporal SoT.

## 3. Quality / cost

High optimality for labeled traversal. Cost: SuiteSparse/C interop vs pure Rust CSR; agents/temporal are weak in this product class.

## 4. Demand

The performance bar Kutha said it must approach, not always beat.

## 5. Niche → effect

`no niche`
