---
phase: "06"
slug: "typed-csr-lease"
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-09-30"
---

# Phase 6 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` (lib + integration) |
| **Config file** | workspace `Cargo.toml` / crate defaults |
| **Quick run command** | `cargo test -p kutha-runtime --offline --test m011_typed_csr` |
| **Full suite command** | `cargo test --workspace --offline` && `uv run kutha-gov ci` |
| **Estimated runtime** | ~60–120 seconds |

---

## Sampling Rate

- **After every task commit:** `cargo test -p kutha-runtime --offline --test m011_typed_csr` (once file exists)
- **After every plan wave:** Full suite + `kutha-gov ci`
- **Before `/gsd-verify-work`:** Full suite green
- **Max feedback latency:** 120 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|----------------|-----------------|-----------|-------------------|-------------|--------|
| 06-W0 | 01 | 0 | CSR-01/02 | — | N/A | stub | `test -f crates/kutha-runtime/tests/m011_typed_csr.rs` | ❌ W0 | ⬜ pending |
| 06-CSR-01 | 01 | 1 | CSR-01 | T-06-01 | Typed edges preserve labels + multiplicity | integration | `cargo test -p kutha-runtime --offline --test m011_typed_csr typed_csr_preserves_relation_labels_and_support_multiplicity -- --exact` | ❌ W0 | ⬜ pending |
| 06-CSR-02 | 01 | 1 | CSR-02 | T-06-02 | Untyped dedup + FF5 path still hold | integration | `cargo test -p kutha-runtime --offline --test m011_typed_csr untyped_csr_neighbor_set_and_ff5_still_hold -- --exact` | ❌ W0 | ⬜ pending |
| 06-GATE | 02 | 2 | GATE-01 | — | FSM + check + bridge; ci HIGH 0 | harness | `uv run kutha-gov ci` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/kutha-runtime/tests/m011_typed_csr.rs` — CSR-01 + CSR-02
- [ ] `TypedCsrLease` / `TypedEdge` / `typed_csr_lease_at` — product API
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
