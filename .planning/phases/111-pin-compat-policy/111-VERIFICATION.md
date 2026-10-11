---
phase: 111-pin-compat-policy
independent: true
verifier_model: parent-session-fresh-pass
commands:
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: KUTHA_GOV_BUDGET=76 uv run kutha-gov ci
    exit: 0
    note: 0 HIGH expected after CHANGELOG; 76 checks including pin-compat-policy
  - cmd: KUTHA_BENCH_N=10000 taskset -c 0 cargo bench -p kutha-runtime --bench baseline -- --quick
    exit: 0
    note: three quiet runs; median ingest_plain.n ≈116.2M as_of.n ≈272.7k
read_only_inputs:
  - .planning/phases/111-pin-compat-policy/111-CONTEXT.md
  - docs/process/pin-compat.md
  - crates/kutha-common/src/event.rs
---

# Phase 111 — independent verification

**Verdict:** passed. Policy **documented rotation to pin scheme v1** in `docs/process/pin-compat.md`; `m051_pin_compat` pins three hardcoded digests; check `pin-compat-policy` green; budget 76; workspace cargo + `kutha-gov ci` 0 HIGH / 76 checks.

## Kill claims checked

1. Ident pin equals hardcoded Named AST vector (not recomputed-only equality).
2. Ident pin diverges from hardcoded legacy UTF-8 domain vector.
3. Non-ident pin equals hardcoded domain+UTF-8 vector.
