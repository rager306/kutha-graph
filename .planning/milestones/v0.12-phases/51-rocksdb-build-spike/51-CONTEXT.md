# Phase 51 Context: RocksDB build spike

**Milestone:** v0.12 / harness M002 S03
**Requirements:** ROCK-00
**Depends on:** Phase 49 (thaw map; shipped). Independent of Phase 50.
**Run rules:** `docs/process/gsd-auto-contract.md` (RED first; oracle preservation; bench gate; CBM primary, GitNexus secondary outside M013)

## Locked decisions

- **D-01 (scope):** prove that RocksDB builds on this host and can hold an `EventId` to file-offset map for `events.jsonl`. Nothing else. Not wired into `Runtime`, `store::open`, `persist` or the fold. No payloads, commit frames, dispositions or term meanings in RocksDB (ROCK-01 later, and only as a droppable lease).
- **D-02 (shape):** an optional Cargo feature `rocks-index-spike` on `kutha-runtime` gates one module `crates/kutha-runtime/src/rocks_spike.rs` (`#[cfg(feature = "rocks-index-spike")]`) and one test `crates/kutha-runtime/tests/m002_rock00.rs` (`#![cfg(feature = "rocks-index-spike")]`). The `rocksdb` dependency is `optional = true`. Default builds, `cargo test --workspace` and the bench never compile RocksDB. No new crate, no change to the public `Runtime` API.
- **D-03 (governor, first task):** `freeze-rocks` currently forbids the string `rocksdb` in `crates/*/Cargo.toml` and the root `Cargo.toml`. Allow it only while `.kutha/STATE.md` names Active Milestone `M002`. Prefer a YAML-only way (existing kinds, see `.kutha/META.md`); if no kind can express it, add one optional `unless:` field to `glob_absent` in `scripts/kutha_gov/kinds.py` with a pytest and a selftest mutation, and ship the YAML row that uses it. The check message names the thaw gate. The other two freeze checks (`freeze-cypher-deps`, `freeze-hnsw-dep`) are unchanged and must keep failing on their needles. This governor change is its own commit, before any Cargo change.
- **D-04 (oracles, RED first):** in `m002_rock00.rs`: `rocks_roundtrips_event_id_to_offset` (persist a seeded runtime, scan `events.jsonl` line offsets, store `EventId` bytes to `u64` offset, close and reopen the DB, every id returns its offset and the bytes at that offset parse to an event with the same id); `rocks_dir_is_droppable` (delete the Rocks directory, rebuild from the log, same map); `rocks_not_reachable_from_default_build` is a governor/CI fact, recorded as a command, not a test. A key absent from the log returns `None`.
- **D-05 (build prerequisites):** record in `docs/architecture/m002-cloud-kernel-io-intake.md` or a new short section of `docs/architecture/benchmark-baseline.md` neighbours, not a new doc: crate versions (`rocksdb` 0.22.0, `librocksdb-sys` 0.16.0+8.10.0), toolchain (clang, cmake, g++), cold and warm build time, target size, whether `--offline` works from the local registry cache. If it cannot build, write the failure and the fallback (a pure-Rust sorted index file as the lease) and stop; do not force it.
- **D-06 (docs):** ADR-010/012 get a dated 2026-10-03 Evidence line "RocksDB builds offline as an optional spike; no wiring". `.kutha/STATE.md` § Thaw map row for the RocksDB crate says "optional feature `rocks-index-spike` since Phase 51". Cells stay **Proposed**; `honeycomb.yaml` restage line. CHANGELOG entry in the last commit.
- **D-07 (bench gate):** only `Cargo.toml` and a feature-gated module change under `crates/`; default build code paths are untouched. Re-run the pair once in an isolated target (interleaved with the previous tag binary is not required; one run each and the ratio) and record it; a regression over 10 percent on medians is a stop.
- **D-08 (no network assumptions):** the crate is already in the local registry cache; `cargo build --offline --features rocks-index-spike` is the proof command. If `Cargo.lock` changes, the diff is limited to the rocksdb subtree.
- **D-09 (roles):** verifier is not the executor; the controller owns STATE, the README lease row and the CHANGELOG lease entry; no `51-VERIFICATION.md` from the executor.

## Out of scope

Segments, manifest, hash chain, incremental append, `delivery_key` index, any Runtime or fold change, any use of RocksDB in default features, ADR-080/081, Cypher, HNSW.
