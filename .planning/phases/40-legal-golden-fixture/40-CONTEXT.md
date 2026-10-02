# Phase 40 Context: Legal golden fixture and verdict

**Milestone:** v0.09 / harness M016 S05
**Requirements:** FIXG-01
**Depends on:** Phase 39 (RULE-03/04)
**Source:** `.kutha/ROADMAP.md` M016 S05; F16; ADR-013 open Q; `docs/architecture/legal-fixture-recon.md` C01
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Fixture is C01 sunset: «пункт 15.1 действует до 01.01.2028» (quote only; do not copy the law-nexus corpus). Probe whether Named/Horn + `as_known_on` can express application-end without folding it into `valid_to`.
- **D-02:** `as_known_on` stays TT-only greatest-anchor lookup (Phase 34). Do not add application/promulgation/edition as core axes.
- **D-03 (fail-closed):** If RuleAst has no temporal atoms and sunset cannot be a Horn body distinct from VT, write a dated ADR-013 Evidence verdict and **stop**. Do not invent five clocks.
- **D-04:** Preserve FF5, FF6, two-hop, `admission_cites_pinned_policy_version`, Horn inverse oracles. Mechanical renames only.
- **D-05 (non-goals):** MATCH; policy AST; honeycomb Accepted; M002; Rocks/Cypher/HNSW; ADR-050 six dicts.
- **D-06:** Verifier ≠ executor. Do not write 40-VERIFICATION. Never edit `.kutha/STATE.md`. No `kutha-gov ci`.
- **D-07:** CBM graph-first; GitNexus secondary; `rg` arbiter.

## Verifier kill-tests

- Named test cites C01 quote and either: (a) Horn/Named sunset + `as_known_on` green at 2027 vs 2028, or (b) fail-closed dated ADR-013 Evidence that two axes + current RuleAst cannot express application-end without a fifth clock.
- `rg` finds no new clock newtype beyond ValidTime/TransactionTime/WallTime.
- FF5 stays green.

## Non-goals

See D-05.

## Harness

Active Milestone M016, Active Slice **S05**.
