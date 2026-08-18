---
id: paper-user-as-code-log
source: paper
axes: [Time, Agent, Space]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Append-only user log + typed checkpoint (User as Code)

Paper: [User as Code](https://arxiv.org/abs/2606.16707) (Li, 2026). Consensus: https://consensus.app/papers/details/48ec07da3da1521481486b8306e32f82/?utm_source=cursor

## 1. Raw idea

Never discard facts: append-only log, periodically checkpointed into typed executable state. Rules run when state changes (alerts), not only on retrieve.

## 2. STCA applicability

Time: log-as-SoT analog for *user* memory. Space: dictionaries as typed objects. Agent: behaviors as functions over the fold. Checkpoint must remain a materialization.

## 3. Quality / cost

Strong “log never lies, view is a fold” rhetoric. Cost: Python-object memory is not a graph engine; do not clone UaC as Kutha UX.

## 4. Demand

Aggregate questions and contradiction-with-rules fail on bag-of-facts RAG.

## 5. Niche → effect

`no niche`
