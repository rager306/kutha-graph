---
id: ruvector-gnn-facade
source: ruvector
axes: [Query, Agent]
usefulness: low
optimality: low
demand: med
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# GNN classify/link-predict facade with dummy scores

CBM `root-vendor-source-ruvector` (coverage: `crates/ruvector-graph/src/hybrid/graph_neural.rs`, no_recorded_issue). `classify_node` returns `vec![0.7, 0.2, 0.1]` and confidence 0.7; `predict_link` score 0.85; `load_model` is `Ok(())` placeholder. Adjacent: `ruvector-gnn` has real layer/attention primitives; MCP `tool_gnn_forward` runs a cached layer — do not confuse primitives with this hybrid facade.

## 1. Raw idea

A graph-hybrid GNN API that always “succeeds” with fixed class probabilities instead of failing closed.

## 2. STCA applicability

Query: same anti-pattern as Cypher empty-success. Agent: GNN rerank is optional *on a projection*, never SoT. Rost/STCA: analyses beyond GNN forecast stay engine APIs; this facade is marketing.

## 3. Quality / cost

Keep as idea (`code` on a facade). Cost: treating dummy scores as readiness. Primitives in `ruvector-gnn` may be borrowable later; this card is the facade.

## 4. Demand

Users will ask for “GNN on the graph.” Dummy success is worse than no API.

## 5. Niche → effect

`no niche`
