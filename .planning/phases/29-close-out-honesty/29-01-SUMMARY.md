---
phase: 29-close-out-honesty
plan: 01
status: complete
requirements-completed: [GOV-01, GOV-02, GOV-03, GOV-04]
executor_model: composer-2.5-fast
commits: 3
---

# Phase 29 Plan 01: Governor close-out honesty Summary

Fail-closed harness checks for CUI budget truncation, artifact-budget globs, verification independence, and shipped I-F oracles with M013 observe names.

## Delivered

- **GOV-01:** `run_checks` emits HIGH `budget` when checks are skipped; `I-cui-budget` kind ledger row.
- **GOV-02:** `glob_max_lines` `min_hits` (artifact-budget steps use 1); selftest proves empty-glob HIGH.
- **GOV-03:** Globbed `file_contains` + `frontmatter_paths_exist`; `23-VERIFICATION` drops non-repo input.
- **GOV-04:** Six M013 tests in `fsm.yaml`; eleven `i-f*` checks restage shipped I-F rows (60 checks ≤ budget 64).

## Commits

| Hash | Message |
|------|---------|
| ad58c2e | feat(29-01): fail closed on check-budget truncation and empty budget globs |
| 218d576 | feat(29-01): glob verification independence and listed input paths |
| 3339d45 | feat(29-01): restage shipped I-F oracles and require M013 tests |

## Deviations

None — plan executed as written. Frontmatter parsing uses line-based `read_only_inputs` extraction so legacy `"; exit:"` command YAML in VERIFICATION files stays untouched.

## Self-Check: PASSED
