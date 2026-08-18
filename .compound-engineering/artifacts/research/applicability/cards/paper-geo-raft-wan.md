---
id: paper-geo-raft-wan
source: paper
axes: [Space, Composition, Time]
usefulness: med
optimality: low
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Geo-Raft / Multi-Raft: WAN consensus is log replication, not a second SoT

Papers: [Raft and Beyond](https://consensus.app/papers/details/47154398e77451f1b34581c951493805/?utm_source=cursor) (Ding et al., 2026) — vanilla Raft write latency tracks WAN RTT; Multi-Raft, flexible quorums, WPaxos/EPaxos/HotStuff; CockroachDB/TiKV/Yugabyte [1]; [BW-Raft / Geo-Raft](https://consensus.app/papers/details/0433151dd1c558f49b2d26e159befe4d/?utm_source=cursor) (Du et al., 2023, DOI: 10.1109/tcc.2022.3161297) and [Elastic, Geo-Distributed RAFT](https://consensus.app/papers/details/2592f8d1739f5e2ba1d6f87a45fe5d94/?utm_source=cursor) (Xu et al., 2019, IWQoS) — secretary/observer scale-out [3][6]; [CD-Raft](https://consensus.app/papers/details/39db22d9f25951e089072b690034c247/?utm_source=cursor) cross-domain leader placement [4]; [GeoCoCo](https://consensus.app/papers/details/c4bf8227cd045c689ff0ee36531bef3d/?utm_source=cursor) WAN sync grouping/pruning [14]. Graph-analytics cousin (not consensus): [RAGraph](https://consensus.app/papers/details/5f3834ce25f85c5086d74c4d75138444/?utm_source=cursor) region-aware geo graph processing [11]. Distinct from `samyama-raft-ha` (single-cluster openraft; query apply empty) and `paper-crdt-graph-eventual` (AP merge, not linearizable log).

## 1. Raw idea

Raft elects one leader; every write waits for a majority including cross-region RTTs, so vanilla Raft **does not geo-scale** [1]. Production NewSQL shards into **Multi-Raft** groups (per-range leaders). Geo-Raft/BW-Raft add stateless **secretaries** (log sync off the leader) and **observers** (read replicas), even on spot instances, claiming 5–7× smaller footprint than Multi-Raft and ~85% cost cut with spots [3][6]. CD-Raft places the leader and splits read/write WAN RTTs [4]. GeoCoCo exploits clustering, triangle-inequality violations, and redundant transfers to cut WAN sync bytes [14]. RAGraph is the other pole: **analytics** over a graph already split by region, advancing local updates and filtering WAN messages — not a GDBMS log [11].

## 2. STCA applicability

Space/Time: the event log remains SoT; Raft/Multi-Raft is **how the log is copied**. Composition: do not route Cypher through the leader (Samyama’s empty `ExecuteQuery` is the warning). P0 stays single-writer; geo is honeycomb 012/020-class. CRDT (just closed) is the AP alternative — pick one story: linearizable log *or* eventual fold, not both as SoT. RAGraph-style WAN-aware *query* scheduling can later sit on leases, not on consensus.

## 3. Quality / cost

Usefulness med: operators will demand multi-region before a correct AS-OF hop. Optimality **low** for Kutha P0: WAN RTT, Multi-Raft ops, secretary/observer extra roles. Cost: replicate the log; serve queries from local leases; revisit Multi-Raft only when a cell owns HA. Do not import blockchain-Raft (LRD-Raft et al.) as graph HA.

## 4. Demand

Cockroach/TiKV mindshare is the question “when do we geo?” Answer: after the log and receipts exist. Engine demand: one-writer durability first.

## 5. Niche → effect

`no niche`
