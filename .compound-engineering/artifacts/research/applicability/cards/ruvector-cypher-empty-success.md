---
id: ruvector-cypher-empty-success
source: ruvector
axes: [Query]
usefulness: low
optimality: low
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Cypher QueryExecutor empty-success facade

Evidence: `crates/ruvector-graph/src/executor/mod.rs` `QueryExecutor::execute_sequential` returns `Ok(Vec::new())` with comment “placeholder”. Public execute still caches that empty result. AE2: keep as idea, do not drop.

## 1. Raw idea

A graph query executor API that reports success with no rows instead of failing closed.

## 2. STCA applicability

Query: anti-pattern for Kutha. Dict-first / fail-closed wants unsupported → error. The *idea* (physical operator pipeline) is still a corpus row.

## 3. Quality / cost

Low quality as borrow. High cost if treated as Cypher readiness. Operators exist beside the placeholder sequential path — do not confuse marketing with kernel.

## 4. Demand

Users will ask for Cypher; empty success is worse than no API.

## 5. Niche → effect

`no niche`
