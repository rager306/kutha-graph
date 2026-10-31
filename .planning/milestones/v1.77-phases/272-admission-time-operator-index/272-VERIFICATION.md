---
phase: 272-admission-time-operator-index
independent: true
status: passed
verifier_model: codebuddy/deepseek-v4.1-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m184_admission_index --test m183_digest_falsifiers --test m182_meta_threats --test m177_pack_manifest
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/tests/m184_admission_index.rs
  - .kutha/STATE.md
---

# Phase 272 verification — admission operator index

The first independent verifier **blocked** this slice and was right: `hydrate` collected every `PackRetire` and applied them only after the install loop, while `commit_event` applied retire inline. A legal `install a → activate → retire a → install b{shared}` rebuilt with `a` still owning `shared`, so a store the runtime had accepted could fail to reopen — reproduced on `open`, `fork_at`, `open_for_append`, and after `checkpoint_sealed_prefix`.

Fixed before close: retire is applied inline in log order; a constructor that skips `replay_check` marks the leases untrusted and admission fails closed; `retired_name_reuse_survives_every_reopen_path` covers live, open, fork, and checkpoint; the two operator views are documented and pinned by a test.

The same verifier also measured that admission still rebuilt the lifecycle tracker per install (213 µs → 2.33 ms from 400 → 3200 installs). That is now a runtime lease with a non-mutating `check`, and measured per-install cost is flat (255 µs → 287 µs).

Honest limits: the linearity guard is a source-shape assertion plus the recorded measurement, not a wall-clock assertion (the wall-clock version flaked under a parallel workspace run). `Op::EnrichmentProposal` admission still replays the lifecycle; this milestone covers pack writes.

Final gate: admission index 10/10, workspace 826 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0 with `run_high=0 run_low=0`.
