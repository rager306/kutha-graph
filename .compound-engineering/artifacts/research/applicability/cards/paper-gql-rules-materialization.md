---
id: paper-gql-rules-materialization
source: paper
axes: [Data, Query, Composition]
usefulness: high
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Deterministic GQL/Cypher rules as derived-graph materialization

Paper: [GQL Rules](https://doi.org/10.1109/access.2026.3686122) (Tomaszuk, 2026). Consensus: https://consensus.app/papers/details/97c9414dd887563ba5dac879ba10fb02/?utm_source=cursor

## 1. Raw idea

MATCH/WHERE body + inflationary MERGE/ENRICH head; stratified NOT EXISTS; fixpoint with provenance and per-rule policies. Derived edges persist, not one-shot query answers.

## 2. STCA applicability

Data: derived views as reversible packs (DBSP cousin, Cypher-shaped). Composition: cascade of rules with budgets. Must be a fold of the log, not a second SoT.

## 3. Quality / cost

Useful contract; PoC is Neo4j/APOC. Cost: naive fixpoint vs semi-naive; static finite graphs vs live event log.

## 4. Demand

Alerts/defaults/derived relations reused across queries — Kutha behaviors without ad-hoc scripts.

## 5. Niche → effect

`no niche`
