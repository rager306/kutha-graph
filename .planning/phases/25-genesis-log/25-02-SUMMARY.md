---
phase: 25-genesis-log
plan: 02
subsystem: runtime
tags: [genesis, allowlist, admit]
requires: [{ phase: 25-genesis-log, provides: RED oracles }]
provides: [genesis AllowRelation on new(), fold-only admit]
affects: [26]
actuals: { tokens: 2628, tasks: 2, commits: 2 }
plan_head_before: 4277ce99e19612844a7cd1b84baeb94d76b0bbcd
plan_head_after: 9df7637940b9ade7eebf86319f79b649ce97959e
tech-stack: { added: [], patterns: [YAML only on empty-log new(); admit fold.relation_allowed_at] }
key-files: { created: [], modified: [crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/tests/m013_genesis_log.rs, crates/kutha-runtime/tests/m010_semantic_open.rs, CHANGELOG.md] }
key-decisions:
  - "Genesis names sorted for D-04 provenance stability"
  - "M010 len proxies vs G; FF6/M012 expects unchanged"
requirements-completed: [GEN-01, GEN-02, GEN-03]
coverage:
  - { id: D1, description: "new() seeds AllowRelation; admit fold-only", requirement: GEN-01, verification: [{ kind: unit, ref: "tests/m013_genesis_log.rs#open_after_relations_path_missing_still_emits", status: pass }], human_judgment: false }
  - { id: D2, description: "D-07 retract relatedTo fail-closed; fork_at admits", requirement: GEN-03, verification: [{ kind: unit, ref: "tests/m013_genesis_log.rs#retract_seeded_related_to_fails_closed_after", status: pass }], human_judgment: false }
  - { id: D3, description: "stripped genesis reject; persist/open keeps genesis; provenance tamper", requirement: GEN-02, verification: [{ kind: unit, ref: "tests/m013_genesis_log.rs#stripped_genesis_rejects_product_relation", status: pass }], human_judgment: false }
duration: 6min
completed: 2026-10-01
status: complete
---
# Phase 25 Plan 02: Genesis wire + green oracles Summary

**`Runtime::new` seeds sorted `Op::AllowRelation`; `admit` is fold-only; persist/open does not re-read YAML.**

## Performance
- **Duration:** 6 min · **Started:** 2026-10-01T15:32:39Z · **Completed:** 2026-10-01T15:38:58Z · **Tasks:** 2 · **Files:** 4

## Accomplishments
- HashSet allowance gone; `load_allowed_names` in `quantum.rs` = 2 (import + `new()`).
- Tests: `m013_genesis_log` 4 ok; `ff6_allowlist` 2 ok; `m012_allowlist_facts` 5 ok; `intern_appends_define_for_new_terms_only` ok.
- D-06 untouched (no `.kutha/STATE.md`, tenant, process YAML).

## M010 len proxies (old → new)
| Old | New |
| `log().len()==0` | G after default; zero `Op::Define` in prefix |
| len==1 after intern Alice | G+1 |
| `as_slice()[0]` Define Alice | `as_slice()[G]` Define Alice |
| re-intern / knows silent len==1 | G+1 |

## Task Commits
1. Genesis on new(); admit fold-only — `2f443ed` (feat)
2. Green oracles + M010 proxy — `9df7637` (feat)

## Oracle preservation
FF6 and `m012_allowlist_facts` expects **unchanged**. Define-silence kept. Genesis tamper detected by `provenance_check` (AllowRelation is not fold-fingerprint input).

## Deviations
**[Rule 2]** Sort YAML names in `new()` for D-04 stability. **Total:** 1. GitNexus kutha-graph **11 commits behind**; CBM `quantum.rs` `metadata_changed`, `m013_genesis_log.rs` `not_tracked` — no reindex.

## Next
Phase 25 plans done. Do not start 26+ / Rocks / Cypher / HNSW.

## Self-Check: PASSED
