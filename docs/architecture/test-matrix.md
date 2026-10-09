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

Counted from `test-matrix.yaml` rows on 2026-10-09 (M032 S01). Recount after you edit the YAML; do not invent a coverage percentage.

| Status | Rows |
|--------|------|
| killed | 76 |
| characterization | 11 |
| gap | 0 |
| blocked | 0 |
| spec-only | 4 |
| gate | 1 |
| **total** | **92** |

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
