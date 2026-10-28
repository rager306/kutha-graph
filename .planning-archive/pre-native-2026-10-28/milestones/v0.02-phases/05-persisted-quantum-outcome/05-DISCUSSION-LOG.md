# Phase 5: Persisted quantum outcome - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-29
**Phase:** 5-Persisted quantum outcome
**Areas discussed:** Persisted outcome form, Progress 0/1/2 encoding, Crash/resume, Fixture width

---

## Persisted outcome form

| Option | Description | Selected |
|--------|-------------|----------|
| Authoritative sidecar (`quantum_outcomes.jsonl`) | Survive open without snapshot; not a lease | ✓ |
| New fold-no-op Op in event log | Same SoT stream as Define | |
| Snapshot-only field | Droppable with snapshot | |

**User's choice:** «реши эти вопросы разумно и при необходимости доисследуй»
**Notes:** ADR-010/014 require outcomes to survive dropping leases; RAM receipt today is not persisted. Sidecar default; Op only if sidecar cannot meet open-without-snapshot.

---

## Progress encoding 0/1/2

| Option | Description | Selected |
|--------|-------------|----------|
| Zero / Partial / Full from aborted + event count | Map max_cascade 0/1/2+ fixtures | ✓ |
| Only aborted bool after open | Loses zero vs partial | |
| Treat Ok() as Full | Contradicts ADR-014 | |

**User's choice:** Claude discretion
**Notes:** Existing `budget_aborts_storm` already demonstrates Partial.

---

## Crash / resume

| Option | Description | Selected |
|--------|-------------|----------|
| Missing terminal Full ⇒ incomplete; explicit resume record | Never infer success on open | ✓ |
| Auto-resume on open | Inferred continuation | |
| Rollback committed prefix on crash | Contradicts ADR-014 prefix-commit | |

**User's choice:** Claude discretion
**Notes:** ADR-014 permits prefix commit; no rollback promise.

---

## Fixture width

| Option | Description | Selected |
|--------|-------------|----------|
| OUT-01 + OUT-02 oracles only | Minimal falsifiers | ✓ |
| Full ADR-062 anti-duplicate | Out of scope | |
| Cryptographic receipt layout | ADR-014 open research | |

**User's choice:** Claude discretion

---

## Claude's Discretion

All four gray areas (plus governor companions D-O5…D-O7). Brief code/ADR research performed during discuss (quantum.rs, receipt.rs, store.rs, ADR-014/010).

## Deferred Ideas

See CONTEXT.md `<deferred>`.
