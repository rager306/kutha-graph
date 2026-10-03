---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 6/6
exit: pass
commands:
  - cmd: cargo tree -p kutha-runtime --offline
    exit: 0
  - cmd: cargo test --workspace --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --features rocks-index-spike --test m002_rock00
    exit: 0
  - cmd: uv run pytest scripts/tests/test_kutha_gov.py -k Freeze
    exit: 0
  - cmd: uv run kutha-gov --check freeze-rocks selftest
    exit: 0
  - cmd: kill-mutations a/c/e (101); d swallow-malformed after 931e965 (101)
    exit: 101
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/rocks_spike.rs
  - crates/kutha-runtime/tests/m002_rock00.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/Cargo.toml
  - Cargo.lock
  - scripts/kutha_gov/kinds.py
  - scripts/tests/test_kutha_gov.py
  - .kutha/dictionaries/checks.yaml
  - .kutha/STATE.md
  - docs/ADR/ADR-010-event-log-runtime-quantum.md
  - docs/ADR/ADR-012-snapshots-tiers-vacuum.md
  - docs/architecture/m002-cloud-kernel-io-intake.md
  - docs/architecture/benchmark-baseline.md
  - docs/process/gsd-auto-contract.md
  - CHANGELOG.md
---
HEAD `931e965`. CBM `root-kutha-graph` stale (`rocks_spike.rs`/`m002_rock00.rs` `not_tracked`). rg arbiter. Benches not re-run. Kill (d) mutated `rocks_spike.rs` in the main tree (default `target/`); restored via `git checkout`.

| claim | evidence | verdict |
|-------|----------|---------|
| 1 freeze-rocks unless M002 | YAML needle `**Active Milestone:** M002`; copy check HIGH off-M002 / missing STATE; cypher/hnsw HIGH under M002; pytest Freeze 10; selftest freeze-* OK; always/never-skip fail Freeze | pass |
| 2 default never compiles Rocks | default `cargo tree` no rocksdb; feature optional; workspace 202 passed; lock additions-only rocksdb subtree; snappy/lz4/zstd/zlib/bzip2 off | pass |
| 3 ROCK-00 oracles | 3 tests: offset round-trip, droppable rebuild, `malformed_log_line_fails_closed` InvalidData; `build_index` drops `DB` before `lookup` | pass |
| 4 scope | crates/ only runtime Cargo.toml, cfg `lib.rs`, `rocks_spike.rs`, `m002_rock00.rs`; no unwrap/expect in lib; ADRs **Proposed**; honeycomb restage only | pass |
| 5 benches | default ingest/`as_of` sources not in diff; recorded ~18% host noise; verifier did not rerun | pass |
| 6 build record | `m002-cloud-kernel-io-intake.md` § Phase 51: versions, toolchain, cold/warm, offline proof, non-proof list | pass |

| mut | failing |
|-----|---------|
| (a) offset+1 | `rocks_roundtrips_event_id_to_offset`, `rocks_dir_is_droppable` |
| (b) skip reopen | N/A (drop then `open_for_read_only`) |
| (c) key `[0u8;16]` | both offset tests |
| (d) skip parse errors | `malformed_log_line_fails_closed` (exit 101; round-trip/droppable still ok) |
| (e) absent → `Some(0)` | `rocks_roundtrips_event_id_to_offset` |
| unless always/never-skip | Freeze tests |

Gaps: none after `931e965`.
