# Phase 61-01 SUMMARY

Start `d71cd91`. Commits: `9d79c0c` docs(61-01) ROCK-02 note (a+b); `fb03636` test(61-02) RED oracles; `89ff743` feat(61-03) `m:v` 2 sealed-list + `verify_full`; `40f12f5` feat(61-04) stream frames; `93b49a3` fix(61-05) drop `log_has_quantum_outcome`; docs(61-06) matrix/ADR/CHANGELOG/bench (this file).

**ROCK-02 (a always-on, b opt-in):** `m:sealed` vs manifest; `verify_full` crc32 of `e:` key||value. Middle poison + matching tip: open keeps dir; `verify_full`/`get_event` `InvalidData`. Sealed-list tamper rebuilds. No log/segment/event format change.

**RED (fb03636 on d71cd91):** `middle_locator_poison_survives_open_or_rebuild` / sealed-list panic `locator segment missing` (seq=0 poison); `verify_full` missing. **GREEN pins:** three DUR-export-race tests (Phase 60 lock). GREEN after 61-03.

**MEM-01** (`GlobalAlloc`, N=500): ingest live HEAD=`7e31784`=867644 B. Persist peak 1506755→1312566 after stream + `split_events` `&[Event]` + `drop(events)` before snapshot. Remaining bpf = whole-bench RSS / N (ingest + persist transients + second Runtime on open). Inherent; no peak-live oracle.

**CLEAN-01 / D-05:** deleted `log_has_quantum_outcome`. Kept `wal::append_events` (wal unit test) + `rocks_spike`. Not wired (no O(1) lookup consumer; Rocks cold ~5.5 min).

**Graph (CBM inbound / GN upstream / rg arbiter).** Did not reindex (CBM lease-HEAD; GN 5 behind). `log_has_quantum_outcome` 0 / incoming.calls 0 (GN impact HAS_METHOD 1 stale) / rg crates 0. `append_events` 1 / 1 / rg 2. `export_jsonl` 1 / 16 (3 direct) / 3 files. `open_or_rebuild` 3 / 25 (3 direct) / 3 files. `write_segment_file` 28 / 11 (7 direct) / rg 5. `verify_full` CBM 0 / GN not found / rg 7.

**Kills** (scratch worktrees, fresh `CARGO_TARGET_DIR`): skip export lock → `export_jsonl_while_persister_holds_lock_fails_closed` rc=101. Stub `verify_full` Ok → `verify_full_rejects_poisoned_middle_locator` rc=101.

**Matrix:** killed 48, characterization 5, gap 3 (SEM S07), blocked 6, spec-only 4, gate 1, total 67.

**Bench** (`d71cd91` vs `93b49a3`, 6 pairs, warm-up discarded, `taskset -c 4`, N=10000): ingest 131.8M→129.6M ns (−1.62%); as_of 323262→303952 (−5.97%); open 3.323s→3.659s (**+10.10% STOP**). bpf median 21673→21138 B. Worktrees removed.

**Gates:** `fmt --check`; clippy workspace all-targets offline (no new); `cargo test --workspace --offline` (m019_dur_b 5); rocks `m002_rock00` 3 / `m002_rock01` 11 / `m019_rocks_b` 3; `kutha-gov py` / precommit / selftest 65/65 / ci (warm cargo then rocks); pytest 181 (`tmp/p61-pytest.log`). **Open:** SEM S07 gaps; `open` O(N); no external chain; bpf RSS inherent; ADRs Proposed. **open.n STOP for controller.**
