# Phase 51 SUMMARY — ROCK-00 RocksDB build spike

**Milestone:** v0.12 / M002 S03. Executor only; no STATE / VERIFICATION edits.

## Commits

- `6294242` feat(51-01): freeze-rocks allows rocksdb during M002
- `035c123` test(51-02): RED oracles for the RocksDB EventId-offset spike
- `8286372` feat(51-03): rocks_spike EventId to events.jsonl offset index
- docs/bench/changelog: ADR-010/012 Evidence, intake build record, bench section, honeycomb restage, `51-01-SUMMARY.md`

## RED

`cargo test -p kutha-runtime --offline --features rocks-index-spike --test m002_rock00`: both oracles panicked `unimplemented!("ROCK-00 RED: EventId-offset index")` — `rocks_roundtrips_event_id_to_offset`, `rocks_dir_is_droppable`. Absent-key `None` is asserted in the round-trip test (not reached on RED).

## Governor

No existing kind can skip `glob_absent` from another file. Added optional `unless_file_contains: {file, needle}` on `glob_absent` (missing file does not skip). `freeze-rocks` uses `.kutha/STATE.md` + `**Active Milestone:** M002`. Cypher/HNSW checks unchanged. Selftest prepends a needle-disable mutation on the copy so inserting `rocksdb` still fails.

## Build record (`CARGO_TARGET_DIR=tmp/p51-target`)

- Offline: `cargo build -p kutha-runtime --offline --features rocks-index-spike` (cache: rocksdb-0.22.0, librocksdb-sys-0.16.0+8.10.0).
- Features: `default-features = false` — no snappy/lz4/zstd/zlib/bzip2 (compile flags had none).
- Toolchain: rustc 1.98.1, clang 18.1.3, cmake 3.28.3, g++ 13.3.0.
- Cold 325.35 s / warm 0.21 s; target 2.6 GiB; `liblibrocksdb_sys*.rlib` 768 MiB.
- `cargo tree -p kutha-runtime --offline` has no rocksdb. Default `cargo test --workspace --offline`: 202 passed, log has no `rocksdb`. Feature oracles: 2 passed.

## Bench (D-07)

Interleaved `24e2306` vs HEAD, `KUTHA_BENCH_N=10000`, own target dirs, binaries from each package dir. Medians: ingest_plain.n 142.9M → 110.4M (−22.7%); as_of.n 340459 → 286208 (−15.9%). Base ingest alone moved 18%. **No ≥10% median regression.**

## Residual risks

- Rocks is not wired; ROCK-01 still a later slice.
- `unless_file_contains` is a substring gate (must stay exact `**Active Milestone:** M002`).
- New files `rocks_spike.rs` / `m002_rock00.rs` are CBM `not_tracked` until integrator reindex.
- Host bench noise ~15–18%; do not read HEAD “faster” as a product win.
