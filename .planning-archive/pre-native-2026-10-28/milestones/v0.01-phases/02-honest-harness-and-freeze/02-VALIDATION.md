---
phase: "02"
slug: "honest-harness-and-freeze"
status: complete
nyquist_compliant: true
wave_0_complete: true
created: "2026-09-29"
---

# Phase 02 — Validation Strategy

> Seeded from `02-RESEARCH.md` § Validation Architecture.
> CONTEXT locks: D-G1…G3, D-10…D-15.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | (A) `kutha-gov` FSM + pytest harness; (B) cargo libtest for product smoke |
| **Config file** | `pyproject.toml` `[tool.pytest.ini_options]`; `.kutha/dictionaries/fsm.yaml` |
| **Quick run command** | `uv run kutha-gov precommit` |
| **Full suite command** | `uv run kutha-gov ci` (+ `cargo test --workspace --offline` at tracer / pre-verify) |
| **Estimated runtime** | tens of seconds–few minutes (warm cache) |

---

## Sampling Rate

- **After every task commit:** `uv run kutha-gov precommit` when harness YAML/docs touched
- **After every plan wave:** `uv run kutha-gov ci` (D-G1) + D-10 SUMMARY trajectory block
- **Before `/gsd-verify-work`:** `ci` HIGH-free + D-11 WARN ledger + D-15 cargo smoke + probe tables complete
- **Max feedback latency:** prefer under a few minutes warm

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 02-01 | 1 | GOV-01 | T-02-01, T-02-02 | ci fail-closed HIGH; green≠Accepted; VERIFICATION skeleton | smoke | `uv run kutha-gov ci` + D-15 cargo | ✅ VERIFICATION | ✅ |
| 02-01-02 | 02-01 | 1 | GOV-01 | T-02-01 | VALIDATION Task IDs + wave_0 + SUMMARY § Trajectory D-10 | docs | grep Task IDs / Trajectory | ✅ | ✅ |
| 02-02-01 | 02-02 | 2 | GOV-02, PLANE-01, PLANE-02, PLANE-03, FREEZE-01, MAP-01 | T-02-03, T-02-04 | Probe paint GOV/PLANE/FREEZE/MAP | path+check | RESEARCH catalog probes | ✅ sources | ✅ |
| 02-02-02 | 02-02 | 2 | GOV-01…MAP-01 | T-02-01 | Wave-2 SUMMARY + Trajectory D-10 | docs | SUMMARY § Trajectory | ✅ SUMMARY | ✅ |
| 02-03-01 | 02-03 | 3 | GOV-01…MAP-01 | T-02-09 | REQUIREMENTS GOV/PLANE/FREEZE/MAP [x] batch | docs | REQUIREMENTS checkboxes | ✅ REQUIREMENTS | ✅ |
| 02-03-02 | 02-03 | 3 | GOV-01…MAP-01 | T-02-10, T-02-11 | STATE/ROADMAP/VALIDATION closeout | docs | STATE/ROADMAP; no `.kutha/STATE.md` edit | ✅ | ✅ |

*Task IDs filled by plan 02-01 (`02-01-01`…`02-03-02`). Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky. All Task IDs ✅ after Plan 02-03.*

---

## Wave 0 Requirements

- [x] `.planning/phases/02-honest-harness-and-freeze/02-VERIFICATION.md` — GOV/PLANE/FREEZE/MAP evidence tables (tracer)
- [x] Task IDs filled in this file; `wave_0_complete: true` when skeleton exists

Existing harness automation covers gates; Wave 0 is GSD evidence artifacts only.

---

## Manual-Only Verifications

None required if automated probes + `ci` cover GOV/PLANE/FREEZE/MAP. D-10 trajectory sentence is documented in SUMMARY (grep-able).

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers MISSING VERIFICATION artifact
- [x] No watch-mode flags
- [x] Feedback latency acceptable
- [x] `nyquist_compliant: true` set when sign-off complete

**Approval:** signed off (Plan 02-03) — REQUIREMENTS GOV/PLANE/FREEZE/MAP `[x]` batch after VERIFICATION `status: passed`; pre-verify `uv run kutha-gov ci` + `explain trajectory` + `cargo test --workspace --offline`; `nyquist_compliant: true`

*Plan 02-03 note:* Wave 3 hygiene complete; evidence SoT remains `02-VERIFICATION.md`; harness `.kutha/STATE.md` untouched (Active Slice None; freeze until M002).
