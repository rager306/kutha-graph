---
phase: 164-migrate-cli-contract
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m093_migrate_cli_contract --test m078_migrate_cli -- --nocapture
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline
    exit: 0
  - cmd: uv run kutha-gov precommit --check test-matrix --check docs-coupling --check changelog-planes --check cite-lease --check state-readme
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/164-migrate-cli-contract/164-CONTEXT.md
  - crates/kutha-runtime/src/bin/kutha-migrate.rs
  - crates/kutha-runtime/tests/m093_migrate_cli_contract.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 164 — independent verification

**Verdict:** passed. CLI help/extra-arg contract green; workspace tests green;
quiet bench `ingest_plain.n` ≈101.8M ns / `as_of.n` ≈241.2k ns; full `ci`
tip_high=0 after close.
