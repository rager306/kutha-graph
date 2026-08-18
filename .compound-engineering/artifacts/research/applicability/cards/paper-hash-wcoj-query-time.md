---
id: paper-hash-wcoj-query-time
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

# Hash WCOJ built at query time (HTAP, no six tries)

Paper: [Adopting WCOJ in RDBMS](https://doi.org/10.14778/3407790.3407797) (Freitag et al., 2020). Consensus: https://consensus.app/papers/details/ae3dee17affd5d1f858dc5e28e5a2ffc/?utm_source=cursor

## 1. Raw idea

Hash-based multiway join using structures built during execution, not precomputed ordered tries. Hybrid optimizer mixes binary and WCOJ. Aimed at HTAP, not read-only graphs.

## 2. STCA applicability

Query: log-fold mutates — query-time hash WCOJ fits better than six static tries. Complements CompactLTJ/Ring (space) and Free Join (plan unification).

## 3. Quality / cost

High for P0–P1 when indexes cannot be frozen. Cost: build overhead per query vs CompactLTJ under updates.

## 4. Demand

AS OF + hops on a live log cannot wait for permutation rebuilds.

## 5. Niche → effect

`no niche`
