# Phase 47: Horn determinism and closure — Context

**Milestone:** v0.11 / harness M018 S03
**Requirements:** DET-02, HORN-CORR-01, CASC-01
**Depends on:** Phase 46
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-06
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

Make derived Horn identity replay-stable, re-derive on Correct, and admit every cascade hop. RED first: invert `SEM-horn-correct-char` only after the new pin fails on HEAD. Do not implement Phase 48 oracles, legal pack, fifth clocks, or CLOCK-01 (`u64::MAX` in eval).

## Locked decisions

- **D-DET-02 hash-stable derived ids:** User-emitted events stay UUID v7 (`Event::new`). Derived cascade events (`Op::Behavior`, follow-on `Retract`) mint id as first 16 bytes of SHA-256 over a Kutha prefix plus rule pin, `caused_by`, binding TermIds, and op discriminant — same pattern as `Event::stable_define`. Two identical ingest sequences yield the same derived ids. Do not change persisted user v7 ids.
- **D-HORN-CORR re-derive:** `rule_follow_ons` treats `Correct` and `CorrectInterval` like a premise change: `retract_derived` of the targeted event_id, then `horn_fires` on the corrected triple. Stale derived Behavior must not stay live at the post-correct cut. Invert `horn_correct_char_*` (characterization → killed of the new rule). Keep FF5 / M010–M017 named oracles except those characterization pair.
- **D-CASC admit hops:** every event committed in `commit_user_cascade` (and Lazy equivalent) passes `admit` before commit, not only the first event’s follow-ons. Unknown head or premise relation on a firing Horn fails closed (`RuntimeError`), not `continue`. Keep existing LIFO pending order unless SEM-cascade-order forces a documented swap.
- **D-SCOPE:** honeycomb **Proposed**. Controller owns `.kutha/STATE.md`. Verifier writes `47-VERIFICATION.md`. TR-02 / C01 pack stay out.

## Deferred

Phase 48 ORACLE-01 / GATE-01. UUID v7 for derived events (rejected). Fifth clocks.

## Verifier kill-tests

- Two `Runtime` ingest of the same Horn fixture: derived Behavior EventIds equal.
- After Correct of a Horn premise, old derived triple is not live; new derivation matches remaining premises (or none).
- A cascade hop that would fail `admit` never appears in the log.
- Unknown relation in a live Horn head/premise: emit returns Err.
- `git diff` does not change FF5 assertions.

## Harness

Active Milestone M018, Active Slice **S03**.
