# Phase 8: End-to-end candidate fixture - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-30
**Phase:** 8-end-to-end-candidate-fixture
**Areas discussed:** fixture packaging, FIX-02 cites, FIX-01 conflict, FIX-03 reconstruct

---

## Fixture packaging

| Option | Description | Selected |
|--------|-------------|----------|
| Mega narrative only | One test reimplements all S03–S07 oracles | |
| Glue + three named oracles | Shared builder; FIX-01/02/03 fns; call existing APIs | ✓ |
| Cite prior tests only | No new product surface; GATE on old files | |

**User's choice:** Claude discretion («реши эти вопросы разумно»)
**Notes:** Prior suites stay regression; S08 owns composed narrative + new justification surface.

---

## FIX-02 summary/action cites

| Option | Description | Selected |
|--------|-------------|----------|
| New fold Op / graph Fact | Summary as projected Fact | |
| Durable sidecar | justifications.jsonl like quantum_outcomes | ✓ |
| Test-only helper | No persist; RAM only | |
| Behavior fields only | Cite only via rule_version on Behavior | |

**User's choice:** Claude discretion
**Notes:** Stale admission = fail-closed check; must mint new row after reevaluation. No ABAC pack.

---

## FIX-01 conflict variant

| Option | Description | Selected |
|--------|-------------|----------|
| Preserve both + thin report | No winner; evidence summary API | ✓ |
| Auto supersede | Winner policy in fold | |
| Full four-valued / ADR-093 | Product kernel | |

**User's choice:** Claude discretion
**Notes:** Aligns with ADR-013 Proposed `(positive_supports, negative_supports)`.

---

## FIX-03 incremental vs reconstruct

| Option | Description | Selected |
|--------|-------------|----------|
| Fingerprint-only | Compare fold fingerprints alone | |
| Values + supports + eligibility + CSR discard + open | Full thin oracle | ✓ |
| DBSP IVM | Incremental maintenance engine | |

**User's choice:** Claude discretion
**Notes:** Exact query contract only; approximate retrieval out of scope.

---

## Claude's Discretion

All four gray areas — user required reasonable decisions + CBM during execution.

## Deferred Ideas

- ABAC packs, four-valued logic, execution replay, vacuum/hold, Accepted honeycomb
