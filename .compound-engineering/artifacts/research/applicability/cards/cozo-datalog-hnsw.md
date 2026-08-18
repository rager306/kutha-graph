---
id: cozo-datalog-hnsw
source: cozo
axes: [Query, Data]
usefulness: high
optimality: med
demand: med
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# In-process Datalog + native HNSW (Cozo)

Evidence: CBM `cozo` — `cozo-core/src/runtime/hnsw.rs` (`HnswIndexManifest`, `hnsw_put`/`hnsw_remove`); `DbInstance.register_fixed_rule`; FTS/LSH/HNSW normalize in `data/program.rs`. Rocks backend `cozorocks`.

## 1. Raw idea

Datalog engine with relations, fixed rules, and HNSW/FTS as first-class indexes on the same store — not a sidecar vector DB.

## 2. STCA applicability

Query: Datalog as an optional pack language (GQL Rules / FlowLog). Data: Helix-style access paths in Rust. Must not replace event-log SoT; Cozo relations are a materialization.

## 3. Quality / cost

Real kernel (unlike Raven). Cost: Cozo script vs Cypher vs Kutha dicts; two hop engines if also Samyama leapfrog.

## 4. Demand

Recursive rules + vector filter in one transaction.

## 5. Niche → effect

`no niche`
