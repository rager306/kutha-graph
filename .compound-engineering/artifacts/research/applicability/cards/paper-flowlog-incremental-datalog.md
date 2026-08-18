---
id: paper-flowlog-incremental-datalog
source: paper
axes: [Query, Data, Time]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Incremental Datalog on Differential Dataflow (FlowLog)

Paper: [FlowLog](https://arxiv.org/abs/2511.00865) (Zhao et al., 2025). Consensus: https://consensus.app/papers/details/ed76187286b459e5808df9e0603f99e0/?utm_source=cursor

## 1. Raw idea

Per-rule relational IR: semi-naïve control vs logical plan. Structural optimizer avoids worst-case joins; sideways information passing; Boolean specialization. Batch and incremental Datalog on Differential Dataflow.

## 2. STCA applicability

Data/Time: IVM of recursive graph views (reachability, motifs) as log deltas. Query: Datalog pack / Cozo-shaped rules without swallowing Cozo.

## 3. Quality / cost

High incrementality. Cost: Differential Dataflow runtime vs DBSP; not P0 core.

## 4. Demand

Continuous recursive queries over a changing fold.

## 5. Niche → effect

`no niche`
