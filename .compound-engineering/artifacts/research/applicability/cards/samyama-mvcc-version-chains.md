---
id: samyama-mvcc-version-chains
source: samyama
axes: [Time, Data]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# MVCC version chains / snapshot isolation on nodes and edges

Evidence: `Node.version` in `src/graph/node.rs`; `tests/mvcc_test.rs`; `benches/mvcc_benchmark.rs` (version access, time-travel). Snapshot isolation tests in `store.rs`.

## 1. Raw idea

Per-object version chains and snapshot reads; not full Snodgrass VT×TT.

## 2. STCA applicability

Time: useful for concurrent hot-store isolation. Legal PIT still needs event log + valid/transaction time; MVCC ≠ bi-temporal SoT.

## 3. Quality / cost

High for OLTP graph. Cost: confusing MVCC “time-travel” with AS OF belief reconstruction after corrections.

## 4. Demand

Concurrent writers without blocking multi-hop readers.

## 5. Niche → effect

`no niche`
