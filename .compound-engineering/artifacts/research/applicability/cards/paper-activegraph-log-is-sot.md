---
id: paper-activegraph-log-is-sot
source: paper
axes: [Time, Agent, Composition, Verify]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Append-only log as SoT; graph is a fold; behaviors react

Paper: [The Log is the Agent](https://arxiv.org/abs/2605.21997) (Nakajima, 2026). Consensus: https://consensus.app/papers/details/d2e268332bee5395b1e2f681a3128e67/?utm_source=cursor

## 1. Raw idea

Invert agent stacks: log first, working graph = deterministic projection, behaviors emit events. Yields replay, cheap fork at any event, lineage to the model call. Determinism contract for replay soundness.

## 2. STCA applicability

Almost a restatement of ADR-000 D1/D2 + dict-first agents. Time: log = proof. Agent: LLM is a behavior, not the runtime. Composition: no component instructs another except via the graph. Verify: replay/fork.

## 3. Quality / cost

Highest narrative fit in this batch. Cost: product is a runtime essay, not a hop engine — Kutha still needs CSR/WCOJ on the projection.

## 4. Demand

Fork-diff and “why this artifact” without a bolted-on logger.

## 5. Niche → effect

`no niche` — this *is* the engine thesis, not a vertical pack.
