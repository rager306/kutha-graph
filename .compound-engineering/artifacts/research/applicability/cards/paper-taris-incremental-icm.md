---
id: paper-taris-incremental-icm
source: paper
axes: [Time, Query, Data]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Incremental time-respecting algorithms (TARIS / ICM), not full recompute

Papers: [TARIS](https://consensus.app/papers/details/4a65890232335b4ba4542b9352f776bd/?utm_source=cursor) (Bhoot et al., 2024, IEEE TPDS, DOI: 10.1109/tpds.2024.3471574); ICM/GRAPHITE [An Interval-centric Model for Distributed Computing over Temporal Graphs](https://consensus.app/papers/details/4d96a2d9c57e52d7b3100272a8a34bb4/?utm_source=cursor) (Gandhi and Simmhan, 2020, ICDE, DOI: 10.1109/icde48307.2020.00102). Related: WICM optimizations (Baranawal et al., 2022); incremental WICM (Kulkarni et al., 2023). Contrast already closed: Raphtory lazy views, GRADOOP TPGM, DBSP IVM, Graphflow Delta GJ.

## 1. Raw idea

Time-respecting algorithms require monotonic timestamps on traversed edges (journeys, infection paths, contact tracing). ICM treats a vertex’s **time-interval as the unit of data-parallel computation** and hides TimeWarp partitioning. GRAPHITE implements ICM on Giraph (TI + TD algorithms; up to 5.5B edges). TARIS extends **windowed ICM to streaming updates**, proves incremental execution ≡ batch ICM, interleaves ingest with compute, and adapts window size to input rate. Eval: up to 2B edges; 3–4 orders of magnitude vs Tink and GRADOOP; 83k–587M mutations/s with seconds–minutes latency. Sibling: Kairos incremental diffusion monitoring; BIC sliding-window connectivity without physical deletes.

## 2. STCA applicability

Time/Query: hot temporal *algorithms* (reachability, foremost path, motifs) must be **leases maintained from log deltas**, not snapshot rebuilds — same contract as DBSP/Graphflow but for time-respecting semantics T-GQL already names. Data: interval as compute grain is a projection descriptor, not SoT. Distinct from Raphtory (in-memory full history + lazy views) and GRADOOP (batch dataflow). Do not import Pregel/Giraph as the Kutha runtime.

## 3. Quality / cost

Usefulness high: equivalence proof incremental ≡ batch is the falsifier Kutha needs for “replay the view.” Optimality high on reported stream rates vs GRADOOP; still analytics-platform, not WCOJ Cypher. Cost: attach ICM-style interval operators to log-delta IVM; keep CSR/HNSW as pictures of the same events.

## 4. Demand

Finance/social streams and contact-tracing hops cannot wait for nightly GRADOOP jobs. Engine demand: incremental time-respecting hops on the event log. Not a substitute for legal PIT receipts.

## 5. Niche → effect

`no niche`
