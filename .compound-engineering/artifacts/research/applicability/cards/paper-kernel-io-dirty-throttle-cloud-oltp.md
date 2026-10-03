---
id: paper-kernel-io-dirty-throttle-cloud-oltp
source: paper
axes: [Data, Packaging]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: open
channels_failed: []
milestone_defer: M002
---

# Kernel writeback stalls on cloud block storage — drains matter more than raw bandwidth

Papers: [Invisible Yet Dominant: Big Stalls of Kernel I/O Mechanisms in Cloud OLTP Databases](https://arxiv.org/abs/2609.12597) (Kondo, NTT; SOSP 2026 poster) [1]; companion [SteelDB: Diagnosing Kernel-Space Bottlenecks in Cloud OLTP Databases](https://arxiv.org/abs/2603.29052) [2]. Related: Fengguang Wu, *IO-less Dirty Throttling* (2012).

## 1. Raw idea

Buffered I/O (PostgreSQL, RocksDB, many AI KV-cache middlewares) delegates write-back to a **single kernel flusher thread (KFT) per block device**. On cloud **distributed block storage** (high latency, shallow multi-tenant queues), one KFT cannot drain dirty pages fast enough. The kernel’s **IO-less Dirty Throttle** then pauses `write()` (and can stall reads that must evict dirty pages). These pauses are **invisible** to `iostat` / `/proc/diskstats`. SteelDB’s fix is zero-patch **multi-volume placement** that multiplies KFTs and I/O queues; this poster proves the mechanism from **inside the kernel** with eBPF (`block_rq_*`, `writeback_*`, `balance_dirty_pages`).

## 2. STCA / Kutha applicability

**M002 (log durability) only** — not honeycomb Accepted, not a reason to thaw Rocks as SoT. When Kutha leases M002:

- Treat **WAL / append-only segment path** as first-class vs page/cache writeback; do not assume one cloud volume + buffered I/O is enough for hot append.
- Prefer **physically isolating** the event-log (and later WAL framing) from fold indexes / Rocks leases across volumes or paths when deploying on cloud block.
- Durability acceptance tests must look beyond `iostat` (dirty-throttle / writeback context), or they will miss the dominant stall.
- SteelDB’s lesson: **more drains (devices/KFTs), not only more provisioned bandwidth on one volume**.

Does **not** change Kutha’s SoT rule: event log remains product SoT; Rocks stays index-only behind the durability protocol.

## 3. Evidence (from [1])

Identical total provisioned IOPS/bandwidth (80K / 2 GB/s), TPC-C / HammerDB / PG 16.4 on AWS: 1 disk → WAL split → 4-disk SteelDB placement. Relative to single disk: **−70%** throttle pauses, **−59%** max transaction latency, **+23%** throughput. Aurora-style proprietary patches vs zero-patch SteelDB: median **254 days** to port Aurora patches to a new PG major (cited from companion).

## 4. Demand

Ops will ask why cloud OLTP latency tails explode under write load while disks “look fine.” Answer for M002: count KFTs/drains and throttle pauses, not only bandwidth counters.

## 5. Niche → effect

`no niche` — intake note for M002 lease; do not start M002 until STATE names it.
