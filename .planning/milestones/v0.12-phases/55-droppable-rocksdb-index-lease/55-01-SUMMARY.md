# Phase 55 SUMMARY — ROCK-01 droppable Rocks index lease

Executor only. No STATE / VERIFICATION / README / ROADMAP edits. Honeycomb stays Proposed.

## Commits

- `4dd6589` feat(55-01): durable freeze-rocks rule
- `f8415f2` test(55-02): RED oracles for the Rocks segment index
- `a5b244c` feat(55-03): droppable Rocks SegmentIndex lease
- this commit: ADR-010/012 Evidence, intake timings, bench table, honeycomb restage, CHANGELOG Trajectory, SUMMARY

## RED (`m002_rock01`, feature `rocks-index`)

10 failed `unimplemented!`: `index_answers_equal_the_log`, `delivery_key_index_equals_runtime_map`, `dropping_rocks_dir_changes_no_answer`, `stale_index_is_rebuilt`, `catch_up_indexes_only_the_tail`, `locator_pointing_at_wrong_frame_is_invalid_data`, `rocks_values_hold_no_payload`, `legacy_dir_rejected_until_migrate`, `uncommitted_suffix_is_not_indexed`, `multi_segment_store_locates_across_rollover`. `persister_source_does_not_write_rocks` passed (source oracle). Default `cargo tree -p kutha-runtime --offline` has no rocksdb; default workspace tests compile 0 rock01 tests.

## Governor (D-06)

`glob_absent` `unless_line_contains`: crate `rocksdb =` only with `optional = true`; workspace `Cargo.toml` forbids `rocksdb =`. Needle `rocksdb =` so `dep:rocksdb` feature lines pass. `unless_file_contains` kept. Cypher/HNSW unchanged. Selftest still appends `rocksdb =` and fails closed. Optional rocksdb passes regardless of Active Milestone.

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

Coverage stale (index 2026-10-03): `rocks_index.rs` / `m002_rock01.rs` `not_tracked`; `lib.rs` / `Cargo.toml` / kinds / checks `metadata_changed`. No `index_repository`. `detect_changes` vs `0245bbd`: Cargo.toml, lib.rs, rocks_index.rs, m002_rock01.rs, kinds.py, test_kutha_gov.py, META, checks, bridges, CHANGELOG (plus untracked CE handoffs, not committed). Persist/open/segment unchanged.

## old → new

Default workspace **299** passed (m002_seg01 61; rock00/rock01 0 without features). Feature `rocks-index`: 11 `m002_rock01`. `rocks-index-spike` / `m002_rock00` still green.

## Bench (D-10) + 10k index

Idle 2-pair vs `0245bbd`: ingest 136.0M → 111.8M (−17.8%); as_of 281069 → 256599 (−8.7%); open.n −0.2%. **No ≥10% median regression.** 10k-assert / 30025-event build 410.4 ms; lookup mean 12.8 µs. Worktrees removed.

## Mutations (D-08)

8/8 killed (restore `git checkout --`): payload; skip CRC/id; skip `m:tip`; catch_up from zero; first-wins delivery_key; index uncommitted; jsonl build; Persister rocks comment.

## Residual

Not wired into Runtime/open/Persister; single reader/writer; rebuild O(N); RocksDB cold compile ~5.5 min. `open_or_rebuild` checks tip EventId, not every locator.
