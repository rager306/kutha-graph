# Phase 35: Living docs and governor honesty — Research

**Researched:** 2026-10-02
**Domain:** harness dictionaries + living architecture notes
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Living docs (ROADMAP M013 CLOSED text, PROJECT idle→M015, semantic-gap F13, ADR-013 Evidence, p0-spike, legal-fixture-recon, CHANGELOG order, id-map-cite needles) match post-M014 + Phase 34 residual.
- **D-02:** `true_on_d_differs_from_known_on_d` in `observe_cargo.required`; honeycomb ADR-013 evidence lists it and M015 known-on oracles.
- **D-03:** With live `.planning/phases/` non-empty, artifact-budget PLAN/SUMMARY/VERIFICATION/CONTEXT globs use `min_hits: 1`; keep `allow_empty` only on verification-independent if still needed when empty.
- **D-04 (non-goals):** M016 rules; five clocks; intern/WAL product fixes; honeycomb Accepted.
- **D-05:** Verifier ≠ executor. Do not write `35-VERIFICATION.md`.
- **D-06:** CBM for any code cites; mostly docs this phase.

### Deferred Ideas (OUT OF SCOPE)
- M016 evaluator / golden fixture; five legal-clock newtypes; intern/WAL; honeycomb Accepted.
</user_constraints>

<research_summary>
## Summary

M014 (v0.07) shipped typed clocks (`ValidTime`/`TransactionTime`/`WallTime`), `Op::TtAnchor`, and `as_known_on`. Phase 34 (M015 S01) landed TT-only `known_at`/`is_known_at` and fail-closed duplicate/backwards `TtAnchor` walls. Live lease is M015 S02. Living notes and governor dictionaries still speak as if M014 were future and F13 fully open.

CBM project `root-kutha-graph` (`list_projects` 2026-10-02): `true_on_d_differs_from_known_on_d` at `crates/kutha-runtime/tests/m014_as_known_on.rs:13`. M015 oracles: `closed_vt_interval_survives_as_known_on`, `duplicate_tt_anchor_wall_is_rejected`. `event.rs` comments still say TT-to-wall is out of M012a S05. `fold_query.rs` coverage `metadata_changed` — read source; do not `index_repository`. GitNexus not used (not M013 24–27).

**Primary recommendation:** One wave, one plan: rewrite living notes to post-M014+34, then restore governor observe/budget honesty. Comment-only in `event.rs`. Executor runs `kutha-gov precommit`; controller runs `ci` at close.
</research_summary>

## Stale inventory (HEAD + working tree)

| Path | Stale claim | Target |
|------|-------------|--------|
| `.kutha/ROADMAP.md` | M013 “does not archive until S09”; tail “stays leased”; M015 “Needs a lease” / later-milestones “needs its own lease” | M013 CLOSED + tagged `v0.06`; M015 LEASED S02 |
| `.planning/PROJECT.md` | Context `Active Milestone: None (M014 CLOSED)`; Next goals still v0.07; “M015 executable rules”; Validated omits clocks | Current = v0.08/M015; Validated includes v0.07 clocks + Phase 34 known-on; Context cites M015 S02 |
| `docs/architecture/semantic-gap-review.md` | Still-open F7 aliases, F13 “no typed clocks”; FF5 “needs M014” | Addendum: F7/F13 clock+anchor+`as_known_on` shipped; Phase 34 TT-only residual landed; five-clock Q open (M016) |
| `docs/ADR/ADR-013-*.md` Evidence | “F13 stays open for M014” | Evidence: F13 ship + `as_known_on`; five-clock Q still open |
| `docs/architecture/p0-spike-inventory.md` | No clocks/`TtAnchor`/`as_known_on`/`known_at` | Add to In spike |
| `docs/architecture/legal-fixture-recon.md` | “two `u64` axes”; known-on needs M014 | Newtypes exist; known-on exists; rules still M016 |
| `.kutha/dictionaries/fsm.yaml` | `observe_cargo.required` misses ANK test | Add `true_on_d_differs_from_known_on_d` |
| `.kutha/dictionaries/honeycomb.yaml` ADR-013 | FF5/interval only | Add ANK + M015 oracles; restage comment |
| `.kutha/dictionaries/checks.yaml` | `min_hits: 0`; description “v0.06 phase 22”; id-map needles stop at v0.06 | Live-phase wording; `min_hits: 1` on four globs; needles include v0.07/v0.08 |
| `.kutha/dictionaries/invariants.yaml` | Matching v0.06-only claims | Same wording as checks |
| `docs/process/governor-intake.md` | Example already `min_hits: 1`; no idle vs live | Document idle `min_hits: 0` vs live `1` |
| `crates/kutha-common/src/event.rs` | TT-to-wall “out of M012a S05” (lines 23, 108–109) | M014 `tt_anchor`; M015 known-on residual |

Already honest: M015 S02 evaluator is unchecked on M016; `.planning/ROADMAP.md` Current Milestone v0.08; 34-VERIFICATION exists so VERIFICATION glob `min_hits: 1` is non-vacuous.

## Standard approach

Unchanged harness: YAML row first (`governor-intake.md`); honeycomb restage with ADR Evidence; CHANGELOG **Process** (and Trajectory if ADR/honeycomb). `allow_empty` stays on `verification-independent` because this executor must not write `35-VERIFICATION.md` (D-05); live `34-VERIFICATION.md` already satisfies artifact-budget hits.

## Do not

Do not edit `.kutha/STATE.md`. Do not start M016. Do not change product semantics. Skip `statute_as_of.rs` (optional; budget). Do not run `kutha-gov ci` in the executor.
