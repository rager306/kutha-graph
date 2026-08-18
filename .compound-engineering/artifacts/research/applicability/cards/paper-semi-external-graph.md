---
id: paper-semi-external-graph
source: paper
axes: [Data, Query, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Vertex state in RAM, edges on SSD — not LSM snapshots and not a second SoT

Papers: semi-external model: **vertices fit in RAM, edges do not** [2][4]; [FlashGraph](https://consensus.app/papers/details/5856a5be5f2d53b0a0245d594ab33c46/?utm_source=cursor) (Zheng et al., 2014) edge lists on commodity SSDs, vertex state in memory; overlap I/O; up to 80% of in-memory, beats distributed PowerGraph [4]; [GraphMP](https://consensus.app/papers/details/2327f53f45a15254864017f38a378a7e/?utm_source=cursor) sliding window + selective shard skip + compressed edge cache; tens of × vs GraphChi/X-Stream/GridGraph [3]; ROSE: *read-only* external edges so one graph image is shared, compressible, no SSD wear from writes [9]; GraphChi-class out-of-core vs SEM survey [1]; Seraph/Clip: less total I/O beats “sequential locality at all costs” [18][20]; FAM-Graph: fabric-attached memory as the edge tier, 1–6× of in-memory Galois at 20× less local RAM [15]. Distinct from `paper-lsm-snapshot-compaction` (version placement on the log), `paper-graph-bulk-load` (first ingest), `paper-k2tree-succinct-graph` (encode adjacency, still assumed in RAM), `samyama-csr-frozen-adjacency` (hot in-memory CSR). GPU/CXL microsecond-latency [11] stays queued.

## 1. Raw idea

Billion-edge graphs do not need a cluster if **O(n) vertex state stays in RAM and O(m) edges stream from SSD** [4]. Algorithms issue “fetch edges of v”; I/O is the cost [9]. The disk holds a **lease of adjacency**, not truth — truth is still the event log (or a sealed snapshot of it).

## 2. STCA applicability

Data: SEM is a materialization *placement*: CSR shards on NVMe, PageRank/labels in RAM. Composition: which shards to cache is a Cui budget. Query: BFS/SSSP/WCC as vertex-centric passes over the lease; Cypher MATCH that needs random hops pays I/O — planner must know the tier. Time: rebuilding shards from the log is bulk-load’s cousin; do not mutate SSD as SoT. ROSE’s read-only edges match “leases are droppable.” Verify: a crash recovers vertex state from the log, not from a dirty edge cache.

## 3. Quality / cost

Usefulness high: legal/science dumps exceed RAM. Optimality high: FlashGraph/GraphMP are measured; sequential-at-all-costs is the wrong pole [18]. Cost: P0 = mmap/CSR if it fits; honeycomb = SEM edge shards + vertex RAM + selective I/O. Do not vendor GraphChi as the quantum. Do not make the SSD the event log.

## 4. Demand

A 40 B-edge fold will not fit in the hot box. Engine demand: an explicit vertex-in-RAM / edges-on-NVMe lease, fail-closed to “spill, don’t OOM.”

## 5. Niche → effect

`no niche`
