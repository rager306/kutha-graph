# Phase 6: Typed CSR lease - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-30
**Phase:** 06-typed-csr-lease
**Areas discussed:** Typed storage shape, Multiplicity vs dedup, Query API, Fixture oracles

---

## Typed storage shape

| Option | Description | Selected |
|--------|-------------|----------|
| Extend `CsrLease` in place | Add optional typed payload; risk breaking untyped callers | |
| Separate `TypedCsrLease` | Parallel lease; leave `from_fold` untyped | ✓ |
| Persist typed CSR to disk | Durable picture beside log | |

**User's choice:** Claude decide reasonably (all areas)
**Notes:** CBM `get_code_snippet` on `from_fold` confirms object-only push + dedup; `trace_path` inbound shows leapfrog/materializer/FF5 depend on untyped API → separate type (D-T1).

---

## Multiplicity vs dedup

| Option | Description | Selected |
|--------|-------------|----------|
| Dedup typed by object | Same as untyped — loses labels/supports | |
| Keep distinct (rel, obj, claim_id) | No collapse across relation or claim_id | ✓ |
| Count-only multiplicity | Aggregate without claim_id | |

**User's choice:** Claude decide
**Notes:** `Fact.claim_id` verified via CBM snippet; semantic-contract probe requires labels + support multiplicity (D-T2).

---

## Query API

| Option | Description | Selected |
|--------|-------------|----------|
| Replace `neighbors` | Force typed everywhere | |
| Additive `typed_csr_lease_at` + `edges_out` | Keep `csr_lease_at` / leapfrog | ✓ |
| Mount typed in `CsrMaterializer` now | Optional IVM surface | |

**User's choice:** Claude decide
**Notes:** Inbound callers of `neighbors`/`from_fold` stay on untyped (D-T3). Materializer deferred.

---

## Fixture oracles

| Option | Description | Selected |
|--------|-------------|----------|
| Full semantic-contract matrix | All probes | |
| Two thin named tests CSR-01/CSR-02 | Labels/multiplicity + untyped/FF5 green | ✓ |
| Docs-only | No cargo oracles | |

**User's choice:** Claude decide
**Notes:** Mirror Phase 4/5 thin oracles + GATE-01 (D-T4…D-T6).

---

## Claude's Discretion

All four gray areas — user asked to decide reasonably, re-research if needed, and use codebase-memory-mcp during execution (reindex was run first; decisions locked against fresh graph).

## Deferred Ideas

GraphBLAS, typed LFTJ/Cypher, materializer cut checks, shared-build cost accounting, n-ary incidence — see CONTEXT.md Deferred.
