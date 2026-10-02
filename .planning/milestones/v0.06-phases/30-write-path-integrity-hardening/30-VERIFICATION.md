---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 4/4
commands:
  - cmd: "cargo test --workspace --offline"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test ff6_allowlist --test m013_genesis_log --test m012_allowlist_facts --test m012_admission_facts --test m012_action_record --test m012a_idempotent_ingest --test m012_rule_registry --test m013_dictionary_facts"; exit: 0
  - cmd: "rg 'pub fn emit(&mut self, op: Op)' pipeline.rs:415; no extra emit arg; src/*.rs ≤800; git diff --exit-code STATE.md example statute_as_of + named ff5/ff6/m010–m013 oracles"; exit: 0
  - cmd: "RED 22e6688 public_emit_assert_kutha_allows + keyed_correct + keyed_correct_interval"; exit: 101
  - cmd: "scratch User Correct+CorrectInterval genesis kutha:allows MetaOpRejected; Retract admitted"; exit: 0
  - cmd: "kill apply delivery_key=None; keyed_correct*"; exit: 101
  - cmd: "kill Lazy collect_live_facts_at empty; dual_mode_ff5_retract_correct_claim_agree"; exit: 101
  - cmd: "kill facade assert_eq→assert_ne T_OLD t50; statute_as_of_via_facade"; exit: 101
  - cmd: "KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline ingest_plain.n=120587606 as_of.n=309576"; exit: 0
  - cmd: "uv run kutha-gov precommit"; exit: 0
  - cmd: "uv run kutha-gov py"; exit: 0
  - cmd: "uv run kutha-gov precommit --check verification-independent"; exit: 0
  - cmd: "uv run kutha-gov precommit --check artifact-budget"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/30-write-path-integrity-hardening/30-CONTEXT.md
  - .planning/phases/29-close-out-honesty/29-VERIFICATION.md
  - docs/architecture/benchmark-baseline.md
  - CHANGELOG.md
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - crates/kutha-runtime/tests/m013_dual_mode.rs
  - crates/kutha-runtime/tests/facade_statute.rs
  - crates/kutha-runtime/examples/statute_as_of.rs
  - .kutha/STATE.md
---

# Phase 30 — independent verification

**ROADMAP SC:** (1) public `kutha:` write refused; RED before GREEN. (2) keyed assert→correct→retry support 1. (3) FF5/retract/correct/claim Eager=Lazy. (4) facade exact triples after assert/correct/retract/open/fork.

| REQ | Status | Evidence |
|-----|--------|----------|
| HARD-01 | satisfied | `admission.rs:83-96` User Assert/Correct/CorrectInterval reserved → `MetaOpRejected`; Retract omitted (D-02); `emit_allow` Dictionary; `emit` sig unchanged; scratch Correct/Interval refused + Retract ok; RED `22e6688` `unwrap_err` on `Ok`; FF6/genesis/dictionary/rule/admission oracles ok; the summaries + CHANGELOG:41 + CONTEXT D-02 agree Retract exception |
| HARD-02 | satisfied | `fold.rs` Correct `572/587` + CorrectInterval `618/635` copy `prior_key`; `live_support_count` 1 both ops; kill `None` → `live_keyed` 0 |
| HARD-03 | satisfied | `m013_dual_mode.rs:121` FF5 shape, retract, correct, `claim_supported_at`; no `ensure_fold_indexes`; kill empty Lazy scan fails `contains` 50-kmh |
| HARD-04 | satisfied | `facade_statute.rs` 14×`assert_eq!` sorted triples incl. persist/open/`fork_at`; example diff empty; kill `assert_ne` left==right |

**Score:** 4/4. D-08 vs P27 `120267790`/`511233` (caps `132294569`/`562356`): this run `120587606` (+0.3%) / `309576` (faster); executor `126962516` / `288999` — ingest noise ~5%, `as_of` ~0.3 ms. CBM stale (`metadata_changed`; dual_mode `not_tracked`); GitNexus emit 0 / apply 5 CRITICAL, 6 commits behind; `rg` arbiter (`self.admit(` pipeline 3). Scratch worktrees removed. The plans/the summaries were not command evidence.
