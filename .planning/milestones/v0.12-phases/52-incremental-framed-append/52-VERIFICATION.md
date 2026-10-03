---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 8/8
exit: 0
commands:
  - cargo test --workspace --offline (exit 0; 221 passed, 0 failed, 59 suites)
  - cargo test -p kutha-runtime --test m002_append01 --offline (exit 0; 19 passed)
  - cargo test -p kutha-runtime --offline --test m010_semantic_open --test m012a_verify_persist --test m013_genesis_log --test m013_fold_indexes (exit 0)
  - uv run kutha-gov precommit (exit 0; verification-independent OK)
  - kill-mutations c,d,f,i,j,k in main-tree src; git checkout -- after each
read_only_inputs:
  - crates/kutha-runtime/src/persist_append.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/wal.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/tests/m002_append01.rs
  - crates/kutha-runtime/benches/baseline.rs
  - crates/kutha-common/src/intern.rs
  - docs/architecture/benchmark-baseline.md
  - docs/process/gsd-auto-contract.md
  - CHANGELOG.md
  - .kutha/STATE.md
---
CBM `root-kutha-graph` stale (`persist_append.rs` not_tracked); source/rg arbiter. Benches **not** rerun (Phase 52 doc medians ingest −6.4% / as_of −1.5%). **D-01:** `hydrate.rs` `from_snapshot` now `dict.intern`s log `Op::Define` names after `snap.dict_strings` (needed so stale snapshot + appended Defines restore TermIds). `TermDictionary::intern` is a no-op for names already in the snapshot.

| claim | evidence | verdict |
| 1 prefix/sync/persist | append-only jsonl/WAL; one WAL `sync_data`; `persist` still `replace_via_tmp`; prefix/noop/one-sync oracles | pass |
| 2 tail+intern | O(last record) jsonl + WAL seek. Independent probe (intern-only; intern+assert; reuse interned object; unused intern; with/without checkpoint; open→attach→append): dict strings+TermIds, graph ids, fingerprint, as_of, `replay_check` match source and full persist | pass |
| 3 torn tails | newline-less jsonl dropped; complete bad line InvalidData; WAL torn drop; m017 oracles in workspace | pass |
| 4 checkpoint | tmp+dir fsync (delta=5); abort-before-rename keeps bytes; auto `CHECKPOINT_EVERY`/`close` | pass |
| 5 crash matrix | append/checkpoint failpoints reopen A or B or InvalidData | pass |
| 6 scope | crates: persist_append/store/wal/benches/tests + `lib.rs` + **hydrate.rs (D-01)**. No Cargo.toml. No existing test file except `m002_append01.rs` (add-only asserts). M010/M012a/M013 Define tests unchanged and green. Fresh snapshot: dict not grown/reordered | pass |
| 7 workspace | 221 passed; default build still no rocksdb (unchanged Cargo.toml) | pass |
| 8 bench | not rerun; `persist_append.n` still in 29-key list | pass |

| mutation | failing tests |
| c skip length | `append_tail_guard_rejects_truncated_or_garbage_logs` |
| d skip last-id | `append_tail_guard_rejects_same_length_last_record_swap` |
| f snapshot `File::create` | `checkpoint_abort_before_snapshot_rename_keeps_previous_bytes`, `checkpoint_leaves_no_tmp_and_fsyncs_dir` (delta 4≠5) |
| i skip durable_count/lengths | `append_twice_then_noop_keeps_prefix_and_matches_persist`, `intern_between_appends_survives_open_without_checkpoint` |
| j revert hydrate Define intern | `intern_between_appends_survives_open_without_checkpoint`, `intern_only_append_reattach_and_open` |
| k attach uses `encoded.last()` not disk tail | `intern_only_append_reattach_and_open` (WAL last frame length mismatch) |

Gaps: none on the intern/guard/checkpoint oracles re-checked this pass.
