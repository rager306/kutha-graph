---
id: paper-pg-constraint-repair
source: paper
axes: [Verify, Data, Agent]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Constraint repair is a logged operator, not silent delete

Papers: [Repairing Property Graphs under PG-Constraints](https://consensus.app/papers/details/6f954c7a709754d3be3c395f7f745e39/?utm_source=cursor) (Spinrath et al., 2026, PVLDB, arXiv:2602.05503, DOI: 10.48550/arxiv.2602.05503) — denial+recursive subset; ILP / greedy; label-delete cuts node/edge deletes ~59% [1]; [User-Centric Property Graph Repairs](https://consensus.app/papers/details/abfdb34f3426549bb47ba25e61089188/?utm_source=cursor) / [Grafixer](https://consensus.app/papers/details/96baf3676e7651f49d3e60840f6e3fa1/?utm_source=cursor) (Pachera et al., 2025) multi-user HITL, Cypher constraints, independent-set assignment [2][3]; [Deducing Certain Fixes to Graphs](https://consensus.app/papers/details/a05134e0c0b65d5f860be72a18f1e519/?utm_source=cursor) (Fan et al., 2019) Church-Rosser certain fixes from rules + ground truth [6]; [GRR / δ-GRR](https://consensus.app/papers/details/f2f88fbdc2ff5e0cb1d4e08df6eea4f0/?utm_source=cursor) (Cheng et al., 2022) incompleteness/conflict/redundancy repair [7]. Cousin: [LLM graph repair](https://consensus.app/papers/details/3cea543ff082576abf8a1a0312ad4633/?utm_source=cursor) — LLM in the loop, do not trust [4]. Distinct from `paper-pgschema-types-keys` (declare/validate), `paper-toki-contradiction-ops` (write-time isolation), and `ruvector-hnsw-delete-repair` (rewire ANN neighbor lists after tombstones — index hygiene, not semantic constraint repair).

## 1. Raw idea

PG-Constraints detect denial/recursive violations; **repair** then deletes nodes, edges, or labels to restore consistency [1]. HITL systems (Grafixer) let several users fix overlapping violations without conflict via independent sets [2][3]. Fan: only apply fixes that are *certain* given rules Σ and ground truth Γ; order of rules does not matter (Church-Rosser) [6]. GRR states how to correct incompleteness and conflicts, not only detect them [7]. LLM repair papers show models can suggest patches but need a human/oracle [4].

## 2. STCA applicability

Verify: a violation is a **fact** on the log (“constraint C failed on subgraph S”). A repair is a **compensating event** (TOKI cousin), not an in-place mute of history. Prefer Fan’s certain-fix fragment: if the engine cannot prove the delete, keep the loser as an audit row. Agent: Grafixer users are Control, not the LLM. Data: label-only repair is cheaper than topology delete [1] — still emit both as events. Distinct from PG-Schema (validation without mutation) and schema discovery (propose types, don’t punch holes).

## 3. Quality / cost

Usefulness high: dirty graphs will exist; silent ILP delete is how legal/clinical graphs lose provenance. Optimality med: greedy vs ILP is a pack choice; Church-Rosser certain-fix is the quality bar. Cost: detect → log violation → optional HITL/certain-fix pack; never auto-delete as SoT.

## 4. Demand

Every schema language ships a “repair” button. Engine demand: violation events + replayable compensations. Product demand: Grafixer-class UI over the log.

## 5. Niche → effect

`no niche`
