# Phase 4: Partial correction with residual intervals - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-29
**Phase:** 4-Partial correction with residual intervals
**Areas discussed:** Interval-patch API shape, Residual identity, Fixture width, Admission of P'

---

## Interval-patch API shape

| Option | Description | Selected |
|--------|-------------|----------|
| New Op variant | Explicit interval-patch op; Correct fold path untouched | ✓ |
| Extend Correct with optional patch bounds | Same enum variant; None = whole-version | |
| Encode as Assert+Retract pairs | No new op; compose from existing ops | |

**User's choice:** «реши эти вопросы разумно» (Claude discretion)
**Notes:** Chose new Op so CORR-02 stays a pure non-regression on today's Correct; ADR-013 forbids silently treating Correct as interval-patch.

---

## Residual identity

| Option | Description | Selected |
|--------|-------------|----------|
| Same claim_id for residuals | Continuations of the same support lineage | ✓ |
| New claim_id per residual | Treat splits as new claims | |
| Residuals as Retract-only gaps | No residual Fact rows | |

**User's choice:** Claude discretion
**Notes:** Matches existing Correct's claim_id preservation and M011 support identity.

---

## Fixture width for S04

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal residual oracle (obs.3 only) | VT 2012/2021 residuals + Correct regression | ✓ |
| Partial obs.1 + obs.3 | Dual supports at t2 plus residuals | |
| Full fixture narrative | Entire semantic-contract story in S04 | |

**User's choice:** Claude discretion
**Notes:** Full narrative belongs to S08; keep S04 falsifiable and small.

---

## Admission of replacement P'

| Option | Description | Selected |
|--------|-------------|----------|
| Defer admission/conflict to S08 | Record replacement object only | ✓ |
| Soft conflict flag in S04 | Report without policy | |
| Full admission policy in S04 | ABAC / allow-action | |

**User's choice:** Claude discretion
**Notes:** Avoid scope creep into FIX-01 / security cells.

---

## Claude's Discretion

All four gray areas (plus governor inheritance companions D-C5…D-C7).

## Deferred Ideas

S05–S08 surfaces; ADR-050; Rocks/Cypher; legal pack — see CONTEXT.md `<deferred>`.
