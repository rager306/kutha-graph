# Phase 38 Research: Rule evaluator

**CBM:** `kutha-graph` reindexed 2026-10-02. `Runtime::follow_ons` (pipeline.rs:451) is the only inverse cascade; inbound via `commit_user_cascade`. `rg` agrees. No GitNexus (not M013).

## Current

- `knows` Assert mints `Behavior{name:inverse_knows, rule_version:""}` with swapped `knownBy`.
- `retract_keeps_loser` leaves inverse live (RULE-02 gap).
- Horn AST exists; no eval. Named rules stay user-emitted Behaviors.

## Approach

1. Genesis `emit_rule_ast` of Horn `knownBy(Y,X) :- knows(X,Y)` (valid_from 0). Dictionary `kutha:rule` excluded from `live_count`.
2. Replace `follow_ons` Assert special-case with Horn unify against live triples; skip if head triple already live. `Behavior.name` is `inverse_knows` when head relation is `knownBy`, else head relation ident.
3. Retract of a premise emits Retract of live Behaviors with `caused_by` that event.
4. `inverse_knows_cascade_is_residual_spike` empty-pin → hashed pin (D-04). `retract_keeps_loser` live_count 1 → 0 (slice goal, not observe_cargo).
5. One n-ary (2-premise) Horn named test. Two-hop eligibility tests unchanged.

## Non-goals

Policy eval, execution replay, MATCH, polynomials, five clocks.
