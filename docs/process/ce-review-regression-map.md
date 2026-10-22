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
| Multi-residual premise combinations (no oracle) | `SEM-m128-horn-multi-combination` | `two_disjoint_ab_windows_mint_two_ac_heads` | M128-S01 |
| `yaml_needles_in_glob` DERIVABLE but unused live | `SEM-m129-yaml-needles-live` | `yaml-needles-live` | M129-S01 |
| selftest parallel submit no serial backstop | `SEM-m130-selftest-submit-backstop` | `selftest-submit-backstop` | M130-S01 |
| empty yaml_map_list select vacuous pass | `SEM-m131-yaml-map-list-nonempty` | `yaml-map-list-nonempty` | M131-S01 |
| reconcile set_len + sync_data (length may not persist) | `SEM-m132-reconcile-sync-all` | `reconcile_truncation_uses_sync_all_after_set_len` | M132-S01 |
| migrate_compat swapped pins-then-op-wire still green | `SEM-m133-migrate-compat-order` | `migrate_compat_calls_op_wire_before_pins` | M133-S01 |
| pin-cite orphan rematerialize needs delete marker | `SEM-m134-pin-cite-delete-rerun` | `migrate_pin_cite_requires_delete_marker_to_rematerialize_after_new_evidence` | M134-S01 |
| migrate_pins unlock window between steps untested | `SEM-m135-migrate-pins-unlock-window` | `migrate_pins_and_compat_do_not_take_parent_lock`, `migrate_pins_gap_blocks_peer_cite_then_converges` | M135-S01 |
| N derived + N retracts examine quadratic | `SEM-m136-retract-n-scaling` | `retract_n_derived_examines_grow_near_linear_not_quadratic` | M136-S01 |
| text-coupled parallel-check-error-parity needles | `SEM-m137-parallel-error-behavioral` | `parallel-check-error-parity` / `test_eval_check_worker_maps_run_exception_to_check_error` | M137-S01 |
| text-coupled kinds-family-split type-annotation needle | `SEM-m138-kinds-family-behavioral` | `kinds-family-split` / `test_runners_dispatch_uses_sibling_family_callables` | M138-S01 |
| multi-premise retract same-rel relation-bucket O(N²) | `SEM-m139-retract-multiprem-fact-index` | `retract_n_multiprem_examines_grow_near_linear_not_quadratic` | M139-S01 |
| text-coupled agent-surface `__main__` hint needles | `SEM-m140-agent-surface-hint-behavioral` | `agent-surface` / `test_check_parser_helpers_are_live_callables` | M140-S01 |
| text-coupled parallel-context-flags docstring needles | `SEM-m141-parallel-context-behavioral` | `parallel-context-flags` / `test_eval_check_worker_propagates_skip_emit_and_changed_paths` | M141-S01 |
| text-coupled observe-skipped assert-snippet needles | `SEM-m142-observe-skipped-behavioral` | `observe-skipped-breadcrumb` / `test_second_observe_cargo_notes_skip_after_prior_high` | M142-S01 |
| text-coupled skip-cargo-log-marker comment needles | `SEM-m143-skip-cargo-marker-behavioral` | `skip-cargo-log-marker` / `test_skip_cargo_does_not_call_cargo_or_tenant` | M143-S01 |
| text-coupled git-auto-base run_high=/tip_high= needles | `SEM-m144-git-auto-base-behavioral` | `git-auto-base` / `test_cmd_ci_prints_run_high_and_tip_high` | M144-S01 |
| cargo exit 101 mints N× observe-missing | `SEM-m145-observe-incomplete` | `observe-incomplete-honesty` / `test_cargo_incomplete_collapses_missing_when_exit_nonzero` | M145-S01 |
| cargo incomplete fails quantum without retry | `SEM-m146-observe-retry` | `observe-retry-incomplete` / `test_observe_retries_once_on_incomplete_then_succeeds` | M146-S01 |
| cargo pack fails on LOW; silent retry recovery | `SEM-m147-observe-retried` | `observe-retried-breadcrumb` / `test_observe_emits_retried_low_after_incomplete_recovery` | M147-S01 |
| observe-retried invisible in ci summary | `SEM-m148-observe-retried-summary` | `observe-retried-summary` / `test_cmd_ci_prints_observe_retried_count` | M148-S01 |
| partial cargo ok + exit 101 → N× missing, no retry | `SEM-m149-partial-observe-incomplete` | `observe-partial-incomplete` / `test_cargo_partial_ok_nonzero_is_incomplete` | M149-S01 |
| bare observe-fail hides elapsed/stderr | `SEM-m150-observe-fail-detail` | `observe-fail-detail` / `test_run_cargo_observe_fail_includes_elapsed_and_stderr_tail` | M150-S01 |
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
| rebuild reindexes dead Behavior into derived_by_cause | `SEM-m110-rebuild-live-derived` | `open_after_correct_keeps_dead_behavior_out_of_retract_index` | M110-S01 |
| hot-path README/code needles drift | `SEM-m111-hot-path-dogfood` | `hot-path-lease-honesty` | M111-S01 |
| unindex full-scans when reverse pointer missing | `SEM-m111-unindex-no-fallback` | `unindex_cause_examines_constant_in_live_d` | M111-S01 |
| replace_regex keeps pattern as substring → VACUOUS | `SEM-m112-selftest-substring` | `selftest-substring-honesty` / `test_replace_regex_with_still_contains_pattern_is_unproven` | M112-S01 |
| declared selftest replace_regex keeps pattern → silent load | `SEM-m113-selftest-mutation-static` | `dict-load-fail-closed` / `test_replace_regex_with_still_contains_pattern_is_load_error` | M113-S01 |
| concurrent cargo/bench races observe_cargo → exit 101 | `SEM-m114-cargo-observe-lock` | `cargo-observe-lock` / `test_observe_times_out_when_cargo_lock_held` | M114-S01 |
| `ci --check ""` runs the full suite | `SEM-m115-empty-check-on-ci` | `agent-surface` / `test_ci_rejects_empty_check_flag` | M115-S01 |
| shared `tmp/p32-02-store` races parallel Cargo | `SEM-m116-known-on-store-dir` | `persist_open_store_dirs_differ_across_calls` | M116-S01 |
| shared `tmp/p32-01-store` races parallel Cargo | `SEM-m117-quantum-known-on-store-dir` | `quantum_known_on_store_dirs_differ_across_calls` | M117-S01 |
| shared `tmp/p34-01-store` races parallel Cargo | `SEM-m118-closed-vt-known-on-store-dir` | `closed_vt_known_on_store_dirs_differ_across_calls` | M118-S01 |
| shared `tmp/p34-02-store` races parallel Cargo | `SEM-m119-anchor-admit-store-dir` | `anchor_admit_store_dirs_differ_across_calls` | M119-S01 |
| shared `tmp/p36-02-fork` races parallel Cargo | `SEM-m120-fork-known-on-store-dir` | `fork_known_on_store_dirs_differ_across_calls` | M120-S01 |
| shared `tmp/p27-facade-statute` races parallel Cargo | `SEM-m121-facade-statute-store-dir` | `facade_statute_store_dirs_differ_across_calls` | M121-S01 |
| `Path::new("tmp/…")` under crates | `SEM-m122-fixed-tmp-persist-dir` | `fixed-tmp-persist-dir` | M122-S01 |
| `.join("tmp").join("literal")` under crates | `SEM-m123-fixed-tmp-join` | `fixed-tmp-persist-dir` | M123-S01 |
| `glob_regex_absent` UNPROVEN without YAML mutations | `SEM-m124-glob-regex-derive` | `glob-regex-absent-derive` | M124-S01 |
| `yaml_needles_in_glob` not DERIVABLE (zero live users) | `SEM-m125-yaml-needles-derive` | `yaml-needles-derive` | M125-S01 |
| `kinds.py` re-growth after family split (no line pin) | `SEM-m126-kinds-lean` | `kinds-lean` | M126-S01 |
| `selftest.py` unbounded growth (no line pin) | `SEM-m127-selftest-lean` | `selftest-lean` | M127-S01 |

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
