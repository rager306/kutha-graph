# 54-02 SUMMARY — Segmented persist, Persister, rollover

Executor Phase 54 / M002 S06 plan 54-02 only. Not verification. Honeycomb stays Proposed. Did not edit STATE/README/REQUIREMENTS/ROADMAP or write VERIFICATION.

## Commits

- `0eed2d4` test(54-02): RED oracles for segmented persist, Persister and rollover
- `4f8bdc2` feat(54-02): write segments on empty dirs and append to the active file
- this commit: Evidence, bench table, CHANGELOG, SUMMARY

## RED (`m002_seg01`)

Failed 18: persist_empty_dir_writes_segments_not_jsonl, persist_segmented_dir_does_not_create_jsonl, prefix_sealed_bytes_unchanged_after_append, prefix_sealed_bytes_unchanged_after_rollover, one_log_sync_per_quantum_on_segmented, append_creates_no_jsonl, append_nothing_new_does_no_io_on_segmented, tail_guard_length_mismatch_writes_nothing, tail_guard_last_id_mismatch_writes_nothing, tail_guard_same_length_different_last_record_writes_nothing, rollover_crash_matrix_reopens_to_prefix, rollover_manifest_flip_fsyncs_dir, checkpoint_dir_fsync_count_exact_on_segmented, open_for_append_heals_and_appends_on_segmented, stale_leases_between_checkpoints_on_segmented, interned_terms_between_appends_survive_on_segmented, second_and_third_appends_keep_earlier_bytes_as_prefix, segmented_replay_parity_with_legacy_full_persist. Preserved: 14×54-01 + persist_legacy_dir_keeps_jsonl_and_wal + legacy_dir_still_opens_and_appends_as_before.

## Empty dir + pins (D-06/D-10)

Empty = no `manifest.json`, no `events.jsonl`, no `events.wal`. Mechanical `ensure_legacy_dir`: `m002_append01`, `m002_snap01`, `m017_replay_integrity`, `m017_verify_killtests`, `m012a` jsonl tests, `m016_s04_kill`, `m002_rock00`, `legacy_dir_still_opens_and_appends_as_before`. Assertions unchanged.

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

Coverage: `persist_append.rs` / `segment.rs` / `m002_seg01.rs` `not_tracked`; store/wal `metadata_changed`. No `index_repository`. `detect_changes` since RED: those three plus the pin files.

## old → new

Workspace 271 tests (was 252; `m002_seg01` 33). Empty/segmented persist writes segments; `Persister` appends to active only; rollover trailer→header→manifest; heal = full segmented persist. Legacy writer unchanged.

## Bench (D-09)

`39af84b` vs `4f8bdc2`, 4 interleaved pairs, `tmp/p54-02-*-target`. ingest 118.5M → 111.5M (−5.9%); as_of 304508 → 304573 (+0.0%); open.n 2.957s → 2.726s (−7.8%); persist.n 5.84s → 2.37s (−59.4%, empty segmented vs legacy); persist_append.n 51.1ms → 14.0ms (−72.6%). **No ≥10% median regression on ingest/as_of/open.n.** Worktrees removed.

## Residual + mutations

All nine kill-mutations failed an oracle (`git checkout --` restore; tracked tree clean): jsonl-on-append; two `sync_data`; skip dir fsync; rollover without trailer; rewrite sealed; manifest before header sync; skip length guard; skip last-id; append without I/O. Residual: `store::migrate` / export / tamper oracles are 54-03.
