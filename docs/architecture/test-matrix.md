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

Counted from `test-matrix.yaml` rows on 2026-10-05. Recount after you edit the YAML; do not invent a coverage percentage.

| Status | Rows |
|--------|------|
| killed | 34 |
| characterization | 3 |
| gap | 17 |
| blocked | 6 |
| spec-only | 4 |
| gate | 1 |
| **total** | **65** |

Planes: product vs harness. Layers: decision, characterization, property, crash, kill, governor, bench.

## Gaps by slice

**M019-S05 (durability A):** `DUR-seg-manifest-sync-all`, `DUR-seg-final-sync-dir`, `DUR-mixed-typed-dict`, `DUR-migrate-tail-guard`, `DUR-active-exact-frame-end`, `DUR-wrong-role-active`, `DUR-two-writer`, `DUR-chain-active-prev`, `DUR-manifest-pre-rename`, `DUR-sealed-trailer-lie`, `DUR-migrate-concurrent`.

**M019-S06 (durability B):** `DUR-export-race`, `DUR-rocks-mid-stale`, `DUR-bytes-per-fact`. Spec-only on the same slice: `DUR-dead-code-fate`, `DUR-index-wiring`. Bench controller: `BENCH-controller`.

**M019-S07 (semantic expansion):** `SEM-cascade-order`, `SEM-csr-fold-diff`, `SEM-horn-correct-char`.

`unowned` is rejected by the governor check.

## Blocked by decision

| Decision | Row | What is waiting |
|----------|-----|-----------------|
| HORN-KC-01 | `SEM-horn-kc` | Horn must not mint when a premise is absent at the knowledge cut. Current pin: `known04_horn_fires_when_premise_hidden_on_early_cut` (characterization). |
| C01 | `SEM-c01-decision` | Sunset / `tempus regit actum` option. Current pin: `fixg01_c01_sunset_inexpressible_as_horn`. |
| ASOF-ELIG-01 | `SEM-asof-elig` | `as_of` vs derivation eligibility. |
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
