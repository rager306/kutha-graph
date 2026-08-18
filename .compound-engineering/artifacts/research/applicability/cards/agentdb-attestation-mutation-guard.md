---
id: agentdb-attestation-mutation-guard
source: agentdb
axes: [Verify, Agent, Security]
usefulness: med
optimality: med
demand: med
confidence: spec
layer5: no niche
status: closed
channels_failed: []
---

# Hash-chained mutation attestation + WASM mutation guard

Evidence: ruflo-agentdb README G7 controllers — `attestationLog` (hash-chained `.swarm/attestation.db`), `mutationGuard` WASM proofs, `GuardedVectorBackend`.

## 1. Raw idea

Mutations to vector/memory state emit an append-only attestation chain; optional proof generation.

## 2. STCA applicability

Verify: analog of quantum receipts for *enrichment* writes. Agent: fail-closed mutations. Not a substitute for the event log.

## 3. Quality / cost

Credible pattern, kernel not in-tree. Cost: dual audit logs if copied naively.

## 4. Demand

Anyone who lets agents write the graph needs “who changed this embedding/fact.”

## 5. Niche → effect

`no niche`
