---
phase: 269-meta-surface-threat-audit-and-adversarial-tests
independent: true
status: passed
verifier_model: codebuddy/deepseek-v4.1-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m182_meta_threats --test m180_replay_meta --test m178_enrichment --test m177_pack_manifest
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - .planning/phases/269-meta-surface-threat-audit-and-adversarial-tests/269-SECURITY.md
  - crates/kutha-runtime/tests/m182_meta_threats.rs
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/enrichment.rs
  - .kutha/STATE.md
---

# Phase 269 verification — meta-surface security

The independent verifier **blocked** the first attempt and was right: unifying replay on the incremental tracker dropped retired-pack operator release, so a log admission accepted could fail to reopen — a fail-closed denial, but it falsified T-269-08 and would have shipped a store that cannot open.

Fixed before close:

- replay now drops a retired pack from the collision set, matching admission;
- `admission_accepted_log_reopens_after_retire` and `replay_rejects_what_admission_rejects` cover both drift directions;
- enrichment replay uses the shared `check_proposal` instead of a copied 60-line check, and the dead `require_sources` parameter is gone;
- `declared_operators_for` errors are propagated instead of swallowed;
- the register now marks T-269-01 as linear-for-lifecycle but O(P²) for manifest collision, and marks T-269-02/T-269-07 as transitively covered rather than directly tested.

Final gate: threats 10/10, workspace 811 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0. One parallel `m019_dur_a` lock flake reproduced only under concurrent workspace runs and passes serially.
