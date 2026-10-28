---
phase: "04"
slug: "partial-correction-with-residual-intervals"
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-09-29"
---

# Phase 04 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded from `04-RESEARCH.md` § Validation Architecture.
> CONTEXT locks: D-C1…D-C7; inherits D-G1…G3, D-10/11/15 (D-C5).
> Requirements: CORR-01, CORR-02, GATE-01.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | (A) Rust libtest via `cargo test` (edition 2021 workspace); (B) `kutha-gov` FSM + pytest harness |
| **Config file** | per-crate `Cargo.toml`; `pyproject.toml` `[tool.pytest.ini_options]`; `.kutha/dictionaries/fsm.yaml` |
| **Quick run command** | `cargo test -p kutha-runtime --test m011_partial_correction --offline` (after 04-01-01 creates the file); `uv run kutha-gov precommit` when dictionaries change |
| **Full suite command** | `cargo test --workspace --offline` then `uv run kutha-gov ci` |
| **Estimated runtime** | tens of seconds–few minutes (warm cache) |

---

## Sampling Rate

- **After every task commit:** `cargo test -p kutha-runtime --test m011_partial_correction --offline` once the integration file exists; `uv run kutha-gov precommit` if harness YAML/docs touched
- **After every plan wave:** `cargo test --workspace --offline` on waves that touch `crates/` (D-15) and `uv run kutha-gov ci` plus `uv run kutha-gov explain trajectory` with D-10 four-part Trajectory in the wave SUMMARY (D-C5 / D-G1). HIGH stops (D-G2). WARN ids ledgered if LOW greater than 0 (D-11)
- **Before `/gsd-verify-work`:** `uv run kutha-gov ci` HIGH-free; WARN ledgered; `cargo test --workspace --offline`; GATE-01 needles registered (04-03)
- **Max feedback latency:** prefer under a few minutes with warm cache

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 04-01-01 | 04-01 | 1 | CORR-01 | T-04-01 T-04-03 T-04-04 | Interval-patch residuals at VT 2012/2021; Correct arm unused for leftovers; serde round-trip | integration | `test -f crates/kutha-runtime/tests/m011_partial_correction.rs && cargo test -p kutha-runtime --offline --test m011_partial_correction interval_patch_leaves_vt_2012_and_2021_residuals -- --exact` | ❌ W0 | ⬜ pending |
| 04-01-02 | 04-01 | 1 | CORR-01 | T-04-02 | Unknown / non-intersect / inverted / not-live interval patch does not append | integration | `test -f crates/kutha-runtime/tests/m011_partial_correction.rs && cargo test -p kutha-runtime --offline --test m011_partial_correction -- --test-threads=1` | ❌ W0 | ⬜ pending |
| 04-01-03 | 04-01 | 1 | CORR-01 | T-04-SC | Product changelog + workspace cargo + wave-close ci HIGH-free + D-10 Trajectory (needles not registered yet) | process | changelog greps + `cargo test --workspace --offline` + `uv run kutha-gov ci` + `uv run kutha-gov explain trajectory` + SUMMARY D-10 greps | ✅ CHANGELOG | ⬜ pending |
| 04-02-01 | 04-02 | 2 | CORR-02 | T-04-05 | Whole-version Correct invents no residuals at 2012/2021 | integration | `test -f crates/kutha-runtime/tests/m011_partial_correction.rs && cargo test -p kutha-runtime --offline --test m011_partial_correction whole_version_correct_does_not_invent_residuals -- --exact` | ✅ after 04-01 | ⬜ pending |
| 04-02-02 | 04-02 | 2 | CORR-02 | T-04-05 T-04-06 | Correct-arm single `facts.push` (Correct-through-CorrectInterval only) + changelog + cargo + wave-close ci HIGH-free + D-10 Trajectory | process | awk Correct-arm push count + changelog grep + `cargo test --workspace --offline` + `uv run kutha-gov ci` + `uv run kutha-gov explain trajectory` + SUMMARY D-10 greps | ✅ CHANGELOG | ⬜ pending |
| 04-03-01 | 04-03 | 3 | GATE-01 | T-04-07 | FSM required names + check needles + bridge (D-C6) | harness | grep counts on fsm.yaml / checks.yaml / bridges.yaml (comment-stripped) | ✅ YAML | ⬜ pending |
| 04-03-02 | 04-03 | 3 | GATE-01 | T-04-08 T-04-09 | Process changelog + ADR-013 evidence only; map stays Proposed | docs | CHANGELOG + honeycomb ADR-013 region + ADR markdown clean + `uv run kutha-gov precommit --check changelog-planes` + `uv run kutha-gov precommit --check docs-coupling` | ✅ | ⬜ pending |
| 04-03-03 | 04-03 | 3 | GATE-01 | T-04-07 T-04-SC | Final `ci` HIGH-free after needles; freeze/lease cites; D-10 in 04-03-SUMMARY | harness | `uv run kutha-gov ci` + `uv run kutha-gov explain trajectory` + `cargo test --workspace --offline` + Cargo.toml members + ROADMAP S04 unchecked + STATE S04 + STATE porcelain + SUMMARY D-10 greps | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky. Task IDs are 04-01-01…04-03-03. GATE-01 needle registration is 04-03; 04-01/04-02 still run `kutha-gov ci` at wave close (D-C5) against already-registered names.*

---

## Wave 0 Requirements

- [ ] `crates/kutha-runtime/tests/m011_partial_correction.rs` — created by 04-01-01 (CORR-01 oracle, serde round-trip, fail-closed cases in 04-01-02); 04-02-01 adds CORR-02
- [ ] `Op::CorrectInterval` + fold/emit/`Display` arms — 04-01-01 (CorrectInterval match arm immediately after Correct, before Define)
- [ ] `.kutha/dictionaries/fsm.yaml` required names + `checks.yaml` / `bridges.yaml` — 04-03-01 (not 04-01/04-02)
- [ ] `CHANGELOG.md` Product (04-01/04-02) then Process (04-03)
- [ ] Framework install: none

Existing M011/FF5 helpers are reusable; they do not cover observation 3. No new test stubs beyond the integration file above.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Emit CorrectInterval on a wide-VT fact for source a and see residual P at VT 2012 and 2021 and replacement at 2017; emit whole-version Correct on the same arrange and see no residuals at 2012/2021; confirm honeycomb ADR-013 still Proposed | CORR-01, CORR-02, GATE-01 | `workflow.human_verify_mode=end-of-phase`; 04-03-03 `<human-check>` | After 04-03 cargo+ci green: run the two named tests mentally or via cargo; `uv run kutha-gov map` / honeycomb ADR-013 `map: Proposed` |

Automated cargo oracles and `kutha-gov ci` remain the pass/fail gates. The human-check does not replace `<automated>`.

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency acceptable
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
