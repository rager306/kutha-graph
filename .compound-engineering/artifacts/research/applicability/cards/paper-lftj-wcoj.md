---
id: paper-lftj-wcoj
source: paper
axes: [Query, Data, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Leapfrog Triejoin (WCOJ) on graph patterns

Paper: [Leapfrog Triejoin](https://arxiv.org/abs/1210.0481) (Veldhuizen, 2012/2014). Already noted in `AGENTS.md`. Compact/dynamic variants 2024–2026 (CompactLTJ, Ring, temporal BGP).

## 1. Raw idea

Multiway join whose work tracks AGM output bound (up to log). Tries + seek; no mandatory pairwise intermediates.

## 2. STCA applicability

Data/Query: hot CSR/sparse materializations. Composition: join cost vs cascade budget. Does not change event-log SoT.

## 3. Quality / cost

Strong optimality story for Cypher-like BGPs. Cost: six trie orders classically, or compact/Ring encodings; AS OF + WCOJ is a separate paper (temporal BGP 2026).

## 4. Demand

Multi-hop / cyclic patterns where pairwise joins explode. Needed if Kutha claims Samyama/Falkor-class hops.

## 5. Niche → effect

`no niche` — engine capability; GTM is performance of the hybrid quadrant, not a vertical pack.
