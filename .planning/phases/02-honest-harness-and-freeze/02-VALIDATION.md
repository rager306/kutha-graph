---
phase: "02"
slug: "honest-harness-and-freeze"
status: draft
nyquist_compliant: false
wave_0_complete: false
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
| TBD | TBD | TBD | GOV-01 | — | ci fail-closed HIGH; green≠Accepted | smoke | `uv run kutha-gov ci` | ✅ harness / ❌ VERIFICATION W0 | ⬜ pending |
| TBD | TBD | TBD | GOV-02 | — | three L_* separate | check | `uv run kutha-gov precommit --check lifecycles` | ✅ | ⬜ pending |
| TBD | TBD | TBD | PLANE-01 | — | plane paths / no root hexagon | path | catalog probes | ✅ sources | ⬜ pending |
| TBD | TBD | TBD | PLANE-02 | — | typed Op only | source | `rg` Op + no merge-patch write | ✅ | ⬜ pending |
| TBD | TBD | TBD | PLANE-03 | — | distinct relation schemas | check | `precommit --check plane-mix-dicts` | ✅ | ⬜ pending |
| TBD | TBD | TBD | FREEZE-01 | — | freeze lease + absence | check+path | `precommit --check freeze` + Cargo.toml | ✅ | ⬜ pending |
| TBD | TBD | TBD | MAP-01 | — | honeycomb map / Proposed | CLI+check | `kutha-gov map`; honeycomb-map/adr-status | ✅ | ⬜ pending |

*Planner fills concrete Task IDs.*

---

## Wave 0 Requirements

- [ ] `.planning/phases/02-honest-harness-and-freeze/02-VERIFICATION.md` — GOV/PLANE/FREEZE/MAP evidence tables (tracer)
- [ ] Task IDs filled in this file; `wave_0_complete: true` when skeleton exists

Existing harness automation covers gates; Wave 0 is GSD evidence artifacts only.

---

## Manual-Only Verifications

None required if automated probes + `ci` cover GOV/PLANE/FREEZE/MAP. D-10 trajectory sentence is documented in SUMMARY (grep-able).

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers MISSING VERIFICATION artifact
- [ ] No watch-mode flags
- [ ] `nyquist_compliant: true` set when sign-off complete

**Approval:** pending
