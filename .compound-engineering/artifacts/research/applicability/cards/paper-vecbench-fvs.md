---
id: paper-vecbench-fvs
source: paper
axes: [Query, Verify]
usefulness: med
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# VecBench: controllable filtered-vector-search benchmark

Paper: [VecBench](https://doi.org/10.1145/3802125) (Zhang et al., 2026). Consensus: https://consensus.app/papers/details/ee551f1a485054e6a76982aeaaa40f71/?utm_source=cursor

## 1. Raw idea

Existing FVS benches are too small/low-dim, use random filters (trivial plans), and lack an end-to-end metric. VecBench: generate high-dim data with bounded distribution error; control selectivity and filter–vector correlation; six-phase eval including concurrency and dynamic updates. Ten methods × four vector DBs.

## 2. STCA applicability

Verify: how Kutha would *falsify* hybrid retrieve claims. Query: stress-tests the Compass/SIEVE/NaviX/ACORN/iFVS poles under correlation, not only selectivity. Not a kernel.

## 3. Quality / cost

Demand is the card. Cost: VecBench is a vector-DB bench, not a graph+log+AS-OF bench — Kutha still needs a temporal cut.

## 4. Demand

Without controllable correlation, “our HNSW is fast” is unfalsifiable for legal/tenant filters.

## 5. Niche → effect

`no niche`
