# ULTRA (DeepGraphLearning/ULTRA)

User-supplied 2026-08-18. PyG foundation model for inductive KG link prediction + UltraQuery logical QA.

| item | value |
|------|--------|
| Repo | https://github.com/DeepGraphLearning/ULTRA |
| Disk / CBM | cloned `/tmp/user-url-scout/ultra` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; `ultra/models.py` (`Ultra`, `RelNBFNet`); `ultra/tasks.py` `build_relation_graph` (hh/tt/ht/th); `ultra/ultraquery.py`; `ultra/rspmm/source/rspmm.h` |
| Papers | arXiv:2310.04562 (ICLR 2024); arXiv:2404.07198 (NeurIPS 2024 UltraQuery) |
| Closed card | `ultra-kg-foundation-reasoner` |
| Do not | treat neural scores as MATCH truth; vendor PyTorch; confuse `rspmm` with MPI CRP-SpMM |

Kutha mapping: Query/Agent **scored inductive lease** over a fold. Relative relation graph, not per-entity embeddings. Distinct from OBDA compile and from `ruvector-gnn-facade`.
