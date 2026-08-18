---
id: paper-compact-ltj
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

# CompactLTJ: WCOJ without six full tries, with updates

Paper: [CompactLTJ](https://doi.org/10.1007/s00778-025-00945-5) (Arroyuelo et al., 2025). Consensus: https://consensus.app/papers/details/d873fc420a3c5c7e98061f87f7fa941c/?utm_source=cursor

## 1. Raw idea

Classic LTJ needs six SPO permutations. Compact tries (one bit per edge) + partial tries: 5–6× less space than classic WCOJ indexes, still WCO time; dynamism under demanding updates without collapsing performance.

## 2. STCA applicability

Query: practical LTJ for a log-fold that *mutates*. Data: compact hot index. Complements Samyama leapfrog (sorted CSR) and Ring.

## 3. Quality / cost

Highest “can we afford six tries?” answer. Cost: compact-structure complexity vs hash-WCOJ-at-query-time (Freitag).

## 4. Demand

Event-log appends make a read-only six-trie story fail.

## 5. Niche → effect

`no niche`
