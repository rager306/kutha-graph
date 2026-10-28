---
phase: "05"
slug: "persisted-quantum-outcome"
# status lifecycle: draft (seeded by plan-phase) → validated (set by validate-phase §6)
# audit-milestone §5.5 distinguishes NOT-VALIDATED (draft) from PARTIAL (validated + nyquist_compliant: false) (#2117)
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-09-29"
---

# Phase 5 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` (lib unit + integration under `crates/kutha-runtime/tests/`) |
| **Config file** | workspace `Cargo.toml` / crate `Cargo.toml` (no pytest for product) |
| **Quick run command** | `cargo test -p kutha-runtime --test m011_quantum_outcome --offline` |
| **Full suite command** | `cargo test --workspace --offline` && `uv run kutha-gov ci` |
| **Estimated runtime** | ~60–120 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p kutha-runtime --test m011_quantum_outcome --offline` (once file exists) or `--lib` filters while scaffolding
- **After every plan wave:** Run `cargo test --workspace --offline` && `uv run kutha-gov ci`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|----------------|-----------------|-----------|-------------------|-------------|--------|
| 05-W0 | 01 | 0 | OUT-01/02 | — | N/A | integration stub | `test -f crates/kutha-runtime/tests/m011_quantum_outcome.rs` | ❌ W0 | ⬜ pending |
| 05-OUT-01 | 01 | 1 | OUT-01 | T-05-01 | Persist then open distinguishes Zero/Partial/Full; no forged Full | integration | `cargo test -p kutha-runtime --test m011_quantum_outcome budgets_0_1_2_distinguish_zero_partial_full_after_persist_open --offline -- --exact` | ❌ W0 | ⬜ pending |
| 05-OUT-02 | 01 | 1 | OUT-02 | T-05-02 | Crash after prefix → no Full; resume only via explicit record | integration | `cargo test -p kutha-runtime --test m011_quantum_outcome crash_after_prefix_has_no_terminal_success_until_explicit_resume --offline -- --exact` | ❌ W0 | ⬜ pending |
| 05-GATE | 02 | 2 | GATE-01 | — | FSM + check + bridge needles; ci HIGH 0 | harness | `uv run kutha-gov ci` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/kutha-runtime/tests/m011_quantum_outcome.rs` — stubs for OUT-01, OUT-02
- [ ] Runtime outcome buffer + `store` read/write of `quantum_outcomes.jsonl` — product surface under test
- [ ] FSM / checks / bridges needles — GATE-01

*Existing `budget_aborts_storm` / `cascade_idles_without_inverse_loop` cover RAM Partial/Full only — not OUT-01 persist.*

---

## Manual-Only Verifications

All phase behaviors have automated verification.

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 120s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
