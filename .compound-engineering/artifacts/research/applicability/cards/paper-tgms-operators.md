---
id: paper-tgms-operators
source: paper
axes: [Time, Agent, Verify, Query]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: "Legal/compliance agents: belief AS OF transaction time after corrections; claim gating vs LLM arithmetic"
status: closed
channels_failed: []
---

# TGMS-style operator algebra (LLM outside trust boundary)

Paper: [TGMS: An Agent-Native Bi-Temporal Graph Management System](https://arxiv.org/abs/2607.10265) (Zhang, 2026). Jina: `https://arxiv.org/html/2607.10265`.

## 1. Raw idea

Fixed typed temporal operators; LLM only plans and verbalizes. Writes: assert / retract / correct. Traces content-addressed; claim verifier vs traces; truncation taint.

## 2. STCA applicability

Time: VT×TT + log. Agent: LLM not SoT. Verify: replay/claim gate. Query: operator registry not free Cypher as truth.

## 3. Quality / cost

High fit to dict-first + event log. Cost: designing a closed operator set and plan IR. 14B planner still fails some multi-operator motifs.

## 4. Demand

Any audited AS OF after corrections. Latest-state RAG cannot reconstruct prior belief.

## 5. Niche → effect

Legal/compliance temporal agents: “what did we believe at T?” with verifier-backed counts; reduces silent LLM arithmetic on graphs.
