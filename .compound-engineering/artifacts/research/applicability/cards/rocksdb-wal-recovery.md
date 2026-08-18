---
id: rocksdb-wal-recovery
source: rocksdb
axes: [Time, Data, Verify]
usefulness: high
optimality: high
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# WAL + point-in-time recovery modes

Evidence: `include/rocksdb/options.h` `WALRecoveryMode` (tolerate last incomplete record vs `kPointInTimeRecovery` default); WAL filter hook during replay. CBM `root-vendor-source-rocksdb`.

## 1. Raw idea

Append-only log with configurable crash recovery; replay is the reconstruction of state.

## 2. STCA applicability

Time: event-log SoT analog. Verify: replay = proof of fold. Data: LSM is a materialization of the WAL, same shape as graph-as-fold.

## 3. Quality / cost

Battle-tested. Cost: graph semantics (VT/TT, behaviors) are not in RocksDB — only the durability physics.

## 4. Demand

Any claim of replay/fork-diff without a real WAL story is vapor.

## 5. Niche → effect

`no niche`
