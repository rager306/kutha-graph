# Phase 46: C01 and knowledge-cut — Context

**Milestone:** v0.11 / harness M018 S02
**Requirements:** C01-01, HORN-KC-01, ASOF-ELIG-01
**Depends on:** Phase 45
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-06
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

Record one C01 option and pin Horn-vs-knowledge-cut plus `as_of` vs eligibility. Behaviour change only if a locked rule contradicts crates; then RED first. Do not implement a legal pack, fifth clocks, Correct re-derive, or deterministic derived ids.

## Locked decisions

- **D-C01 (a) pack stamp:** C01 sunset / `tempus regit actum` is a typed pack annotation, not core algebra. Forbidden: fifth axis (D013-1), Horn body, treating `valid_to` as application-end. Oracle stays `fixg01_c01_sunset_inexpressible_as_horn`. SPIKE-LAW-01 waits for a pack. Out of wedge for *core*.
- **D-HORN-KC ingest-at-head:** Horn fires at log-head TT (`search_prems` / `live_horns` at `u64::MAX` = current). `as_known_on` never re-derives; a derived support minted after a cut is hidden by TT like any Assert. Do not invert `known04_horn_fires_when_premise_hidden_on_early_cut`; promote it from characterization to the killed pin of this rule. Reject “must not mint at a historical cut” as an ingest-time requirement.
- **D-ASOF-ELIG support-live:** `as_of` = `live_at(MAX, vt)` stays support-live (Phase 45). Retract of a premise does not retract derived supports. Eligibility is `derivation_eligible_at`. Reject “`as_of` must hide ineligible rows.” Add a named pin if missing.
- **D-SOT acknowledged:** sealed `KUTHASEG` + `manifest.json` are D1; `events.jsonl` after migrate is an export. Not a fifth clock.
- **D-SCOPE:** `eval.rs` / fold behaviour unchanged unless a pin proves a contradiction. Honeycomb stays **Proposed**. Verifier ≠ executor. Controller owns `.kutha/STATE.md`.

## Deferred (not this phase)

DET-02, HORN-CORR-01, CASC-01 → Phase 47. TR-02 / legal pack with C01. Residual harness cleanup. Persister wiring.

## Verifier kill-tests

- ADR-013 dated verdict names option (a) and forbids fifth axis / Horn / `valid_to`-as-application.
- `known04_horn_fires_when_premise_hidden_on_early_cut` still asserts ingest mint + `as_known_on` TT hide; not inverted.
- Named `as_of` support-live pin exists; `derivation_eligible_at` remains a distinct API.
- `git diff` on `eval.rs` is empty (unless RED contradiction — then stop).
- Matrix: SEM-c01-decision / SEM-horn-kc / SEM-asof-elig leave `blocked`; statuses match oracles.

## Harness

Active Milestone M018, Active Slice **S02**.
