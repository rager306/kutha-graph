---
id: tarantool-compute-near-data
source: tarantool
axes: [Composition, Data, Agent]
usefulness: med
optimality: med
demand: med
confidence: spec
layer5: no niche
status: closed
channels_failed: []
---

# Lua/C stored procedures (compute near data)

Evidence: Tarantool platform docs (Jina): LuaJIT app platform; stored procedures callable locally and via IPROTO. Wikipedia/dbdb: Lua/C procedures, in-mem + disk. No kernel tree → `spec`.

## 1. Raw idea

Run application logic in the DB address space (LuaJIT), hot RAM / cold disk.

## 2. STCA applicability

Composition: behaviors as in-process programs over the log fold — same *placement*, different language (Rust packs vs Lua). Agent: tools as stored procs. Data: tiered storage analog to Helix cache/object split.

## 3. Quality / cost

Proven placement. Cost: Lua sandbox ≠ dict-first typed ports; Rust core should not grow a second VM unless a pack demands it.

## 4. Demand

Cascade/behavior latency if every hop is an RPC to an app.

## 5. Niche → effect

`no niche`
