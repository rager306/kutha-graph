---
phase: "08"
slug: "end-to-end-candidate-fixture"
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-09-30"
---

# Phase 8 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` (lib + integration) |
| **Config file** | workspace `Cargo.toml` / crate defaults |
| **Quick run command** | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture` |
| **Full suite command** | `cargo test --workspace --offline` && `uv run kutha-gov ci` |
| **Estimated runtime** | ~90–180 seconds |

---

## Sampling Rate

- **After every task commit:** `cargo test -p kutha-runtime --offline --test m011_e2e_fixture` (once file exists)
- **After every plan wave:** Full suite + `kutha-gov ci`
- **Before `/gsd-verify-work`:** Full suite green
- **Max feedback latency:** 180 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|----------------|-----------------|-----------|-------------------|-------------|--------|
| 08-W0 | 01 | 0 | FIX-* | — | N/A | stub | `test -f crates/kutha-runtime/tests/m011_e2e_fixture.rs` | ❌ W0 | ⬜ pending |
| 08-FIX-01 | 01 | 1 | FIX-01 | T-08-01 | Supports + conflict at named cuts | integration | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture e2e_fixture_supports_and_conflict_at_named_cuts -- --exact` | ❌ W0 | ⬜ pending |
| 08-FIX-02 | 01 | 1 | FIX-02 | T-08-02 | Justification cites; stale admission fails | integration | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture e2e_justification_cites_sources_and_rejects_stale_admission -- --exact` | ❌ W0 | ⬜ pending |
| 08-FIX-03 | 01 | 1 | FIX-03 | T-08-03 | Incremental matches reconstruct after lease discard | integration | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture e2e_incremental_matches_reconstruct_after_discarding_leases -- --exact` | ❌ W0 | ⬜ pending |
| 08-GATE | 02 | 2 | GATE-01/02/03 | — | FSM + check + bridge; ci HIGH 0; Proposed | harness | `uv run kutha-gov ci` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `crates/kutha-runtime/tests/m011_e2e_fixture.rs` — FIX-01/02/03
- [ ] `justifications.jsonl` sidecar + Runtime admission APIs
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
- [ ] Feedback latency < 180s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
