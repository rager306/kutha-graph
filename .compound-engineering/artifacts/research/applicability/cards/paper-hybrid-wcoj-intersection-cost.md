---
id: paper-hybrid-wcoj-intersection-cost
source: paper
axes: [Query]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Hybrid binary + WCOJ plans with intersection-cost (Graphflow optimizer)

Paper: [Optimizing Subgraph Queries by Combining Binary and Worst-Case Optimal Joins](https://doi.org/10.14778/3342263.3342643) (Mhedhbi et al., 2019). Consensus: https://consensus.app/papers/details/5863de1fe7b85df5a85f917252cabfa5/?utm_source=cursor

Extended: [one-time and continuous subgraph queries](https://doi.org/10.1145/3446980) (Mhedhbi et al., 2021) — DP optimizer, adaptive vertex orderings, greedy combined plan for continuous/delta queries with shared intersections. Consensus: https://consensus.app/papers/details/fb85e7286871584bad37fa37aac53834/?utm_source=cursor

## 1. Raw idea

WCOJ plans match one query vertex at a time via multiway intersection. The optimizer picks vertex orderings *and* hybrid plans that mix binary joins with WCOJ intersections, using a cost metric **intersection-cost**. Adaptive reordering at runtime. Continuous side shares intersections across delta subgraph queries.

## 2. STCA applicability

Query: planner for Kutha, not a second engine. Complements Free Join (algebraic unify) with *cost-based* hybrid + continuous sharing — attaches to Graphflow Delta Generic Join already closed. Event log remains SoT; delta plans are IVM over the fold.

## 3. Quality / cost

High citations; implemented in GraphflowDB. Cost: intersection-cost estimates under skew (ADOPT later uses RL). Do not import Graphflow as SoT.

## 4. Demand

Real BGPs mix stars and cycles; continuous “when this pattern appears” needs shared intersections, not N independent LTJs.

## 5. Niche → effect

`no niche`
