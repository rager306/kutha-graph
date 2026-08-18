---
id: paper-coagent-mtpo
source: paper
axes: [Agent, Composition, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# CoAgent / MTPO: advisory concurrency; agent repairs, runtime undoes

Paper: [CoAgent](https://arxiv.org/abs/2606.15376) (Lyu et al., 2026). Consensus: https://consensus.app/papers/details/3c0ef19d9a81540eb3ec2d990ae55a5c/?utm_source=cursor

Companion: [Atomix](https://arxiv.org/abs/2602.14849) — progress-aware transactions; commit only when per-resource frontiers show no earlier conflicting work can still arrive; irreversible effects gated. Consensus: https://consensus.app/papers/details/9bfc9995bcee58a2b95b055b0c194247/?utm_source=cursor

Related: [SagaLLM](https://consensus.app/papers/details/fb333f7884265bc493c0f90edc691ba5/?utm_source=cursor) (Chang et al., 2025, DOI: 10.14778/3750601.3750611) — sagas, compensation, independent validators; relaxes ACID for workflow-wide recovery.

## 1. Raw idea

Locks block minutes of inference; OCC retries throw away the whole thought. MTPO: fix serialization order at launch; serve order-filtered reads; speculative in-place writes; notify the reader to re-judge; mechanically undo misplaced writes via registered saga inverses. At quiescence the run is serializable in the pre-order. CoAgent is toolcall middleware; ToolSmith grows undoable tools.

## 2. STCA applicability

Agent/Composition: distinct from the isolation *lattice* (which anomalies exist) — this is *how* long-running agents share Kutha without 2PL. Verify: inverses belong on the event log (Cordon/saga), not in the LLM. LLM-as-repair-judge stays off the SoT (TOKI/TGMS); the log records the patch. Atomix adds when irreversible tools may settle.

## 3. Quality / cost

Useful existence proof vs 2PL/OCC on agent timescales. Cost: LLM-on-the-repair-path; Kutha should log the notification and inverse, not trust the judge. Not P0 engine.

## 4. Demand

Several agents on one graph will deadlock or lose minutes if Kutha only offers locks or OCC.

## 5. Niche → effect

`no niche`
