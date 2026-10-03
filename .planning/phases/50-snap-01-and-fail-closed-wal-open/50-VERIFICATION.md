---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 7/7
exit: pass
commands:
  - cmd: cargo test --workspace --offline
    exit: 0
  - cmd: git diff 7e31784..HEAD -- Cargo.toml crates/kutha-runtime/Cargo.toml
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test m002_snap01 persist_leaves_no_tmp_files
    exit: 0
  - cmd: kill-mutations a b-prime b-double-prime in git worktree tmp/p50-verify
    exit: 101
read_only_inputs:
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/wal.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/tests/m002_snap01.rs
  - crates/kutha-runtime/tests/m017_replay_integrity.rs
  - crates/kutha-runtime/Cargo.toml
  - docs/ADR/ADR-012-snapshots-tiers-vacuum.md
  - docs/ADR/ADR-060-strict-replay.md
  - docs/architecture/benchmark-baseline.md
  - docs/process/gsd-auto-contract.md
  - CHANGELOG.md
  - .kutha/STATE.md
---
HEAD `71b6d00`. CBM stale (`store.rs` metadata_changed; `m002_snap01.rs` not_tracked). rg arbiter. Counter is thread-local (`store.rs` 31–46).

| claim | evidence | verdict |
|-------|----------|---------|
| 1 atomic sidecars + dir fsync | `replace_via_tmp` 154–175; writers; end `sync_dir` 131–133; `persist_leaves_no_tmp_files` `delta == 7` | pass |
| 2 prefix/unparseable/mid-line absent; non-prefix InvalidData | `load_jsonl_sidecar`; `sidecar_or_stale_prefix`; `mismatched_outcomes_sidecar_fails_open` | pass |
| 3 log-ahead tail; in-range extra fail; `replay_check` exact | `open` `check_tt_anchors_range(..., Some(next_tt))`; `check_tt_anchors` → `None`; `tampered_tt_anchor_wall_fails_replay_check` | pass |
| 4 WAL fail-closed | `recover_events` short header InvalidData; non-EOF propagate; jsonl fallback only InvalidData; `wal_header_only_*`/`wal_magic_only_*` | pass |
| 5 crash matrix A\|B; jsonl+ Ok(B) | `persist_until`; `crash_matrix_every_step_reopens_to_prefix` | pass |
| 6 scope | no Cargo.toml vs `7e31784`; persist order unchanged; ADRs **Proposed** | pass |
| 7 workspace | `cargo test --workspace --offline`: 202 passed, 0 failed (57 bins) | pass |

| mut | failing |
|-----|---------|
| (a) `write_terms` `File::create` | `persist_leaves_no_tmp_files` count=6 |
| (b') drop end `sync_dir` | same, count=6 |
| (b'') skip `sync_dir` after `terms.jsonl` rename | same, count=6 |
| (b–g prior) | unchanged from first pass |

D-06: not re-run (`cargo bench` not executed). Recorded medians in `docs/architecture/benchmark-baseline.md` Phase 50: ingest 116422669→116580979 (+0.1%), as_of 282387→286284 (+1.4%) — arithmetic ≤10% vs `7e31784`. Consecutive pair a/b (+23.7%/+15.9% ingest) is host noise (base drifted ~15%); controller interleaved re-measure. Residual: `bytes_per_fact` RSS 17218→25200 carried to Phase 52. Gaps: none on SNAP-01 oracles.
