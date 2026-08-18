---
id: scirs-graph-scientific
source: scirs
axes: [Query, Data, Composition]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# NetworkX-class algorithms are an analytics lease — not the hop SoT

User URL: https://github.com/cool-japan/scirs (`scirs2-graph`, cloned `/tmp/cool-japan/scirs`, not CBM-indexed). Workspace v0.6.5; README claims 3.1M SLoC / 29 crates — **do not vendor**. Read: `scirs2-graph/README.md`; `src/lib.rs` (algorithms, measures, spectral, generators, io); `src/compressed.rs` — `CsrGraph` (`row_ptr`, `col_indices`, `values`) for neighbor iteration and SpMV-style PageRank; `src/algorithms/hypergraph.rs` — cuts, minimal transversals/hitting sets on `Hypergraph`. Distinct from `samyama-csr-frozen-adjacency` (hot hop materialization), `falkor-graphblas-sparse-adj` (query as GraphBLAS in a graph DB), `paper-k2tree-succinct-graph` (succinct lease), `paper-hypergraph-higher-order` (n-ary *fact* model), `paper-subgraph-iso-vs-homomorphism` (MATCH semantics; VF2 here is an algorithm crate), `ruvector-gnn-facade` (GNN marketing/facade).

## 1. Raw idea

A scientific graph library: adjacency + CSR, BFS/Dijkstra/Bellman-Ford, centrality, Louvain, spectral clustering, VF2, flows, generators, optional wgpu BFS/SSSP (workspace README). Hypergraph module computes cuts/transversals on an in-memory `Hypergraph`, not on event-log n-ary facts. GNN/embedding modules exist in the README surface — treat as facades until a separate spike; 0.6.5 notes that spectral clustering and Hungarian were recently *un-stubbed*.

## 2. STCA applicability

Query 070/Data 041: shortest-path / centrality / community are **leases over the fold**, same CSR family as Samyama but *analytics* not conjunctive MATCH. Composition: do not replace leapfrog with NetworkX. Hypergraph algorithms consume the n-ary model card; they do not define it. GNN stays optional Agent/Query pack (`ruvector-gnn-facade`). GPU paths are queued skip (not P0).

## 3. Quality / cost

Usefulness high: legal citation graphs and scientific archives will ask for PageRank/betweenness/community. Optimality med: CSR is the right layout; this crate is a SciPy port, not AGM-optimal joins. Cost: call `scirs2-graph` (or a thin subset: CSR + Dijkstra + VF2) on a projected snapshot; never make SciRS the runtime quantum. Ignore the 28 non-graph crates.

## 4. Demand

Analytics users will not wait for Cypher `*` to grow Louvain. Engine demand: snapshot → CSR → algorithm → write results back as events (not mutate SoT in-place).

## 5. Niche → effect

`no niche`
