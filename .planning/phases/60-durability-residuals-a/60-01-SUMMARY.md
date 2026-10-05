# Phase 60-01 SUMMARY

Start `1f9fb14`. Commits: `e5bf5dc` docs(60-01) GROUP-01 (c); `6d36a34` test(60-02) RED oracles; `97430c5` feat(60-03) `kutha.lock`; docs(60-04) matrix/ADR/CHANGELOG + clippy `truncate(false)` (this file).

**GROUP-01 (c):** no Event/Op/Snapshot/frame change. (a) needs a group-length header; (b) needs an Op marker. Hard-stop not taken.

**RED on lease HEAD (bugs, then fixed):** `two_writer_attach_fails_closed`, `second_writer_names_kutha_lock`, `persist_migrate_race_serialized_or_fails_closed`, `lock_file_ignored_by_dir_listings`. **Pinned (not bugs):** omitted-QO snapshot-ahead, jsonl inode, manifest `sync_all`, final `sync_dir`, mixed SelfCommit prefix (characterization), migrate tail_guard, exact frame-end prefix, wrong-role, chain prev, pre-rename leftover, sealed trailer lie, reader-while-writer, dead-process lock, panic Drop.

**Lock:** `kutha.lock`, `File::try_lock`, `WouldBlock` + message names the file; persist/migrate/export/attach/`open_for_append`; readers unlocked; dead-process does not block; panic Drop; listings ignore the file (only mechanical listing edit). Failed append/checkpoint releases so heal can run; attach validates before lock.

**Graph (CBM inbound / GN upstream impact / rg arbiter 46 files, 338 hits):** attach 52 / 52 (50 direct); persist 151 / 146 (105); migrate 4 / 4; export_jsonl 1 / 1; open_for_append 2 / 2. GN `acquire_store_write_lock` missing (index 3 commits behind). CBM `store.open` polluted by `File::open`. Coverage: `store.rs`/`persist_append.rs` `metadata_changed`; `m019_dur_a.rs` `not_tracked`. Did not reindex.

**Kills** (`tmp/p60-run-mutations.sh`, unique `tmp/p60-td-*`; role/lie needed layered skips — see log): all 13 named tests FAILED (rc=101). role: sequence+active+hydrate guards. lie: sealed chain-link + snapshot/replay.

**Matrix:** killed 46, characterization 4, gap 6, blocked 6, spec-only 4, gate 1, total 67.

**Bench** (2 interleaved pairs, worktrees `1f9fb14` vs `97430c5`, `KUTHA_BENCH_N=10000`; cleaned): ingest_plain.n 114.5M→117.1M ns (+2.3%); open.n 2.768s→2.837s (+2.5%); as_of.n 291829→560996 ns (**+92% STOP** — pair1 head 892233 is a 3× outlier; pair2 229758 vs 301341). Worktrees removed.

**Gates:** `fmt --check` ok; clippy workspace all-targets offline 0 (no new); `cargo test --workspace --offline` ok; rocks `m002_rock00` 3 + `m002_rock01` 11; `kutha-gov py` / precommit / ci (0 HIGH, 65 checks, selftest 65/65) ok; pytest 181 (`tmp/p60-pytest.log`). **Open:** S06 `DUR-export-race`/`DUR-rocks-mid-stale`/`DUR-bytes-per-fact`; no external chain; `open` O(N); `Persister` not in `kutha-tenant`. ADRs stay Proposed. **as_of median STOP for controller.**
