---
id: paper-pact-argument-provenance
source: paper
axes: [Agent, Security, Verify]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Argument-level provenance for tool calls (PACT)

Paper: [PACT / granularity mismatch](https://arxiv.org/abs/2605.11039) (Fan et al., 2026). Consensus: https://consensus.app/papers/details/7a00c6d2b9bd5ed1b114777206b7ee40/?utm_source=cursor

## 1. Raw idea

Indirect injection is dangerous when untrusted content determines an *authority-bearing argument*, not when it merely appears in context. Monitor roles + provenance per argument vs whole-invocation allow/deny.

## 2. STCA applicability

Agent/Security: tool policy at argument grain. Verify: provenance across replans. Complements Cordon (task tx) and TGMS (LLM outside trust).

## 3. Quality / cost

High: matches dict-first “who may bind this field.” Cost: provenance inference is the remaining bottleneck (paper says so).

## 4. Demand

In-DB agents with tools will mix trusted ids and untrusted web text in one call.

## 5. Niche → effect

`no niche`
