---
phase: 27-one-write-path-and-facade
reviewed: 2026-10-02T00:00:00Z
depth: quick
files_reviewed: 7
files_reviewed_list:
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/examples/statute_as_of.rs
  - crates/kutha-runtime/tests/facade_statute.rs
findings:
  critical: 0
  warning: 3
  info: 2
  total: 5
status: issues_found
---

# Phase 27: Code Review Report

**Reviewed:** 2026-10-02T00:00:00Z  
**Depth:** quick (pattern scan + targeted read on pipeline / allow lease / facade)  
**Files Reviewed:** 7  
**Status:** issues_found

## Summary

Phase 27 scope holds: `fn append_meta` is absent from `crates/`, dictionary constructors and user `emit` route through `pipeline` / `pipeline_kind`, typed justification/action use `pipeline_events`, and `allows_by_name` is maintained on every `push_fact` with liveness checked in `relation_allowed_at` (consistent with `retract_allow_relation_drops_admit_at_later_cut` elsewhere). Facade example and integration test mirror the statute scenario on the public API only.

Remaining gaps are mostly D-01 purity and lease/documentation edges: a non-pipeline `Define` path via `intern`, cascade commits that skip a per-event `admit` call, and raw `emit` still able to mint `kutha:allows` without the `emit_allow` constructor. No proven allowlist false-negative or data-loss bug in the reviewed paths.

## Warnings

### WR-01: Cascade commits follow-on events without calling `admit` on the committed instance

**File:** `crates/kutha-runtime/src/pipeline.rs:243-266`  
**Issue:** `commit_user_cascade` calls `admit` only on `follow_ons(&first)` before the loop. Inside the loop, `follow_ons(&event)` mints **new** `Event` values (new ids) and pushes them to `pending`; those instances are `commit_event`’d without an `admit` call. Today `follow_ons` only emits the fixed `inverse_knows` Behavior and pre-loop admit covers the same op shape, so allowlist behavior likely matches tests, but the shared pipeline contract (admit every committed op in the quantum) is not enforced for cascade tails. Any future multi-hop `follow_ons` that depends on admit side effects or per-event gates would be wrong by construction.  
**Fix:** Admit each follow-on immediately before push or commit, e.g. inside the loop:

```rust
for e in follow {
    self.admit(&e.op)?;
    pending.push(e);
}
```

(or admit on pop before `commit_event`).

### WR-02: `Runtime::intern` is a second write path outside `pipeline`

**File:** `crates/kutha-runtime/src/quantum.rs:315-328`  
**Issue:** New dictionary strings append `Op::Define` via `fold.apply` + `append_indexed`, bypassing `pipeline`, receipt modes, and `reject_forged_meta`. Phase 27 D-01 targets one path for typed records and user writes; `intern` remains a special case that can desync ingest indexes vs pipeline receipts if extended carelessly.  
**Fix:** Route Define through `pipeline_kind(..., ReceiptMode::Dictionary)` (or a dedicated internal mode) so all log appends share the same commit/receipt discipline, or document `intern` as the sole bootstrap exception in module docs and keep it from gaining product semantics.

### WR-03: Raw `emit` can mint `kutha:allows` without `emit_allow` / Dictionary receipt mode

**File:** `crates/kutha-runtime/src/pipeline.rs:85-98`, `crates/kutha-runtime/src/pipeline.rs:414-417`  
**Issue:** `reject_forged_meta` blocks forged `kutha:admission` and `kutha:action` Asserts on public `emit`, but not `kutha:allows`. A caller can `emit(Op::Assert { relation: allows, delivery_key: Some(name), ... })` on the **User** path (cascade + log `QuantumOutcome`) instead of `emit_allow` (Dictionary, +1 log growth, no extra QO). Allow facts still fold and index `allows_by_name`, but receipt/log shape and empty-name validation differ from the constructor.  
**Fix:** Extend `reject_forged_meta` (or `admit`) to reject user Asserts on `KUTHA_ALLOWS` / `KUTHA_RULE` / `KUTHA_POLICY` unless routed through `emit_allow` / `emit_rule` / `emit_policy`, mirroring admission/action.

## Info

### IN-01: `record_justification` admission `delivery_key` is the policy pin, not the justification id

**File:** `crates/kutha-runtime/src/justification.rs:87-96`, `crates/kutha-runtime/src/fold_query.rs:289-307`  
**Issue:** `live_admission_fact`’s delivery-key branch (`delivery_key == justification_id`) never matches normal rows; lookup relies on parsing `justification_id` as the admission `EventId`. Works today because `jid` is `event.id.to_string()`, but the lease contract is easy to misread when extending admission queries.  
**Fix:** Set `delivery_key: Some(jid.clone())` on the admission Assert (and keep policy pin on subject/object), or drop the unused delivery-key branch and document EventId-only lookup.

### IN-02: Idempotent delivery-key retry reports `events_in_quantum: 0` with a non-empty receipt

**File:** `crates/kutha-runtime/src/admission.rs:182-187`  
**Issue:** `delivery_key_retry` returns `QuantumOutcome { events_in_quantum: 0, receipt: ... one id ... }`, which disagrees with `QuantumOutcome` semantics used elsewhere (`commit_user_cascade` sets `events_in_quantum` to committed count). Callers keyed on `events_in_quantum` for disposition may mis-classify retries.  
**Fix:** Set `events_in_quantum: 1` (or align with `disposition` rules for zero-commit retries) and add a test assertion.

---

_Reviewed: 2026-10-02T00:00:00Z_  
_Reviewer: Claude (gsd-code-reviewer)_  
_Depth: quick_
