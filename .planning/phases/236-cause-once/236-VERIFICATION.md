---
phase: 236-cause-once
independent: true
status: pass-with-findings
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --lib
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/quantum_invariants.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-common/src/event.rs
---

# Phase 236 — independent verification

**Verdict: PASS-WITH-FINDINGS.** `record_derived_by_cause` skips an id already in that cause list. Both `append_indexed` and `index_live_behavior_event` call it. Same (id, cause) twice → one entry. Two causes stay length 1 after repeating only one. Sibling heads under one cause survive a repeat. Rebuild does not duplicate; fingerprint unchanged. Retract after a duplicate index leaves both lists empty of the head. No new Op (`kutha-common` untouched at 4558323). Indexes `pub(crate)`. CBM `kutha-graph`: `metadata_changed` on pipeline/invariants; source read. No `index_repository`.

## Claims

| id | claim | result |
|----|--------|--------|
| a | same cause once | PASS executor + two-cause repeat |
| b | two causes length 1 after repeat of one | PASS `derived_by_cause_two_causes_stay_len_one_after_repeat_index` |
| c | retract clears every cause | PASS `derived_by_cause_retract_after_duplicate_index_clears_every_cause` |
| d | rebuild via `index_live_behavior_event` | PASS `derived_by_cause_rebuild_index_live_does_not_duplicate` |
| e | no Op / fingerprint | PASS inspect + rebuild fingerprint |

## Findings

1. **LOW** `shared_head_duplicate_cause_is_stored_once` comment still says `derived_by_cause` may list the head twice; the body still asserts `>= 1`.

## Commands

- `cargo test -p kutha-runtime --offline --lib` **0** (33/33)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`

No production edit. `tmp/` empty.
