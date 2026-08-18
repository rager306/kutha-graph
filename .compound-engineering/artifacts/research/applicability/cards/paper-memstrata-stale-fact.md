---
id: paper-memstrata-stale-fact
source: paper
axes: [Time, Query, Agent]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: "Legal statutory QA: temporal validity as a hard constraint; cosine RAG serves superseded law"
status: closed
channels_failed: []
---

# Deterministic (s,r,o) supersession; cosine cannot see contradiction

Paper: [MemStrata](https://arxiv.org/abs/2606.26511) (Yadav, 2026). Consensus: https://consensus.app/papers/details/9a214f04828f50768d4970f0c1552185/?utm_source=cursor

## 1. Raw idea

Embedding similarity cannot tell a contradicted fact from a duplicate (AUROC ~0.59). When a value is contradicted, a deterministic subject-relation-object rule retires the stale row in a bi-temporal ledger — no similarity threshold, no LLM on the write.

## 2. STCA applicability

Time: supersession = retract/correct without LLM SoT. Query: retrieval must respect validity, not cosine. Agent: RAG-as-memory is a structural fail. Aligns TOKI/Engram; Kutha log already *is* the ledger if writes are typed.

## 3. Quality / cost

High: falsifies “just embed it.” Cost: keying facts as (s,r,o) vs free text nodes.

## 4. Demand

Stale-fact error 15–40% when RAG is forced to answer; legal/code agents cannot tolerate that.

## 5. Niche → effect

Legal statutory QA: temporal validity as a hard constraint (see also post-cutoff / recency-bias failures on statutes). Not an intake filter.
