# Phase 172 Context — Horn pin overlap

**Milestone:** v0.95 (M101-S01)
**Status:** ready (discuss skipped — lease names the slice)

## Decision

`live_horns` currently keeps a `kutha:rule` pin only when `is_live_at(MAX, trigger.valid_from)`. A pin whose window starts later is dropped even when `vt_intersect` with the trigger window is non-empty. Keep pins that are transaction-time live and whose valid-time overlaps the trigger window `(valid_from, valid_to)`. `horn_fires_except` already conjuncts `pin_from`/`pin_to` with the premise windows, so the head is the intersection.

## Oracle

Register a single-premise horn (`knows` → `relatedTo`) on `[2015, 2020)`, then assert `knows` from 2010 open-ended. The derived `relatedTo` is live at 2017 with valid-time `[2015, 2020)` and absent at 2012. Today the pin is skipped and no head is minted.

## Out of scope

Single-combination DFS, unindex reverse pointer, policy-pin election scan, honeycomb thaw.
