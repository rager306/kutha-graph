---
id: paper-spectral-sparsification
source: paper
axes: [Data, Query, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Keep the vertices, drop edges by effective resistance — not supernodes and not k²

Papers: [Spielman & Srivastava](https://consensus.app/papers/details/3f7db731ea01554b88e5240b65629eab/?utm_source=cursor) (2011/2008, SICOMP) every graph has a spectral sparsifier of nearly linear size; Laplacian quadratic form approximates the original; nearly-linear construction [20]; [CACM survey](https://consensus.app/papers/details/e9d8f1f026ce50f28a79bb65a1ef436a/?utm_source=cursor) (Batson, Spielman, Srivastava, Teng, 2013) spectral ≈ cut structure; key tool for SDD Laplacian solvers and approx max-flow/min-cut [4]; sample edges ∝ effective resistance [20][2]; coarsening with restricted spectral/cut guarantees is a *different* reduction (merge vertices) [1] — keep it as contrast, not this card’s noun; directed/Eulerian spectral sparsifiers with polylog update [3]; hypergraph spectral sparsifiers stay queued. Distinct from `paper-graph-summarization-quotient` (supernodes + corrections; *n* shrinks), `paper-k2tree-succinct-graph` (encode *all* remaining adjacency), `paper-horae-temporal-sketches` (stream sketches), `paper-semi-external-graph` (placement of full edges). Quantum sparsification stays GPU-queued.

## 1. Raw idea

Same vertex set, far fewer weighted edges, cuts and Laplacian spectrum stay within (1±ε) [20][4]. Analytics (PageRank-like quadratic forms, cuts) run on the sparse lease. The log still has every edge.

## 2. STCA applicability

Data: sparsifier is a reversible materialization (040) of the *fold’s undirected skeleton*, not SoT. Query: planner may route cut/spectral workloads to the sparsifier and MATCH to CSR. Composition: ε and edge budget are Cui-shaped. Time: rebuild after the log moves (or maintain fully-dynamic directed sparsifiers [3]); do not treat sampled weights as valid-time. Verify: (1±ε) is a numeric receipt of approximation, not how-provenance. LLM does not sample edges.

## 3. Quality / cost

Usefulness high: cut/Laplacian work on huge folds. Optimality high: Spielman–Srivastava is the standard. Cost: P0 = full CSR; honeycomb = resistance-sampled sparsifier lease for analytics packs. Do not answer exact MATCH on the sparsifier. Do not confuse with summarization’s supernodes.

## 4. Demand

“Approximate the cut / solve the Laplacian” should not scan every log edge. Engine demand: an optional spectral-sparsifier lease, fail-closed to exact CSR.

## 5. Niche → effect

`no niche`
