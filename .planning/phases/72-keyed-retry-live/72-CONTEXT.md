# Phase 72: Keyed retry requires live fact — Context

**Milestone:** v0.16 / harness M022 S02  
**Requirements:** KEY-RETRY-01, ADM-PIN-01  
**Depends on:** Phase 71  
**Mode:** Controller + owner (GSD auto)

## Locked

- **D-KEY-RETRY:** `delivery_key_retry` returns a short-circuit outcome only when the prior keyed Assert still has a **live** fold fact. After Retract, identical keyed Assert remints.
- **D-ADM-PIN:** `admission_status_at` resolves via admission EventId; delivery_key holds policy pin. Dead fallback (delivery_key == justification_id) stays unused — characterization pin, no rewrite of admit path.
- Bench ≤10% on `ingest_plain.n` / `as_of.n`. Honeycomb Proposed.

## Verifier kill-tests

- Named RED-then-green: retract keyed Assert, re-emit same key → new live support (events_in_quantum ≥ 1 or as_of contains).
- Identical retry without retract still short-circuits (events_in_quantum == 0).
- Admission EventId pin green.
