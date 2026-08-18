---
id: paper-llm-compiler-not-executor
source: paper
axes: [Agent, Verify, Time]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Scientific/agent research loops: figures and numbers only from executed views, not from the model"
status: closed
channels_failed: []
---

# LLM as query compiler, never executor

Paper: [Confining Nondeterminism](https://arxiv.org/abs/2607.10508) (Kim et al., 2026). Jina: `https://arxiv.org/html/2607.10508`.

## 1. Raw idea

Project is a deterministic versioned dataflow. LLM may only edit the plan. Executor never calls the LLM; asserted results require an execution behind them.

## 2. STCA applicability

Agent: matches dict-first / fail-closed validate. Time: versioned views. Verify: no number without a log event.

## 3. Quality / cost

High alignment, vision-stage (full version in prep). Cost: treating every GenAI enrichment as a versioned pack with CA cache (already ADR-000).

## 4. Demand

Stops “announced a number no tool returned” — same failure Kutha forbids for legal facts.

## 5. Niche → effect

Scientific archive / agent research loops: claims and figures only from executed folds; LLM proposes pipeline edits.
