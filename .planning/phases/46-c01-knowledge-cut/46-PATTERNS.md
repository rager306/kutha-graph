# Phase 46: C01 and knowledge-cut — Pattern Map

**Mapped:** 2026-10-06 · **Files:** 11 · **Analogs:** 10/11 (CHANGELOG controller-only)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match |
|-------------------|------|-----------|----------------|-------|
| `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` | config | transform | same file L138–158 | exact |
| `docs/ADR/ADR-023-test-strategy-and-oracle-matrix.md` | config | transform | same file L95–102 | exact |
| `docs/architecture/semantic-gap-review.md` | config | transform | A-F05 row (~L256) | role |
| `docs/architecture/test-matrix.yaml` | config | batch | L534–599 (Phase 62) | exact |
| `docs/architecture/test-matrix.md` | config | transform | L46–55 blocked table | exact |
| `.kutha/dictionaries/honeycomb.yaml` | config | batch | L16–17, L174–199 | exact |
| `crates/kutha-runtime/tests/m017_known_on_horn.rs` | test | event-driven | self (43-02) | exact |
| `crates/kutha-runtime/tests/m011_claim_supports.rs` or new `m017_*` | test | event-driven | `derived_q_loses_eligibility_*` | role |
| `scripts/tests/test_test_matrix.py` | test | batch | fixtures L51–183 | role |
| `CHANGELOG.md` | config | transform | Phase 45/62 Trajectory bullets | role |
| `crates/kutha-runtime/src/eval.rs` | — | — | **must not edit** | — |

## Pattern Assignments

### ADR-013 Evidence / Verdict (append only; do not replace FIXG-01)

**Analog:** `docs/ADR/ADR-013-bitemporal-facts-invalidation.md`

- **Heading:** `### Verdict (2026-10-06; Phase 46 / M018 S02; Proposed)` or `### Evidence (2026-10-06; …)` after L156.
- **C01 (a):** copy tone of L11 + L140 — pack stamp; forbid fifth axis, Horn body, `valid_to` as application-end; oracle `fixg01_c01_sunset_inexpressible_as_horn`.
- **HORN-KC:** ingest-at-head (`search_prems` / `live_horns` at `u64::MAX`); `known04_horn_fires_when_premise_hidden_on_early_cut` is the **killed pin** (do not invert assertions); reject historical-cut mint requirement.
- **ASOF-ELIG:** `as_of` = support-live (L154); eligibility = `derivation_eligible_at`; reject hiding ineligible rows from `as_of`.
- **Footer (every block):** `D013-1…D013-4 unchanged. Cell remains **Proposed**.`

### test-matrix.yaml + test-matrix.md

**Analog:** `docs/architecture/test-matrix.yaml` L568–599; `test-matrix.md` L46–55.

- **`SEM-c01-decision` / `SEM-horn-kc` / `SEM-asof-elig`:** keep `status: blocked` (46-CONTEXT verifier).
- Update `invariant` / `oracle` text to record **owner choice** vs rejected mutation; add oracle fn for ASOF-ELIG if new pin lands (`oracle: []` today at L594).
- **`SEM-horn-tt-max`:** stay `characterization`; same oracle `known04_*`.
- **md:** refresh status counts + blocked rows; never duplicate full YAML rows (L1–3).

### honeycomb.yaml

**Analog:** header L36 + `ADR-013` block L174–199.

- New `# YYYY-MM-DD restage (M018 S02 Phase 46: …): ADR-013[, ADR-023]; map/delivery/capability unchanged.`
- Append oracle fn names under `evidence:` with `# Phase 46 / M018 S02 — …; cell still Proposed`

### Characterization / pins (no `eval.rs`)

**known04 (unchanged body):** `crates/kutha-runtime/tests/m017_known_on_horn.rs` L17–86 — module comment cites F05; asserts mint at MAX + `as_known_on` TT hide (43-02-PLAN).

**New ASOF pin (if absent):** mirror `derived_q_loses_eligibility_when_last_premise_support_withdrawn` L217–226 — after last premise retract: `!derivation_eligible_at(…, MAX, vt)` **and** `claim_supported_at` / `as_of(vt)` still true. Doc comment like `m019_sem_meta.rs` L424–427 (decision id + characterization).

**FIXG-01:** `m016_golden_fixture.rs` L10–12 — `// Characterization oracle (ADR-023 D023-2)`; no logic change for C01.

## Shared Patterns

- **Dated ADR blocks:** Phase 45 `45-CONTEXT` D-04; Phase 43 `43-02-PLAN` append-after-prior-Evidence.
- **Matrix honesty:** `59-CONTEXT` D-03 — blocked ≠ `#[ignore]`; characterization labelled.
- **Governor:** oracle names must match `#[test]` fn ids (`test_test_matrix.py` ignore fixture).
- **Scope gate:** `git diff --exit-code -- crates/kutha-runtime/src/eval.rs` before close.

## No Analog Found

| File | Reason |
|------|--------|
| `CHANGELOG.md` | Controller-only lease row; follow prior M018 S01/S02 Trajectory one-liners |

## Metadata

**Search:** CBM `search_graph` + source reads; `rg` for matrix/oracle literals. **Scope:** `docs/`, `.kutha/dictionaries/`, `crates/kutha-runtime/tests/` (not `eval.rs`).
