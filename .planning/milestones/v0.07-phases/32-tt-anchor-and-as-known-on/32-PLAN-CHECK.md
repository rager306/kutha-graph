# Phase 32 plan check (2026-10-02)

**Verdict: PASS** (with **FLAG** advisories — no **BLOCK**)

Goal-backward: two plans deliver ANK-01–03 and CONTEXT D-01–D-06; Phase 33 fixture wraps, retract-by-EventId, and executor `32-VERIFICATION` stay out of scope per D-04/D-05.

## Hard gates (controller + gsd-auto-contract)

| Gate | Result |
|------|--------|
| Plans | 2 (32-01 Wave 1, 32-02 Wave 2) — OK (≤2) |
| PLAN line budget | 123 / 112 lines — OK (<150 each) |
| CONTEXT line budget | 30 lines — OK (<80) |
| Tasks/plan | 3 each — OK (not >4 warning band) |
| `depends_on` | 32-02 → `32-01`; acyclic — OK |
| `.kutha/STATE.md` | Forbidden in tasks/verify — OK |
| `kutha-gov ci` | Not in executor plans (contract) — OK |
| Estimate (smart zone) | 45k / 40k vs 100k budget — OK; confidence `low` |

## Requirement coverage (REQUIREMENTS.md ANK)

| ID | Plans | Mechanism |
|----|-------|-----------|
| ANK-01 | 01 | `Op::TtAnchor`, emit path, fold lease, no Fact (task 1–2) |
| ANK-02 | 01, 02 | `as_known_on` Option + greatest anchor; cfg(test) + `m014_as_known_on` miss/W1&lt;D≤W2 (tasks 1–2 in both plans) |
| ANK-03 | 02 | Named `true_on_d_differs_from_known_on_d`; FF5/FF6 bytes guarded (tasks 1, 3) |

## Context / kill-tests

| Decision / test | Coverage |
|-----------------|----------|
| D-01 tt_anchor on log | 32-01 task 1 |
| D-02 greatest anchor + None miss | 32-01 task 1; 32-02 task 2 |
| D-03 named as_of vs as_known_on | 32-02 task 1 |
| D-04 non-goals (no FF5 wraps) | verify `git diff --exit-code` on FF5/FF6 throughout |
| D-05 no executor VERIFICATION | stated in objective/output/tasks |
| D-06 CBM primary; no GitNexus | 32-01 blast radius; M014 outside M013 pair |
| Verifier kill-tests (CONTEXT) | miss, W1&lt;D≤W2, divergence — 32-02 tasks 1–2 |

No scope reduction (`v1`, stub, defer) against locked decisions.

## Plan structure

`verify.plan-structure`: both plans `valid: true`; all six tasks have files, action, verify, done.

## Dimensions skipped

- **7c** Architectural tier — no Responsibility Map in 32-RESEARCH.md
- **8** Nyquist — no `Validation Architecture` in 32-RESEARCH.md; no `32-VALIDATION.md` (same skip as Phase 31)
- **12** PATTERNS.md — absent
- **Review incorporation** — no REVIEWS.md

## Probe: failing direction (8f)

`gsd_run check verify-failure-directions 32`: **6× `missing`** (no `<fails_when>` on any `<automated>` block). Treated as **FLAG** (advisory) while Dimension 8 is skipped; recommend adding `<fails_when>` before execute if Nyquist is enabled later.

Verify path probe: `not_applicable` (no blockers).

## FLAG advisories (non-blocking)

1. **Nyquist / fails_when** — all six automated verify blocks lack `<fails_when>` (see probe).
2. **32-01 task 2** — verify is structural (`rg` + FF5/STATE diff) only; runtime proof of `rebuild_tt_anchors` / persist deferred to 32-02 task 2 (acceptable cross-plan wiring).
3. **Verify chains** — semicolon pipelines without explicit `set -e`; a failed intermediate `rg` may not fail the whole block (same pattern as Phase 31).

## BLOCK issues

None.

## Required plan edits

None.

Proceed: execute 32-01, then 32-02; independent verifier writes `32-VERIFICATION.md` (D-05) after both plans.
