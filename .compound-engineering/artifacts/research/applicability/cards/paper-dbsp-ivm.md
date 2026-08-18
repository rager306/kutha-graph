---
id: paper-dbsp-ivm
source: paper
axes: [Data, Query, Time]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# DBSP incremental view maintenance for rich queries

Paper: [DBSP](https://doi.org/10.14778/3587136.3587137) (Budiu et al., 2022). Consensus: https://consensus.app/papers/details/f5f3656f43365d99a40ecde06ef970b2/?utm_source=cursor

## 1. Raw idea

IVM as a stream calculus: SQL and Datalog both compile to DBSP; incrementalize arbitrary compositions (nested relations, aggregation, recursion) without per-language heuristics.

## 2. STCA applicability

Data: CSR/HNSW/temporal views as incremental folds of the event log, not periodic rebuild. Time: log deltas → view deltas. Query: keeps WCOJ/Cypher plans cheap after appends.

## 3. Quality / cost

High: this is the missing physics behind “reversible materializations.” Cost: DBSP runtime vs hand-rolled compact; not a second SoT.

## 4. Demand

Without IVM, every AS OF / hop view becomes a full rebuild — the P0 failure mode.

## 5. Niche → effect

`no niche`
