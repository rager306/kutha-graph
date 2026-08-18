---
id: paper-fvs-postgres-system-costs
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

# Filter-agnostic FVS inside a real DBMS: I/O beats library theory

Paper: [An In-Depth Study of Filter-Agnostic Vector Search on a PostgreSQL Database System](https://doi.org/10.1145/3802011) (Lu et al., 2026). Consensus: https://consensus.app/papers/details/f170ae6f66d550b9b78229cfcbeca833/?utm_source=cursor

## 1. Raw idea

FVS algorithms measured in ANN libraries make assumptions that fail in a production PostgreSQL-compatible engine. Optimal choice is not distance-compute cost alone: page accesses and filter checks dominate. Graph methods (NaviX/ACORN-class) can issue prohibitive filter checks vs clustering indexes (ScaNN); theoretical library wins cancel.

## 2. STCA applicability

Query: planner must cost **engine I/O**, not copy GitHub ANN numbers. Complements DaMoN scan-vs-probe (selectivity crossover) with *system-level* overheads. HNSW remains an access path on a projection; embedding it into Kutha’s executor can invert NaviX/ACORN rankings.

## 3. Quality / cost

Highest “don’t ship the paper algorithm blindly” result this wave. Cost: Postgres-shaped engine ≠ in-process Rust core — still a warning, not a port.

## 4. Demand

Hybrid Cypher+vector will look slow for the wrong reason (filter checks), then get “fixed” by a sidecar vector DB.

## 5. Niche → effect

`no niche`
