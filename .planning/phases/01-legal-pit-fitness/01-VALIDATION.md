---
phase: "01"
slug: "legal-pit-fitness"
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-09-29"
---

# Phase 01 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.
> Seeded from `01-RESEARCH.md` § Validation Architecture.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust libtest via `cargo test` (edition 2021 workspace) |
| **Config file** | per-crate `Cargo.toml`; no custom harness crate |
| **Quick run command** | `cargo test --workspace --offline -q` |
| **Full suite command** | `cargo test --workspace --offline` |
| **Estimated runtime** | ~seconds–minutes (local; depends on cache) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --workspace --offline -q`
- **After every plan wave:** Run `cargo test --workspace --offline`
- **Before `/gsd-verify-work`:** Full suite must be green + FIT evidence map complete
- **Max feedback latency:** prefer under a few minutes with warm cache

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-*-* | TBD | TBD | FIT-01 | T-01-01 / — | PIT AS OF truth | integration | `cargo test --workspace --offline` (+ evidence `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`) | ✅ | ⬜ pending |
| 01-*-* | TBD | TBD | FIT-02 | T-01-02 / — | Unknown relation rejected | integration | hard gate + `ff6_unknown_relation_does_not_append` | ✅ | ⬜ pending |
| 01-*-* | TBD | TBD | FIT-03 | — | Semantic open / Define | integration | hard gate + m010 fns | ✅ | ⬜ pending |
| 01-*-* | TBD | TBD | FIT-04 | — | Claim/support + P→Q | integration | hard gate + m011 fns | ✅ | ⬜ pending |
| 01-*-* | TBD | TBD | FIT-05 | — | H2/H4 AS OF dogfood | integration | hard gate + h2/h4 fns | ✅ | ⬜ pending |

*Planner fills concrete Task IDs. Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements. No new test stubs required unless a FIT is red (then stop per CONTEXT D-07).

---

## Manual-Only Verifications

All phase behaviors have automated verification via cargo hard gate + evidence map.

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency acceptable
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
