---
phase: "01"
slug: "legal-pit-fitness"
status: draft
created: "2026-09-29"
---

# Phase 01 — Verification Evidence (Legal PIT fitness)

Evidence SoT for FIT-01…05 (CONTEXT D-02 / D-04). Pass/fail cells stay `pending` until Plan 01-02 paints them from a hard-gate run.

## Hard gate

| Field | Value |
|-------|-------|
| **Command** | `cargo test --workspace --offline` |
| **Exit code** | `0` |
| **Timestamp (UTC)** | `2026-09-29T07:21:21Z` |
| **Harness cite** | Active Milestone M011; Active Slice None; Phase H4; freeze until explicit M002 (`.kutha/STATE.md` — not edited) |

Arg parity: `.kutha/dictionaries/fsm.yaml` `observe_cargo.args` and `scripts/kutha_gov/observe.py` `default_args=["test", "--workspace", "--offline"]`.

## FIT evidence map

| FIT-id | test file | test fn | pass/fail |
|--------|-----------|---------|-----------|
| FIT-01 | crates/kutha-runtime/tests/ff5_legal_pit.rs | ff5_as_of_t1_differs_from_as_of_t2_on_statute_log | pending |
| FIT-02 | crates/kutha-runtime/tests/ff6_allowlist.rs | ff6_unknown_relation_does_not_append | pending |
| FIT-03 | crates/kutha-runtime/tests/m010_semantic_open.rs | open_without_snapshot_recovers_intern_meanings | pending |
| FIT-03 | crates/kutha-runtime/tests/m010_semantic_open.rs | open_without_snapshot_or_terms_file_recovers_from_define_ops | pending |
| FIT-03 | crates/kutha-runtime/tests/m010_semantic_open.rs | intern_appends_define_for_new_terms_only | pending |
| FIT-04 | crates/kutha-runtime/tests/m011_claim_supports.rs | retracting_one_support_leaves_claim_supported | pending |
| FIT-04 | crates/kutha-runtime/tests/m011_claim_supports.rs | unknown_claim_does_not_append | pending |
| FIT-04 | crates/kutha-runtime/tests/m011_claim_supports.rs | replay_rejects_behavior_without_prior_cause | pending |
| FIT-04 | crates/kutha-runtime/tests/m011_claim_supports.rs | derived_q_loses_eligibility_when_last_premise_support_withdrawn | pending |
| FIT-05 | crates/kutha-runtime/tests/h2_harness_tenant.rs | h2_harness_status_as_of_t1_differs_from_as_of_t2 | pending |
| FIT-05 | crates/kutha-runtime/tests/h2_harness_tenant.rs | h2_same_second_status_as_of_uses_emitted_cut | pending |
| FIT-05 | crates/kutha-runtime/tests/h4_process_allows.rs | h4_prior_cut_keeps_status_membership_after_later_edition_drops_it | pending |

Twelve rows align to `fsm.yaml` `states.observe_cargo.required` (exact fn name strings).

## Notes

- Tracer plan 01-01: hard gate observed end-to-end; cells remain `pending` for wave 2 evidence paint (D-04).
- Diagnostic (not a substitute gate): `cargo test --workspace --offline -- --list` — all twelve evidence-map fn names present (≥12 matches).
- Do not treat `uv run kutha-gov ci` as Phase 1 hard gate (D-03).
- Do not use cargo `--test` / `--exact` filters as the acceptance gate (D-01).
