---
id: crp-spmm-comm-reduced
source: crp-spmm
axes: [Data, Query, Composition]
usefulness: high
optimality: high
demand: med
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Distributed SpMM is a communication schedule — not CSR-in-one-process and not WCOJ

User URL: https://github.com/scalable-matrix/CRP-SpMM (cloned `/tmp/user-url-scout/crp-spmm`, not CBM-indexed). SC23 paper: *Communication-Reduced Sparse-Dense Matrix Multiplication with Adaptive Parallelization* (repro notes in `deprecated/SC23_AD/readme.md`; CombBLAS comparison branch). Read: `src/para2d_spmm.h` / `para2d_spmm.c` — 2D process grid `(pm × pn)`, allgather A on the row communicator, then 1D row-parallel SpMM; `src/rowpara_spmm.h` — local CSR (`A_rowptr`/`A_colidx`/`A_val`) + alltoallv/p2p redistributes of dense B; MPI + OpenMP + optional MKL. Distinct from `falkor-graphblas-sparse-adj` (GraphBLAS *API* in a graph DB, `spec`), `samyama-csr-frozen-adjacency` (single-node hop materialization), `scirs-graph-scientific` (in-process CSR analytics), ULTRA `rspmm` (relational GNN kernel, not MPI). GPU WCOJ stays queued skip.

## 1. Raw idea

`C := A * B` with sparse A (Matrix Market / CSR) and tall dense B. The cost at cluster scale is **replicating A and redistributing B**, not the local FMA. CRP-SpMM chooses a 2D partition so communication volume drops; local SpMM is MKL/OpenMP. This is CombBLAS-class engineering, not a query language.

## 2. STCA applicability

Data 041 / Query: GraphBLAS-like hops and analytics (PageRank, multi-source BFS as SpMV/SpMM) are **leases**. When a lease outgrows one box, the *schedule* of A/B redistributes is this card — not a second SoT. Composition: do not replace leapfrog with SpMM; use SpMM for algebra kernels on a frozen CSR snapshot. Time: partition is of the snapshot, not of the event log.

## 3. Quality / cost

Usefulness high for the GraphBLAS bar at multi-node. Optimality high *for communication-reduced SpMM* (SC23 + real CSR/MPI). Demand med for P0 (single-node CSR first). Cost: P0 = Samyama-style CSR; honeycomb = optional MPI SpMM worker for analytics leases; do not take Intel-MPI/MKL as a Kutha runtime quantum.

## 4. Demand

Citation graphs and scientific archives will ask for multi-source SpMV at cluster size. Engine demand: snapshot → CSR → SpMM worker → write scores back as events.

## 5. Niche → effect

`no niche`
