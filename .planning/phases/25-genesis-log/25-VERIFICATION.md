---
phase: 25-genesis-log
verified: 2026-10-01T15:45:00Z
status: passed
score: 3/3 requirements verified (GEN-01..03); 3/3 roadmap success criteria
independent: true
verifier_model: composer-2.5-fast
behavior_unverified: 0
overrides_applied: 0
covered_files:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/25-genesis-log/25-CONTEXT.md
  - .planning/phases/25-genesis-log/25-01-PLAN.md
  - .planning/phases/25-genesis-log/25-02-PLAN.md
  - .planning/phases/25-genesis-log/25-01-SUMMARY.md
  - .planning/phases/25-genesis-log/25-02-SUMMARY.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/allow.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/tests/m013_genesis_log.rs
  - crates/kutha-runtime/tests/ff6_allowlist.rs
  - crates/kutha-runtime/tests/m012_allowlist_facts.rs
  - crates/kutha-runtime/tests/m010_semantic_open.rs
covered_digest: "v2:sha256:1f67219cdea0603dc8fa346bcf016c699b3269ebb3049871552a6f3d5dfb50a9"
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/25-genesis-log/25-CONTEXT.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/allow.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/tests/m013_genesis_log.rs
  - crates/kutha-runtime/tests/ff6_allowlist.rs
  - crates/kutha-runtime/tests/m012_allowlist_facts.rs
  # PLAN/SUMMARY listed in covered_files for GSD fingerprint gate only; not used for kill-test design.
commands:
  - cmd: "cargo test -p kutha-runtime --offline --test m013_genesis_log"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test ff6_allowlist"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test m012_allowlist_facts"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --lib intern_appends_define_for_new_terms_only -- --exact"
    exit: 0
    note: "0 tests in lib crate (name lives in integration test m010_semantic_open); supplementary D-05 run below"
  - cmd: "cargo test -p kutha-runtime --offline --test m010_semantic_open intern_appends_define_for_new_terms_only -- --exact"
    exit: 0
    note: "supplementary D-05 oracle (not in gsd-auto-contract kill list)"
  - cmd: "rg -n 'allowed\\.contains' crates/kutha-runtime/src"
    exit: 1
  - cmd: "rg -n 'load_allowed_names' crates/kutha-runtime/src/quantum.rs"
    exit: 0
    note: "2 hits (import + Runtime::new genesis seed only)"
  - cmd: "rg -n 'allowed:\\s*HashSet' crates/kutha-runtime/src/quantum.rs"
    exit: 1
  - cmd: "codebase-memory-mcp list_projects"
    exit: 0
  - cmd: "codebase-memory-mcp check_index_coverage (kutha-graph, cited paths)"
    exit: 0
    note: "metadata_changed / not_tracked on several paths; verification used source reads"
---

# Phase 25: Genesis log — Independent Verification

**Phase goal (ROADMAP):** The log owns relation admission from its first event; no file is read at admit time.

**Verified:** 2026-10-01T15:45:00Z  
**Status:** passed  
**Re-verification:** No — initial independent verification  
**Inputs:** REQUIREMENTS GEN-01..03, ROADMAP Phase 25 success criteria, `25-CONTEXT.md` locked decisions, runtime sources and oracles only (no PLAN/SUMMARY for kill-test design).

## Kill-tests

| # | Command / check | Result | Status |
|---|-----------------|--------|--------|
| 1 | `cargo test -p kutha-runtime --offline --test m013_genesis_log` | 4/4 pass (retract before/after cut, YAML missing after open, persist/replay, stripped genesis) | ✓ |
| 2 | `cargo test -p kutha-runtime --offline --test ff6_allowlist` | 2/2 pass | ✓ |
| 3 | `cargo test -p kutha-runtime --offline --test m012_allowlist_facts` | 5/5 pass | ✓ |
| 4 | `cargo test -p kutha-runtime --offline --lib intern_appends_define_for_new_terms_only -- --exact` | exit 0, **0 tests** (not registered under `--lib`) | ⚠ advisory |
| 4b | Supplementary D-05: `--test m010_semantic_open … --exact` | 1/1 pass | ✓ |
| 5a | `rg 'allowed.contains'` in `crates/kutha-runtime/src` | no matches | ✓ |
| 5b | `load_allowed_names` in `quantum.rs` | 2 lines (import + `Runtime::new` only) | ✓ |
| 5c | `allowed: HashSet` field on `Runtime` | no field; struct has log/fold/dict indexes only | ✓ |
| 6 | `admit` path (source) | relation checks use `self.fold.relation_allowed_at(&name, tt, vt)` only; no YAML/env read | ✓ |

## Roadmap success criteria

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | New log starts with genesis events; opening existing log never reads YAML | ✓ VERIFIED | `Runtime::new` sorts YAML names and `emit(AllowRelation)`; `from_snapshot` / `from_dict_and_events` / `store::open` replay events with no `load_allowed_names`; `open_after_relations_path_missing_still_emits` |
| 2 | Retract seeded relation: fail closed after, admit before | ✓ VERIFIED | `retract_seeded_related_to_fails_closed_after` + M012 `retract_allow_relation_drops_admit_at_later_cut` |
| 3 | FF6 and M012 allowlist oracles green, assertions unchanged | ✓ VERIFIED | kill-tests 2–3 |

## Requirements (REQUIREMENTS.md)

| ID | Status | Evidence |
|----|--------|----------|
| GEN-01 | ✓ VERIFIED | Genesis `AllowRelation` events at log start from YAML only in `Runtime::new`; persisted log carries allowance in events |
| GEN-02 | ✓ VERIFIED | `admit` (lines 892–931) delegates to `fold.relation_allowed_at`; `store.rs` has no relations-path reads |
| GEN-03 | ✓ VERIFIED | Retract oracle pair + FF6 green |

## Locked decisions (`25-CONTEXT.md`)

| ID | Status | Evidence |
|----|--------|----------|
| D-01 | ✓ | YAML → genesis `AllowRelation` in `new` only |
| D-02 | ✓ | No `Runtime.allowed`; admit fold-only |
| D-03 | ✓ | Open path no YAML; `stripped_genesis_rejects_product_relation` |
| D-04 | ✓ | Genesis in log; `persist_open_replay_keeps_genesis` |
| D-05 | ✓ | Genesis loop does not intern YAML names; M010 intern oracle green (supplementary cmd) |
| D-06 | ✓ | Process plane unchanged (out of scope; no product/harness merge in diff scope) |
| D-07 | ✓ | RED retract test green post-change |

## Key implementation surfaces

| Surface | Role |
|---------|------|
| `quantum.rs` `Runtime::new` | Reads `load_allowed_names`, emits sorted genesis `AllowRelation` |
| `quantum.rs` `admit` | `relation_allowed_at` at fold cut `(tt, vt)` |
| `fold.rs` `relation_allowed_at` | Live allow entries at cut |
| `allow.rs` | YAML parser; used only from new-log constructor |
| `store.rs` `open` / `persist` | Event replay; genesis survives round trip (oracle) |

## Code graph (advisory)

- CBM `list_projects`: kutha-graph indexed.
- `check_index_coverage`: several cited paths `metadata_changed` / `not_tracked`; claims grounded in source + kill-tests, not graph-only.

## Observable truths

| Truth | Status |
|-------|--------|
| Relation allowance is log events from first event | ✓ |
| Admit time has no file/env allowlist read | ✓ |
| Retract seeded relation is fail-closed at later cut | ✓ |
| Legacy FF6 / M012 oracles remain green | ✓ |
| No `allowed.contains` / RAM allow set on `Runtime` | ✓ |

**Gaps:** none. **Advisory:** gsd-auto-contract kill-test #4 targets `--lib` but D-05 oracle is under `--test m010_semantic_open` (supplementary run recorded).

---

_Verified: 2026-10-01T15:45:00Z_  
_Verifier: gsd-verifier (composer-2.5-fast, independent)_
