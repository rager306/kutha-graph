---
id: paper-duckdb-adaptive-factorization
source: paper
axes: [Query, Data]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# DuckDB: defer factorization / WCOJ to runtime via linear-chained hashes

Paper: [Adaptive Factorization Using Linear-Chained Hash Tables](https://consensus.app/papers/details/387a6ee1730c53f9a65e5e917e32f6ff/?utm_source=cursor) (Gross et al., 2025).

## 1. Raw idea

Factorized aggregation and WCOJ in DuckDB, but **only when they help**. Collision-free linear-chained hash tables make factorized/WCOJ processing cheap. Decision moves from optimize-time to runtime: heuristics and on-the-fly sketches during hash-join build, even when inputs are subqueries or Parquet (no stats). Heuristics nearly match ML models and stay explainable.

## 2. STCA applicability

Query: “WCOJ is a runtime access-path choice,” same family as hash-WCOJ-at-query-time and DaMoN scan-vs-probe. Distinct from Free Join (compile-time unification) and Kuzu (always-on factorized GDBMS). Kutha projections often lack stats (fresh log cut) — sketches during build are the honest path.

## 3. Quality / cost

High practical fit; 4 citations, 2025. Cost: DuckDB is not a graph kernel. Borrow the *deferral*, not the product.

## 4. Demand

Always-on LTJ on acyclic legal patterns is a tax. Runtime “only if cyclic/skew” is the demand.

## 5. Niche → effect

`no niche`
