---
phase: 270-direct-falsifiers-for-manifest-and-payload-digests
independent: true
status: passed
verifier_model: codebuddy/deepseek-v4.1-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m183_digest_falsifiers --test m182_meta_threats --test m177_pack_manifest --test m180_replay_meta
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/m183_digest_falsifiers.rs
  - crates/kutha-runtime/src/pack_manifest.rs
  - crates/kutha-runtime/src/provenance.rs
  - .planning/milestones/v1.75-phases/269-meta-surface-threat-audit-and-adversarial-tests/269-SECURITY.md
  - .kutha/STATE.md
---

# Phase 270 verification — Direct falsifiers for manifest and payload digests

Independent verifier PASSed. The manifest falsifier was mutation-checked: deleting the raw SHA-256 guard fails exactly that test, and both digest tests assert the policy-hash premise passes the generic `StoreBlob` gate, so the rejection cannot come from the generic gate.

The indexed collision path is correct for collision and retire; the verifier found that a same-pack re-declare left stale ownership. That was unreachable (duplicate installs are rejected earlier) but is now fixed and covered by an `OperatorIndex` unit test. Admission semantics are unchanged: admission still uses `validate_install`/`declared_operators`, and the indexed path is replay-only with the same rule.

Honest limit recorded: admission itself remains O(P) per install; only replay is linear. The scaling test is tolerant rather than high-power.

Final gate: digest falsifiers 5/5, workspace 816 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0 with `run_high=0` (one LOW `observe-retried` breadcrumb, evidence only).
