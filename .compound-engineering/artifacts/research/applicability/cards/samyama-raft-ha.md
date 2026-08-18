---
id: samyama-raft-ha
source: samyama
axes: [Composition, Time, Verify]
usefulness: med
optimality: med
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Samyama Raft: replicated writes via openraft; query apply is empty

CBM `root-samyama-graph` (coverage: `src/raft/state_machine.rs`, `src/raft/mod.rs`, no_recorded_issue). `openraft` 0.9; `GraphStateMachine::apply` persists CREATE/DELETE/UPDATE through `PersistenceManager`; `Request::ExecuteQuery` returns `QueryResult { rows: 0 }`. Module docs: writes through leader, reads may be stale. ADR-004.

Paper baseline: [In Search of an Understandable Consensus Algorithm](https://consensus.app/papers/details/7953a8b1f76e5dfaa8f2e683fcf0bbe5/?utm_source=cursor) (Ongaro & Ousterhout, 2014). Q24 hits were mostly blockchain Raft variants — not graph-native HA.

## 1. Raw idea

Cluster HA: leader election + log replication so graph mutations survive node loss. Samyama wires Raft to Rocks persistence.

## 2. STCA applicability

Time/Verify: the **event log** is already the SoT; Raft is how that log is *replicated*, not a second truth. Composition: do not let Raft become the query bus (`ExecuteQuery` stub proves the temptation). Kutha P0 can stay single-writer; HA is a later honeycomb.

## 3. Quality / cost

Real apply path for mutations (`code`). Cost: snapshot atomicity still a follow-up in Samyama plans; geo-Raft WAN pain is well documented. Do not pull Multi-Raft into P0.

## 4. Demand

Operators will ask for HA before a correct as-of hop. Answer: replicate the log, serve queries from local leases.

## 5. Niche → effect

`no niche`
