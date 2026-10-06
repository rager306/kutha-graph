---
phase: 46-c01-knowledge-cut
verified: 2026-10-06T10:02:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
score: 5/5 kill-tests
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
  - docs/architecture/test-matrix.yaml
  - crates/kutha-runtime/tests/m011_claim_supports.rs
  - crates/kutha-runtime/tests/m017_known_on_horn.rs
  - crates/kutha-runtime/tests/m016_golden_fixture.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/quantum.rs
commands:
  - "grep 'Verdict (2026-10-06; Phase 46' docs/ADR/ADR-013-bitemporal-facts-invalidation.md → exit 0"
  - "grep SEM-c01-decision|SEM-horn-kc|SEM-asof-elig + status blocked in test-matrix.yaml → exit 0"
  - "git diff --exit-code -- crates/kutha-runtime/src/eval.rs; git diff HEAD~20 -- eval.rs → 0 bytes → exit 0"
  - "cargo test m016 fixg01_c01_sunset_inexpressible_as_horn --exact → exit 0"
  - "cargo test m017 known04_horn_fires_when_premise_hidden_on_early_cut --exact → exit 0"
  - "cargo test m011 asof_elig01_as_of_contains_derived_when_ineligible --exact → exit 0"
covered_files: [crates/kutha-runtime/src/eval.rs, crates/kutha-runtime/tests/m011_claim_supports.rs, crates/kutha-runtime/tests/m016_golden_fixture.rs, crates/kutha-runtime/tests/m017_known_on_horn.rs, docs/ADR/ADR-013-bitemporal-facts-invalidation.md, docs/architecture/test-matrix.yaml]
covered_digest: "v3:sha256:4a4b4e549e78e1b0cc0bcf6678caa84ef4ac8e269490d76e40cbbc60f63a55da"
behavior_unverified: 0
re_verification:
  previous_status: passed
  regressions: []
---

# Phase 46 — Independent Verification (no PLAN/SUMMARY read)

**Contract:** C01-01 / HORN-KC-01 / ASOF-ELIG-01 (`REQUIREMENTS.md`).

| Kill-test | Evidence | Status |
| --- | --- | --- |
| ADR-013 pack stamp (a), no fifth axis | L160–166: (a) pack stamp; forbids fifth axis, Horn body, `valid_to`-as-application | ✓ |
| `known04_*` not inverted | `m017`: mint at TT=MAX; `!known_2014` premise+derived; `known_2020` shows A–B | ✓ PASS |
| ASOF pin when ineligible | `m011` L307–313: `!derivation_eligible_at` + `as_of(2017).contains`; separate APIs in `provenance`/`quantum` | ✓ PASS |
| `eval.rs` unchanged | clean WT diff; 0-byte diff vs `HEAD~20` on `src/eval.rs` | ✓ |
| SEM three rows blocked | `SEM-c01-decision` / `SEM-horn-kc` / `SEM-asof-elig`: `status: blocked`, rewritten invariants + oracles | ✓ |

**Verdict:** `passed` — locked decisions documented; named oracles green; product fold/eval untouched. Doc-only regression would fail `asof_elig01` or inverted `known04`.

_CBM: kutha-graph indexed (coverage metadata_changed on touched src; source read for cites). Verifier: gsd-verifier independent._
