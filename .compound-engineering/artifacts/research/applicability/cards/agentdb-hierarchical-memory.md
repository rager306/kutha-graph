---
id: agentdb-hierarchical-memory
source: agentdb
axes: [Agent, Data]
usefulness: med
optimality: low
demand: med
confidence: spec
layer5: no niche
status: closed
channels_failed: []
---

# Hierarchical agent memory + controller registry

Evidence: `vendor-source/ruflo/plugins/ruflo-agentdb/README.md` — 15 `agentdb_*` tools, init levels, `graphAdapter` disabled pending external graph DB. No AgentDB kernel tree in Wave 1.

## 1. Raw idea

Typed memory controllers (hierarchical recall, causal edges, sessions) over SQLite/HNSW, not a graph SoT.

## 2. STCA applicability

Agent: optional intelligence pack behind dictionaries. Data: must not replace event log. Disabled `graphAdapter` is a warning: memory product assumes an external graph.

## 3. Quality / cost

Spec-only here. Cost: swallowing AgentDB as “the brain” fights D3 dict-first.

## 4. Demand

Harnesses want persistent agent memory; Kutha still needs an engine, not a sidecar.

## 5. Niche → effect

`no niche`
