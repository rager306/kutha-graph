---
id: samyama-leapfrog-triejoin
source: samyama
axes: [Query, Data]
usefulness: high
optimality: high
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Leapfrog Triejoin on sorted CSR adjacency

Evidence: `src/query/executor/leapfrog.rs` — seekable `AdjacencyIterator`, WCOJ for triangles/cliques; requires sorted CSR frozen neighbors.

## 1. Raw idea

Multiway intersection of sorted adjacency instead of pairwise join intermediates.

## 2. STCA applicability

Query: same family as paper-lftj-wcoj, but here it is an engine, not a paper. Composition: join work vs cascade budget.

## 3. Quality / cost

High optimality on cyclic patterns. Cost: needs sorted CSR; bi-temporal AS OF on top is not this file.

## 4. Demand

Cypher cycles / motifs where pairwise joins explode.

## 5. Niche → effect

`no niche`
