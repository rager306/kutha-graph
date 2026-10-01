---
phase: 26-dictionary-entries-as-facts
verified: 2026-10-01T16:39:23Z
status: passed
score: 7/7 must-haves verified
independent: true
verifier_model: composer-2.5-fast
covered_files:
  - .planning/phases/26-dictionary-entries-as-facts/26-01-PLAN.md
  - .planning/phases/26-dictionary-entries-as-facts/26-01-SUMMARY.md
  - .planning/phases/26-dictionary-entries-as-facts/26-02-PLAN.md
  - .planning/phases/26-dictionary-entries-as-facts/26-02-SUMMARY.md
  - .planning/phases/26-dictionary-entries-as-facts/26-CONTEXT.md
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m013_dictionary_facts.rs
covered_digest: "v2:sha256:8e127b215adebc5e101c0d3edeff7b502c270e53ccecb7f11f278d65474a0bf0"
behavior_unverified: 0
overrides_applied: 0
re_verification: false
---

# Phase 26: Dictionary entries as facts — Verification Report

**Phase Goal:** One substrate: dictionary entries are ordinary bi-temporal facts; the five meta kinds disappear.

**Verified:** 2026-10-01T16:39:23Z  
**Status:** passed  
**Re-verification:** No — initial independent verification (PLAN/SUMMARY not used to design kill-tests; used only for fingerprint coverage and SUB-04 rename-table existence check).

**Evidence sources:** REQUIREMENTS SUB-01..04, ROADMAP Phase 26 success criteria, `26-CONTEXT.md`, `crates/kutha-common/src/event.rs`, `crates/kutha-runtime/src/fold.rs`, `crates/kutha-runtime/src/quantum.rs`, integration tests (kill-tests below).

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | `rg 'Op::(AllowRelation|RegisterRule|PinPolicy|RecordAdmission|RecordAction)' crates` is empty | ✓ VERIFIED | Ripgrep over `crates/`: 0 matches. Legacy names appear only in test message strings/comments, not as `Op::` variants. |
| 2 | Relation, rule, policy, admission, action are `kutha:*` facts; bodies via `StoreBlob` (SUB-01) | ✓ VERIFIED | `KUTHA_*` constants and `KUTHA_RESERVED_RELS` in `event.rs`; `Op::StoreBlob` present; `emit_allow` / `emit_rule` / `emit_policy` append `StoreBlob` + `Assert` in `quantum.rs`; fold `store_blob_once` / `rebuild_blobs`. No `AllowEntry`/`RuleEntry`/side-table structs. |
| 3 | Five meta side tables and duplicate `is_live_at` copies removed; one retract/correct path (SUB-02) | ✓ VERIFIED | Grep: no `has_allow_entry`, `AllowEntry`, etc. Single `Fact::is_live_at` in `fold.rs`. Dictionary queries use `relation_allowed_at`, `live_policy_pin_at`, etc. on facts. Retract by `EventId` exercised in `m013_dictionary_facts`. |
| 4 | Product `as_of` excludes `kutha:`; `dictionary_as_of` includes dictionary relations (D-05) | ✓ VERIFIED | `LiveFactFilter::Product` → `!dict`; `Dictionary` → `dict` via `is_dictionary_rel`. `as_of` → `live_at` → `live_facts_at` (Product); `dictionary_as_of` → `dictionary_live_at` (Dictionary). |
| 5 | Justification/policy refs are EventId or body hash; Runtime RAM justification buffer removed (SUB-03) | ✓ VERIFIED | `Runtime` has no `justifications: Vec`; no `attach_justifications`. `record_justification` mints admission `EventId`; `check_admission_row` reads `fold.fact_index_by_event_id`. `justification_records()` rebuilds from log `JustificationCite` only. |
| 6 | M012 / FF5 / FF6 / M013 named oracles remain green (SUB-04, D-06) | ✓ VERIFIED | Kill-test cargo runs: all passed (see Behavioral Spot-Checks). |
| 7 | Mechanical rename table documented in phase SUMMARY (SUB-04, ROADMAP SC4) | ✓ VERIFIED | `26-02-SUMMARY.md` § "Rename table (old → new)" with seven rows (not used to design tests). |

**Score:** 7/7 truths verified (0 present, behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `crates/kutha-common/src/event.rs` | Lean `Op` without five meta variants; `StoreBlob`; kutha constants | ✓ VERIFIED | `Op` ends at `StoreBlob`; no meta dictionary variants. |
| `crates/kutha-runtime/src/fold.rs` | Unified fact fold; filtered `as_of` / `dictionary_as_of` | ✓ VERIFIED | `LiveFactFilter`, `dictionary_rels`, blob map wired in apply/rebuild. |
| `crates/kutha-runtime/src/quantum.rs` | `emit_*` helpers; EventId admissions; no RAM buffer | ✓ VERIFIED | Genesis `emit_allow`; helpers for rule/policy; admission as Assert. |
| `crates/kutha-runtime/tests/m013_dictionary_facts.rs` | D-08 GREEN dictionary oracles | ✓ VERIFIED | 2 tests pass; Retract + fork_at behavior. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `emit_rule` / `emit_policy` | `GraphFold.blobs` | `Op::StoreBlob` on log + fold apply | ✓ WIRED | `append_meta(Op::StoreBlob)` then Assert. |
| `record_justification` | `check_admission_row` | `fold.fact_index_by_event_id` + live policy pin hash | ✓ WIRED | No RAM vec on admission path. |
| `as_of` | product triples | `LiveFactFilter::Product` | ✓ WIRED | Excludes interned `kutha:*` rel ids. |
| `dictionary_as_of` | dictionary triples | `LiveFactFilter::Dictionary` | ✓ WIRED | Includes only dictionary relations. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `dictionary_as_of` | live triples | `facts` vec filtered by `dictionary_rels` | Yes — from applied Assert events | ✓ FLOWING |
| `relation_allowed_at` | allow decision | live `kutha:allows` facts | Yes — genesis + `emit_allow` | ✓ FLOWING |
| `justification_records()` | cite rows | scan `log` for `JustificationCite` | Yes — not hardcoded empty | ✓ FLOWING |

### Behavioral Spot-Checks (independent kill-tests)

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| M013 dictionary facts | `cargo test -p kutha-runtime --offline --test m013_dictionary_facts` | 2 passed | ✓ PASS |
| M012 allowlist | `cargo test -p kutha-runtime --offline --test m012_allowlist_facts` | 5 passed | ✓ PASS |
| M012 rule registry | `cargo test -p kutha-runtime --offline --test m012_rule_registry` | 5 passed | ✓ PASS |
| M012 admission | `cargo test -p kutha-runtime --offline --test m012_admission_facts` | 6 passed | ✓ PASS |
| M012 action | `cargo test -p kutha-runtime --offline --test m012_action_record` | 4 passed | ✓ PASS |
| FF5 legal pit | `cargo test -p kutha-runtime --offline --test ff5_legal_pit` | 2 passed | ✓ PASS |
| FF6 allowlist | `cargo test -p kutha-runtime --offline --test ff6_allowlist` | 2 passed | ✓ PASS |
| M013 genesis | `cargo test -p kutha-runtime --offline --test m013_genesis_log` | 4 passed | ✓ PASS |
| Five meta `Op::` absent | `rg 'Op::(AllowRelation|RegisterRule|PinPolicy|RecordAdmission|RecordAction)' crates` | empty | ✓ PASS |
| No RAM `attach_justifications` | `rg attach_justification crates/kutha-runtime/src` | empty | ✓ PASS |

### Probe Execution

Step 7c: SKIPPED — no phase-declared `probe-*.sh` for this slice.

### Requirements Coverage

| Requirement | Description | Status | Evidence |
| ----------- | ----------- | ------ | -------- |
| SUB-01 | kutha namespace facts + content-addressed blobs | ✓ SATISFIED | Truths 2, 4; `StoreBlob` + `KUTHA_*` |
| SUB-02 | Remove five meta Ops, side tables, duplicate live paths | ✓ SATISFIED | Truths 1, 3 |
| SUB-03 | EventId/hash refs; RAM buffer gone | ✓ SATISFIED | Truth 5 |
| SUB-04 | Oracles + rename table | ✓ SATISFIED | Truths 6, 7 |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | — | — | None (no TBD/FIXME/XXX in `fold.rs` / `quantum.rs` touched paths) |

### Human Verification Required

None — behavior covered by named integration tests and source wiring checks.

### Gaps Summary

None. Phase goal achieved in codebase.

---

_Verified: 2026-10-01T16:39:23Z_  
_Verifier: gsd-verifier (independent, composer-2.5-fast)_
