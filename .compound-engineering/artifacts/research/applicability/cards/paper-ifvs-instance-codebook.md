---
id: paper-ifvs-instance-codebook
source: paper
axes: [Query, Data]
usefulness: med
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# iFVS: instance-optimized codebooks for filtered vector search

Paper: [iFVS: Towards Instance-Optimized Filtered Vector Search](https://arxiv.org/abs/2607.22922) (Al-Mamun et al., 2026). Consensus: https://consensus.app/papers/details/860955e09559521f9ebcb47e6de10335/?utm_source=cursor

## 1. Raw idea

Product quantization with a *fixed* codebook hurts FVS because the relevant subspace changes with the predicate. iFVS keeps compact per-vector storage but conditions distance estimation on the query vector **and** the filter predicate (query-specific codebook / instance-optimized toward a workload).

## 2. STCA applicability

Query/Data: another access-path knob beside Compass/SIEVE/NaviX/ACORN/DaMoN scan-vs-probe. Quantization is a materialization of embeddings, not SoT. Legal as-of filters change the active set — a global PQ codebook is the stale-fact analog for distances.

## 3. Quality / cost

High conceptual fit; 0 citations, arXiv 2026. Cost: needs a representative workload; ad-hoc filters may miss the instance.

## 4. Demand

Hybrid retrieve at scale will compress vectors. Wrong codebook under filters silently drops recall.

## 5. Niche → effect

`no niche`
