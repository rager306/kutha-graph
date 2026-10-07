---
independent: true
verifier_model: composer-2.5-fast
status: passed
phase: 68
milestone: M021
slice: S01
verified_at: 2026-10-07
---

# Phase 68 independent verification (M021 S01)

Verifier did **not** read `.planning/phases/68-wedge-decisions/*PLAN*`, `*SUMMARY*`, or other executor artifacts under that directory.

## Requirements checked (`.planning/REQUIREMENTS.md` v0.15)

| Req ID | Criterion | Result |
|--------|-----------|--------|
| TR-02-01 | Force=VT; publication=TtAnchor/as_known_on; decision=as_of(vt); named oracle; no fifth axis | **pass** (ADR-090 Evidence + oracle green) |
| C01-02 | Pack Assert appliesTo window; FIXG-01 fails as Horn | **pass** (ADR-013 Verdict + oracles green) |
| MATRIX-01 | SEM-c01-decision, SEM-horn-kc, SEM-asof-elig, SEM-tr02-legal `killed` with oracles | **pass** (test-matrix.yaml) |

## Read-only inputs

- `.planning/REQUIREMENTS.md` (v0.15 pins)
- `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` (Verdict 2026-10-07)
- `docs/ADR/ADR-090-legal-reference-pack.md` (TR-02 Evidence 2026-10-07)
- `docs/architecture/test-matrix.yaml` (SEM-* rows)
- `crates/kutha-runtime/tests/` (oracle names only; via `cargo test`)
- CBM: `list_projects`, `check_index_coverage` on cited test paths (best-effort; one path `not_tracked`)

## Kill-tests (commands and exit codes)

| # | Command | Exit | Notes |
|---|---------|------|-------|
| 1 | `rg 'Verdict (2026-10-07; Phase 68' docs/ADR/ADR-013-bitemporal-facts-invalidation.md` | 0 | Match at line 174; C01/HORN-KC/ASOF/TR-02 cross-cite present |
| 2 | `rg 'TR-02 mapping on two core axes' docs/ADR/ADR-090-legal-reference-pack.md` | 0 | Match at line 233; table + oracle name documented |
| 3 | `set -a && source .env && set +a && unset CARGO_TARGET_DIR && cargo test -p kutha-runtime --offline --test m021_wedge_decisions --test m016_golden_fixture --test m017_known_on_horn --test m011_claim_supports -- --exact tr02_force_vt_publication_anchor_decision_as_of c01_applies_to_assert_encodes_application_window fixg01_c01_sunset_inexpressible_as_horn known04_horn_fires_when_premise_hidden_on_early_cut asof_elig01_as_of_contains_derived_when_ineligible` | 0 | **5/5** tests passed |
| 4 | `rg -n 'id: SEM-(c01-decision|horn-kc|asof-elig|tr02-legal)' -A6 docs/architecture/test-matrix.yaml` + manual `status: killed` | 0 | All four IDs present; `status: killed` on lines 575, 587, 598, 620 (within +7 of id; `-A6` shows oracle block) |
| 5 | `git diff --exit-code HEAD -- crates/kutha-runtime/src/eval.rs crates/kutha-runtime/src/fold.rs crates/kutha-runtime/src/fold_query.rs` | 0 | No dirty diff vs HEAD on fold/eval surfaces |

## Kill-test score

**5 / 5** kill-test gates green.

## Oracle ↔ matrix mapping (public)

| SEM id | Oracle(s) | Test file |
|--------|-----------|-----------|
| SEM-tr02-legal | `tr02_force_vt_publication_anchor_decision_as_of` | `m021_wedge_decisions.rs` |
| SEM-c01-decision | `fixg01_c01_sunset_inexpressible_as_horn`, `c01_applies_to_assert_encodes_application_window` | `m016_golden_fixture.rs`, `m021_wedge_decisions.rs` |
| SEM-horn-kc | `known04_horn_fires_when_premise_hidden_on_early_cut` | `m017_known_on_horn.rs` |
| SEM-asof-elig | `asof_elig01_as_of_contains_derived_when_ineligible` | `m011_claim_supports.rs` |

## CBM coverage note

`check_index_coverage` (project `kutha-graph`): `m021_wedge_decisions.rs` flagged `not_tracked`; other three test paths `metadata_match`. Verification relied on executed tests and YAML/ADR text, not graph completeness.

## Verdict

Phase 68 S01 requirements TR-02-01, C01-02, and MATRIX-01 are satisfied by ADR pins, matrix kills, passing oracles, and clean eval/fold/fold_query tree vs HEAD.
