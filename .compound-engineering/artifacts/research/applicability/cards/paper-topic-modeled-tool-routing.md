---
id: paper-topic-modeled-tool-routing
source: paper
axes: [Agent, Query, Composition]
usefulness: med
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Tool/agent routing as a compile table, not an always-hit embedding NN

Papers: [Rethinking Predictive Modeling for LLM Routing](https://consensus.app/papers/details/565e0d2b8e17550589d0df6f3cc1c2ea/?utm_source=cursor) (Li, 2025, arXiv:2505.12601, DOI: 10.48550/arxiv.2505.12601); surveys [Dynamic Model Routing and Cascading](https://consensus.app/papers/details/51fb2ef93d885ad595c750ecb069622e/?utm_source=cursor) (Moslem et al., 2026, arXiv:2603.04445) and [Doing More with Less](https://consensus.app/papers/details/fa77b886b6035006b1b58981ef5ea52c/?utm_source=cursor) (Varangot-Reille et al., 2025, JAIR); [Multi-Agent Routing as Set-Valued Prediction](https://consensus.app/papers/details/131303c17a36538883edfa1654616c9c/?utm_source=cursor) (Bala et al., 2026); [AgentRouter](https://consensus.app/papers/details/6b9b81df59e3573a8f2a5932a87a6c93/?utm_source=cursor) (Zhang et al., 2025, arXiv:2510.05445); [Tool-to-Agent Retrieval](https://consensus.app/papers/details/f4a0f5370887581cb2619fa5ac15fbdf/?utm_source=cursor) (Lumer et al., 2025, arXiv:2511.01854). AZ observation (not indexed here): Research Assistant dropped cosine-NN exemplars because they always hit; replaced with topic modeling over real queries. Distinct from `paper-az-projected-schema-cypher` (Cypher compile) and `paper-az-research-plan-dag` (question DAG).

## 1. Raw idea

Once you have many models/tools/agents, the product problem is **which pack runs**, not another LLM in the loop. Li: locality of model performance in embedding space lets a tuned **kNN** beat complex learned routers, with lower sample complexity [1]. Surveys taxonomize pre-generation routing (difficulty, clustering, uncertainty, RL, cascading) vs MoE-inside-one-model [6][8]. Multi-agent routing is **set-valued**: one query may need several agents; over-select burns cost; supervised routers beat NN and zero-shot LLM on a fixed catalog [9]. AgentRouter encodes query+entities+agents as a KG and trains a heterogeneous GNN from empirical rewards [13]. Tool-to-Agent Retrieval embeds tools *and* parent agents so coarse agent blurbs do not hide tool granularity [18]. AZ: embedding-NN over exemplars was a false-positive machine; a **topic → agent map** from real traffic was the production router.

## 2. STCA applicability

Agent/Composition: routing is a **Control compile** — dictionaries + observed query topics bind a request to packs (Cypher view, RAG lease, receipt verify). Do not make the router an unconstrained LLM (Dify Agent node) or an always-hit vector lookup. Query: intent classification is cheaper than executing the wrong WCOJ. Distinct from projected-schema Cypher (what to *run* once the KG agent is chosen) and research-plan DAGs (question graph, not tool catalog). AgentRouter’s KG-of-agents is a *router index*, not Kutha SoT.

## 3. Quality / cost

Usefulness med: every multi-pack product needs this; it is not engine physics. Optimality med: kNN/topic tables are enough when the catalog is small and labeled; GNN routers overfit ops dashboards. Cost: ship a **topic/intent table** (AZ) or a supervised set-router over dicts; log misroutes as events; never “semantic search over tool docs” as the only gate.

## 4. Demand

MCP/tool zoos already overflow. Engine demand: a compile-time catalog. Product demand: cost/latency routing across models — keep it outside the graph core.

## 5. Niche → effect

`no niche`
