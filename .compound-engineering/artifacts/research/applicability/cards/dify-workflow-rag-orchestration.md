---
id: dify-workflow-rag-orchestration
source: dify
axes: [Agent, Query, Composition]
usefulness: med
optimality: low
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Dify: visual agentic workflows + RAG knowledge as extra “truth”

Repo: [langgenius/dify](https://github.com/langgenius/dify). README + docs.dify.ai; GitHub raw `api/models/dataset.py`, `api/core/rag/datasource/vdb/vector_factory.py`, `vector_type.py` (not CBM-indexed). Dify Open Source License (Apache-2.0 plus conditions).

## 1. Raw idea

Open-source **LLM app platform**: visual Workflow/Chatflow canvas, Prompt IDE, model hub, 50+ agent tools, LLMOps logs, BaaS APIs. **Knowledge** is a RAG pipeline: ingest documents → chunk/embed → retrieve into the LLM context. Official knowledge guide states RAG uses custom knowledge as an **additional source of truth**. Retrieval node: query text/images → one or more knowledge bases → rerank (weighted semantic vs keyword, optional rerank model) → `result` array of chunks for downstream LLM; citations optional. Agent node: packaged ReAct / function-calling loop with tools (Google Search, etc.) after a Knowledge Retrieval node. `Dataset` SQLAlchemy model (`datasets` table): tenant, embedding model, `index_struct` JSON (vector type + collection), `retrieval_model`, keyword table, optional `pipeline_id`, multimodal flag. `Vector.resolve_vector_type` reads `VECTOR_STORE` or per-dataset `index_struct_dict["type"]`. `VectorType` is a zoo (Weaviate, Qdrant, Milvus, PGVector, Chroma, Elasticsearch, OpenSearch, TiDB, OceanBase, …). App/runtime state lives in PostgreSQL (Grafana dashboards query it). External knowledge API: skip migration, call someone else’s RAG.

## 2. STCA applicability

Agent: canvas + ReAct is a **product orchestrator**, not Kutha Control SoT (ADR-000 rejected hard FSM as sole agent control). Same family as Apache Burr (AZ) and CoAgent sagas — compile a workflow *pack*, do not make Dify the runtime quantum. Query: hybrid retrieve + rerank is a **lease** over documents (NaviX/SIEVE/Compass family), not a graph join. Data: pluggable VDB + Postgres datasets is Gupta’s multi-backend trap from YOTG Vol. 31; chunks are not dated facts. Verify: citations/attributions are UI provenance, not constant-size log receipts. Time: document enable/archive flags, not bi-temporal invalidate. Distinct from Graphiti (LLM-extracted *edges*), AZ Research Assistant (projected-schema Cypher over BIKG), and Raven (in-DB agents). InfraNodus blog on Dify site is a third-party graph overlay, not a native KG.

## 3. Quality / cost

Usefulness med: huge demand for “ship an agent UI + RAG” without writing glue; parent-child chunks, hybrid search, metadata filters are real RAG engineering. Optimality low for Kutha: no WCOJ/CSR, no event-log SoT, vector type enum is an integration matrix not a kernel. Confidence `code` only for the Dataset/VDB surface actually read; workflow graph engine path was 404 on the guessed file — canvas semantics stay `spec` via docs. Cost: borrow **nothing as SoT**. Optional later: a Dify *adapter pack* that compiles Chatflow nodes into dict-first tool calls against Kutha views (compiler-not-executor). Do not vendor Dify’s Postgres+Weaviate as the graph.

## 4. Demand

Teams already build customer-support and internal-wiki agents on Dify. They will ask Kutha to “be the knowledge node.” Answer: knowledge retrieval must hit **executed views/receipts**, not a chunk store branded as truth. Engine demand: a retrieval port; product demand: they may keep Dify as the canvas.

## 5. Niche → effect

`no niche` — contrast / adjacent product, not a vertical pack.
