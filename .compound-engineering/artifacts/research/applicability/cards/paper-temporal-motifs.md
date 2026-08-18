---
id: paper-temporal-motifs
source: paper
axes: [Query, Time, Data]
usefulness: med
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Temporal motifs are small timestamped subgraphs — not journeys and not WCOJ

Papers: [Motifs in Temporal Networks](https://consensus.app/papers/details/1f5cbe6a566956c2b05e134cd5eb8d55/?utm_source=cursor) (Paranjape et al., 2017, WSDM, DOI: 10.1145/3018661.3018731) induced subgraphs on edge sequences; domain-distinct frequencies [10]; [model survey](https://consensus.app/papers/details/c6c75486d1905a5d93673b71a10746e1/?utm_source=cursor) (Liu et al., 2020/21) temporal inducedness vs timing windows [3]; [Sarıyüce lens](https://consensus.app/papers/details/95de792319e75080ad049627a2f91440/?utm_source=cursor) (2025) motifs as a standard mining primitive [6]; [timed-automata BGP](https://consensus.app/papers/details/1e3e7c269bd95e91ab427095ed33f91d/?utm_source=cursor) (Aghasadeghi et al., 2022/23) general temporal constraints as automata [9]; [time-respecting flow patterns](https://consensus.app/papers/details/4d562da58dbd5e578c883741ef1bb635/?utm_source=cursor) (Gao et al., 2021) rooted time-respecting flow graphs [13]; [TIMEST](https://consensus.app/papers/details/eb1c6e8dad6b58909add57e63b5f795f/?utm_source=cursor) sampling estimator for larger motifs [5]. Distinct from `paper-tvg-journeys-restless` (s–t paths / Δ-wait) and `paper-taris-incremental-icm` (streaming ICM algorithms). GPU miners (Everest/Mint) stay queued.

## 1. Raw idea

A *temporal motif* is a small pattern plus an edge-order / duration window, not “any time-respecting path.” Counting explodes (same edge, many timestamps). Models disagree on inducedness and windows [3]. Timed automata generalize “journey vs motif vs delay” into one constraint language [9]. Flow-graph matching asks for a *rooted* time-respecting subgraph [13].

## 2. STCA applicability

Query: motif/flow matching is a **pack** over the log fold (fraud, diffusion), not the default Cypher `*`. Time: same time-respecting physics as journeys; different *shape* (k-node vs s–t). Do not put motif enumeration on the write path. Explainability-via-motifs for TGNNs [7] is Agent-pack, not kernel.

## 3. Quality / cost

Usefulness med for the engine, high for vertical analytics. Optimality med: exact count dies at ~4 vertices; estimators/GPU are not P0. Cost: expose motif/flow as Query pack; default hops stay polynomial journeys.

## 4. Demand

Fraud/AML and contact patterns show up as 3-edge motifs, not as Cypher novels. Engine demand: optional pattern miner on leases.

## 5. Niche → effect

`no niche`
