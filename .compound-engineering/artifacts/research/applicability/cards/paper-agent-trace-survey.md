---
id: paper-agent-trace-survey
source: paper
axes: [Verify, Agent]
usefulness: med
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Execution provenance as a typed graph (survey)

Paper: [From Agent Traces to Trust](https://arxiv.org/abs/2606.04990) (Wang et al., 2026). Consensus: https://consensus.app/papers/details/aac67a6ecc345004adeea568093c6227/?utm_source=cursor

## 1. Raw idea

Execution provenance = typed graph of a run; evidence tracing = projection onto support relations. Taxonomy: sources, units, granularity, trust functions.

## 2. STCA applicability

Verify: replay/fork already *is* an execution graph if the log is SoT. Agent: traces must not be a sidecar logger.

## 3. Quality / cost

Map, not an algorithm. Cost: over-building observability before P0 physics.

## 4. Demand

“Why this number / tool call / memory write” without final-answer accuracy.

## 5. Niche → effect

`no niche`
