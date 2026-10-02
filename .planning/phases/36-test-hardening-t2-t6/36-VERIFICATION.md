---
independent: true
verifier_model: composer-2.5
status: passed
score: 5/5
exit: pass
commands:
  - cmd: "cargo test -p kutha-common --offline --test ui_clock_mix"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test m016_cite_tt_vt --test m016_known_on_hardening --test ff5_legal_pit --test m014_as_known_on"; exit: 0
  - cmd: "rg -F 'error[E0308]' crates/kutha-common/tests/ui/clock_mix.stderr"; exit: 0
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/36-test-hardening-t2-t6/36-CONTEXT.md
  - crates/kutha-common/tests/ui/clock_mix.rs
  - crates/kutha-common/tests/ui/clock_mix.stderr
  - crates/kutha-runtime/tests/m016_cite_tt_vt.rs
  - crates/kutha-runtime/tests/m016_known_on_hardening.rs
  - .kutha/STATE.md
---

# Phase 36 — independent verification

**ROADMAP SC:** TH-01..TH-05.

| REQ | Status | Evidence |
|-----|--------|----------|
| TH-01 | satisfied | trybuild `clock_mix_is_e0308`; stderr has `error[E0308]` for Tt and WallTime → ValidTime |
| TH-02 | satisfied | `swapped_justification_cite_tt_vt_fails_equality` |
| TH-03 | satisfied | Eager==Lazy `as_known_on`; fork_at before later anchor differs + persist/open |
| TH-04 | satisfied | mid-cut snapshot + non-empty tail Assert/TtAnchor recovers known-on |
| TH-05 | satisfied | `graph_fold_replay_as_known_on_matches_live` (bind_reserved on replay) |

**Score:** 5/5. FF5/M014 green. D-04: no rules. Fingerprint still facts-only (documented residual).
