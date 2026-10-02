# Phase 38 Context: Rule evaluator and support counting

**Milestone:** v0.09 / harness M016 S03
**Requirements:** RULE-02
**Depends on:** Phase 37 (RULE-01)
**Source:** `.kutha/ROADMAP.md` M016 S03; F9; `follow_ons` inverse_knows cascade; `derivation_eligible_at`
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Delete hard-coded `Runtime::follow_ons` `knows`→`inverse_knows` (empty pin). Inverse is a registered Horn (or Named) rule; `rule_version` is the body hash.
- **D-02:** Evaluator matches live CSR/fold triples against registered rule bodies. Multi-hop and n-ary use the same eval, not a second walk language.
- **D-03:** Retract of a premise must drop live derived support at a later cut (historical rows stay). Count supports; do not leave a stale inverse after the last premise retracts.
- **D-04:** `inverse_knows_cascade_is_residual_spike` may change the empty-pin assertion to a hashed pin (slice goal). FF5, FF6, two-hop eligibility, and M012 registry oracles keep their assertions. Mechanical constructor/helper renames only elsewhere.
- **D-05 (non-goals):** Policy body eval and execution replay (39); golden fixture (40); MATCH compiler; provenance polynomials; M002; honeycomb Accepted.
- **D-06:** Verifier ≠ executor. Do not write 38-VERIFICATION in this phase.
- **D-07:** CBM graph-first; GitNexus secondary; `rg` arbiter. Executors never edit `.kutha/STATE.md`.

## Verifier kill-tests

- `knows` Assert still yields `inverse_knows` with a live hashed pin; `relatedTo` does not.
- Retract of the `knows` premise drops live inverse support at a later cut.
- Two-hop eligibility still walks `caused_by` (existing named tests).
- `follow_ons` inverse cascade is gone (`rg` arbiter).

## Non-goals

See D-05.

## Harness

Active Milestone M016, Active Slice **S03**.
