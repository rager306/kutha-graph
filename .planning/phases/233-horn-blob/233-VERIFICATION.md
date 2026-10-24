---
phase: 233-horn-blob
independent: true
status: pass-with-findings
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b7_horn_blob_kill
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-common/src/rule.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-common/src/pin_migrate.rs
---

# Phase 233 — independent verification

**Verdict: PASS-WITH-FINDINGS.** Missing blob → `horn_behavior_reproducible` `false` → `replay_check` `ExecutionReplayFailed`. One-line `eval.rs` (`!is_empty()` → `false`). No new Op. CBM `kutha-graph`: `eval.rs` `metadata_changed`, kill file `not_tracked`; source read. No `index_repository`.

## Claims

| id | claim | result |
|----|--------|--------|
| a | missing nonempty pin → ExecutionReplayFailed | PASS `kill_missing_nonempty_pin_is_execution_replay_failed` |
| b | empty pin → ExecutionReplayFailed | PASS `kill_empty_pin_is_execution_replay_failed` |
| c | present Horn match → Ok | PASS `kill_present_horn_match_is_ok` |
| d | present Named body → skip Ok | PASS `kill_present_non_horn_body_skips_ok` |
| e | garbage not RuleAst → fail closed | PASS `kill_garbage_bytes_not_rule_ast_fail_closed` |
| f | no new Op | PASS 954131a `src/` is only `eval.rs` |

## Fixture diffs

OK, not HIGH. Nine tests (m011, m071, m072, m075, m076, m078, m091, m134, m135) insert Named `derive_pq` blobs so `open`/`replay_check` survive fail-closed. `remap_event_pins` still rewrites only Behavior pins; remapped_lines / cite / CLI oracles unchanged. m011 still fingerprint-equal + provenance-unequal; r2 Named is skip because `derive_pq` is Named; the loop now retargets only `derive_pq` so inverse_knows Horn still runs.

## Findings

1. **LOW** First `kutha-gov ci --jobs 1` HIGH `disk-free` (0.20 GiB). Host, not product. After reclaiming `/tmp` session scratch, rerun **0**, `run_high=0`.
2. **LOW** Named fixtures take the skip arm. They still prove pin migrate / cite remap / CLI.

## Commands

- `cargo test -p kutha-runtime --offline --test b7_horn_blob_kill` **0** (5/5)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`

In-memory kill-tests; `tmp/` empty. No production edit.
