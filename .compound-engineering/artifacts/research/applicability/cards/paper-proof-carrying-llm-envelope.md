---
id: paper-proof-carrying-llm-envelope
source: paper
axes: [Verify, Agent, Query]
usefulness: high
optimality: med
demand: med
confidence: paper
layer5: "Legal/clinical/finance: verify the deterministic envelope, not the model weights"
status: closed
channels_failed: []
---

# Proof-carrying certificates around the LLM (not of the LLM)

Paper: [Proof-Carrying Certificates for LLM Pipelines](https://arxiv.org/abs/2605.16407) (Koomullil, 2026). Consensus: https://consensus.app/papers/details/62cc877d14c65101bad1040310692639/?utm_source=cursor

## 1. Raw idea

Verify structured computations around the model (grounding, embedding stability, Hoare-style agent actions). Maximal Certifiable Residue: abstention becomes the largest still-certifiable claim set.

## 2. STCA applicability

Verify: same family as TGMS claim-vs-trace and “LLM as compiler.” Query: emission gates. Does not make the LLM SoT.

## 3. Quality / cost

Vision-grade Lean artifact. Cost: certificate tax on every hop; overkill for P0 physics.

## 4. Demand

High-stakes packs (patent/legal retrieval, irreversible tools) need a residue, not a vibe.

## 5. Niche → effect

Legal/clinical/finance: verify the envelope; Kutha quantum receipts are the engine-native cousin.
