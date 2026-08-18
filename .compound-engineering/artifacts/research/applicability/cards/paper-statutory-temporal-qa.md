---
id: paper-statutory-temporal-qa
source: paper
axes: [Time, Query, Agent]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal statutory QA: post-cutoff staleness and recency bias; temporal validity as a hard retrieve constraint"
status: closed
channels_failed: []
---

# Statutory QA: post-cutoff staleness and recency bias

Paper: [Asking For An Old Friend](https://arxiv.org/abs/2605.23497) (Prior et al., 2026). Consensus: https://consensus.app/papers/details/7fe3645c2dc8545099e1487543db6603/?utm_source=cursor

## 1. Raw idea

Two temporal failure modes on German statutory QA (312 expert pairs): *post-cutoff staleness* (apply superseded rules after amendments) and *recency bias* (prefer newer text when a historical version governs). Vanilla LLMs degrade badly post-cutoff; RAG that filters versions by extracted fact date helps; web search is unstable and recency-biased. Claim: reliable legal QA needs temporal validity as a **hard constraint**.

## 2. STCA applicability

Time/Query: MemStrata’s “cosine cannot see contradiction” now has a *domain probe* — the wrong statute version is a stale fact. Agent: LLM parametric memory is not SoT. Event log + validity intervals (T-GQL/Engram) are the retrieve contract; the LLM compiles the question.

## 3. Quality / cost

High demand evidence; benchmark is German statutes, not a graph engine. Cost: do not build a legal product from this card — it only falsifies “embed the code.”

## 4. Demand

Any as-of retrieve over evolving norms (law, APIs, policies). Without version filtering, hybrid RAG serves superseded law.

## 5. Niche → effect

Legal statutory QA: temporal validity as a hard retrieve constraint. Not an intake filter on the matrix.
