---
id: paper-stale-implicit-conflict
source: paper
axes: [Time, Agent, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Implicit conflict: retrieve-updated ≠ act-updated (STALE)

Paper: [STALE](https://arxiv.org/abs/2605.06527) (Chao et al., 2026). Consensus: https://consensus.app/papers/details/8ad8e97361415dc78e10f9ff7877319e/?utm_source=cursor

## 1. Raw idea

A later observation invalidates an earlier memory *without explicit negation*. Benchmark: state resolution, premise resistance, implicit policy adaptation. Best models ~55% overall.

## 2. STCA applicability

Time: contradiction is not only LWW of identical keys (MemStrata) — also commonsense invalidation. Agent: dictionaries must encode state, not bag-of-facts. Verify: premise resistance = fail-closed on stale presupposition.

## 3. Quality / cost

High demand signal. Cost: LLM judge on implicit conflict fights “LLM not SoT”; prefer typed state + TOKI/MemStrata first.

## 4. Demand

Legal/agent “you still think X” after a silent world change.

## 5. Niche → effect

`no niche`
