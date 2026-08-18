---
id: paper-engram-bitemporal-memory
source: paper
axes: [Time, Query, Agent]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Bi-temporal agent memory: invalidate-not-delete, as-of hybrid read

Paper: [Engram](https://arxiv.org/abs/2606.09900) (Wang, 2026). Consensus: https://consensus.app/papers/details/342ca530fe025e429d9f831cb0277af3/?utm_source=cursor

## 1. Raw idea

Fast write path appends lossless episodes (no LLM on critical path). Async extract of (s,p,o) into a bi-temporal KG; contradictions invalidate, never delete. Hybrid read + as-of filter; lean retrieved slice beats full history.

## 2. STCA applicability

Time: VT×TT + supersession chain. Agent: LLM off the write path (Memanto D6). Query: hybrid retrieve. Graphiti-class storage with stronger invalidate semantics — still not Kutha log-SoT unless the episode log *is* the engine log.

## 3. Quality / cost

Strong empirical vs full-context. Cost: memory product; do not replace event-log SoT with Engram’s KG.

## 4. Demand

AS OF after corrections without stuffing 80k tokens of chat.

## 5. Niche → effect

`no niche`
