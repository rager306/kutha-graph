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
Closed this wave: #35/#36 → `SEM-m056-hygiene-*` (M056-S05); #28 → `SEM-m056-norm-view-derived-correct` (M056-S04); #26 → `SEM-m056-delivery-key-same-relation` (M056-S03); #29/#30 → `SEM-m056-persist-*` (M056-S02); lease-bundle → `SEM-m056-lease-bundle` (M056-S01). Follow-ons: STATE→ROADMAP → `SEM-m057-lease-roadmap` (M057-S01); disk_free → `SEM-m058-disk-free` (M058-S01); budget-template → `SEM-m059-budget-template` (M059-S01); budget-covers → `SEM-m060-budget-covers` (M060-S01).

## 2026-10-15 HEAD review (`5dd5338`) — closed this wave

| Finding / claim | Matrix id | Oracle(s) | Owner |
|-----------------|-----------|-----------|-------|
| #1 pin-cite stale justifications sidecar | `SEM-m091-pin-cite-justifications` | `migrate_pin_cite_reopens_after_nonempty_justifications_sidecar` | M091-S01 |
| #13 migrate torn tails uncounted | `SEM-m092-migrate-dropped-tail` | `migrate_op_wire_reports_dropped_jsonl_tail`, `migrate_pin_wire_reports_dropped_jsonl_tail`, `migrate_pin_cite_reports_dropped_jsonl_tail` | M092-S01 |
| #9 kutha-migrate CLI contract | `SEM-m093-migrate-cli-contract` | `kutha_migrate_help_exits_zero_on_stdout`, `kutha_migrate_rejects_extra_positionals` | M093-S01 |
| #6–#8 dictionary silent-pass | `SEM-m094-dict-load-fail-closed` | `dict-load-fail-closed` + selftest load-error oracles | M094-S01 |
| #3 retract second-phase full live scan | `SEM-m095-retract-second-phase-index` | `retract_second_phase_examines_constant_in_unrelated_live_d`, `retract_non_trigger_still_drops_after_premise_rel_index` | M095-S01 |
| #2 bench rss_before before factor loops | `SEM-m096-bench-rss-before-sample` | `bench_rss_before_is_taken_immediately_before_sample` | M096-S01 |
| #4 Horn point-gated multi-premise miss | `SEM-m097-horn-overlap-aware` | `late_starting_premise_fires_when_trigger_starts_earlier`, `horn_overlap_discovery_is_order_independent` | M097-S01 |
| #5 kinds-split oracle was test-matrix | `SEM-m054-id-kinds-split` | `kinds-family-split` | M098-S01 |
| Correct/CorrectInterval of Behavior leaves retract indexes | `SEM-m100-correct-unindex-behavior` | `correct_of_behavior_drops_retract_index`, `correct_interval_of_behavior_drops_retract_index` | M100-S01 |
| rule-pin VT truncation (pin starts after the trigger) | `SEM-m101-horn-pin-overlap` | `late_starting_rule_pin_fires_on_overlap` | M101-S01 |
| seg-files left after migrate verify failure | `SEM-m102-migrate-verify-segments` | `migrate_verify_fault_leaves_no_segment_files` | M102-S01 |
| keep-first duplicate Define TermId | `SEM-m103-define-keep-first` | `from_strings_keeps_first_id_for_duplicate_name` | M103-S01 |
| unindex_behavior scans every cause bucket | `SEM-m104-unindex-cause-pointer` | `unindex_cause_examines_constant_in_live_d` | M104-S01 |
| policy pin election scans every pin | `SEM-m105-policy-pin-newest` | `newest_policy_pin_examines_one_live_row` | M105-S01 |
| unindex scans every premise-rel bucket | `SEM-m105-unindex-premise-pointer` | `unindex_premise_examines_only_that_behaviors_rels` | M105-S01 |
| open copies FoldIndexMode and keeps the env lease | `SEM-m106-open-index-lease` | `open_keeps_lazy_scan_when_env_is_eager`, `open_keeps_eager_range_when_env_is_lazy` | M106-S01 |
| unindex walks every live Behavior on a non-behavior | `SEM-m107-unindex-live-skip` | `unindex_non_behavior_skips_live_behavior_vec` | M107-S01 |
| unindex of one Behavior walks every live id | `SEM-m108-unindex-live-set` | `unindex_one_behavior_live_visits_stay_constant` | M108-S01 |
| warm known_at walks every fact including later decoys | `SEM-m109-known-at-ingest` | `known_at_examines_stay_constant_vs_late_decoys` | M109-S01 |

Testing gaps from the same report, closed in M099 (not new product findings):

| Gap | Oracle |
|-----|--------|
| `--check ""` runs the full suite | `test_precommit_empty_check_flag_is_contract_error` |
| `yaml_map_list` drops scalar rows | `test_yaml_map_list_rejects_scalar_rows` |
| multi-check asserts inclusion only | exact check-set in `test_multi_check.py` |
| `migrate_compat` order unpinned | `test_migrate_compat_calls_op_wire_before_pins` |
| mapped finding loses its oracle | `test_mapped_findings_name_living_oracles` |

## Rule

New CE findings that land a fix must add a `killed` matrix row with a real
`kill_mutation` and named `#[test]` / check oracle in the same wave. Gap rows
are the honest parking lot until a slice is leased — not a silent pass.
