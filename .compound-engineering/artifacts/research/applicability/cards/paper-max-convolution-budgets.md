---
id: paper-max-convolution-budgets
source: paper
axes: [Composition, Query, Time]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Compose module budgets with (max,+) convolution — not per-query admission and not CNN

Papers: knapsack ≡ **(max,+)/(min,+) convolution** of two sequences; combining packings is the convolution [1][3]; Bateni et al. (STOC 2018) near-linear knapsack when sizes/values are small integers, vs classic `O(nt)` DP and the quadratic convolution barrier vs FFT `(+,·)` [3]; Bringmann et al. (2024) `Õ(n + t√p_max)` via rectangular monotone min-plus convolution [2]; higher-dimensional knapsack reduces to 1-D concave convolution [1]; Serang (2015) numerical **max-convolution on the tropical semiring** (`O(k log k)` estimate) for max-product inference [4]; Brysiewicz et al. tropical-geometry view and quasi-linear integer max-convolution [8]. Distinct from `paper-query-admission-control` (admit/reject *this* query vs remaining V), `paper-ring-wcoj` ([Ring compact join index](https://doi.org/10.1145/3644824), not an algebraic ring or semiring), `paper-dbsp-ivm` (stream deltas). CNN “convolution” hits discarded.

## 1. Raw idea

STCA (Cui): each pack/materialization is a **generalized knapsack item** `h_i(v)` — value at local budget `v`. The global feasible envelope is the **max-convolution** of those functions: `(f ⊕ g)(t) = max_i f(i)+g(t−i)`. That is exactly tropical `(max,+)` / infimal convolution [4][8]. Combining two modules is combining two knapsacks [1][3]. Hard in general (quadratic barrier); tractable when sequences are concave/monotone or values are small [1][2][3].

## 2. STCA applicability

Composition 030 / Time 014: cascade budget V is this envelope, not a YAML quota. Query: cardinality (closed) estimates *cost of one query*; convolution allocates *across concurrent packs* (CSR rebuild vs HNSW vs agent). Admission (closed) is a *gate* on one envelope; convolution *builds* the envelope. Do not let an LLM pick `v`.

## 3. Quality / cost

Usefulness high: honeycomb 030 is empty without the operator. Optimality high: the equivalence to convolution is the theory; algorithms exist for structured cases. Cost: P0 = independent caps; honeycomb = discrete `h_i` tables + `(max,+)` combine (or concave special case); skip avocado-CNN and educational GA knapsacks.

## 4. Demand

CSR, HNSW, and an AS-OF agent will contend for the same CPU/RAM quantum. Engine demand: compose `h_i` then admit (previous card).

## 5. Niche → effect

`no niche`
