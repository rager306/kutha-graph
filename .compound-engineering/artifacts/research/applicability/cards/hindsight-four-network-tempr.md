---
id: hindsight-four-network-tempr
source: hindsight
axes: [Agent, Time, Query, Verify]
usefulness: high
optimality: low
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Four epistemic memory networks + 4-arm recall — not a graph SoT

User URL: https://github.com/vectorize-io/hindsight (cloned `/tmp/hindsight`, not CBM-indexed). Paper: [Hindsight is 20/20](https://arxiv.org/html/2512.12818) (Boschi, Neeser, Bartholomew, Srivastava, Wang, Ramakrishnan; Vectorize + VT + WaPo). Read: `engine/retain/fact_extraction.py`, `engine/search/retrieval.py`, `engine/search/fusion.py` (RRF k=60 over semantic/BM25/graph/temporal), `engine/search/graph_retrieval.py`, `engine/retain/link_creation.py`, `engine/retain/fold.py`, `engine/mental_model_refresh.py`. Distinct from `graphiti-bitemporal-fact-edges` (LLM extract into Neo4j), `paper-engram-bitemporal-memory` (LLM off write), `agentdb-hierarchical-memory`, `claim-yotg-context-graph-layers`, `ruvector-hybrid-bm25-dense` (two arms, no graph/temporal).

## 1. Raw idea

Agent memory as **four logical networks**: world (objective), experience (first-person), opinion (belief + confidence), observation (synthesized entity summaries). Ops: **retain / recall / reflect** (TEMPR + CARA). Retain: LLM extracts narrative facts with occurrence interval (τs, τe) and mention time τm, resolves entities, builds memory-to-memory links (temporal / semantic ANN / causal) in Postgres (`memory_units`, `unit_entities`, `memory_links`). Recall: four parallel arms, per-arm cap, RRF merge, cross-encoder rerank, token budget. Reflect: disposition (skepticism/literalism/empathy) shapes generation and mutates opinions. Code `fold.py` coalesces queued *retain jobs* (append-only, claim-time) — **not** Kutha’s graph fold. Extract schema in code is world/experience(/assistant); observations are synthesized; recall may drop `opinion` from fact_type lists.

## 2. STCA applicability

Agent: this is a **memory pack**, Graphiti-class. Time: occurrence vs mention is a useful *stamp split*, but Postgres rows are not an event-log SoT; invalidation is LLM/consolidation, not TOKI. Query: 4-arm hybrid is demand for Query 071 — graph+temporal as *retrieval arms*, not WCOJ. Verify: paper’s epistemic split (evidence vs belief) is the right *vocabulary*; opinions with confidence must not become L_KB facts. LLM on retain is the compiler-not-executor anti-pattern unless receipts. Do not import CARA dispositions into Control.

## 3. Quality / cost

Usefulness high: LongMemEval/LoCoMo scores are why buyers ask “where is agent memory?” Optimality low for Kutha kernel: LLM extract, pgvector/HNSW in PG, no journey operators, no receipts. Cost: keep 4-network *labels* and 4-arm recall as pack UX over the log fold; reject PG/Oracle as SoT; do not confuse retain-fold with STCA fold.

## 4. Demand

Fortune-500 memory sidecar. Engine demand: hybrid retrieve + typed fact/opinion split. Product: optional intelligence pack, never the quantum.

## 5. Niche → effect

`no niche`
