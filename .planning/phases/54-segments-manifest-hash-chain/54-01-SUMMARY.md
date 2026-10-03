# 54-01 SUMMARY — Segment format, manifest, open

Executor Phase 54 / M002 S06 plan 54-01 only. Not verification. Honeycomb stays Proposed. Did not edit STATE/README/REQUIREMENTS/ROADMAP or write VERIFICATION.

## Commits

- `de1c565` test(54-01): RED oracles for segment format, manifest and open
- `189757c` feat(54-01): load sealed segments when a manifest is present
- `8326c09` test(54-01): reject a sealed file that has no trailer
- this commit: Evidence, bench table, CHANGELOG, SUMMARY

## RED (`m002_seg01`)

Failed 12: round_trip_tiny_segmented_dir, missing_sealed_segment_is_invalid_data, missing_active_segment_is_invalid_data, truncated_sealed_is_invalid_data, flipped_byte_in_sealed_frame_is_invalid_data, swapped_adjacent_sealed_frames_is_invalid_data, reordered_or_deleted_sealed_frames_is_invalid_data, manifest_file_disagreement_is_invalid_data, torn_active_tail_dropped, seal_trailer_on_named_active_ends_stream, unnamed_segment_leftover_ignored, manifest_wins_over_disagreeing_jsonl. Preserved 1: legacy_dir_still_opens_and_appends_as_before.

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

`load_events_d04` rg: definition + `load_log_events` legacy branch only. `store::open` rg ~92 call sites (M010–M017, m002_*, benches, `Runtime::open`, examples). CBM inbound `open` 14 (tests off) / 66 with tests (stale index). `wal::frame_bytes` now also `segment.rs`; `recover_events`/`write_wal`/`last_event_id_at_end` unchanged callers (`store` persist, `persist_append`). Coverage: `segment.rs` / `m002_seg01.rs` `not_tracked`; store/lib/wal `metadata_changed`. No `index_repository`.

## old → new

None of M010–M017 / m002_snap01 / append01 / commit01 assertions. `open_report` loads via `load_log_events` (manifest → segments, else D-04). `replace_via_tmp` / `load_events_d04` `pub(crate)`. Persist/Persister still jsonl+WAL.

## Bench (D-09)

`3c469fe` vs HEAD, 4 interleaved pairs, `tmp/p54-*-target`. ingest 108.9M → 109.6M (+0.6%); as_of 280366 → 289168 (+3.1%); open.n 3.011s → 2.853s (−5.2%). Segmented vs legacy open (~10k asserts, 30025 encoded): 3.086s → 3.053s (−1.1%). **No ≥10% median regression.** Worktrees removed.

## Residual + mutations

Writer still legacy (54-02). Skip-manifest / jsonl-on-manifest: missing_sealed + manifest_wins. Torn-drop truncated sealed: truncated_sealed. Accept sealed without trailer survived truncated_sealed (tear into trailer still InvalidData); added `sealed_without_trailer_is_invalid_data` (killed). Skip chain: swapped_adjacent. Skip order: reordered_or_deleted. Each `git checkout --` left a clean tracked tree.
