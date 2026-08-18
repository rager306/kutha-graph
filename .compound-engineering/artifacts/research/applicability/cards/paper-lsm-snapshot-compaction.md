---
id: paper-lsm-snapshot-compaction
source: paper
axes: [Time, Data, Packaging]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Snapshots are a compaction policy — place them where queries land, not on a wall clock

Papers: [LSM compaction design space](https://consensus.app/papers/details/fe01e75e48205586aa93ab490183b9c1/?utm_source=cursor) (Sarkar et al., 2021) four primitives: trigger, layout, granularity, movement [2]; [query-distribution snapshots](https://consensus.app/papers/details/8751803d426d520582799ebb00acb4af/?utm_source=cursor) (Luo et al., 2020) cluster historical-query timestamps; same storage, ≥70% fewer redo/undo ops vs uniform interval snapshots [10]; [LSM-Subgraph](https://consensus.app/papers/details/6cc4545855da5772b0a9c6057a4366cf/?utm_source=cursor) (Ma et al., 2023) PMA snapshots + in-between logs; fluctuation-aware shard cuts [3]; [Time-tired compaction](https://consensus.app/papers/details/0aeef45fdb945cf894e605ae34edca26/?utm_source=cursor) (Zhang et al., 2024) pick SSTables by estimated chronological query load; −30% range latency [6]; [IoTDB multi-column compaction](https://consensus.app/papers/details/357d688518aa5383905b76c15d0e1874/?utm_source=cursor) out-of-order + multi-column duplicates [19]. Distinct from `paper-bach-lsm-csr-bridge` (BACH already closed: adjacency→CSR *inside* LSM levels) and `rocksdb-wal-recovery` (crash WAL, not snapshot placement).

## 1. Raw idea

Event-sourced graphs recreate history from nearest snapshot + log. Uniform wall-clock snapshots waste space on cold time and starve hot AS-OF. Place snapshots where **queries actually land** [10]. Hybrid copy+log (LSM-Subgraph) trades bytes vs replay [3]. LSM compaction is a four-knob policy, not “LevelDB default” [2]. Time-series engines already compact *by time*, not by size-tier only [6].

## 2. STCA applicability

Time/Packaging: honeycomb 012 — snapshots/segments/tiered storage are **leases of the log**, reversible. Compaction that drops losers is TOKI-illegal; compaction that *reorganizes layout* (adj-list → CSR, fat JSON → columns) is BACH-class and allowed. Do not treat LSM levels as SoT. Query-driven snapshot placement is how legal PIT stays cheap without a Raphtory-in-RAM history.

## 3. Quality / cost

Usefulness high: every AS-OF is snapshot+redo. Optimality med: clustering queries assumes a stable query mix. Cost: log is SoT; snapshot cadence is a Control/pack policy; reuse Rocks/BACH physics, add *when* not only *how*.

## 4. Demand

Ops will ask “how often do we snapshot?” Answer: where the AS-OF histogram is, not every N minutes. Engine demand: snapshot index + compaction policy object.

## 5. Niche → effect

`no niche`
