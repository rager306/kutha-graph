---
id: ruvector-hnsw
source: ruvector
axes: [Data, Query]
usefulness: high
optimality: high
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# HNSW as a reversible access method (not SoT)

Evidence: `crates/ruvector-core/src/index/hnsw.rs` — `HnswIndex::new/add/remove` wraps a real HNSW (`m`, `ef_construction`, DashMap id maps). CBM project `root-vendor-source-ruvector`.

## 1. Raw idea

Hierarchical NSW ANN over dense vectors; insert/search/remove in-process.

## 2. STCA applicability

Data: hot HNSW view behind a port (ADR-042). Query: hybrid retrieve. Must be a lease/materialization, not event-log SoT.

## 3. Quality / cost

High: actual index, not a facade. Cost: fence vs Samyama HNSW so Kutha does not vendor two ANN cores.

## 4. Demand

Any hybrid graph+vector path; without it Kutha cannot claim Samyama/Falkor-class recall.

## 5. Niche → effect

`no niche`
