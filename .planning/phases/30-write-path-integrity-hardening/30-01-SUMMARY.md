---
phase: 30-write-path-integrity-hardening
plan: 01
status: complete
requirements-completed: [HARD-01, HARD-02]
executor_model: composer-2.5
subsystem: kutha-runtime write-path
commits: 2
plan_head_before: dec4618d433d72a10767a36baf4e1cb9e159bcb3
plan_head_after: ""
tags: [admit, delivery_key, kutha-reserved, m013]
---

# Phase 30 Plan 01: Write-path integrity (HARD-01/02) Summary

**One-liner:** User-origin reserved `kutha:` Assert/Correct/CorrectInterval refused via `ReceiptMode`; Correct paths copy `delivery_key` for keyed retry.

## Blast radius (before → after)

| Tool | Symbol / site | Before | After |
|------|----------------|--------|-------|
| CBM inbound | `Runtime.emit` | 87 | (unchanged API) |
| CBM inbound | `admit` | 5 | +`ReceiptMode` param |
| CBM inbound | `correct` | 2 | fold copy only |
| GitNexus direct | `emit` / `admit` | 0 / 0 | 0 / 0 (known test gap) |
| GitNexus direct | `GraphFold::apply` | 5 CRITICAL | same callers |
| rg | `self.admit(` in pipeline | 3 | 3 (mode threaded) |

## Decisions

- **Retract exception:** User `Retract` on reserved dictionary facts remains admitted so M012/M013 oracles that retract `kutha:allows` / rules / policies through public `emit` stay green; only Assert/Correct/CorrectInterval are blocked at User origin.
- **Retry payload:** After Correct, keyed Assert retry matches the original minted Assert op (ING-01 map still points at first event id).

## Commits

1. `test(30-01): RED reserved kutha emit and keyed correct identity`
2. `feat(30-01): admit write-origin and copy delivery_key on correct`

## Deviations

None — plan executed; test retry uses pre-correct Assert payload (aligned with `by_delivery_key` semantics).

## Self-Check: PASSED
