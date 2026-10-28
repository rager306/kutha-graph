---
phase: 26-dictionary-entries-as-facts
reviewed: 2026-10-01T16:38:00Z
depth: quick
files_reviewed: 4
files_reviewed_list:
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m013_dictionary_facts.rs
findings:
  critical: 0
  warning: 1
  info: 1
  total: 2
status: issues_found
---

# Phase 26: Code Review Report

**Reviewed:** 2026-10-01T16:38:00Z  
**Depth:** quick  
**Files Reviewed:** 4  
**Status:** issues_found

## Summary

Quick pass on Phase 26 scope: `Op::StoreBlob`, reserved `kutha:*` dictionary facts, product `as_of` exclusion (D-05), admit paths, and removal of the five legacy dictionary meta ops. The four in-scope files align with the plan: no `AllowRelation` / `PinPolicy` / etc. in `Op`; fold stores blobs once and treats dictionary asserts as normal facts; product cuts use `LiveFactFilter::Product`; helpers use `StoreBlob` + `Assert` and `append_meta` for log-native cites/outcomes. Integration test `m013_dictionary_facts.rs` exercises `dictionary_as_of`, genesis allows, and fail-closed product emit after retract. One warning on error typing for blob admit; one info note on store-once semantics.

## Warnings

### WR-01: Policy / generic StoreBlob digest failures map to `UnknownRuleVersion`

**File:** `crates/kutha-runtime/src/quantum.rs:581-586`, `1046-1050`  
**Issue:** `admit_store_blob` and the `Op::StoreBlob` branch in `admit` return `RuntimeError::UnknownRuleVersion { pin: digest }` whenever `blob_digest_matches` fails. That path is shared by `emit_policy`, public `emit(Op::StoreBlob)`, and rule bodies. Callers testing policy ingest will misread a digest/bytes mismatch as a rule-registry failure.  
**Fix:** Introduce a dedicated variant (e.g. `BlobDigestMismatch { digest: String }`) or branch on intended use (policy vs rule vs raw blob) before mapping the error.

## Info

### IN-01: Duplicate digest keeps first bytes silently

**File:** `crates/kutha-runtime/src/fold.rs:330-334`  
**Issue:** `store_blob_once` returns early when the digest key already exists, so a later log event with the same digest and different bytes would not update the lease (runtime admit normally prevents this on the write path).  
**Fix:** Optional debug assert or `BlobDigestConflict` on byte inequality if strict content-addressed integrity is required beyond store-once.

---

_Reviewed: 2026-10-01T16:38:00Z_  
_Reviewer: Claude (gsd-code-reviewer)_  
_Depth: quick_
