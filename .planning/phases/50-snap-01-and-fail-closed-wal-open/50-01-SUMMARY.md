# 50-01 SUMMARY — SNAP-01 and fail-closed WAL open

Executor Phase 50 / M002 S02. Not verification. Honeycomb stays Proposed. Did not edit STATE/README/REQUIREMENTS/ROADMAP or write VERIFICATION.

## RED (`cargo test -p kutha-runtime --offline --test m002_snap01`)

Failed (8): `sidecar_truncated_mid_line_opens_from_log`, `sidecar_stale_prefix_opens_from_log`, `sidecar_unparseable_terms_is_absent`, `persist_leaves_no_tmp_files`, `log_ahead_of_snapshot_opens_with_tail_fold`, `wal_short_header_without_jsonl_is_err`, `wal_read_io_error_not_masked`, `crash_matrix_every_step_reopens_to_prefix`.
Passed (preservation): `sidecar_spoof_not_prefix_still_fails`, `extra_anchor_inside_snapshot_range_fails`.

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

`persist` inbound 51 (integration tests, `Runtime::persist`, `kutha-tenant`, `benches/baseline`). `open` inbound 54. `recover_events` → `load_events_d04` → `open`. `check_tt_anchors` → `open` + `replay_check`. `replace_via_tmp` → WAL/jsonl/snapshot + sidecars + abort seams. Tests: M010–M017 persist/open oracles; FF5/FF6 untouched. `hydrate.rs` not edited. After edits: store/wal/provenance `metadata_changed`; `m002_snap01.rs` `not_tracked` (no `index_repository`).

## old → new

`persist` body → `persist_until` (`persist_with_failpoint` seam). `check_tt_anchors` stays exact; open uses `check_tt_anchors_range(..., Some(snap.next_tt))`. `read_terms` returns `Option<Vec<String>>` (unparseable = absent). Test helper `log_ids` drops `Op::Define` (persist rewrites via `stable_define`). No FF5/FF6/M010–M017 assertion edits.

## Bench (STOP)

Same host, `KUTHA_BENCH_N=10000`, isolated `CARGO_TARGET_DIR`. `7e31784` vs HEAD.

| run | base ingest | HEAD ingest | Δ | base as_of | HEAD as_of | Δ |
|-----|------------:|------------:|--:|-----------:|-----------:|--:|
| a | 88490215 | 109468422 | +23.7% | 227344 | 252921 | +11.3% |
| b | 101718878 | 117933202 | +15.9% | 284070 | 291312 | +2.5% |

Gate ≤10%: **STOP** (ingest both runs). Ingest/`as_of` sources not edited; `bytes_per_fact` RSS 17218→25200 includes extra persist fsync. Do not close.

## Residual / SNAP-01 out of scope

Non-Unix `sync_dir` is no-op. Crash matrix is an injected failpoint, not a kernel crash. Persist still rewrites the whole log (S04). COMMIT-01 S05. Segments, Rocks, hash chain, typed clocks, C01, Horn, `attach_outcomes` unchanged.
