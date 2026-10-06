---
phase: 47-horn-determinism-closure
verified: 2026-10-06T12:35:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
score: 7/7 must-haves verified
behavior_unverified: 0
read_only_inputs: [47-CONTEXT.md, REQUIREMENTS.md DET-02/HORN-CORR-01/CASC-01, test-matrix.yaml SEM-det02-ids SEM-horn-correct SEM-casc-admit; excluded 47-*-PLAN.md and 47-*-SUMMARY.md]
commands:
  - cmd: cargo test -p kutha-runtime --test m019_sem_meta det02_two_ingest_derived_event_ids_equal -- --exact
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m019_sem_meta horn_correct --
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m019_sem_meta casc01 --
    exit: 0
  - cmd: cargo test -p kutha-runtime --test ff5_legal_pit --
    exit: 0
covered_files: ["crates/kutha-common/src/event.rs", "crates/kutha-runtime/src/eval.rs", "crates/kutha-runtime/src/pipeline.rs", "crates/kutha-runtime/tests/m019_sem_meta.rs", "docs/ADR/ADR-013-bitemporal-facts-invalidation.md", "docs/architecture/test-matrix.yaml"]
covered_digest: "v3:sha256:2176afabc7b1dff8df0a87bb40940d8ff9c3d1fe78cf54f02a6b3d0b3d9b5990"
---

# Phase 47: Horn determinism closure — Verification

**Goal (47-CONTEXT):** replay-stable derived ids, Correct re-derive, cascade admit every hop. **Status:** passed · **7/7**

| Truth | Status | Evidence |
| --- | --- | --- |
| DET-02 two-ingest derived ids equal; user Assert v7 | ✓ | `Event::stable_derived` + `Event::new`→`now_v7`; test `det02_two_ingest_derived_event_ids_equal` |
| HORN-CORR Correct/CorrectInterval re-derive | ✓ | `eval.rs` `rule_follow_ons` retract+`horn_fires`; horn_correct_* tests |
| CASC-01 hop admit; unknown head Err | ✓ | `pipeline.rs` admit before push; `UnknownRelation` in `eval.rs`; casc01_* tests |
| FF5 oracle regression | ✓ | `ff5_legal_pit` 2/2 pass |
| ADR-013 honeycomb Proposed | ✓ | ADR-013 header + Phase 47 evidence still **Proposed** |
| Matrix pins killed | ✓ | SEM-det02-ids, SEM-horn-correct, SEM-horn-correct-char, SEM-casc-admit → status killed |

**Wiring:** `stable_derived` from `eval.rs` head/retract paths; cascade first event `Event::new` in `commit_user_cascade`. CBM index stale on touched paths (metadata_changed/not_tracked); claims from source + executed tests.

_Verifier: independent gsd-verifier (composer-2.5-fast)_
