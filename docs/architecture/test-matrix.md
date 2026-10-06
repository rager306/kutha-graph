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

Counted from `test-matrix.yaml` rows on 2026-10-06. Recount after you edit the YAML; do not invent a coverage percentage. Status counts unchanged (Phase 46 rewrote three blocked invariants; no row added or re-statused).

| Status | Rows |
|--------|------|
| killed | 55 |
| characterization | 6 |
| gap | 0 |
| blocked | 6 |
| spec-only | 4 |
| gate | 1 |
| **total** | **72** |

Planes: product vs harness. Layers: decision, characterization, property, crash, kill, governor, bench.

## Gaps by slice

**M019-S05 (durability A):** closed in Phase 60 — 12 `killed` + `DUR-mixed-typed-dict` `characterization` (GROUP-01 verdict (c); Typed/Dictionary SelfCommit prefix can be visible after a crash).

**M019-S06 (durability B):** closed in Phase 61 — `DUR-export-race` and `DUR-rocks-mid-stale` `killed`; `DUR-bytes-per-fact` `characterization` (streaming persist + drop `encoded_log`; `bytes_per_fact` is whole-bench VmRSS delta / live facts; residual rise not shown as an engine lower bound). Spec-only: `DUR-dead-code-fate`, `DUR-index-wiring`. Bench controller: `BENCH-controller`.

**M019-S07 (semantic expansion):** closed in Phase 62 — `SEM-cascade-order`, `SEM-csr-fold-diff`, `SEM-bitemporal-model`, `SEM-order-independence`, `SEM-replay-idempotence`, `SEM-persist-roundtrip-random`, `SEM-replay-tamper-random` `killed`; `SEM-horn-correct-char` `characterization` (HORN-CORR-01; Correct/CorrectInterval of a Horn premise does not retract or re-mint derived inverse today; superseded premise is not live at the post-correction cut). Default ≥64 seeds; `KUTHA_SEEDS` may raise; persist/tamper/export use `max(12, n/5)`. Tamper equal-fold swap is `commutes` only on disjoint (s,r); otherwise a dependent Retract is minted so `KUTHA_SEEDS=256` seed=79 stays green. Persist: two passes per index (legacy, rollover, migrated, export-imported; WithHorn through segmented/migrated). `Fact::is_live_at` vs naive at EDGE_VT (skip `valid_from` is not equivalent: Eager `live_at` via `vt_by_from` would mask it). CRC: fail-closed InvalidData plus detector pin `CRC mismatch`.

`unowned` is rejected by the governor check.

## Blocked by decision

| Decision | Row | What is waiting |
|----------|-----|-----------------|
| HORN-KC-01 | `SEM-horn-kc` | Ingest-at-head: Horn fires at log-head TT. Killed pin: `known04_horn_fires_when_premise_hidden_on_early_cut` (do not invert). Status stays blocked. |
| C01 | `SEM-c01-decision` | Pack stamp; not fifth axis, not Horn, not `valid_to`-as-application. Oracle: `fixg01_c01_sunset_inexpressible_as_horn`. Status stays blocked (legal pack out of wedge). |
| ASOF-ELIG-01 | `SEM-asof-elig` | `as_of` stays support-live; eligibility is `derivation_eligible_at`. Oracle: `asof_elig01_as_of_contains_derived_when_ineligible`. Status stays blocked. |
| DET-02 | `SEM-det02-ids` | Seed-stable derived EventId vs keep UUID v7 and compare triples. |
| TR-02 | `SEM-tr02-legal` | Force vs publication vs decision. FF5 is two VT windows, not this. |
| HORN-CORR-01 | `SEM-horn-correct` | Correct of a Horn premise. S07 may characterize current behaviour without choosing the rule. |

Do not implement a behaviour change for these rows in S04. Do not mark them `#[ignore]`.

## Non-claims (D023-5)

- This page is not a coverage number and not a pass/fail of the product.
- Governor green on `test-matrix` means the named oracles exist and the statuses are internally consistent.
- Governor green ≠ ADR **Accepted** ≠ capability.
- Listing an M010–M017 oracle does not re-open that slice and does not authorise changing those assertions.
- `cargo-mutants` on the host is a verifier tool, not a gate.
