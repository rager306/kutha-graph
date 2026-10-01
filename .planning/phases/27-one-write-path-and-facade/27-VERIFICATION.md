---
phase: 27-one-write-path-and-facade
verified: 2026-10-01T18:02:23Z
status: passed
score: 8/8 (D-07 controller override)
independent: true
verifier_model: composer-2.5-fast
behavior_unverified: 0
overrides_applied: 1
overrides:
  - must_have: "D-07 ingest_plain.n and as_of.n ≤10% vs Phase 22"
    reason: "After kutha:allows lease, ingest +26.1% (Phase 24 band); as_of +514% (fold walk). Absolute ≤10% not met; recorded in benchmark-baseline.md."
    accepted_by: "controller (bench honesty 2026-10-02)"
    accepted_at: "2026-10-02T00:00:00Z"
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/examples/statute_as_of.rs
  - crates/kutha-runtime/tests/facade_statute.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - docs/architecture/benchmark-baseline.md
commands:
  - cmd: "cargo build -p kutha-runtime --offline --example statute_as_of"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test facade_statute --test m013_write_path"; exit: 0
  - cmd: "wc -l crates/kutha-runtime/src/*.rs (max fold.rs 667)"; exit: 0
  - cmd: "rg 'fn append_meta' crates"; exit: 1
  - cmd: "cargo test -p kutha-runtime --offline --test ff5_legal_pit --test ff6_allowlist --test m013_genesis_log"; exit: 0
---

# Phase 27: One write path and facade — independent verification

**PIPE-01..04.** Inputs: REQUIREMENTS, public API, oracles, baseline.

## Kill-tests

| # | Result |
|---|--------|
| 1 example `statute_as_of` | build OK |
| 2 facade_statute + m013_write_path | 2/2 pass |
| 3 `src/*.rs` ≤800 | max `fold.rs` 667; `quantum.rs` 554 |
| 4 `fn append_meta` in crates | no matches |
| 5 ff5, ff6, m013_genesis_log | 8/8 pass |
| 6 bench vs Phase 22 | ingest +26.1%; as_of +514%; STOP documented (override) |

## Requirements

| ID | Status |
|----|--------|
| PIPE-01 | ✓ one pipeline; no `append_meta` |
| PIPE-02 | ✓ src split; no file >800 |
| PIPE-03 | ✓ public facade |
| PIPE-04 | ✓ statute example + integration |

**Gaps:** none under documented D-07 override. Absolute ≤10% vs Phase 22 remains false for `as_of.n`.
