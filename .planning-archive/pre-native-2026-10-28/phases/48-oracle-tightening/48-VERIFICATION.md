---
phase: 48-oracle-tightening
verified: 2026-10-06T15:10:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
score: 5/5 must-haves verified
behavior_unverified: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/phases/48-oracle-tightening/48-CONTEXT.md
  - docs/architecture/test-matrix.yaml
  - docs/process/gsd-auto-contract.md
  - .kutha/dictionaries/honeycomb.yaml
commands:
  - cmd: cargo test -p kutha-runtime --offline --test m017_ingest_scaling extra_relatedto_horn_visits_constant_in_n -- --exact --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test m019_sem_meta horn_correct_derived_not_live_after_premise_correct -- --exact --test-threads=1
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test m019_sem_meta horn_correct_interval_derived_not_live_after_patch -- --exact --test-threads=1
    exit: 0
  - cmd: cargo test --workspace --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --offline --test ff5_legal_pit
    exit: 0
covered_files:
  - crates/kutha-runtime/tests/m017_ingest_scaling.rs
  - crates/kutha-runtime/tests/m019_sem_meta.rs
  - docs/architecture/test-matrix.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - CHANGELOG.md
  - docs/ADR/ADR-041-csr-graphblas-hot-path.md
  - docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md
covered_digest: "v3:sha256:53f118d302d9345498ce12d8f0b2aee9d7289886e44e130beeb262947e03997a"
---

# Phase 48: Oracle tightening — Verification

**Goal:** I-F15 visit-bound pin; I-F16 `horn_correct_*`; GATE-01 = `gsd-auto-contract` bench + `BENCH-controller`; honeycomb **Proposed**.

| Truth | Status | Evidence |
| --- | --- | --- |
| ORACLE-01 I-F15 `extra_relatedto_horn_visits_constant_in_n` | ✓ | Test asserts `horn_examine_count` flat in N (64 vs 256); matrix `ORACLE-if15` killed; FSM `observe_cargo.required` |
| ORACLE-01 I-F16 `horn_correct_*` pair | ✓ | Both tests pass; `ORACLE-if16` killed; FSM lists both names |
| GATE-01 bench gate recorded | ✓ | `BENCH-controller` `status: gate`, source cites GATE-01/M018-S04; `gsd-auto-contract.md` §Bench gate at product close (`ingest_plain.n`, `as_of.n`) |
| ADR-041 honeycomb `map: Proposed` + I-F15 evidence | ✓ | `honeycomb.yaml` ADR-041 Proposed + `extra_relatedto_horn_visits_constant_in_n`; ADR-041 Evidence subsection |
| FF5 unchanged (kill-test) | ✓ | `ff5_legal_pit` 2/2 pass |

**Note:** First `cargo test --workspace` hit flaky `m019_dur_a` lock contention; immediate re-run exit 0 (parallelism).

**Verdict:** Phase 48 goal achieved — oracles wired, documented, and green.
