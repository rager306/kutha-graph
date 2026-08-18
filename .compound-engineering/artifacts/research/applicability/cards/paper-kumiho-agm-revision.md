---
id: paper-kumiho-agm-revision
source: paper
axes: [Time, Agent, Verify]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# AGM belief revision on a property-graph memory (Kumiho)

Paper: [Kumiho](https://arxiv.org/abs/2603.17244) (Park, 2026). Consensus: https://consensus.app/papers/details/41a1a7f82ae157e6a1ef74fd1426800b/?utm_source=cursor

## 1. Raw idea

Immutable revisions + mutable tag pointers; operational semantics satisfy AGM K*2–K*6 and Hansson Relevance/Core-Retainment. Dual-store Redis+Neo4j.

## 2. STCA applicability

Time: revision as versioned write. Verify: formal postulates vs TOKI isolation. Agent: memory product — Neo4j as long-term store fights Kutha self-contained core.

## 3. Quality / cost

Strong theory, host-DB architecture. Cost: LLM rerank and prospective indexing on the write path.

## 4. Demand

Adversarial refusal / implicit constraints — related to STALE, not a hop engine.

## 5. Niche → effect

`no niche`
