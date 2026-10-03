# Phase 43 Context: Determinism and knowledge-cut characterization

**Milestone:** v0.10 / harness M017 S03 (unleased until Phase 42 closes)
**Requirements:** DET-01, KNOWN-04
**Depends on:** Phase 42
**Source:** audit findings F05, F09, F11, F12 (code subagent)
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01 (no new dependency):** property tests use a seeded deterministic generator (fixed seed list, small LCG) — no `proptest` / `bolero` crate. Seeds are in the test source so failures replay.
- **D-02 (DET-01 properties):** for generated logs of mixed Assert / Retract / CorrectInterval / TtAnchor / Define: `GraphFold::replay(log).fingerprint == fold.fingerprint`; Eager == Lazy on `as_of`, `as_known_on`, `live_at`; the same after `persist` + `open`; independent of delivery-key retry order where the log is identical.
- **D-03 (KNOWN-04 is characterization):** a named test records what a Horn-derived support does when its premise becomes visible later than a `TtAnchor` (Horn fires on `TT = MAX`; `as_known_on(early)` may hide the premise). The test asserts today's behaviour and its comment cites the finding. The behaviour is written into ADR-013 Evidence as current, with "decision pending" if it looks wrong.
- **D-04 (fixture hygiene):** one shared log-backed statute fixture helper replaces the copies of `seed_statute_between_anchors` / `leased_pin_and_derive` only if every named oracle keeps its assertions; otherwise note and skip.
- **D-05:** drop the `C01.contains` string tautology in the FIXG-01 test; keep the VT vs known-on divergence assertion.
- **D-06 (non-goals):** changing Horn evaluation, typed `Op` clocks, new public API, benches.
- **D-07:** RED first where a property finds a real defect: record the shrunk seed, stop for a decision if it needs a semantic change.
- **D-08:** Verifier ≠ executor; controller owns `.kutha/STATE.md`.

## Verifier kill-tests

- Mutating `fold` to ignore a `CorrectInterval` fails the replay==fold property on a recorded seed.
- Flipping Eager/Lazy default fails nothing but a deliberately broken Lazy index fails the equality property.
- The KNOWN-04 test fails if Horn starts honouring the knowledge cut (so a future change must update the test and ADR-013 together).
- `cargo test --workspace` count of named oracles (FF5, FF6, M010–M016) unchanged.

## Harness

Active Milestone M017, Active Slice **S03** (when leased).
