# Phase 37 Context: Rule AST and hash

**Milestone:** v0.09 / harness M016 S02
**Requirements:** RULE-01
**Depends on:** Phase 36
**Source:** `.kutha/ROADMAP.md` M016 S02; F9; existing `Op::RegisterRule` + `rule_version` pin
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Build on existing `RegisterRule` / definition-hash pin (`m012_rule_registry`). Canonical body encoding is explicit (struct or bytes), not free-string `rule_version`.
- **D-02:** Unknown or non-canonical bodies fail closed at admit (extend existing mismatch path).
- **D-03:** Do not delete `inverse_knows` cascade yet (Phase 38). Do not implement full evaluator.
- **D-04 (non-goals):** Evaluator (38); policy replay (39); golden fixture (40); M002; honeycomb Accepted.
- **D-05:** Verifier ≠ executor.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.

## Verifier kill-tests

- Registering a non-canonical body is rejected (or Behavior with non-hash pin still fails).
- Canonical encode → hash equals `rule_version` used by Behavior pin.
- FF5/FF6/M012 rule registry oracles stay green (rename-only if needed).

## Non-goals

See D-04.

## Harness

Active Milestone M016, Active Slice **S02**.
