# Post-M002 code/tests audit (2026-10-04)

Read-only. CBM `root-kutha-graph` gen `2026-10-04T14:36:49Z`; GitNexus `kutha-graph` lastCommit `36fedb9`. Coverage: cited paths `metadata_match` / `no_recorded_issue` (best-effort, not completeness). No `index_repository`.

**CBM vs GitNexus (rg arbiter):** (1) GN `impact(Persister struct)` inbound=0 vs CBM `Persister.attach` in=50 — GN **struct-level false negative**; method `context(attach)` and `rg Persister::attach` agree (store `open_for_append_until`, bench, tests). (2) CBM inbound `store.open` lists `rocks_index::open_db` / `wal::last_event_id_at_end` — **name collision** with `File::open`/`DB::open`; `rg 'store::open\(|Runtime::open\('` is production truth. (3) GN process `Open_hydrated → Read_jsonl_events` is ranking, not a call; `load_log_events` (`store.rs:544–549`) skips jsonl when `manifest.json` exists.

## 1. Public API vs call sites

`lib.rs`: `pub mod store`; `persist_append`/`commit`/`segment`/`wal`/`hydrate` are private; `Persister` re-exported from `store` (`store.rs:815–819`). `rocks_index` / `rocks_spike` are feature-gated `pub mod`, **not** on `Runtime`.

| Symbol | Non-test reach | Verdict |
|--------|----------------|---------|
| `store::persist` / `Runtime::persist` | `kutha-tenant` main:46, `benches/baseline.rs:202`, `examples/statute_as_of` via Runtime, `open_for_append_until` heal | **live** whole-file rewrite |
| `store::open` / `Runtime::open` | bench:231, example `statute_as_of`, **not** CLI (`kutha-tenant` persist-only) | **live** |
| `Persister::attach/append` | bench:213–221 only in prod; CLI never | **live in bench**, else tests / `open_for_append` |
| `store::migrate` | CBM inbound=4, GN context=4 tests in `m002_seg01` | **test-only** |
| `store::export_jsonl` | 1 caller: `export_jsonl_roundtrips_and_is_never_read_on_open` | **test-only** |
| `commit::committed_prefix` | via `store::open_report` | **live on every open** |
| `segment::persist_log` / `load_events` | persist empty/segmented dirs; migrate | **live** |
| `wal::*` | persist legacy + Persister + segment frames | **live** |
| `hydrate_from_log` / `outcomes_from_events` | persist, checkpoint, `finish_open` | **live** |
| `SegmentIndex::*` | GN `open_or_rebuild` upstream=3 tests only; file comment `rocks_index.rs:2` | **test-only lease** |
| `rocks_spike` | still scans `events.jsonl` (`rocks_spike.rs:18–49`) | **dead vs SEG-01**; feature spike |

**Whole-file persist still exists (HIGH):** `persist_until` (`store.rs:290–325`) always rewrites the log (`segment::persist_log` or `replace_events_wal`+`replace_events_jsonl`) plus terms/snapshot/outcomes/justifications. Incremental append is opt-in `Persister`. Empty dirs write KUTHASEG; **legacy dirs (`events.jsonl`/`events.wal`, no manifest) still treat jsonl as D-04 SoT** (`load_events_d04`, `store.rs:466–478`). `persist` never auto-migrates (`legacy_dir_without_migrate_stays_legacy_after_persist`). JSONL-as-SoT is gone **only** after manifest.

## 2. Durability oracles (kill test → surviving mutation)

| Invariant | Test that kills | Mutation tests miss |
|-----------|-----------------|---------------------|
| Atomic sidecar tmp+rename | `persist_replaces_events_jsonl_atomically` (`m012a_verify_persist.rs:87`); abort-before-rename family in `m002_snap01` / `m017_replay_integrity` | Skip `sync_all` on tmp **before** rename on **segmented** `write_manifest` only (legacy jsonl is covered). |
| Dir fsync | `persist_leaves_no_tmp_files` (`m002_snap01.rs:225`, expects 7); `rollover_manifest_flip_fsyncs_dir` (`m002_seg01.rs:870`) | Drop **final** `sync_dir` after segmented `persist_log` (count ≠ legacy 7). |
| Committed prefix | `payload_without_outcome_is_not_visible`, `product_assert_without_any_outcome_is_hidden`, `horn_followon_quantum_is_atomic` (`m002_commit01.rs`) | **Typed/Dictionary mixed suffix:** product `Assert` then `kutha:*` / `JustificationCite` without QO — prefix cut at first `NeedsOutcome` (`commit.rs:56–72`) drops trailing SelfCommit. Tests cover each class **alone**. |
| Tail guard | `tail_guard_*` `m002_seg01.rs:750–784`; `append_tail_guard_*` `m002_append01.rs` | **After `migrate`:** `append_after_migrate_resumes` does happy attach+append, not same-length last-frame swap on the new active. |
| Chain verify | `writer_flipped_manifest_chain_hex_*`, chain-link tests `m002_seg01.rs:1168+` | Flip **active** header prev-hash while sealed chain stays valid (active check exists in `read_active:505`; confirm named writer test). |
| Manifest flip | rollover failpoints `RolloverManifest` in `m002_seg01` | Crash after new header file exists, **before** manifest rename, then second process `open` (no lock). |
| Torn active tail | `torn_active_tail_dropped` / `writer_torn_active_tail_is_dropped` (append **partial** bytes) | Truncate **active** at **exact complete frame end** (should keep prefix; `scan_segment` sealed=false `off==len` break at `segment.rs:352–356`) — not distinguished from mid-frame. |
| Sealed truncate at frame boundary | `writer_truncated_sealed_at_frame_boundary_is_invalid_data` (`m002_seg01.rs:1261`) | Truncate sealed so file_len still equals `trailer.bytes` after shrinking payload+rewriting trailer field (bytes check may pass if both lie). |
| Missing active | `missing_active_segment_is_invalid_data` (`m002_seg01.rs:365`) | Manifest `active` names a **sealed** file that exists (wrong role) rather than missing path. |
| Migrate | `migrate_preserves_ids_and_fold_fingerprint`, crash matrix, `migrate_verify_fault_*` | Concurrent `persist` on same legacy dir during migrate (no flock; `rg flock` empty). |
| export_jsonl | `export_jsonl_roundtrips_and_is_never_read_on_open` | Race: `export_jsonl` `load_log_events` then `replace_events_jsonl` vs `Persister.append` (no test; CLI unused). |
| Rocks rebuild | `stale_index_is_rebuilt`, `catch_up_indexes_only_the_tail` (`m002_rock01.rs:291–369`) | Poison **middle** locator while `m:tip` still matches (`tip_matches_log` `rocks_index.rs:393–399`) — `open_or_rebuild` returns stale; catch_up refuses to rewrite prefix (test **asserts** that). |
| Two-writer open | **none** | Two `Persister::attach` on one dir; no advisory lock in crate. |

**HIGH:** mixed Typed/Dictionary uncommitted group; no multi-writer; rocks tip-only staleness. **MED:** migrate×tail_guard, export race, checkpoint×uncommitted snapshot (`snapshot_ahead_of_committed_prefix_fails_closed` kills ahead-offset; miss: checkpoint after omitted QO then `open_for_append` heal vs fail-closed).

## 3. Semantic core (independent of M002)

- **Horn TT=MAX vs as_known_on:** characterized, not forbidden — `known04_horn_fires_when_premise_hidden_on_early_cut` (`m017_known_on_horn.rs:20`); `eval.rs:123` `search_prems(..., u64::MAX)`, `live_horns` `is_live_at(u64::MAX, vt)`. Missing oracle: derive **must not** mint when a premise is absent at the knowledge cut (would fail today).
- **Derived identity:** `head_event` → `Event::new` → `Uuid::now_v7()` (`event.rs:279`, `eval.rs:213`). Defines on persist are `stable_define` (SHA-256), live intern still v7. Missing: derived `Behavior` id reuse across replay; DET-01 already compares **sorted triples**, not ids (`m017_determinism.rs:1–2`).
- **Cascade order:** `cascade_idles_without_inverse_loop` (`quantum_invariants.rs:67`) checks `events_in_quantum==2` only. Missing: stable `knows` vs `inverse_knows` order across Eager/Lazy.
- **Correct re-derivation:** `m011_partial_correction` / `m013_dual_mode` patch VT residuals; no oracle that Correct of a Horn **premise** retracts/re-mints `Behavior` at as_known_on.
- **CSR vs fold:** `m011_typed_csr`, `m012a_hot_indexes` fixture equality with `CsrLease::from_fold`. **No randomized differential** (no proptest/bolero; DET-01 is seeded LCG mix of ops, not CSR). Missing: ∀ seed, `typed_csr_lease_at(tt,vt)` neighbor set = `fold.live_at(tt,vt)`.
- **Seeded determinism:** 8 seeds persist_open (`m017_determinism.rs`). EventIds not reproducible (`now_v7`). Missing: inject clock/rng so ids are seed-stable.

## 4. Performance

**bytes_per_fact 17218→25200 B (Phase 50, peak RSS / live_count, `benches/baseline.rs:305–314`):** not a new `Fact` field. Phase 50 made persist allocate a **second full encoded copy**: `encoded_log` clones every event + `stable_define` per dict string (`store.rs:704–716`); `wal::frame_bytes` `serde_json::to_vec` + frame `Vec` per event (`wal.rs:27–38`); `replace_events_wal` plus jsonl; `Runtime::snapshot` clones `GraphFold` (`hydrate.rs:60–69`, `snapshot.rs:5–14`). Confirm: heaptrack/dhat on `persist_until`; or skip WAL write / skip `fold.clone` and re-run `KUTHA_BENCH_N=10000` bpf.

**open() is O(N) at:** `segment::load_events` `fs::read` + scan every frame (`segment.rs:517–543`); legacy `recover_events`/`read_jsonl_events`; `committed_prefix` linear; `from_snapshot` walks **all** graph events (`hydrate.rs:88–101`); `hydrate_from_log` + `rebuild_log_indexes` (`pipeline.rs:21–38`); `replay_check`. Snapshot does not make open O(tail).

## 5. Complexity / dead code

CBM hotspots (`get_architecture` kutha-runtime): `Runtime.intern` fan_in=165, `default` 154, `emit` 145, `EventLog.len/iter`, `store.open/persist` ~111 (GN `intern` upstream risk CRITICAL, 21 direct). Cycles: eval↔pipeline↔admission (size 24); open↔load_events_d04 (12); Persister append/rollover (4) — expected, not a split win.

**MED simplify:** two persist layouts in `persist_until` + duplicated fsync accounting; `rocks_spike` jsonl vs `rocks_index` segments; `wal::append_events` `#[allow(dead_code)]` (`wal.rs:102`). **LOW:** `log_has_quantum_outcome` dead (`hydrate.rs:22`); migrate/export_jsonl public with no CLI; CBM `store.open` fan_in inflated by `open` name collision.

## Findings

- **HIGH** Whole-file `persist` + legacy jsonl-SoT still on `Runtime`/`kutha-tenant`/`bench`. Evidence: `persist_until:290`, tenant:46, `load_events_d04:466`.
- **HIGH** Uncommitted mixed Typed/Dictionary group; no multi-writer lock. Evidence: `committed_prefix:56–72`; `rg flock` empty.
- **HIGH** Rocks tip-only staleness; index not on Runtime. Evidence: `tip_matches_log:393`; `rocks_index.rs:2`; GN tests-only.
- **MED** Horn derive at TT=MAX vs as_known_on is characterized (`m017_known_on_horn`), not a kill. Derived ids are wall-clock v7.
- **MED** RSS = persist buffering (`encoded_log`+`frame_bytes`+`GraphFold` clone), not per-fact CSR. open O(N) in load+hydrate+replay.
- **LOW** migrate/export_jsonl/SegmentIndex/rocks_spike unused by product entry points.
- **LOW** No CSR↔fold randomized differential; DET-01 is not one.
