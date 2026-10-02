---
phase: 30-write-path-integrity-hardening
reviewed: 2026-10-02T06:21:00Z
depth: standard
files_reviewed: 8
files_reviewed_list:
  - CHANGELOG.md
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/facade_statute.rs
  - crates/kutha-runtime/tests/m013_dual_mode.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - docs/architecture/benchmark-baseline.md
findings:
  critical: 0
  warning: 4
  info: 2
  total: 6
status: issues_found
---

# Phase 30: Code Review Report

**Reviewed:** 2026-10-02T06:21:00Z
**Depth:** standard
**Files Reviewed:** 8
**Status:** issues_found

## Summary

CBM coverage on these paths is `not_tracked` / `metadata_changed` (index 2026-09-30); claims below are from source/`rg`. User `admit` now rejects Assert/Correct/CorrectInterval on reserved `kutha:` (`starts_with("kutha:")` or `KUTHA_RESERVED_RELS`). `open` / `fork_at` / `from_snapshot` / `replay_check` fold-apply only — they do **not** re-admit, so persisted dictionary Asserts are not rejected. Cascade follow-ons are `inverse_knows` only. Tenant JSONL maps to product labels, not `kutha:`. Retract of dictionary facts via `emit` remains User-admitted (documented). Correct still targets one `fact_index_by_event_id` row. Interval split clones one key onto prefix/interior/suffix; VT halves are disjoint so `live_support_count` at a cut stays 1. `by_delivery_key` is still the minting Assert EventId (ING-01), not the replacement row.

## Warnings

### WR-01: User `Behavior` can still mint reserved `kutha:` facts

**File:** `crates/kutha-runtime/src/admission.rs:83-97`
**Issue:** Reserved gating is Assert/Correct/CorrectInterval only. `emit(Op::Behavior { relation: kutha:*, .. })` after a live rule pin skips that block; `op_relation` then `starts_with("kutha:")` admits. Fold Behavior rows have `delivery_key: None`, so they miss `allows_by_name` / hash leases, but they still appear in `dictionary_as_of`. `Runtime::assert`/`correct`/`pipeline` User are gated; this public `emit` path is not.
**Fix:** Treat User Behavior like Assert: if `relation_name_reserved(*relation)` → `MetaOpRejected`.

### WR-02: HARD-04/03 Correct oracles do not observe Correct

**File:** `crates/kutha-runtime/tests/facade_statute.rs:56-62`
**Issue:** `correct(..., "50-kmh", VF_FIRST)` rewrites the same object. Pre/post `as_of(2015)=[t50,t30]` and `as_of(2021)=[t30,t30]` are the derive overlap, not a changed object — a skipped Correct still matches. `m013_dual_mode.rs:145-160` only `assert_eq` Eager vs Lazy after Correct; no `40-kmh` pin and no pre/post `as_of` delta. Both modes no-op would still pass.
**Fix:** Correct 50→40 (facade) and assert 50 absent / 40 present; dual-mode `assert!(new.contains(&(.., limit_40)))` after Correct.

### WR-03: Interval key oracle only samples interior; retry does not prove fold copy

**File:** `crates/kutha-runtime/tests/m013_write_path.rs:167-191`
**Issue:** `live_keyed` filters `is_live_at(..., 2017)` (interior only). Prefix/suffix could drop the key and the test would still pass. Retry compares log Assert payload via `by_delivery_key` (`admission.rs:198-209`); that path is unchanged by `fold.rs` `prior_key`. Whole-version Correct `live_keyed==1` does pin the copy; interval does not pin leftovers.
**Fix:** Assert keyed live count 1 at 2012, 2017, and 2021; keep retry as ING-01, not as proof of fold copy.

### WR-04: Keyed Correct changes fold fingerprint vs pre-copy snapshots

**File:** `crates/kutha-runtime/src/fold.rs:572-588`
**Issue:** Replacement rows used `delivery_key: None`; fingerprint hashes keys (`fold_query.rs:97-104`). `open` snapshot+`replay_check` will `ReplayDivergence` on a pre-change snapshot whose log contains keyed Correct/CorrectInterval. Same-commit persist/open agrees. Unkeyed Correct still copies `None`.
**Fix:** Document the fingerprint break, or rebuild snapshots / replay-only open for that class of logs.

## Info

### IN-01: Product changelog over-attributes retry and as_of

**File:** `CHANGELOG.md:41`
**Issue:** “copies … so keyed ingest retry stays idempotent” is the wrong cause (log map). Bench `as_of` −43.5% (`benchmark-baseline.md:184`) is a single run with no query-path edit in this slice.
**Fix:** State fold-row identity vs log retry separately; mark as_of as one-run variance.

### IN-02: HARD-01 test is Assert/`kutha:allows` only

**File:** `crates/kutha-runtime/tests/m013_write_path.rs:39-61`
**Issue:** No oracle for User Correct/CorrectInterval of a genesis allow, or Assert of `kutha:rule`/`kutha:policy`. Admit code covers them.
**Fix:** One Correct of `emit_allow` event_id → `MetaOpRejected`.

---

_Reviewed: 2026-10-02T06:21:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
