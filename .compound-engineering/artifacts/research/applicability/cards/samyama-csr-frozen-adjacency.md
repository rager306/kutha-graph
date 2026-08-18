---
id: samyama-csr-frozen-adjacency
source: samyama
axes: [Data, Query]
usefulness: high
optimality: high
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# CSR frozen adjacency + write-buffer compact

Evidence: `/root/samyama-graph/src/graph/store.rs` — `FrozenAdjacency` / multi-segment CSR, compact write buffer into immutable tier (DS-07). CBM `root-samyama-graph`.

## 1. Raw idea

Hot graph as compressed sparse row segments plus a mutable write buffer; compact is a materialization step.

## 2. STCA applicability

Data: exactly D5 hot CSR as reversible projection of the log (Kutha log still SoT; Samyama store is the pattern to copy, not the SoT).

## 3. Quality / cost

High: real packed adjacency, LDBC-oriented. Cost: incremental compact vs rebuild; lease invalidation on log append.

## 4. Demand

Multi-hop without pointer-chasing property-graph maps.

## 5. Niche → effect

`no niche`
