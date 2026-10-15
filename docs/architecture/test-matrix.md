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

**M052 S02 (cascade atomicity):** Phase 113 — `SEM-m052-cascade-atomic` `killed` (owner `M052-S02`). Oracles: `later_hop_unknown_head_leaves_log_fold_outcomes_unchanged`, `inverse_knows_cascade_writes_quantum_outcome`.

**M052 S03 (lock before heal):** Phase 114 — `SEM-m052-lock-before-heal` `killed` (owner `M052-S03`). Oracles: `losing_attach_while_writer_holds_lock_does_not_heal`, `attach_heals_interrupted_seal_after_lock_released`.

**M052 S04 (snapshot-free open replay):** Phase 115 — `SEM-m052-open-replay` `killed` (owner `M052-S04`). Oracles: `snapshot_backed_open_rejects_empty_pin_inverse`, `define_backed_open_without_snapshot_rejects_empty_pin_inverse`, `terms_backed_open_without_snapshot_rejects_empty_pin_inverse`.

**M053 S01 (CE review regressions):** Phase 116 — `SEM-m053-hydrate-outcomes`, `SEM-m053-persist-dir-anchor`, `SEM-m053-rollover-layout-seam`, `SEM-m053-legacy-meta-ops-fail-open` `killed` (owner `M053-S01`). Map: `docs/process/ce-review-regression-map.md`.

**M054 S01 (Horn soundness):** Phase 117 — `SEM-m054-horn-correct-eligible`, `SEM-m054-horn-nontrigger-retract`, `SEM-m054-horn-vt-intersect`, `SEM-m054-horn-residual-dedup`, `SEM-m054-horn-no-dup-retract` `killed` (owner `M054-S01`).

**M054 S02 (Persist durability):** Phase 118 — `SEM-m054-persist-torn-heal`, `SEM-m054-persist-gen-unique` `killed` (owner `M054-S02`).

**M054 S03 (Governor honesty):** Phase 119 — `SEM-m054-gov-plan-budget`, `SEM-m054-gov-matrix-schema`, `SEM-m054-gov-pool-once` `killed` (owner `M054-S03`).

**M054 S04 (Scaling + bench):** Phase 120 — `SEM-m054-scale-bench-rss` / `bench-asof` / `retract-index` / `horn-rel` / `append-suffix` / `pin-index` `killed` (owner `M054-S04`).

**M054 S05 (ID + structure):** Phase 121 — `SEM-m054-id-uuid-v8` / `id-shrink-sha` / `id-sem-split` / `id-kinds-split` `killed` (owner `M054-S05`).

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
