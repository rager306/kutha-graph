---
phase: 232-fork-query
independent: true
status: pass-with-findings
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b6_fork_query_kill
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 101
  - cmd: cargo test -p kutha-runtime --offline --test b1_checkpoint_kill3 -- --test-threads=1
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/fold_diff.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - docs/ADR/ADR-061-fork-and-diff.md
---

# Phase 232 — independent verification

**Verdict: PASS-WITH-FINDINGS.** `Runtime::diff` is `diff_folds(self.fold(), other.fold())` (ADR-061 glue). CBM: `fold_diff.rs` `metadata_changed`, kill file `not_tracked`; source read. No `crates/*/src` edit. No `index_repository`.

## Claims

| id | claim | result |
|----|--------|--------|
| a | `Runtime::diff` equals `diff_folds`; `diff(a,a)` empty; mirror | PASS `kill_runtime_diff_equals_diff_folds_aa_empty_and_mirror` |
| b | seq-only empty via the method; real field still changed | PASS `kill_seq_only_empty_via_the_method` |
| c | Retract is changed not removed; Scan drops; fork keeps | PASS `kill_retract_is_changed_not_removed_and_scan_drops` |
| d | expired/dead rows in diff; Scan does not | PASS `kill_expired_rows_stay_in_diff_not_in_scan` |
| e | checkpoint `1..=c` Err; `0` and `n>c` Ok | PASS `kill_checkpoint_legal_cut_vs_interior_err` |
| f | Prepared from one runtime is not the other's picture | PASS `kill_prepared_from_one_runtime_is_not_the_other_picture` |
| g | unknown relation fail-closed on fork; Scan still ok | PASS `kill_unknown_relation_fail_closed_on_fork_scan_still_ok` |

## Findings

1. **LOW** Prepared is a fold snapshot with no Runtime identity. `execute` always re-prepares. Holding `prepare(parent)` while reading the fork is a caller misuse; the API cannot silently rebind. No API change.
2. **LOW** workspace parallel: `b1_checkpoint_kill3::r5_recheckpoint_leaves_exactly_one_receipt` WouldBlock on `kutha.lock`. Serial `--test-threads=1` 12/12. Not a `Runtime::diff` defect. `m019_dur_a` 19/19 in the same run.

## Commands

- `cargo test -p kutha-runtime --offline --test b6_fork_query_kill` **0** (7/7)
- `cargo test --workspace --offline --no-fail-fast` **101** (finding 2 only)
- serial `b1_checkpoint_kill3` **0**
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`

In-memory kill-tests; `tmp/` empty.
