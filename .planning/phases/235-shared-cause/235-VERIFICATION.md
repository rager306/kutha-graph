---
phase: 235-shared-cause
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
  - crates/kutha-runtime/src/hydrate.rs
---

# Phase 235 — independent verification

**Verdict: PASS-WITH-FINDINGS.** Shared EventId + distinct `caused_by` stores every cause; Retract drops that id from `derived_by_cause` under each. Single-cause still clears. No new Op (`kutha-common` untouched at e903a9d). Rebuild fingerprint unchanged. CBM `kutha-graph`: `metadata_changed` on pipeline/quantum/invariants; source read. No `index_repository`.

## Claims

| id | claim | result |
|----|--------|--------|
| a | both causes stored | PASS three-cause + rebuild |
| b | retract clears every cause | PASS executor + three-cause |
| c | same cause not stored twice | PASS `shared_head_duplicate_cause_is_stored_once` |
| d | single-cause unindex | PASS `shared_head_single_cause_retract_clears_that_list` |
| e | unknown Retract no panic | PASS `shared_head_unknown_retract_leaves_indexed_head` |
| f | rebuild keeps both | PASS `shared_head_rebuild_keeps_both_causes` |
| g | no Op / fingerprint move | PASS inspect + rebuild fingerprint |

## Findings

1. **LOW** `derived_by_cause` still `push`es a repeated `(id, cause)`. `cause_of_behavior` stays unique. Unindex `retain` drops every copy.

## Commands

- `cargo test -p kutha-runtime --offline --lib` **0** (28/28)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`

Indexes stay `pub(crate)`. `tmp/` empty. No production edit.
