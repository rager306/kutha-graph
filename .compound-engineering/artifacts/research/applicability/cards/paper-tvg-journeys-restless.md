---
id: paper-tvg-journeys-restless
source: paper
axes: [Time, Query, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Time-respecting journeys are the hop primitive — restless waiting makes them hard

Papers: [Δ-restless temporal paths](https://consensus.app/papers/details/b57d5f7e265757e893950cd599ba871e/?utm_source=cursor) (Casteigts et al., 2021, Algorithmica, DOI: 10.1007/s00453-021-00831-w) — unrestricted temporal paths are P; Δ-wait (restless) is W[1]-hard even on almost-paths [1][16]; [interval vs contact-sequence](https://consensus.app/papers/details/adfa296b2d5957d1a596aedec1f2ed34/?utm_source=cursor) (Jain et al., 2022) — interval model is a *superset*; some path problems NP-hard on intervals but P on contacts [2]; [Wu ICDE reachability index](https://consensus.app/papers/details/5869e75972ec554a896f053a85918657/?utm_source=cursor) (2016) [3]; [timed transitive closure](https://consensus.app/papers/details/d3fd6bea19d45e1db9a617fd44acd80f/?utm_source=cursor) (Brito et al., 2021) unsorted contact insert, O(log τ) reachability [7]; [temporal reachability graphs](https://consensus.app/papers/details/c2874d3dbde153f4af3c8da54cbb3aa9/?utm_source=cursor) (Whitbeck et al., 2012) (τ,δ)-journeys [5]; [Kempe temporal networks](https://consensus.app/papers/details/ecb5b839681a52549a6c6678a3461d32/?utm_source=cursor) (2000) time-respecting paths; Menger fails [19]. Distinct from `paper-tgql-intervals` (QL + validity intervals) and `paper-taris-incremental-icm` (streaming ICM / Pregel grain).

## 1. Raw idea

A *journey* (temporal path) uses edges with non-decreasing timestamps. Foremost = earliest arrival; min-hop = fewest edges; fastest = smallest duration. **Restless**: time spent at a vertex ≤ Δ (infection immunity, packet TTL). Unrestricted time-respecting paths are polynomial; restless is hard [1]. Contact-sequence vs interval encodings are not equivalent: interval graphs can need far fewer edges and make some problems jump to NP [2]. Indexes (2-hop, TTC) answer reachability without materializing all journeys [3][7]. Span-reachability relaxes order to “same window” [13] — a different predicate, not a drop-in.

## 2. STCA applicability

Time/Query: Cypher `*n` without a time-respecting iterator is a **static lie**. Kutha’s hop lease must expose journey operators (foremost / min-hop / Δ-restless) over the log fold, not BFS on the latest CSR. Data: interval edges (T-GQL already) are the right *encoding*; this card is the **algorithmic contract** (P vs restless NP, TTC as a reversible index). Do not import betweenness/#Temporal Path as P0 [12]. Distinct from TARIS: that maintains *algorithms* incrementally; this names which path *semantics* are even computable.

## 3. Quality / cost

Usefulness high: every AS-OF multi-hop is a journey. Optimality med: unrestricted foremost is cheap; restless/counting are traps. Cost: default = time-respecting polynomial operators + optional Δ as a pack constraint; TTC/2-hop as leases, not SoT.

## 4. Demand

Legal amendment chains and contact-style hops both fail if waiting is unbounded or order is ignored. Engine demand: temporal path primitive in Query 070.

## 5. Niche → effect

`no niche`
