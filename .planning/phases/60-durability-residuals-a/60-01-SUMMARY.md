# Phase 60-01 SUMMARY

Start `1f9fb14`. Commits: `e5bf5dc` docs(60-01) GROUP-01 (c); `6d36a34` test(60-02) RED oracles; `97430c5` feat(60-03) `kutha.lock`; docs(60-04) matrix/ADR/CHANGELOG + clippy `truncate(false)`; fix(60-05) Persister poison; docs(60-05) honesty + residuals (this file).

**GROUP-01 (c):** no Event/Op/Snapshot/frame change. (a) needs a group-length header; (b) needs an Op marker. Hard-stop not taken.

**RED on lease HEAD (bugs, then fixed):** `two_writer_attach_fails_closed`, `second_writer_names_kutha_lock`, `persist_migrate_race_serialized_or_fails_closed`, `lock_file_ignored_by_dir_listings`. **Fix-round RED (642ddd3):** `persister_is_poisoned_after_failed_append_and_cannot_write_without_lock` — after tail-guard `InvalidData` the lock is released and the live object could still write. **Pinned (not bugs):** omitted-QO snapshot-ahead, jsonl inode, manifest `sync_all`, final `sync_dir`, mixed SelfCommit prefix (characterization), migrate tail_guard, exact frame-end prefix, wrong-role, chain prev, pre-rename leftover, sealed trailer lie, reader-while-writer, dead-process lock, panic Drop.

**Lock:** `kutha.lock`, `File::try_lock`, `WouldBlock` + message names the file; persist/migrate/export/attach/`open_for_append`; readers unlocked; dead-process does not block; panic Drop; listings ignore the file (only mechanical listing edit). Failed append/checkpoint releases so heal can run; attach validates before lock. Fail-closed `append` (`ErrorKind::InvalidData`) poisons that object — later `append`/`checkpoint`/`close` return `InvalidData` and write nothing; heal is a new `open_for_append`. Failpoints stay `Other` and do not poison (`checkpoint_omitted_qo`, `m002_commit01` heal unchanged).

**Graph (CBM inbound / GN upstream impact / rg arbiter 46 files, 338 hits):** attach 52 / 52 (50 direct); persist 151 / 146 (105); migrate 4 / 4; export_jsonl 1 / 1; open_for_append 2 / 2. GN `acquire_store_write_lock` missing (index 3 commits behind). CBM `store.rs`/`persist_append.rs` `metadata_changed`; `m019_dur_a.rs` `not_tracked`. Did not reindex.

**Kills** (`tmp/p60-run-mutations.sh` then `tmp/p60-run-fix-mutations.py`, unique `tmp/p60-td-*`): S05 named tests FAILED (rc=101). role: sequence-only **survives**; yaml layered **kills**. lie: yaml `return scan+expected_prev` without `read_active` prev **survives** (later sealed / active chain-link; this store has 9 sealed files); layered skip sealed trailer checks + do not propagate `scan.chain` + skip sealed and `read_active` prev **kills** (hydrate not required). poison: `do not poison` **kills**. Single-check removal is covered by neighbouring checks.

**Matrix:** killed 46, characterization 4, gap 6, blocked 6, spec-only 4, gate 1, total 67.

**Bench** (2 interleaved pairs, worktrees `1f9fb14` vs `97430c5`, `KUTHA_BENCH_N=10000`; cleaned): ingest_plain.n 114.5M→117.1M ns (+2.3%); open.n 2.768s→2.837s (+2.5%); as_of.n 291829→560996 ns (**+92% STOP** — pair1 head 892233 is a 3× outlier; pair2 229758 vs 301341). Worktrees removed.

**Gates:** `fmt --check` ok; clippy workspace all-targets offline 0 (no new); `cargo test --workspace --offline` ok; rocks `m002_rock00` 3 + `m002_rock01` 11; `kutha-gov py` / precommit / ci (0 HIGH, 65 checks, selftest 65/65) ok; pytest 181. **Open:** S06 `DUR-export-race`/`DUR-rocks-mid-stale`/`DUR-bytes-per-fact`; no external chain; `open` O(N); no fsync of the `kutha.lock` dirent; advisory local lock only (NFS / other host not covered); writes that bypass `Persister`/`store::*` not covered; `kutha-tenant` only through `store::persist`. ADRs stay Proposed. **as_of median STOP for controller.**
