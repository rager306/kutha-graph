# CE whole-repo regression map (2026-10-11)

Process note. Maps review findings to matrix rows and oracles so the same
defect classes cannot silent-regress. Not an ADR. Honeycomb stays **Proposed**.

Source review: `.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/`
plus `advisor-correction.md`.

## Closed / demoted → killed regressions

| Finding / claim | Matrix id | Oracle(s) | Owner |
|-----------------|-----------|-----------|-------|
| #1 Op decode (demoted; documented break) | `SEM-m050-op-compat-policy`, `SEM-m053-legacy-meta-ops-fail-open` | `m050_op_compat` + `all_five_legacy_dictionary_meta_ops_fail_deserialize` | M050-S01 / M053-S01 |
| #7 lock before heal | `SEM-m052-lock-before-heal` | `m052_lock_before_heal` | M052-S03 |
| #8 cascade atomicity | `SEM-m052-cascade-atomic` | `m052_cascade_atomic` | M052-S02 |
| #11 snapshot-free open replay | `SEM-m052-open-replay` | `m052_open_replay` | M052-S04 |
| Withdrawal: hydrate outcomes | `SEM-m053-hydrate-outcomes` | `fork_at_rebuilds_outcomes_from_log_quantum_outcomes` | M053-S01 |
| Withdrawal: persist_dir anchor | `SEM-m053-persist-dir-anchor` | `common_persist_dir_anchors_under_manifest_tmp_not_cwd` | M053-S01 |
| Withdrawal: rollover layout seam | `SEM-m053-rollover-layout-seam` | `try_persist_with_layout_preserves_rollover_seam_contract` | M053-S01 |
| #4 Correct lineage eligible | `SEM-m054-horn-correct-eligible` | `correct_premise_refire_is_derivation_eligible` | M054-S01 |
| #3 non-trigger retract | `SEM-m054-horn-nontrigger-retract` | `retract_non_trigger_premise_drops_derived` | M054-S01 |
| #2 VT intersect | `SEM-m054-horn-vt-intersect` | `horn_head_vt_is_conjunction_of_premises` | M054-S01 |
| #5 residual dedup | `SEM-m054-horn-residual-dedup` | `overlap_dedup_emits_uncovered_vt_residual` | M054-S01 |
| #3 cascade dup Retract | `SEM-m054-horn-no-dup-retract` | `correct_cascade_does_not_duplicate_derived_retract` | M054-S01 |
| #9 torn heal | `SEM-m054-persist-torn-heal` | `m054_persist_durability` | M054-S02 |
| #10 gen-unique | `SEM-m054-persist-gen-unique` | `m054_persist_durability` | M054-S02 |
| #6 tip rollback | `SEM-gap-persist-poison` | `m055_append_tip_rollback` | M055-S01 |
| #14 PLAN budget | `SEM-m054-gov-plan-budget` | artifact-budget | M054-S03 |
| #22/#23 matrix schema | `SEM-m054-gov-matrix-schema` | test_matrix | M054-S03 |
| #34 pool once | `SEM-m054-gov-pool-once` | parallel pool | M054-S03 |
| #15/#25/#16/#17/#19/#27 scaling | `SEM-m054-scale-*` | `m054_scaling_bench` | M054-S04 |
| #24/#20/#12/#21 ID | `SEM-m054-id-*` | `m054_event_id` / `m054_id_structure` | M054-S05 |

Pin scheme (full-branch #24 / M051) stays `SEM-m051-pin-compat-policy` — distinct from whole-repo EventId #24.

## Open problem classes

| Class | Findings | Matrix id | Owner |
|-------|----------|-----------|-------|
Closed this wave: #35/#36 → `SEM-m056-hygiene-*` (M056-S05); #28 → `SEM-m056-norm-view-derived-correct` (M056-S04); #26 → `SEM-m056-delivery-key-same-relation` (M056-S03); #29/#30 → `SEM-m056-persist-*` (M056-S02); lease-bundle → `SEM-m056-lease-bundle` (M056-S01). Follow-ons: STATE→ROADMAP → `SEM-m057-lease-roadmap` (M057-S01); disk_free → `SEM-m058-disk-free` (M058-S01); budget-template → `SEM-m059-budget-template` (M059-S01).

## Rule

New CE findings that land a fix must add a `killed` matrix row with a real
`kill_mutation` and named `#[test]` / check oracle in the same wave. Gap rows
are the honest parking lot until a slice is leased — not a silent pass.
