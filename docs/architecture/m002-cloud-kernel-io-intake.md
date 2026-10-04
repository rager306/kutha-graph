# M002 intake: cloud kernel I/O writeback stalls

**Status:** note only — M002 is **CLOSED** (2026-10-04, GSD v0.12). IO-01 (cloud kernel writeback soak) remains intake-only; it is not a leased slice.  
**Captured:** 2026-10-02  
**Card:** `.compound-engineering/artifacts/research/applicability/cards/paper-kernel-io-dirty-throttle-cloud-oltp.md`

## Sources

- Mitsumasa Kondo. *Invisible Yet Dominant: Big Stalls of Kernel I/O Mechanisms in Cloud OLTP Databases.* arXiv:2609.12597 (SOSP 2026 poster).
- Companion: *SteelDB* arXiv:2603.29052.

## Claim (for M002 planning)

On cloud distributed block storage, buffered write-back is limited by a **single kernel flusher thread per device**. When drain lags, **IO-less Dirty Throttle** pauses writers; stalls are invisible to standard disk counters. Adding **drains** (multi-volume placement) beats adding bandwidth on one volume for TPC-C-style write load.

## Implications when M002 is leased

1. Segmented append-only log / WAL framing should not rely on “one volume + buffered I/O is fine.”
2. Prefer isolating the event-log path from index/Rocks leases across devices when on cloud block.
3. Acceptance / soak tests for durability must instrument writeback / dirty-throttle (or equivalent), not only `iostat`.
4. Rocks remains **indexes only** behind the durability protocol — this note does not thaw Rocks as SoT.

## Phase 51 build record

Host 2026-10-03. Isolated `CARGO_TARGET_DIR=tmp/p51-target`. Feature `rocks-index-spike` on `kutha-runtime` only.

| item | value |
|------|-------|
| crates | `rocksdb` 0.22.0, `librocksdb-sys` 0.16.0+8.10.0 |
| features | `default-features = false` (no snappy / lz4 / zstd / zlib / bzip2) |
| rustc / cargo | 1.98.1 |
| clang | Ubuntu 18.1.3 |
| cmake | 3.28.3 |
| g++ | 13.3.0 |
| offline proof | `cargo build -p kutha-runtime --offline --features rocks-index-spike` (registry cache already had both crates) |
| cold build | 325.35 s (`Finished` 5 m 25 s); peak RSS 871364 KiB |
| warm rebuild | 0.21 s |
| target size | 2.6 GiB (`tmp/p51-target`); `libkutha_runtime.rlib` 15 MiB; `liblibrocksdb_sys*.rlib` 768 MiB |
| default tree | `cargo tree -p kutha-runtime --offline` has no `rocksdb` / `librocksdb-sys` |

What this spike does **not** prove: Runtime/store/fold/hydrate wiring, incremental append, segments/manifest/hash chain, `delivery_key` index, payloads or term meanings in Rocks, Cypher, HNSW, or that Rocks is SoT.

## Phase 55 index timings (ROCK-01)

Host 2026-10-04. Release `cargo test -p kutha-runtime --offline --features rocks-index` (default `CARGO_TARGET_DIR`). Segmented store from the real writer: **10000** `relatedTo` asserts → **30025** encoded events. Feature `rocks-index` (`SegmentIndex::build` then 10000 `get_event` of the midpoint id).

| item | value |
|------|-------|
| encoded events | 30025 |
| index build | 410442137 ns (~410.4 ms) |
| lookup mean (10k gets) | 12819 ns (~12.8 µs) |
| lookup total (10k) | 128196038 ns |
| layout | locators/ids only (`e:` / `d:` / `m:v` / `m:tip`); not wired into open/persist |

Cold `librocksdb-sys` for this release binary: 5 m 13 s `Finished` (same order as Phase 51 ~5.5 min). Default `cargo tree -p kutha-runtime --offline` still has no rocksdb.

## Non-goals

- Do not start M002 from this note alone.
- Do not invent a fifth legal clock or change honeycomb Status.
- Do not treat SteelDB / Aurora architecture as Kutha product design — only the kernel-drain lesson transfers.
