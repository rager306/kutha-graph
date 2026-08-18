---
id: oxify-dag-llm-orchestration
source: oxify
axes: [Agent, Composition, Query]
usefulness: med
optimality: low
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Rust DAG of LLM nodes is still an orchestrator — not Control SoT

User URL: https://github.com/cool-japan/oxify (cloned `/tmp/cool-japan/oxify`, not CBM-indexed). README: graph-based LLM workflows; nodes Start/End/LLM/Retriever/Vision/Code/IfElse/Tool; Qdrant/pgvector; MCP; CeleRS distributed tasks; engine README “Codename: Absolute Zero”. Read: `crates/oxify-engine/src/lib.rs` — `execute` / `execute_sequential`, Kahn topological sort on `Workflow` nodes/edges (`topological_sort`), checkpoints, WASM/Rhai code nodes, vector search. Distinct from `dify-workflow-rag-orchestration` (Python/TS canvas + Postgres datasets), `paper-az-research-plan-dag` (question DAG, not LLM node runtime), `raven-in-db-ai-agents`, `paper-wasm-udf-sandbox` (DB UDF sandbox vs workflow Code node).

## 1. Raw idea

A workflow is a **DAG of typed nodes**. Engine validates acyclicity, Kahn-sorts, runs ready nodes, passes variables, optional CeleRS queue. Retriever nodes hit Qdrant/pgvector (hybrid BM25+RRF per engine README). Code nodes: Rhai or Wasmer. This is Dify’s product shape in Rust, not a property-graph query engine. “Graph-based” here means the *workflow* graph.

## 2. STCA applicability

Agent 050/052: compile a DAG *pack* into dict-first tool calls against Kutha views (compiler-not-executor). Composition: Kahn order is scheduling under V, not a second log. Query: Retriever is a RAG lease (NaviX/SIEVE/Dify family). Do not import CeleRS, Qdrant, or pgvector as SoT. Do not confuse OxiFY branding “Absolute Zero” with AZ projected-schema Cypher.

## 3. Quality / cost

Usefulness med: buyers want a typed DAG runner; Rust is closer to Kutha’s language than Dify. Optimality low for the kernel: no WCOJ, no event-log fold, vector DBs are the Gupta multi-backend trap. Cost: optional adapter pack; borrow nothing as quantum. WASM Code node stays a cousin of the UDF sandbox card.

## 4. Demand

Same as Dify: ship agent glue without a Python canvas. Engine demand: an explicit workflow-compile surface, not an in-process LLM orchestrator as truth.

## 5. Niche → effect

`no niche`
