# Phase 29 Context: Close-out honesty (documents and governor)

**Milestone:** v0.06 / harness M013 S08 (tail)
**Requirements:** DOC-01, DOC-02, DOC-03, DOC-04, GOV-01, GOV-02, GOV-03, GOV-04
**Depends on:** Phase 28
**Source of gaps:** `.planning/v0.06-MILESTONE-AUDIT.md`; project review 2026-10-02 (roadmap, ADR, code, tests, harness)
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Documents and harness plane only (`docs/`, `.planning/`, `.kutha/`, `scripts/kutha_gov`). `git diff --stat -- crates` is empty for the phase.
- **D-02 (DOC-01):** M013 stays leased until S09 closes; no archive, no tag. Fix stale text: `.kutha/ROADMAP.md` tail wording, `PROJECT.md` (stale leased-in-progress `L_delivery` token, Active list, last-updated), `STATE.md`, the audit body (match `28-VERIFICATION`; keep `gaps_found` only for real debt), BASE/REC checkboxes and traceability (evidence: `22-VERIFICATION`, `23-VERIFICATION`, `legal-fixture-recon.md`). `.kutha/STATE.md` is the controller's; do not edit it.
- **D-03 (DOC-02):** Post-M013 addendum in `semantic-gap-review.md`. Closed with oracle names: F10 (`m013_genesis_log`), F11 (pipeline, no `append_meta`), F12 (`facade_statute`), F14 (EventId refs), F15 (`m013_fold_indexes`). Open: F1 encoding (jsonl atomic, snapshot and sidecars not), F7, F8 remainder (typed CSR still `from_fold`, no fork diff), F9, F13, F16. State that FF5 compares two valid-time windows at transaction time max, so it is not “as known on D”.
- **D-04 (DOC-03):** ADR notes are dated Evidence or Clarification blocks of at most six lines each; `## Status` stays Proposed. ADR-011: live `Op` taxonomy and EventId targets. ADR-010: pipeline, `append_meta` gone. ADR-012/014: jsonl rename-atomic, `snapshot.json` truncating create, sidecars. ADR-013: FF5 scope. ADR-040: fold maps and `allows_by_name` leases. ADR-041: leapfrog is intersect-only. ADR-050: YAML seeds an empty log only; admit reads `kutha:allows`; five reserved relations are not six dictionaries (freeze unchanged). ADR-060/061: replay and fork_at scope. ADR-000: clocks are `u64` aliases pointer only. `honeycomb.yaml`: rename evidence entries to the real tests (verify each name with `rg 'fn <name>'`). `docs/ADR/README.md`: one note mapping ADR Proposed, honeycomb map/delivery/capability, and “Accepted = implemented”.
- **D-05 (DOC-04):** `kutha-harness.md` and `META.md`: `ci` runs checks, selftest, then cargo; `--skip-cargo` leaves selftest on; `glob_max_lines` zero-match rule. `codex-subagents.md`: note the Cursor roles used in 24–30. `governor-intake.md`: current example. Verify `gsd-auto-contract.md` § Gate stacking before adding anything (it exists).
- **D-06 (GOV-01..04):** invariants row first, then the check (`governor-intake.md`). Prefer existing kinds; a new kind needs META, `kinds.py`, a test, and a selftest mutation. GOV-01: after the quantum, `skipped > 0` is HIGH, plus a unit test that budget is at least the check count. GOV-02: `glob_max_lines` gets `min_hits` (default 0; Phase 28 rows use 1) with HIGH on fewer. GOV-03: glob all `*-VERIFICATION.md` from phase 22, parse frontmatter, require every `read_only_inputs` path exists and none matches `-PLAN.md` or `-SUMMARY.md`. GOV-04: add M013 test names to `fsm.yaml` `observe_cargo.required` (each `rg`-verified); restage the shipped `deferred` I-F rows without deleting bridges or `rust_test_asserts` evidence.
- **D-07:** No check is deleted. No change to product code, ADR-000 locks, or honeycomb status. Model-family independence cannot be proven by the governor; say so in `gsd-auto-contract.md` instead of implying it.
- **D-08:** The verifier is a different model from the executor and from every Phase 22–28 verifier. Executor model is named in the final SUMMARY.

## Tool contract

CBM only where code is read (`scripts/kutha_gov/kinds.py`, `fsm.py`, `selftest.py`): `list_projects`, `search_graph`, `get_code_snippet`, `check_index_coverage`. Do not run `index_repository`. Governor: `uv run kutha-gov precommit`, `selftest --check <id>`, `py`, `pytest`; one full `uv run kutha-gov ci` at close.

## Verifier kill-tests

- Scratch copy: `budget` below the check count makes `ci` HIGH (not a silent skip).
- Scratch copy: a PLAN glob with `min_phase` matching nothing makes `artifact-budget` HIGH.
- Scratch copy: a VERIFICATION without `independent: true`, and one listing a missing `read_only_inputs` file, each make the governor HIGH.
- Stale `L_delivery` leased-in-progress wording is gone from bookkeeping paths; BASE and REC are `[x]`; audit YAML status and body agree.
- Each edited ADR still has `Status: Proposed`; `git diff -- docs/ADR | rg '^[-+].*D(10|[1-9])\b'` shows no lock rewrite.
- `git diff --stat -- crates` is empty; `uv run kutha-gov ci` has 0 HIGH.

## Non-goals

Product code, typed clocks, rules, archive or tag, promoting any cell, removing checks.

## Harness

Active Milestone M013, Active Slice **S08** while this phase runs (controller leases it).
