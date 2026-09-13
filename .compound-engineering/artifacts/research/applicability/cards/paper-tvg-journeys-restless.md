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

# Journeys traverse time; snapshot paths share one instant

Papers: [Δ-restless temporal paths](https://consensus.app/papers/details/b57d5f7e265757e893950cd599ba871e/?utm_source=cursor) (Casteigts et al., 2021, Algorithmica, DOI: 10.1007/s00453-021-00831-w) — unrestricted temporal paths are P; Δ-wait (restless) is W[1]-hard even on almost-paths [1][16]; [interval vs contact-sequence](https://consensus.app/papers/details/adfa296b2d5957d1a596aedec1f2ed34/?utm_source=cursor) (Jain et al., 2022) — interval model is a *superset*; some path problems NP-hard on intervals but P on contacts [2]; [Wu ICDE reachability index](https://consensus.app/papers/details/5869e75972ec554a896f053a85918657/?utm_source=cursor) (2016) [3]; [timed transitive closure](https://consensus.app/papers/details/d3fd6bea19d45e1db9a617fd44acd80f/?utm_source=cursor) (Brito et al., 2021) unsorted contact insert, O(log τ) reachability [7]; [temporal reachability graphs](https://consensus.app/papers/details/c2874d3dbde153f4af3c8da54cbb3aa9/?utm_source=cursor) (Whitbeck et al., 2012) (τ,δ)-journeys [5]; [Kempe temporal networks](https://consensus.app/papers/details/ecb5b839681a52549a6c6678a3461d32/?utm_source=cursor) (2000) time-respecting paths; Menger fails [19]. Distinct from `paper-tgql-intervals` (QL + validity intervals) and `paper-taris-incremental-icm` (streaming ICM / Pregel grain).

## 1. Raw idea

A *journey* traverses available edges at successive times: `t_(i+1) >= t_i + latency(e_i, t_i)`; zero latency gives non-decreasing times. A snapshot path instead requires every edge at one instant. A co-temporal interval path requires a nonempty intersection of edge-validity intervals. These predicates differ: availability `A→B [1,2)` and `B→C [3,4)` permits a zero-latency journey with waiting, but no snapshot path from A to C. This separation follows the [TVG definitions, §§4.4–4.5](https://arxiv.org/pdf/1012.0009); the example is a direct counterexample, not a benchmark result.

Foremost = earliest arrival; min-hop = fewest edges; fastest = smallest duration. **Restless** adds a maximum wait Δ at intermediate vertices. Polynomial reachability results for unrestricted temporal paths do not extend automatically to restless or counting problems [1]. Contact-sequence and interval encodings can change complexity [2]. Indexes (2-hop, TTC) answer reachability without enumerating all journeys [3][7]. “Same window” without traversal order is a different predicate [13].

## 2. STCA applicability

Time/Query: compile an explicit temporal predicate before choosing an iterator: snapshot at a selected cut, common-validity interval, or journey. A traversal over CSR bound to the requested cut can answer a snapshot question; it cannot establish a journey from a timeless projection. Journey operators must declare traversal latency, ordering, waiting constraints, and objective. Data: interval or contact records feed the selected operator; TTC is a droppable index. TARIS maintains temporal algorithms incrementally; this card names their query semantics. These remain proposed capabilities, not a P0 parser or journey engine.

## 3. Quality / cost

Usefulness high: separates “which relations held together” from “what could propagate over time.” Optimality med: unrestricted foremost and restless/counting have different complexity. Cost: current P0 has a cut-bound CSR lease and `leapfrog_intersect`, not a full path matcher; a future journey operator needs its own declared fragment and budget. TTC/2-hop remain leases, not SoT.

## 4. Demand

Legal applicability may ask for coexisting valid relations; contact-style propagation asks for ordered traversal. Engine demand: distinct, named temporal predicates in Query 070 rather than silently treating every AS-OF question as a journey.

## 5. Niche → effect

`no niche`
