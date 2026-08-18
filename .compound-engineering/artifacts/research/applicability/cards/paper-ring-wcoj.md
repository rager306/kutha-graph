---
id: paper-ring-wcoj
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

# Ring: WCOJ on triples in almost no extra space

Paper: [The Ring](https://doi.org/10.1145/3644824) (Arroyuelo et al., 2024). Consensus: https://consensus.app/papers/details/a7af727519325dca9850d190f3ecdf29/?utm_source=cursor

## 1. Raw idea

Index RDF/property triples as cyclic strings; wavelet columns navigate all join orders. One ring replaces six B-tree orders; space ≈ the graph plus sublinear extras.

## 2. STCA applicability

Query: compact cousin of LFTJ (already in Samyama leapfrog). Data: hot index as materialization. Complements paper-temporal-wcoj-bgp for time-labeled edges.

## 3. Quality / cost

High space optimality. Cost: wavelet trees vs CSR seek; updates/dynamism weaker than hash-WCOJ-at-query-time.

## 4. Demand

In-memory multi-hop when six permutation indexes will not fit.

## 5. Niche → effect

`no niche`
