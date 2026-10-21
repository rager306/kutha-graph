# Test matrix (human page)

Machine-readable source: [`test-matrix.yaml`](./test-matrix.yaml). Do not copy rows here. Cell: [ADR-023](../ADR/ADR-023-test-strategy-and-oracle-matrix.md) (**Proposed**). Governor check: `test-matrix`.

## How to read

Each YAML row is one invariant. `oracle` is a list of real `#[test]` function names or governor check ids (copy the identifier exactly). `status` is honest:

| Status | Means |
|--------|--------|
| `killed` | Named test exists and a `kill_mutation` is recorded (not `none`) |
| `characterization` | Pins current behaviour; labelled; may be replaced when the owner decides |
| `gap` | No kill yet; `owner` is the M019 slice that will close it |
| `blocked` | Waits on a named decision; never `#[ignore]` / xfail |
| `spec-only` | Recorded fate or process proof that is not a check id |
| `gate` | Controller or feature observe; not a coverage number |

`gate: feature:observe_rocks` is the Phase 57 template: the named tests live under `cfg(feature)` and the FSM row must exist. `gate: controller` is honour-system (bench).

## Counts (this file)

Counted from `test-matrix.yaml` rows on 2026-10-15 (M089 S01). Recount after you edit the YAML; do not invent a coverage percentage.

| Status | Rows |
|--------|------|
| killed | 167 |
| characterization | 11 |
| gap | 0 |
| blocked | 0 |
| spec-only | 4 |
| gate | 1 |
| **total** | **183** |

Planes: product vs harness. Layers: decision, characterization, property, crash, kill, governor, bench.

## Gaps by slice

**M019-S05 (durability A):** closed in Phase 60 — 12 `killed` + `DUR-mixed-typed-dict` `characterization` (GROUP-01 verdict (c); Typed/Dictionary SelfCommit prefix can be visible after a crash).

**M019-S06 (durability B):** closed in Phase 61 — `DUR-export-race` and `DUR-rocks-mid-stale` `killed`; `DUR-bytes-per-fact` `characterization` (streaming persist + drop `encoded_log`; `bytes_per_fact` is whole-bench VmRSS delta / live facts; residual rise not shown as an engine lower bound). Spec-only: `DUR-dead-code-fate`, `DUR-index-wiring`. Bench controller: `BENCH-controller`.

**M019-S07 (semantic expansion):** closed in Phase 62 — `SEM-cascade-order`, `SEM-csr-fold-diff`, `SEM-bitemporal-model`, `SEM-order-independence`, `SEM-replay-idempotence`, `SEM-persist-roundtrip-random`, `SEM-replay-tamper-random` `killed`. Default ≥64 seeds; `KUTHA_SEEDS` may raise; persist/tamper/export use `max(12, n/5)`. Tamper equal-fold swap is `commutes` only on disjoint (s,r); otherwise a dependent Retract is minted so `KUTHA_SEEDS=256` seed=79 stays green. Persist: two passes per index (legacy, rollover, migrated, export-imported; WithHorn through segmented/migrated). `Fact::is_live_at` vs naive at EDGE_VT (skip `valid_from` is not equivalent: Eager `live_at` via `vt_by_from` would mask it). CRC: fail-closed InvalidData plus detector pin `CRC mismatch`.

**M018-S03 (Horn closure):** Phase 47 — `SEM-det02-ids`, `SEM-horn-correct`, `SEM-horn-correct-char`, `SEM-casc-admit` `killed` (owner `M018-S03`). Oracles: `det02_two_ingest_derived_event_ids_equal`, inverted `horn_correct_*`, `casc01_*`.

**M018-S04 (oracle tightening):** Phase 48 — `ORACLE-if15` / `ORACLE-if16` `killed` (owner `M018-S04`). GATE-01 is `BENCH-controller` (`status: gate`). DIFF-01 stays closed as `SEM-csr-fold-diff`.

**M024 (fold identity + write-path):** Phase 79–80 — `SEM-m024-fold-identity` / `SEM-m024-write-path` `characterization` (owners `M024-S01` / `M024-S02`). Oracles: `m024_fold_identity`, `m024_write_path`.

**M025 S01 (as_known_on TT-only):** Phase 81 — `SEM-m025-akn-tt-only` `killed` (owner `M025-S01`). Oracles: `m025_as_known_on_vt`.

**M025 S02 (clock-safe facade):** Phase 82 — `SEM-m025-clock-facade` `killed` (owner `M025-S02`). Oracles: `ui_clock_facade`.

**M025 S03 (legal answer envelope):** Phase 83 — `SEM-m025-norm-view` `killed` (owner `M025-S03`). Oracles: `m025_norm_view`.

**M025 S04 (demo + fixtures):** Phase 84 — `SEM-m025-demo-fixtures` `killed` (owner `M025-S04`). Oracles: `m025_demo_fixtures`; example `statute_as_of`.

**M026 S01 (generators / missing oracles):** Phase 85 — `SEM-m026-generators` / `SEM-m026-gc04-assert-id` / `SEM-m026-gc05-facade` / `SEM-m026-gc11-empty` `killed` (owner `M026-S01`). Oracles: `m026_generators`.

**M026 S02 (matrix honesty):** Phase 86 — honor-system rows re-labelled or given real product mutants; tautologies moved to `characterization` (`SEM-naive-as-of-alias`, `SEM-known-at-mirror`, `SEM-seed-budget`; `SEM-seeded-det` → characterization). Oracles: `m026_matrix_honesty`.

**M028 S01 (dict retract on open):** Phase 88 — `SEM-m028-dict-retract` `killed` (owner `M028-S01`). Oracles: `m028_dict_retract`.

**M029 S01 (rollover seal heal):** Phase 89 — `SEM-m029-rollover-heal` `killed` (owner `M029-S01`). Oracles: `m029_rollover_heal`.

**M030 S01 (writer lock retention):** Phase 90 — `SEM-m030-writer-lock` `killed` (owner `M030-S01`). Oracles: `m030_writer_lock`.

**M031 S01 (WAL-tail heal):** Phase 91 — `SEM-m031-wal-tail-heal` `killed` (owner `M031-S01`). Oracles: `m031_wal_tail_heal`.

**M032 S01 (derive live premise):** Phase 92 — `SEM-m032-derive-live-premise` `killed` (owner `M032-S01`). Oracles: `m032_derive_live_premise`.

**M033 S01 (CI admission VT):** Phase 93 — `SEM-m033-ci-admission-vt` `killed` (owner `M033-S01`). Oracles: `m033_ci_admission_vt`.

**M034 S01 (Correct horn re-fire):** Phase 94 — `SEM-m034-correct-refire` `killed` (owner `M034-S01`). Oracles: `m034_correct_refire`.

**M035 S01 (replay dict bind):** Phase 95 — `SEM-m035-replay-dict-bind` `killed` (owner `M035-S01`). Oracles: `m035_replay_dict_bind`.

**M036 S01 (VT-aware Horn dedup):** Phase 96 — `SEM-m036-vt-aware-dedup` `killed` (owner `M036-S01`). Oracles: `m036_vt_aware_dedup`.

**M041 S01 (persist-trace identity + shrink gate):** Phase 101 — `SEM-m041-persist-trace-identity` / `SEM-m041-shrink-ddmin` `killed` (owner `M041-S01`). Oracles: `m041_persist_trace_identity`. Open path uses `expected_open_event_ids` (not export Define-prefix).

**M042 S01 (m028×layouts + shrink corpus):** Phase 102 — `SEM-m042-dict-retract-layouts` / `SEM-m042-shrink-corpus` `killed` (owner `M042-S01`). Oracles: `m042_m028_layouts`.

**M043 S01 (FSM quantum honest name):** Phase 103 — `SEM-m043-fsm-quantum-honest` `killed` (owner `M043-S01`). Oracle: `fsm-quantum-honest-name`.

**M044 S01 (nameless dict retain):** Phase 104 — `SEM-m044-nameless-dict-retain` `killed` (owner `M044-S01`). Oracle: `nameless-dict-retain`.

**M045 S01 (glob_absent comment policy):** Phase 105 — `SEM-m045-glob-absent-comment` `killed` (owner `M045-S01`). Oracle: `glob-absent-comment-policy`.

**M046 S01 (parallel check-error parity):** Phase 106 — `SEM-m046-parallel-check-error` `killed` (owner `M046-S01`). Oracle: `parallel-check-error-parity`.

**M047 S01 (observe-skipped breadcrumb):** Phase 107 — `SEM-m047-observe-skipped` `killed` (owner `M047-S01`). Oracle: `observe-skipped-breadcrumb`.

**M048 S01 (parallel Context flags):** Phase 108 — `SEM-m048-parallel-context-flags` `killed` (owner `M048-S01`). Oracle: `parallel-context-flags`.

**M049 S01 (skip-cargo log marker):** Phase 109 — `SEM-m049-skip-cargo-log-marker` `killed` (owner `M049-S01`). Oracle: `skip-cargo-log-marker`.

**M050 S01 (Op compat policy):** Phase 110 — `SEM-m050-op-compat-policy` `killed` (owner `M050-S01`). Oracles: `legacy_allow_relation_op_fails_to_deserialize`, `modern_kutha_allows_assert_roundtrips`.

**M051 S01 (pin scheme stability):** Phase 111 — `SEM-m051-pin-compat-policy` `killed` (owner `M051-S01`). Oracles: `ident_pin_matches_hardcoded_v1_vector`, `ident_pin_diverges_from_legacy_utf8_domain_vector`, `nonident_pin_matches_hardcoded_domain_utf8_vector`.

**M071 S01 (pin-wire Behavior migrate):** Phase 142 — `SEM-m071-pin-wire` `killed` (owner `M071-S01`). Oracles: `migrate_pin_wire_rewrites_behavior_legacy_utf8_pin`, `migrate_pin_wire_leaves_v1_pin_unchanged`, `remap_derive_pq_legacy_utf8_to_v1`.

**M072 S01 (pin-cite JustificationCite migrate):** Phase 143 — `SEM-m072-pin-cite` `killed` (owner `M072-S01`). Oracles: `migrate_pin_cite_rewrites_justification_legacy_utf8_pin`, `migrate_pin_cite_leaves_orphan_legacy_cite_unchanged`, `remap_events_pins_rewrites_cite_when_behavior_evidences_ident`.

**M073 S01 (git auto clean-tree coupling):** Phase 144 — `SEM-m073-git-auto-base` `killed` (owner `M073-S01`). Oracle: `git-auto-base`.

**M074 S01 (agent surface discoverability):** Phase 145 — `SEM-m074-agent-surface` `killed` (owner `M074-S01`). Oracle: `agent-surface`.

**M075 S01 (pin migrate sequence facade):** Phase 146 — `SEM-m075-migrate-pins` `killed` (owner `M075-S01`). Oracle: `migrate_pins_rewrites_behavior_then_cite_in_one_call`.

**M076 S01 (compat migrate sequence facade):** Phase 147 — `SEM-m076-migrate-compat` `killed` (owner `M076-S01`). Oracle: `migrate_compat_rewrites_op_wire_then_pins_in_one_call`.

**M077 S01 (multi-check CLI select):** Phase 148 — `SEM-m077-multi-check` `killed` (owner `M077-S01`). Oracle: `agent-surface` (pytest `test_multi_check` enrolled via check needles).

**M078 S01 (compat migrate operator CLI):** Phase 149 — `SEM-m078-migrate-cli` `killed` (owner `M078-S01`). Oracles: `kutha_migrate_runs_compat_on_store_dir`, `kutha_migrate_missing_dir_is_contract_error`.

**M079 S01 (repeatable --check flags):** Phase 150 — `SEM-m079-check-append` `killed` (owner `M079-S01`). Oracle: `agent-surface` (pytest `test_precommit_repeated_check_flags_runs_both`).

**M080 S01 (observe build both operator bins):** Phase 151 — `SEM-m080-migrate-build` `killed` (owner `M080-S01`). Oracle: `tenant-bin` (pytest `test_observe_default_build_includes_migrate`).

**M081 S01 (built migrate bin discoverability):** Phase 152 — `SEM-m081-migrate-bin-path` `killed` (owner `M081-S01`). Oracle: `agent-surface` (pytest `test_agents_md_prefers_built_migrate_bin`).

**M082 S01 (process notes built migrate path):** Phase 153 — `SEM-m082-migrate-process-notes` `killed` (owner `M082-S01`). Oracles: `op-compat-policy`, `pin-compat-policy`, `agent-surface` (pytest `test_process_notes_prefer_built_migrate_bin`).

**M083 S01 (harness CutPort migrate-bin):** Phase 154 — `SEM-m083-harness-migrate-cutport` `killed` (owner `M083-S01`). Oracles: `dogfood`, `agent-surface` (pytest `test_harness_cutport_names_built_migrate`).

**M084 S01 (.env.example built migrate path):** Phase 155 — `SEM-m084-env-migrate-path` `killed` (owner `M084-S01`). Oracles: `toolchain`, `agent-surface` (pytest `test_env_example_documents_built_migrate`).

**M085 S01 (json --check append oracle):** Phase 156 — `SEM-m085-json-check-append` `killed` (owner `M085-S01`). Oracle: `agent-surface` (pytest `test_json_repeated_check_flags_runs_both`).

**M086 S01 (selftest --check append oracle):** Phase 157 — `SEM-m086-selftest-check-append` `killed` (owner `M086-S01`). Oracle: `agent-surface` (pytest `test_selftest_repeated_check_flags_runs_both`).

**M087 S01 (harness --check surface discoverability):** Phase 158 — `SEM-m087-harness-check-surfaces` `killed` (owner `M087-S01`). Oracles: `dogfood`, `agent-surface` (pytest `test_harness_documents_check_surfaces`).

**M088 S01 (AGENTS --check not for ci):** Phase 159 — `SEM-m088-agents-check-not-ci` `killed` (owner `M088-S01`). Oracle: `agent-surface` (pytest `test_agents_md_check_not_for_ci`, `test_ci_rejects_check_flag`).

**M089 S01 (--check reject redirect hint):** Phase 160 — `SEM-m089-check-reject-hint` `killed` (owner `M089-S01`). Oracle: `agent-surface` (pytest `test_ci_rejects_check_flag` hint line).

**M090 S01 (--skip-cargo reject redirect hint):** Phase 161 — `SEM-m090-skip-cargo-reject-hint` `killed` (owner `M090-S01`). Oracle: `agent-surface` (pytest `test_non_ci_rejects_skip_cargo_flag` hint line).

**M091 S01 (pin-cite justifications sidecar):** Phase 162 — `SEM-m091-pin-cite-justifications` `killed` (owner `M091-S01`). Oracle: `migrate_pin_cite_reopens_after_nonempty_justifications_sidecar`.

**M092 S01 (migrate dropped_tail):** Phase 163 — `SEM-m092-migrate-dropped-tail` `killed` (owner `M092-S01`). Oracles: `migrate_op_wire_reports_dropped_jsonl_tail`, `migrate_pin_wire_reports_dropped_jsonl_tail`, `migrate_pin_cite_reports_dropped_jsonl_tail`.

**M093 S01 (kutha-migrate CLI contract):** Phase 164 — `SEM-m093-migrate-cli-contract` `killed` (owner `M093-S01`). Oracles: `kutha_migrate_help_exits_zero_on_stdout`, `kutha_migrate_rejects_extra_positionals`.

**M094 S01 (dictionary loader fail-closed):** Phase 165 — `SEM-m094-dict-load-fail-closed` `killed` (owner `M094-S01`). Oracle: `dict-load-fail-closed` (pytest scalar/dup/stepless load errors).

**M095 S01 (retract second-phase index):** Phase 166 — `SEM-m095-retract-second-phase-index` `killed` (owner `M095-S01`). Oracles: `retract_second_phase_examines_constant_in_unrelated_live_d`, `retract_non_trigger_still_drops_after_premise_rel_index`.

**M096 S01 (bench rss_before at sample):** Phase 167 — `SEM-m096-bench-rss-before-sample` `killed` (owner `M096-S01`). Oracle: `bench_rss_before_is_taken_immediately_before_sample`.

**M097 S01 (Horn overlap-aware discovery):** Phase 168 — `SEM-m097-horn-overlap-aware` `killed` (owner `M097-S01`). Oracles: `late_starting_premise_fires_when_trigger_starts_earlier`, `horn_overlap_discovery_is_order_independent`. Design: overlap-aware (not retry-at-candidate-starts).

**M100 S01 (Correct unindexes Behavior):** Phase 171 — `SEM-m100-correct-unindex-behavior` `killed` (owner `M100-S01`). Oracles: `correct_of_behavior_drops_retract_index`, `correct_interval_of_behavior_drops_retract_index`.

**M101 S01 (Horn pin overlap):** Phase 172 — `SEM-m101-horn-pin-overlap` `killed` (owner `M101-S01`). Oracle: `late_starting_rule_pin_fires_on_overlap`.

**M102 S01 (migrate verify drops segments):** Phase 173 — `SEM-m102-migrate-verify-segments` `killed` (owner `M102-S01`). Oracle: `migrate_verify_fault_leaves_no_segment_files`.

**M103 S01 (duplicate Define keeps first id):** Phase 174 — `SEM-m103-define-keep-first` `killed` (owner `M103-S01`). Oracle: `from_strings_keeps_first_id_for_duplicate_name`.

**M104 S01 (unindex cause reverse pointer):** Phase 175 — `SEM-m104-unindex-cause-pointer` `killed` (owner `M104-S01`). Oracle: `unindex_cause_examines_constant_in_live_d`.

**M105 S01 (audit residuals):** Phases 176–179 — `SEM-m105-policy-pin-newest`, `SEM-m105-unindex-premise-pointer` `killed` (owner `M105-S01`). Oracles: `newest_policy_pin_examines_one_live_row`, `unindex_premise_examines_only_that_behaviors_rels`. `FoldIndexMode::Lazy` full scan stays the thin-write contract.

**M106 S01 (open index lease):** Phase 180 — `SEM-m106-open-index-lease` `killed` (owner `M106-S01`). Oracles: `open_keeps_lazy_scan_when_env_is_eager`, `open_keeps_eager_range_when_env_is_lazy`.

**M107 S01 (unindex live skip):** Phase 181 — `SEM-m107-unindex-live-skip` `killed` (owner `M107-S01`). Oracle: `unindex_non_behavior_skips_live_behavior_vec`.

**M108 S01 (live set):** Phase 182 — `SEM-m108-unindex-live-set` `killed` (owner `M108-S01`). Oracle: `unindex_one_behavior_live_visits_stay_constant`.

**M109 S01 (known_at ingest):** Phase 183 — `SEM-m109-known-at-ingest` `killed` (owner `M109-S01`). Oracle: `known_at_examines_stay_constant_vs_late_decoys`.

**M110 S01 (rebuild live derived):** Phase 184 — `SEM-m110-rebuild-live-derived` `killed` (owner `M110-S01`). Oracle: `open_after_correct_keeps_dead_behavior_out_of_retract_index`.

**M111 S01 (hot-path dogfood):** Phase 185 — `SEM-m111-hot-path-dogfood` / `SEM-m111-unindex-no-fallback` `killed` (owner `M111-S01`). Oracles: `hot-path-lease-honesty`, `unindex_cause_examines_constant_in_live_d`, `unindex_premise_examines_only_that_behaviors_rels`.

**M112 S01 (selftest substring):** Phase 186 — `SEM-m112-selftest-substring` `killed` (owner `M112-S01`). Oracle check: `selftest-substring-honesty` (pytest `test_replace_regex_with_still_contains_pattern_is_unproven`).

**M113 S01 (selftest mutation static):** Phase 187 — `SEM-m113-selftest-mutation-static` `killed` (owner `M113-S01`). Oracle check: `dict-load-fail-closed` (pytest `test_replace_regex_with_still_contains_pattern_is_load_error`).

**M114 S01 (cargo observe flock):** Phase 188 — `SEM-m114-cargo-observe-lock` `killed` (owner `M114-S01`). Oracle check: `cargo-observe-lock`.

**M115 S01 (empty --check on ci):** Phase 189 — `SEM-m115-empty-check-on-ci` `killed` (owner `M115-S01`). Oracle check: `agent-surface` (pytest `test_ci_rejects_empty_check_flag`).

**M116 S01 (unique known-on persist dir):** Phase 190 — `SEM-m116-known-on-store-dir` `killed` (owner `M116-S01`). Oracle: `persist_open_store_dirs_differ_across_calls`.

**M117 S01 (unique quantum known-on persist dir):** Phase 191 — `SEM-m117-quantum-known-on-store-dir` `killed` (owner `M117-S01`). Oracle: `quantum_known_on_store_dirs_differ_across_calls`.

**M118 S01 (unique closed-VT known-on persist dir):** Phase 192 — `SEM-m118-closed-vt-known-on-store-dir` `killed` (owner `M118-S01`). Oracle: `closed_vt_known_on_store_dirs_differ_across_calls`.

**M119 S01 (unique anchor-admit persist dir):** Phase 193 — `SEM-m119-anchor-admit-store-dir` `killed` (owner `M119-S01`). Oracle: `anchor_admit_store_dirs_differ_across_calls`.

**M120 S01 (unique fork known-on persist dir):** Phase 194 — `SEM-m120-fork-known-on-store-dir` `killed` (owner `M120-S01`). Oracle: `fork_known_on_store_dirs_differ_across_calls`.

**M121 S01 (unique facade-statute persist dir):** Phase 195 — `SEM-m121-facade-statute-store-dir` `killed` (owner `M121-S01`). Oracle: `facade_statute_store_dirs_differ_across_calls`.

**M122 S01 (fixed-tmp persist-dir dogfood):** Phase 196 — `SEM-m122-fixed-tmp-persist-dir` `killed` (owner `M122-S01`). Oracle check: `fixed-tmp-persist-dir`.

**M123 S01 (fixed-tmp join-after-tmp dogfood):** Phase 197 — `SEM-m123-fixed-tmp-join` `killed` (owner `M123-S01`). Oracle check: `fixed-tmp-persist-dir` (`glob_regex_absent`).

**M124 S01 (glob_regex_absent selftest derive):** Phase 198 — `SEM-m124-glob-regex-derive` `killed` (owner `M124-S01`). Oracle check: `glob-regex-absent-derive`.

**M125 S01 (yaml_needles derive + DERIVABLE completeness):** Phase 199 — `SEM-m125-yaml-needles-derive` `killed` (owner `M125-S01`). Oracle check: `yaml-needles-derive`.

**M126 S01 (kinds.py line ceiling):** Phase 200 — `SEM-m126-kinds-lean` `killed` (owner `M126-S01`). Oracle check: `kinds-lean`.

**M127 S01 (selftest.py line ceiling):** Phase 201 — `SEM-m127-selftest-lean` `killed` (owner `M127-S01`). Oracle check: `selftest-lean`.

**M128 S01 (Horn multi-combination):** Phase 202 — `SEM-m128-horn-multi-combination` `killed` (owner `M128-S01`). Oracle: `two_disjoint_ab_windows_mint_two_ac_heads`. Kill: first-only `PremMatch` in `search_at_all`.

**M129 S01 (yaml_needles live dogfood):** Phase 203 — `SEM-m129-yaml-needles-live` `killed` (owner `M129-S01`). Oracle check: `yaml-needles-live`. Kill: glob → `AGENTS.md`.

**M130 S01 (selftest submit serial backstop):** Phase 204 — `SEM-m130-selftest-submit-backstop` `killed` (owner `M130-S01`). Oracle check: `selftest-submit-backstop`. Kill: dict-comprehension submit.

**M131 S01 (yaml_map_list empty-select):** Phase 205 — `SEM-m131-yaml-map-list-nonempty` `killed` (owner `M131-S01`). Oracle check: `yaml-map-list-nonempty`. Kill: drop `or not rows`.

**M132 S01 (reconcile tip truncate sync_all):** Phase 206 — `SEM-m132-reconcile-sync-all` `killed` (owner `M132-S01`). Oracle: `reconcile_truncation_uses_sync_all_after_set_len`. Kill: `sync_data` after `set_len`.

**M133 S01 (migrate_compat call order):** Phase 207 — `SEM-m133-migrate-compat-order` `killed` (owner `M133-S01`). Oracle: `migrate_compat_calls_op_wire_before_pins`. Kill: swap `migrate_pins` before `migrate_op_wire`.

**M134 S01 (pin-cite delete/rerun):** Phase 208 — `SEM-m134-pin-cite-delete-rerun` `killed` (owner `M134-S01`). Oracle: `migrate_pin_cite_requires_delete_marker_to_rematerialize_after_new_evidence`. Kill: drop marker early-return or `delete pin-cite.v1` needle.

**M135 S01 (migrate_pins unlock window):** Phase 209 — `SEM-m135-migrate-pins-unlock-window` `killed` (owner `M135-S01`). Oracles: `migrate_pins_and_compat_do_not_take_parent_lock`, `migrate_pins_gap_blocks_peer_cite_then_converges`. Kill: parent `acquire_store_write_lock` in `migrate_pins`.

**M136 S01 (retract N-scaling):** Phase 210 — `SEM-m136-retract-n-scaling` `killed` (owner `M136-S01`). Oracle: `retract_n_derived_examines_grow_near_linear_not_quadratic`. Kill: drop single-premise second-phase skip.

**M137 S01 (parallel error behavioral):** Phase 211 — `SEM-m137-parallel-error-behavioral` `killed` (owner `M137-S01`). Oracle check: `parallel-check-error-parity` (pytest `test_eval_check_worker_maps_run_exception_to_check_error`). Kill: `return check.run(ctx)` in worker.

**M138 S01 (kinds-family behavioral):** Phase 212 — `SEM-m138-kinds-family-behavioral` `killed` (owner `M138-S01`). Oracle check: `kinds-family-split` (pytest `test_runners_dispatch_uses_sibling_family_callables`). Kill: rename RUNNERS `test_matrix` key.

**M139 S01 (multiprem retract fact-index):** Phase 213 — `SEM-m139-retract-multiprem-fact-index` `killed` (owner `M139-S01`). Oracle: `retract_n_multiprem_examines_grow_near_linear_not_quadratic`. Kill: restore phase-2 `behaviors_by_premise_rel` candidates.

**M140 S01 (agent-surface hint behavioral):** Phase 214 — `SEM-m140-agent-surface-hint-behavioral` `killed` (owner `M140-S01`). Oracle check: `agent-surface` (pytest `test_check_parser_helpers_are_live_callables` + softened CLI reject oracles). Kill: restore exact `__main__.py` hint-string needles.

**M141 S01 (parallel-context behavioral):** Phase 215 — `SEM-m141-parallel-context-behavioral` `killed` (owner `M141-S01`). Oracle check: `parallel-context-flags` (pytest `test_eval_check_worker_propagates_skip_emit_and_changed_paths`). Kill: restore docstring/assert-snippet needles.

**M142 S01 (observe-skipped behavioral):** Phase 216 — `SEM-m142-observe-skipped-behavioral` `killed` (owner `M142-S01`). Oracle check: `observe-skipped-breadcrumb` (pytest `test_second_observe_cargo_notes_skip_after_prior_high`). Kill: restore assert-snippet needles.

**M143 S01 (skip-cargo-marker behavioral):** Phase 217 — `SEM-m143-skip-cargo-marker-behavioral` `killed` (owner `M143-S01`). Oracle check: `skip-cargo-log-marker` (pytest `test_skip_cargo_does_not_call_cargo_or_tenant`). Kill: restore CE #38 comment/assert-snippet needles.

**M052 S02 (cascade atomicity):** Phase 113 — `SEM-m052-cascade-atomic` `killed` (owner `M052-S02`). Oracles: `later_hop_unknown_head_leaves_log_fold_outcomes_unchanged`, `inverse_knows_cascade_writes_quantum_outcome`.

**M052 S03 (lock before heal):** Phase 114 — `SEM-m052-lock-before-heal` `killed` (owner `M052-S03`). Oracles: `losing_attach_while_writer_holds_lock_does_not_heal`, `attach_heals_interrupted_seal_after_lock_released`.

**M052 S04 (snapshot-free open replay):** Phase 115 — `SEM-m052-open-replay` `killed` (owner `M052-S04`). Oracles: `snapshot_backed_open_rejects_empty_pin_inverse`, `define_backed_open_without_snapshot_rejects_empty_pin_inverse`, `terms_backed_open_without_snapshot_rejects_empty_pin_inverse`.

**M053 S01 (CE review regressions):** Phase 116 — `SEM-m053-hydrate-outcomes`, `SEM-m053-persist-dir-anchor`, `SEM-m053-rollover-layout-seam`, `SEM-m053-legacy-meta-ops-fail-open` `killed` (owner `M053-S01`). Map: `docs/process/ce-review-regression-map.md`.

**M054 S01 (Horn soundness):** Phase 117 — `SEM-m054-horn-correct-eligible`, `SEM-m054-horn-nontrigger-retract`, `SEM-m054-horn-vt-intersect`, `SEM-m054-horn-residual-dedup`, `SEM-m054-horn-no-dup-retract` `killed` (owner `M054-S01`).

**M054 S02 (Persist durability):** Phase 118 — `SEM-m054-persist-torn-heal`, `SEM-m054-persist-gen-unique` `killed` (owner `M054-S02`).

**M054 S03 (Governor honesty):** Phase 119 — `SEM-m054-gov-plan-budget`, `SEM-m054-gov-matrix-schema`, `SEM-m054-gov-pool-once` `killed` (owner `M054-S03`).

**M054 S04 (Scaling + bench):** Phase 120 — `SEM-m054-scale-bench-rss` / `bench-asof` / `retract-index` / `horn-rel` / `append-suffix` / `pin-index` `killed` (owner `M054-S04`).

**M054 S05 (ID + structure):** Phase 121 — `SEM-m054-id-uuid-v8` / `id-shrink-sha` / `id-sem-split` / `id-kinds-split` `killed` (owner `M054-S05`). `id-kinds-split` oracle is `kinds-family-split` (M098 / CE #5).

**M055 S01 (append tip rollback):** Phase 122 — `SEM-gap-persist-poison` (#6) `killed` (owner `M055-S01`). Oracles: `segmented_wal_sync_failpoint_retry_keeps_exact_event_id_sequence`, `segmented_wal_write_failpoint_retry_keeps_exact_event_id_sequence`.

**M056 S01 (lease-bundle):** Phase 123 — `SEM-m056-lease-bundle` `killed` (owner `M056-S01`).

**M056 S02 (persist heal):** Phase 124 — `SEM-m056-persist-short-orphan` / `persist-migrate-sync` `killed` (owner `M056-S02`).

**M056 S03 (delivery-key same-relation):** Phase 125 — `SEM-m056-delivery-key-same-relation` `killed` (owner `M056-S03`).

**M056 S04 (NormView derived Correct):** Phase 126 — `SEM-m056-norm-view-derived-correct` `killed` (owner `M056-S04`).

**M056 S05 (hygiene):** Phase 127 — `SEM-m056-hygiene-uv-only` / `hygiene-fixture-lang` `killed` (owner `M056-S05`).

**M057 S01 (STATE→ROADMAP):** Phase 128 — `SEM-m057-lease-roadmap` `killed` (owner `M057-S01`).

**M058 S01 (disk_free):** Phase 129 — `SEM-m058-disk-free` `killed` (owner `M058-S01`).

**M059 S01 (budget template):** Phase 130 — `SEM-m059-budget-template` `killed` (owner `M059-S01`).

**M060 S01 (budget covers checks):** Phase 131 — `SEM-m060-budget-covers` `killed` (owner `M060-S01`).

`unowned` is rejected by the governor check.

## Decision rows (M021 S01 / Phase 68)

| Decision | Row | Lock |
|----------|-----|------|
| HORN-KC-01 | `SEM-horn-kc` | `killed` — ingest-at-head; product mutant `live_horns` at cut TT (M026 S02) |
| C01 | `SEM-c01-decision` | `killed` — pack stamp; mutant `RuleTerm::constant` calendar (M026 S02) |
| ASOF-ELIG-01 | `SEM-asof-elig` | `killed` — support-live; pin `asof_elig01_*` |
| TR-02 | `SEM-tr02-legal` | `killed` — force=VT; mutant VT-filter on `as_known_on` (M026 S02) |

No `blocked` decision rows remain. Do not mark them `#[ignore]`. Full ADR-090 ontology stays frozen until STATE thaws a legal pack.

## Non-claims (D023-5)

- This page is not a coverage number and not a pass/fail of the product.
- Governor green on `test-matrix` means the named oracles exist and the statuses are internally consistent.
- Governor green ≠ ADR **Accepted** ≠ capability.
- Listing an M010–M017 oracle does not re-open that slice and does not authorise changing those assertions.
- `cargo-mutants` on the host is a verifier tool, not a gate.
