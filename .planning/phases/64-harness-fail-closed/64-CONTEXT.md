# Phase 64: Harness fail-closed — Context

**Milestone:** v0.14 / harness M020 S02
**Requirements:** FC-01 nameless dict id, FC-02 missing cite path, FC-03 bad JSONL fold, FC-04 `min_phase` unprefixed dir, FC-05 empty heading glob
**Depends on:** Phase 63 closed; lease S02 already Active
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

Harness silent-pass / crash paths become fail-closed findings. RED mutation (copy / `_write_mini`) first. No product crates. Honeycomb stays **Proposed**. Do not edit `.kutha/STATE.md`. Do not write `64-VERIFICATION.md`.

## Locked decisions

- **D-01 (FC-01):** `get_checks`: mapping row with missing/blank `id` → `LoadErrorCheck` HIGH (same shape as empty-list load error). Do not drop the row.
- **D-02 (FC-02):** `cite_equals` when a cite `path` is set and the file is absent → `_high_missing` HIGH. Empty `_cite_files` result is not a pass.
- **D-03 (FC-03):** `fold_log`: invalid JSONL → `ValueError` with 1-based file line number (wrap `JSONDecodeError`). `fsm` `fold_log` catches it, appends Finding HIGH + observation `("fold_log", "fail")`, returns `fail`.
- **D-04 (FC-04):** `min_phase>0` and path under `.planning/phases/` whose dirname lacks `NN-` (`_PHASE_DIR_RE`) → HIGH in `_glob_max_lines_hits` and `_kind_glob_absent`. Valid `NN-` with `nn < min_phase` still omitted.
- **D-05 (FC-05):** `_kind_markdown_heading_tag`: glob matches zero files while parent dir exists → HIGH (empty scan is not a pass).
- **D-06:** no `crates/` edits; no ADR / `honeycomb.yaml` restage; no executor VERIFICATION; STATE lease is controller-only.

## Deferred (not this phase)

Persist atomic → 65. Horn/fold → 66. Process lease/bench → 67. Non-dict `checks` rows. Empty cite `glob`. `last_logged_membership` decode skip.

## Verifier kill-tests

- Two-row `checks.yaml` (one nameless `id`, one valid) → `get_checks` is `_dictionary` HIGH, not the valid id alone.
- Cite `path` pointing at a non-file → HIGH `missing`.
- Garbage JSONL line N → `fold_log` raises `ValueError` containing `N`; fsm observation `fold_log`/`fail`.
- File `.planning/phases/nophase/x.md` with `min_phase>0` → HIGH.
- Heading glob with existing parent and zero files → HIGH.
- Live tree: `uv run kutha-gov selftest` and `uv run kutha-gov ci --jobs 1` exit 0.

## Harness

Active Milestone M020, Active Slice **S02**.
