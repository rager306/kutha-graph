---
id: paper-graphflow-columnar-list
source: paper
axes: [Data, Query]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# GraphflowDB: columnar layout + list-based processing for many-to-many joins

Paper: [Columnar Storage and List-based Processing for Graph Database Management Systems](https://doi.org/10.14778/3476249.3476297) (Gupta et al., 2021). Consensus: https://consensus.app/papers/details/1f1ab2f258195f4abdc6e5cc3f1fb376/?utm_source=cursor

## 1. Raw idea

Columnar RDBMS tricks do not copy onto GDBMS access patterns. GraphflowDB: list-based processor (avoid block copies under m-n joins), single-indexed edge property pages + edge IDs, Jacobson bitvectors for NULLs/empty lists. Complements CSR adjacency with columnar *properties*.

## 2. STCA applicability

Data: Samyama/Falkor CSR is adjacency; this is how **properties** sit beside frozen adj without row copies. Query: list-based operators vs block Volcano. Event log stays SoT; columns are a projection layout.

## 3. Quality / cost

High empirical GDBMS paper. Cost: in-memory GraphflowDB; Kutha still needs disk/WAL (Rocks). Do not import a second analytics SoT.

## 4. Demand

Hop queries with fat edge properties: CSR-only still copies property blocks.

## 5. Niche → effect

`no niche`
