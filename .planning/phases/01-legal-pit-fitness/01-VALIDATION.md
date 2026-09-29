---
phase: "01"
slug: "legal-pit-fitness"
status: draft
nyquist_compliant: false
wave_0_complete: true
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
| 01-01-01 | 01-01 | 1 | FIT-01…05 | T-01-01 | Hard gate + evidence skeleton | integration | `cargo test --workspace --offline` | ✅ | ⬜ pending |
| 01-01-02 | 01-01 | 1 | FIT-01…05 | T-01-02 | Twelve fns in `--list` + VALIDATION Task IDs | diagnostic | `cargo test --workspace --offline -- --list` | ✅ | ⬜ pending |
| 01-02-01 | 01-02 | 2 | FIT-01…05 | T-01-05 | Twelve-row pass/fail map | integration | hard gate + evidence cells | ✅ | ✅ |
| 01-02-02 | 01-02 | 2 | FIT-01…05 | T-01-06 | SUMMARY + VALIDATION ✅ | docs | file presence + FIT cites | ✅ | ✅ |
| 01-03-01 | 01-03 | 3 | FIT-01…05 | T-01-09 | REQUIREMENTS FIT [x] batch | docs | grep FIT [x] count == 5 | ✅ | ⬜ pending |
| 01-03-02 | 01-03 | 3 | FIT-01…05 | T-01-11 | STATE/ROADMAP/SUMMARY closeout | docs | STATE Phase 1 + no `.kutha/STATE.md` diff | ✅ | ⬜ pending |

*Task IDs filled by plan 01-01 (`01-01-01`…`01-03-02`). Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky — leave pending until later plans paint.*

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

**Approval:** pending (Plan 01-03 hygiene — REQUIREMENTS FIT `[x]` batch + full sign-off; `nyquist_compliant` stays false until then)

*Plan 01-02 note:* cargo hard gate + twelve-row evidence sampling satisfied for wave 2; leave frontmatter `nyquist_compliant: false` until Plan 03 closes checkbox/sign-off hygiene.
