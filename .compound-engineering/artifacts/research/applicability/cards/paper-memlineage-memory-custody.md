---
id: paper-memlineage-memory-custody
source: paper
axes: [Verify, Agent, Security]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Chain-of-custody on agent memory (MemLineage)

Paper: [MemLineage](https://arxiv.org/abs/2605.14421) (Ouyang et al., 2026). Consensus: https://consensus.app/papers/details/c158cc76e9c75949b7db6eaa31d50dcc/?utm_source=cursor

## 1. Raw idea

RFC-6962 Merkle log + Ed25519 per principal; weighted derivation DAG; refuse sensitive actions whose justification descends from an untrusted ancestor. Not a content filter.

## 2. STCA applicability

Verify: quantum-receipt cousin for *memory writes*. Security: PACT at memory grain. Agent: recall allowed, act blocked if tainted.

## 3. Quality / cost

Strong zero-ASR on a harness. Cost: LLM-mediated lineage edges vs purely log-derived provenance.

## 4. Demand

Poisoned memory that later authorizes a tool call.

## 5. Niche → effect

`no niche`
