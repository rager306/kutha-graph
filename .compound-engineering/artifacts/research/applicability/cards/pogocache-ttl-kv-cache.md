---
id: pogocache-ttl-kv-cache
source: pogocache
axes: [Data, Composition]
usefulness: low
optimality: med
demand: med
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Pogocache contrast: fast TTL KV cache, not a graph SoT

CBM `root-vendor-source-pogocache` (coverage: `src/main.c` no_recorded_issue; `src/pogocache.c` parse_partial — claims qualified). C hashmap server with Memcache/RESP/HTTP/Postgres wire protocols; embeddable `pogocache.c`; entries carry `expires`/TTL on save/load (`src/save.c`).

## 1. Raw idea

A from-scratch low-latency KV cache (faster/cheaper than Redis/Memcache in vendor benches). Optional embed in-process. Not a property graph, not a log, not a join engine.

## 2. STCA applicability

Data: at most a **lease cache** in front of a hot projection (CSR/HNSW), never SoT. Composition: wire-protocol polyglot is a product lesson, not an architecture. Contrast with RocksDB WAL (durable log) and Helix object-store SoT.

## 3. Quality / cost

Real C kernel (`code`). Cost: no graph semantics; promoting it to Kutha storage would revive “cache as truth.” TTL is eviction, not valid-time.

## 4. Demand

P0 in-process Rust core may still want a tiny hot cache for leases; this card records the adjacent extreme, not a dependency.

## 5. Niche → effect

`no niche`
