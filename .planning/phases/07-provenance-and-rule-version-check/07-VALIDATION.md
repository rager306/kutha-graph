---
phase: "07"
slug: "provenance-and-rule-version-check"
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-09-30"
---

# Phase 7 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` (lib + integration) |
| **Config file** | workspace `Cargo.toml` / crate defaults |
| **Quick run command** | `cargo test -p kutha-runtime --offline --test m011_provenance` |
| **Full suite command** | `cargo test --workspace --offline` && `uv run kutha-gov ci` |
| **Estimated runtime** | ~60–120 seconds |

---

## Sampling Rate

- **After every task commit:** `cargo test -p kutha-runtime --offline --test m011_provenance` (once file exists)
- **After every plan wave:** Full suite + `kutha-gov ci`
- **Before `/gsd-verify-work`:** Full suite green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|----------------|-----------------|-----------|-------------------|-------------|--------|
| 07-W0 | 01 | 0 | PROV-01/02 | — | N/A | stub | `test -f crates/kutha-runtime/tests/m011_provenance.rs` | ❌ W0 | ⬜ pending |
| 07-PROV-01 | 01 | 1 | PROV-01 | T-07-01 | Swap valid caused_by; state FP match; provenance fails | integration | `cargo test -p kutha-runtime --offline --test m011_provenance provenance_detects_caused_by_swap_when_state_fingerprint_matches -- --exact` | ❌ W0 | ⬜ pending |
| 07-PROV-02 | 01 | 1 | PROV-02 | T-07-02 | Change only rule_version; state FP match; provenance fails | integration | `cargo test -p kutha-runtime --offline --test m011_provenance provenance_detects_rule_version_change_when_state_fingerprint_matches -- --exact` | ❌ W0 | ⬜ pending |
| 07-GATE | 02 | 2 | GATE-01 | — | FSM + check + bridge; ci HIGH 0 | harness | `uv run kutha-gov ci` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/kutha-runtime/tests/m011_provenance.rs` — PROV-01 + PROV-02
- [ ] `Op::Behavior.rule_version` + `provenance_fingerprint` / `provenance_check` — product API
- [ ] FSM / checks / bridges needles — GATE-01

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
