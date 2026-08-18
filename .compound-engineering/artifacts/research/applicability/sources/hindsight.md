# Hindsight (vectorize-io/hindsight)

User-supplied 2026-08-18. Agent memory system (TEMPR retain/recall + CARA reflect). Paper: https://arxiv.org/abs/2512.12818

| item | value |
|------|--------|
| Repo | https://github.com/vectorize-io/hindsight |
| Disk / CBM | cloned `/tmp/hindsight` for this wave; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; `engine/retain/fact_extraction.py`; `engine/search/retrieval.py`; `fusion.py`; `graph_retrieval.py`; `link_creation.py`; `fold.py`; `mental_model_refresh.py`; arXiv HTML |
| Closed card | `hindsight-four-network-tempr` |
| Do not | treat Postgres/Oracle as Kutha SoT; put LLM extract on the write path without receipts; confuse retain-job `fold` with STCA graph fold; import CARA dispositions into Control |

Kutha mapping: optional intelligence pack. Four-network labels (fact vs opinion) and 4-arm hybrid recall are the borrowable *shape*. Graphiti/Engram remain the contrast family.
