# Phase 68: Wedge decisions lock — Context

**Milestone:** v0.15 / harness M021 S01
**Requirements:** TR-02-01, C01-02, MATRIX-01
**Depends on:** M020 closed; Phase 46/47 locks stand
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped — user: decide elegantly + plan/execute)

## Phase Boundary

Record remaining owner-gated wedge decisions and pin them with oracles. No fifth clock. No Horn rewrite. No CorrectInterval Retract fix (Phase 69). Honeycomb stays **Proposed**.

## Locked decisions (this milestone)

- **D-TR-02 three moments / two axes:** Legal force = VT of the in-force / applicability fact (`valid_from` / `valid_to`). Publication / knowledge = wall via `Op::TtAnchor` → `as_known_on`. Decision / act clock of a case = the `as_of(vt)` query argument. Forbidden: silent substitution; fifth peer algebraic axis in core. Oracle: named TR-02 pin (force visible on `as_of` after `valid_from`, hidden before; knowledge cut independent via `as_known_on`).
- **D-C01 pack facts (confirm + spike):** Sunset / `tempus regit actum` stays pack stamp (Phase 46 (a)). Core still cannot express calendar sunset as Horn (`fixg01_*`). Pack expresses application window as ordinary `Assert` (e.g. `appliesTo`) with its own VT — not `valid_to` of the text-as-force, not Horn Const. Spike oracle proves Assert-only expressibility.
- **D-HORN-KC / D-ASOF-ELIG (confirm):** Unchanged from Phase 46. Matrix rows SEM-horn-kc / SEM-asof-elig / SEM-c01-decision leave `blocked` → `killed` with existing oracles as pins (no behaviour change).
- **D-SCOPE:** `eval.rs` untouched. `fold.rs` / `fold_query.rs` untouched in this phase (Retract-all-live is Phase 69). Verifier ≠ executor. Controller owns `.kutha/STATE.md`.

## Deferred

- Retract all live residuals for CorrectInterval event id → Phase 69.
- SPIKE-LAW-01 real-text corpus; legal pack crate; agent live∩eligible facade.
- Pi Durable harness replay / request_id (out of wedge).

## Verifier kill-tests

- ADR-013 / ADR-090 dated Evidence names D-TR-02 and C01 pack-fact spike; forbids fifth axis.
- Named `tr02_*` and `c01_applies_to_*` tests green; `fixg01_*` / `known04_*` / `asof_elig01_*` still green.
- SEM-tr02-legal / SEM-c01-decision / SEM-horn-kc / SEM-asof-elig are `killed` with oracles filled.
- `git diff` on `eval.rs` empty; fold Retract behaviour unchanged this phase.
- Honeycomb ADR-013 / ADR-090 restaged; cells Proposed.

## Harness

Active Milestone M021, Active Slice **S01**.
