---
phase: "03"
slug: "lease-gated-next-slice"
status: draft
nyquist_compliant: false
wave_0_complete: true
created: "2026-09-29"
---

# Phase 03 — Validation Strategy

> Seeded from `03-RESEARCH.md` § Validation Architecture.
> CONTEXT locks: D-L1…D-L6; inherits D-G1…G3, D-10/11/15.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | (A) `kutha-gov` FSM + pytest; (B) cargo libtest smoke; (C) shell path/git lease probes |
| **Config file** | `pyproject.toml`; `.kutha/dictionaries/fsm.yaml` |
| **Quick run command** | `uv run kutha-gov precommit` |
| **Full suite command** | `uv run kutha-gov ci` (+ `cargo test --workspace --offline` at tracer / pre-verify) |
| **Estimated runtime** | tens of seconds–few minutes (warm cache) |

---

## Sampling Rate

- **After every task commit:** `uv run kutha-gov precommit` if harness YAML/docs touched; re-check Active Slice None if `.kutha/STATE.md` could change
- **After every plan wave:** `uv run kutha-gov ci` + D-10 SUMMARY + D-L4 lease snapshot pair
- **Before `/gsd-verify-work`:** `ci` HIGH-free + D-11 WARN ledger + D-15 cargo + GOV-03/NEXT tables + D-L4 still None
- **Max feedback latency:** prefer under a few minutes warm

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 03-01 | 1 | GOV-03 | T-03-03 | No new slice while Active Slice None | path+git+check | catalog + `precommit --check trajectory`; tracer ci+VERIFICATION skeleton | ✅ 03-VERIFICATION.md | ⬜ pending |
| 03-01-01 | 03-01 | 1 | D-L4 | T-03-03 | Lease still None | process | Active Slice rg open/close | ✅ STATE + VERIFICATION | ⬜ pending |
| 03-01-02 | 03-01 | 1 | D-L3 inherit | T-03-01 T-03-02 | wave_0 + SUMMARY Trajectory D-10 + D-L4 | process | VALIDATION Task IDs; `explain trajectory` in SUMMARY | ✅ | ⬜ pending |
| 03-02-01 | 03-02 | 2 | GOV-03 | T-03-03 | Paint GOV-03 probe rows | path+git+check | catalog + `precommit --check trajectory` | ✅ | ✅ |
| 03-02-01 | 03-02 | 2 | NEXT-01 | T-03-03 | Paint NEXT-01; optional ROADMAP D-G3 | path+check | catalog + h4-lease/freeze | ✅ | ✅ |
| 03-02-01 | 03-02 | 2 | NEXT-02 | T-03-01 | Paint NEXT-02 captions | prose+check | catalog + honeycomb-map | ✅ | ✅ |
| 03-02-01 | 03-02 | 2 | D-L4 | T-03-03 | Paint D-L4 pass/fail | process | Active Slice rg open/close | ✅ | ✅ |
| 03-02-02 | 03-02 | 2 | D-10 | T-03-01 | SUMMARY + Trajectory D-10 | process | `uv run kutha-gov ci` + `explain trajectory` | ✅ | ✅ |
| 03-03-01 | 03-03 | 3 | GOV-03 NEXT-01 NEXT-02 | T-03-01 | REQUIREMENTS batch after VERIFICATION passed | process | checkbox batch only | ✅ REQUIREMENTS.md | ⬜ pending |
| 03-03-02 | 03-03 | 3 | phase closeout | T-03-02 | STATE/ROADMAP/VALIDATION closeout | process | GSD STATE/ROADMAP only; never `.kutha/STATE.md` | ✅ | ⬜ pending |

---

## Wave 0 Requirements

- [x] `03-VERIFICATION.md` — GOV-03 / NEXT-01 / NEXT-02 / D-L4 evidence tables
- [x] Task IDs in this file; `wave_0_complete: true` when skeleton exists

Existing `ci` + checks cover automated gates. Wave 0 is GSD evidence only.

---

## Manual-Only Verifications

None if automated probes + `ci` cover GOV-03/NEXT. D-10 trajectory is documented in SUMMARY.

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity
- [ ] Wave 0 covers MISSING VERIFICATION artifact
- [ ] No watch-mode flags
- [ ] `nyquist_compliant: true` when sign-off complete

**Approval:** pending
