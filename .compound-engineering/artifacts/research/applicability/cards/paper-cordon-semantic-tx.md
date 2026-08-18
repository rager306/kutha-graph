---
id: paper-cordon-semantic-tx
source: paper
axes: [Agent, Composition, Verify]
usefulness: med
optimality: med
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Semantic transactions for tool-using agents

Paper: [Cordon](https://arxiv.org/abs/2606.17573) (Chen et al., 2026). Jina: `https://arxiv.org/html/2606.17573`.

## 1. Raw idea

Task-scoped boundary: stage irreversible effects, validate composed flow, then commit or rollback. Not per-RPC tools.

## 2. STCA applicability

Composition: cascade quantum / reversible packs. Agent: tool policy. Verify: audit of staged vs committed.

## 3. Quality / cost

Useful analog for runtime quantum receipts. Cost: mapping “external effects” onto event log + reversible materializers without a second workflow engine.

## 4. Demand

In-DB agents that call tools: need commit/rollback of a whole request, not one tool call.

## 5. Niche → effect

`no niche` — control-plane engine feature; verticals inherit it.
