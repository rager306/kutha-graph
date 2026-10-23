---
phase: 230-fork-diff
independent: true
status: failed
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --no-fail-fast
    exit: 101
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 101
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 1
read_only_inputs:
  - docs/ADR/ADR-061-fork-and-diff.md
  - crates/kutha-runtime/src/fold_diff.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/b3_checkpoint_cut.rs
  - crates/kutha-runtime/tests/common/mod.rs
  - docs/process/gsd-auto-contract.md
---

# Phase 230 — independent verification

**Verdict: FAIL.** `diff_folds` / `GraphFold::diff` is a `facts()` query (ADR-061 Evidence 2026-10-23). CBM: `fold_diff.rs` `not_tracked`; source read. No product edit.

## Claims

| id | claim | result |
|----|--------|--------|
| a | totality: apply_diff(a, diff(a,b)) = b modulo seq | PASS `kill_totality_mirror_aa_random_pairs` |
| b | equal-key collisions + full duplicates | PASS `kill_equal_keys_full_duplicates_and_rank_not_insertion` |
| c | shuffled order / build path → same picture | PASS except Option sentinel (HIGH) |
| d | seq-only empty; real field still changed | PASS `kill_seq_only_empty_but_real_field_change_is_not` |
| e | dead/expired rows in domain | PASS `kill_dead_rows_are_in_the_domain` |
| f | diff(a,a) empty; mirror | PASS (every random pair) |
| g | checkpointed fork vs plain twin (cut in tail) | PASS `kill_checkpointed_fork_matches_plain_twin` |
| h | fork_at n in 1..=c Err; diff unreachable | PASS `kill_fail_closed_cut_never_reaches_diff` |
| i | mutants | seq-in-identity and insertion-rank would fail (b)/(d); live code is the `vt_ord` mutant |

## Findings

1. **HIGH** `kill_open_interval_is_not_u64_max_identity` — `identity_key` / `canon_cmp` map `valid_to` with `unwrap_or(u64::MAX)`, so `None` and `Some(u64::MAX)` share a key. Diff is `changed` (not add+remove). `Fact::is_live_at(ValidTime::MAX)` still distinguishes them.
2. **HIGH** `kill_invalidated_at_none_vs_max_rank_is_total` — same sentinel on `invalidated_at` ranks two distinct rows equal; insertion order leaks; same bag shuffled yields two `changed`.
3. **LOW** workspace `m019_dur_a` WouldBlock under parallel (`kutha.lock`); serial `--test-threads=1` 19/19. Not a `diff_folds` defect.

## Commands

- `cargo test -p kutha-runtime --offline --no-fail-fast` **101** (only `b4_fork_diff_kill` 6 pass / 2 fail).
- `cargo test --workspace --offline --no-fail-fast` **101** (same two + `m019_dur_a` flake).
- `uv run kutha-gov ci --jobs 1` **1**, `run_high=2` (observe_cargo 101 on the kill file; incomplete). `test-matrix` / docs-coupling 0.

No `index_repository`. PersistScratch RAII; `tmp/` empty.
