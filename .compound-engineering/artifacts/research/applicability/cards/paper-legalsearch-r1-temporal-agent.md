---
id: paper-legalsearch-r1-temporal-agent
source: paper
axes: [Agent, Time, Query]
usefulness: med
optimality: low
demand: high
confidence: paper
layer5: "Legal statutory research: RL must bind query time to the governing statute version; agent is a pack over AS-OF views, not Kutha SoT"
status: closed
channels_failed: []
---

# LegalSearch-R1: RL search agents that punish retroactive law

Papers: [Can LLMs Time Travel? Enhancing Temporal Consistency in Legal Agentic Search through Reinforcement Learning](https://consensus.app/papers/details/49ec0da332b9534c94ff389527b33ed8/?utm_source=cursor) (Fan et al., 2026, arXiv:2605.25920, DOI: 10.48550/arxiv.2605.25920) — LegalSearch-R1 [1]; cousins [Search-R1](https://consensus.app/papers/details/58192257da745001bdf401595ac7eaf2/?utm_source=cursor) (Jin et al., 2025) generic RL search [3]; [RL for long-horizon legal document search](https://consensus.app/papers/details/3a2cee02b6a35e57a72d9863cc3c3645/?utm_source=cursor) (Kalyan et al., 2025) [2]; [LegalMALR](https://consensus.app/papers/details/ada2a27369aa5398a0725bde7b7a9162/?utm_source=cursor) multi-agent statute retrieval + GRPO [7]; [LRAS](https://consensus.app/papers/details/310e64c2201053e18827b0ec948137cd/?utm_source=cursor) closed-loop legal LLMs vs active inquiry [13]. Distinct from `paper-statutory-temporal-qa` (hard retrieve constraint, no RL loop) and `paper-sat-graph-legal-rag` (ontology Graph RAG).

## 1. Raw idea

Legal agentic search fails when the model ignores **when** the facts happened: training-cutoff temporal bias, search queries without date constraints, web search without pin-cites. LegalSearch-R1 trains a 7B agent with RL on temporally indexed statutes across amendment periods; pairs **local statute RAG** (precise articles) with web search (broader knowledge). Gains: +12.9–29.8% vs deep-research/legal LLMs; **+57.7–80.3% temporal consistency** [1]. Search-R1 is the generic “learn to query a search engine” backbone [3]. LegalMALR reformulates colloquial multi-issue queries via multi-agent policy (GRPO) then LLM-reranks [7]. LRAS: parametric closed-loop legal LLMs are overconfident; force active inquiry [13].

## 2. STCA applicability

Agent/Time: this is a **pack** that must compile “case date → AS-OF statute view,” not a second SoT. The engine supplies dated folds + receipts; RL may learn *when to call* that view. Do not put the LLM in the write path (TOKI). Query: local statute RAG is a lease over the legal pack (ADR-090), not chunk-truth (Dify). Distinct from statutory QA’s *retrieve filter* (no search policy) and SAT-Graph’s mereology.

## 3. Quality / cost

Usefulness med: proves demand that agents will otherwise apply tomorrow’s law to yesterday’s facts. Optimality **low** for the kernel: 7B RL, web search, GitHub agent — product, not WCOJ/CSR. Cost: expose temporal operators as tools the agent is *rewarded* to use; keep Search-R1-class loops outside the trust boundary.

## 4. Demand

Legal research desks already buy “AI search.” Engine demand: AS-OF + citation-shaped Observations (AZ cousin). Product demand: RL searcher as an optional pack.

## 5. Niche → effect

Legal statutory research: RL must bind query time to the governing statute version; agent is a pack over AS-OF views, not Kutha SoT.
