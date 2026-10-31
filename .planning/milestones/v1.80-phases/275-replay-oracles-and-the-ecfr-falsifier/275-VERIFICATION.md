---
phase: 275-replay-oracles-and-the-ecfr-falsifier
independent: true
status: passed
verified_at: 2026-10-31
verifier: fresh subagent, round 3 (no part in the implementation)
verifier_model: cline/cline-pass/deepseek-v4.1-flash-1017
commands:
  - cmd: "cargo test -p kutha-runtime --test m180_replay_meta --test m167_ecfr_pit --no-fail-fast (11-binary targeted run, this round; 74 passed / 0 failed)"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --test m180_replay_meta (mutant: empty source_event_ids, reason asserts kept; round 2 record, not re-run here)"
    exit: 101
  - cmd: "cargo test -p kutha-runtime --test m180_replay_meta (mutant: state guard dropped, then version equality dropped; round 2 record)"
    exit: 101
  - cmd: "cargo test -p kutha-runtime --test m167_ecfr_pit (mutant: lost edition, then gained edition, then runtime-side loss; round 2 record)"
    exit: 101
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/tests/m180_replay_meta.rs
  - crates/kutha-runtime/tests/m167_ecfr_pit.rs
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/tests/fixtures/ecfr/304.1_2015-12-18.txt
  - crates/kutha-runtime/tests/fixtures/ecfr/304.1_2017-01-23.txt
  - crates/kutha-runtime/tests/fixtures/ecfr/304.1_2017-03-14.txt
  - docs/architecture/ecfr-edition-ambiguity.md
---

# Phase 275 verification — replay oracles and the eCFR falsifier

**Verdict: `passed`** on TEST-275-01, -02, -03. The tests themselves are read
and checked this round; the mutant exits above are round-2 records, labelled as
inherited and not re-run.

**TEST-275-01.** Both enrichment tests now build `source_event(0)` and assert
the named reason: "not installed" and "cache key". Without a real source the
runtime rejects earlier with "source_event_ids is empty", so the asserted
reasons are only reachable with a real source.

**TEST-275-02.** `activate_with_wrong_version_fails_replay`,
`second_activate_fails_replay` and `second_retire_fails_replay` each assert the
`PackLifecycleRejected` variant, `pack_id` and the exact reason. Round 2 showed
each fails for its own mechanism (state guard vs version equality), not for a
shared earlier check.

**TEST-275-03.** The redundant count assert is gone; m167 keeps
`assert_eq!(edition_text(0), edition_text(1))` (gained edition) and
`assert_ne!(edition_text(1), edition_text(2))` (lost edition), plus the AS_OF
cut asserts that catch a lost edition on the runtime path. Both new tests pass
13/13 and 1/1 here.

**Scope.** No product behaviour change; the verifier edited nothing under
`crates/`. `cargo test --workspace` is inherited from round 2 (exit 0 after a
rerun; run 1 flaked in the pre-existing `m019_dur_a` writer-lock case) and was
not run by this verifier.
