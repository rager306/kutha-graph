# Dify (langgenius/dify)

User-supplied 2026-08-18. Open-source LLM app platform (workflow canvas, RAG knowledge bases, ReAct agent node, model hub, LLMOps).

| item | value |
|------|--------|
| Repo | https://github.com/langgenius/dify |
| Disk / CBM | not vendored; not indexed |
| Default confidence | `code` only for GitHub files actually read; docs otherwise `spec` |
| Read | README; `api/models/dataset.py`; `api/core/rag/datasource/vdb/vector_factory.py`; `vector_type.py`; docs knowledge + Knowledge Retrieval + Agent node |
| Closed card | `dify-workflow-rag-orchestration` |
| Do not | treat RAG knowledge as SoT; import Postgres+VDB zoo; use canvas FSM as Kutha Control |

Kutha mapping: product orchestrator + document retrieve leases. Graph/temporal truth stays the event log.
