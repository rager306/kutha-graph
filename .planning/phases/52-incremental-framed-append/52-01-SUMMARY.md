# 52-01 SUMMARY — Incremental framed append

Executor Phase 52 / M002 S04. Not verification. Honeycomb stays Proposed. Did not edit STATE/README/REQUIREMENTS/ROADMAP or write VERIFICATION.

## Commits

- `c1d5379` test(52-01): RED oracles for incremental append
- `758ba43` fix(52-02): drop torn jsonl tails without a newline
- `3d01a1f` feat(52-02): Persister framed append with tail guard and checkpoint
- `eb314b6` feat(52-02): add persist_append.n to the baseline bench
- this commit: ADR evidence, honeycomb restage, CHANGELOG, bench table, SUMMARY

## RED (`cargo test -p kutha-runtime --offline --test m002_append01`)

Failed (10): `append_keeps_prefix_bytes_of_jsonl_and_wal`, `append_one_log_sync_per_quantum`, `append_nothing_new_does_no_io`, `append_torn_jsonl_tail_without_newline_is_dropped`, `append_replay_parity_with_full_persist`, `append_tail_guard_mismatch_fails_closed_without_writing`, `append_crash_matrix_reopens_to_prefix`, `open_between_checkpoints_uses_stale_leases`, `checkpoint_every_threshold_refreshes_snapshot`, `append_after_open_resumes`.
Passed (preservation): `append_torn_wal_tail_drops_only_tail`, `complete_malformed_jsonl_line_still_fails`.

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

`persist` inbound ~51 (M010–M017 tests, `Runtime::persist`, `kutha-tenant`, `benches/baseline`). `open` inbound ~54. `read_jsonl_events` → `load_events_d04` → `open` (test-local jsonl helpers unchanged). `recover_events` → `load_events_d04` + wal unit test. Coverage after edits: store/wal/lib/baseline `metadata_changed`; `persist_append.rs` / `m002_append01.rs` `not_tracked` (no `index_repository`). FF5/FF6/M010–M017 assertions untouched.

## old → new

`write_wal` body → `write_frames` + `sync_all` + log-sync counter. `encoded_log` / sidecar writers / `sync_dir` / `json_err` are `pub(crate)` for `persist_append.rs`. Bench `expected` keys 28→29 (`persist_append.n`). Factor-1 retract live_count allows the 1% append tail. No FF5/FF6/M010–M017 assertion edits.

## Bench (D-10)

Interleaved `bd5fa4b` vs HEAD, 4 pairs, `KUTHA_BENCH_N=10000`, own target dirs. Medians: ingest_plain.n 119.7M → 112.1M (−6.4%); as_of.n 305280 → 300691 (−1.5%). **No ≥10% median regression.** HEAD persist.n median 5.940s vs persist_append.n 48.9 ms.

## Residual (D-12)

WAL still duplicates jsonl until S06 segments. `append` is single-writer (no lock file). Acknowledged-vs-unacknowledged is `append` `Ok`, not the quantum-outcome sidecar (COMMIT-01). In-memory tail check uses `durable_graph_count` because `encoded_log` re-prefixes `stable_define`; file guard is lengths + last `EventId` from the end. Full `persist` still rewrites.
