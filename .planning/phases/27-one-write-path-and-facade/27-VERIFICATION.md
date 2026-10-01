---
phase: 27-one-write-path-and-facade
verified: 2026-10-01T18:02:23Z
status: passed
score: 8/8 must-haves verified (ROADMAP SC4 / D-07 via controller override)
independent: true
verifier_model: composer-2.5-fast
behavior_unverified: 0
overrides_applied: 1
overrides:
  - must_have: "ROADMAP SC4 / D-07 — baseline re-run shows no regression over 10% (ingest_plain.n and as_of.n vs Phase 22)"
    reason: "After kutha:allows lease, ingest_plain.n is +26.1% vs Phase 22 (Phase 24 hybrid write-cost band). as_of.n remains +514% vs Phase 22 (documented single-shot fold walk; 63% below STOP). Absolute ≤10% gate is not met; controller accepts the same honesty pattern as Phase 24 D-06 — functional PIPE verified, regression recorded in benchmark-baseline.md § Phase 27 delta, not a silent pass."
    accepted_by: "controller (bench honesty 2026-10-02 per benchmark-baseline.md Phase 27 delta)"
    accepted_at: "2026-10-02T00:00:00Z"
covered_files:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/27-one-write-path-and-facade/27-01-PLAN.md
  - .planning/phases/27-one-write-path-and-facade/27-01-SUMMARY.md
  - .planning/phases/27-one-write-path-and-facade/27-02-PLAN.md
  - .planning/phases/27-one-write-path-and-facade/27-02-SUMMARY.md
  - .planning/phases/27-one-write-path-and-facade/27-CONTEXT.md
  - crates/kutha-runtime/examples/statute_as_of.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/facade_statute.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - docs/architecture/benchmark-baseline.md
covered_digest: "v2:sha256:1a1e95e87bdee57a553667b45e5d6f4b17155be51cb6c530814ebe59e24eca76"
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/27-one-write-path-and-facade/27-CONTEXT.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/examples/statute_as_of.rs
  - crates/kutha-runtime/tests/facade_statute.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - docs/architecture/benchmark-baseline.md
  # PLAN/SUMMARY in covered_files for GSD fingerprint only; not used to design kill-tests.
commands:
  - cmd: "cargo build -p kutha-runtime --offline --example statute_as_of"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test facade_statute --test m013_write_path"
    exit: 0
  - cmd: "wc -l crates/kutha-runtime/src/*.rs (max fold.rs 667)"
    exit: 0
  - cmd: "rg 'fn append_meta' crates"
    exit: 1
  - cmd: "cargo test -p kutha-runtime --offline --test ff5_legal_pit --test ff6_allowlist --test m013_genesis_log"
    exit: 0
---

# Phase 27: One write path and facade — Independent Verification

**Phase goal (ROADMAP):** A single propose, admit, commit, receipt pipeline and a public facade used by something other than in-crate tests.

**Verified:** 2026-10-01T18:02:23Z  
**Status:** passed  
**Re-verification:** No — initial independent verification  
**Inputs:** REQUIREMENTS PIPE-01..04, ROADMAP Phase 27, `27-CONTEXT.md`, runtime sources listed in `read_only_inputs` (no PLAN/SUMMARY for kill-test design).

## Kill-tests

| # | Command / check | Result | Status |
|---|-----------------|--------|--------|
| 1 | `cargo build -p kutha-runtime --offline --example statute_as_of` | build OK | ✓ |
| 2 | `cargo test -p kutha-runtime --offline --test facade_statute --test m013_write_path` | 2/2 tests pass | ✓ |
| 3 | `wc -l crates/kutha-runtime/src/*.rs` — all ≤800 | max `fold.rs` 667; `quantum.rs` 554 | ✓ |
| 4 | `rg 'fn append_meta' crates` | no matches | ✓ |
| 5 | Named oracles: `ff5_legal_pit`, `ff6_allowlist`, `m013_genesis_log` | 8/8 tests pass | ✓ |
| 6 | Bench vs Phase 22 (`benchmark-baseline.md` § Phase 27 delta) | ingest +26.1% after lease; as_of +514%; STOP documented | ✓ (override) |

## Goal achievement

### Observable truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | All record kinds use one propose→admit→commit→receipt path; no `append_meta` side path | ✓ VERIFIED | `pipeline.rs` `pipeline` / `pipeline_kind` / `pipeline_events`; `emit` → `pipeline`; typed constructors call `pipeline` or `pipeline_events`; `rg 'fn append_meta' crates` empty |
| 2 | No `crates/kutha-runtime/src/*.rs` file exceeds 800 lines | ✓ VERIFIED | `wc -l`: largest `fold.rs` 667 |
| 3 | Public facade + statute example/integration test on public API only | ✓ VERIFIED | `lib.rs` module split; example + `facade_statute` import only `kutha_runtime::{Runtime, RuntimeError}`; build + test green |
| 4 | `as_of(2015) ≠ as_of(2021)` plus admission and action in statute scenario | ✓ VERIFIED | `facade_statute` / `statute_as_of`: assert_ne on AS OF; `record_justification` + `record_action` |
| 5 | Unknown relation rejected consistently for Assert, justification, action | ✓ VERIFIED | `m013_write_path::unknown_relation_rejected_for_assert_justification_and_action` |
| 6 | Baseline ≤10% vs Phase 22 (ingest + as_of) | ✓ PASSED (controller override) | Documented in `benchmark-baseline.md`; absolute gate **not met** for as_of; ingest in Phase 24 band — see frontmatter `overrides` |
| 7 | PIPE-01..04 (REQUIREMENTS.md) | ✓ VERIFIED | Maps to truths 1–4 above |
| 8 | Named D-07 oracles unchanged (smoke) | ✓ VERIFIED | ff5, ff6, m013_genesis_log all green |

**Score:** 8/8 truths verified (1 via documented controller override)

### Required artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `src/pipeline.rs` | One write path (D-01) | ✓ VERIFIED | 456 lines; shared `pipeline*` + thin `emit` |
| `src/admission.rs` | Admit gates | ✓ VERIFIED | `admit` used from pipeline |
| `src/justification.rs` | Typed records → pipeline | ✓ VERIFIED | `record_*` → `pipeline_events(..., ReceiptMode::Typed)` |
| `src/quantum.rs` | Facade methods + module glue | ✓ VERIFIED | assert/retract/correct/derive/as_of/live_at/persist/open public |
| `examples/statute_as_of.rs` | D-04 consumer | ✓ VERIFIED | compiles |
| `tests/facade_statute.rs` | D-04 integration | ✓ VERIFIED | 1 test pass |
| `tests/m013_write_path.rs` | Shared admission oracle | ✓ VERIFIED | 1 test pass |

### Key link verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `quantum.rs` assert/retract/correct/derive | `pipeline.rs` | `self.pipeline(Op::…)` | ✓ WIRED | grep confirms |
| `pipeline.rs` `emit` | `pipeline` | `self.pipeline(op)` after `reject_forged_meta` | ✓ WIRED | |
| `justification.rs` record_* | `pipeline_events` | `ReceiptMode::Typed` | ✓ WIRED | |
| `emit_allow` / `emit_rule` / `emit_policy` | `pipeline_kind` | `ReceiptMode::Dictionary` | ✓ WIRED | dictionary path through same admit/commit |

### D-07 / Phase 27 bench (honesty)

| key | Phase 22 ref | Phase 27 (after `kutha:allows` lease) | vs Phase 22 |
|-----|-------------|----------------------------------------|-------------|
| ingest_plain.n | 95 383 904 ns | 120 267 790 ns | **+26.1%** (Phase 24 hybrid band) |
| as_of.n | 83 195 ns | 511 233 ns | **+514%** (documented; not ≤10%) |

First STOP run (+2168% ingest) is recorded; lease fix restored ingest scaling (2n/n 1.69 vs Phase 22 1.55). **Override on record** — not a silent waiver (same pattern as Phase 24 `24-VERIFICATION.md`).

### Behavioral spot-checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Facade statute AS OF | `cargo test --test facade_statute` | pass | ✓ PASS |
| Shared unknown-relation admit | `cargo test --test m013_write_path` | pass | ✓ PASS |
| Example builds | `cargo build --example statute_as_of` | pass | ✓ PASS |

### Requirements coverage

| Requirement | Description | Status | Evidence |
| ----------- | ----------- | ------ | -------- |
| PIPE-01 | One pipeline; no side append | ✓ SATISFIED | `pipeline.rs`; no `append_meta` |
| PIPE-02 | src split; no file >800 | ✓ SATISFIED | module layout in `lib.rs`; wc |
| PIPE-03 | Public facade surface | ✓ SATISFIED | statute consumer + public methods on Runtime |
| PIPE-04 | Example + integration statute scenario | ✓ SATISFIED | kill-tests 1–2 |

### Anti-patterns found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | none in cited phase paths | — | — |

### Human verification required

None — bench regression disposition is documented + controller override in frontmatter (explicit, not silent).

### Gaps summary

No blocking gaps for phase close under documented D-07 controller policy. Absolute ≤10% vs Phase 22 remains **false** for `as_of.n`; recorded in baseline and override.

---

_Verified: 2026-10-01T18:02:23Z_  
_Verifier: gsd-verifier (composer-2.5-fast, independent)_
