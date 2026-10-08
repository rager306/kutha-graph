# Phase 75: Restore green — Context

**Milestone:** v0.17 / harness M023 S01  
**Requirements:** GREEN-01, GREEN-02, GREEN-03  
**Run rules:** `docs/process/gsd-auto-contract.md`  
**Mode:** Controller + owner (GSD auto)

## Phase Boundary

HEAD `18151af` fails `cargo test --workspace --offline --no-fail-fast`: (1) `m013_write_path` keyed-correct pair (M022 Phase 72 regression: log 16 -> 18); (2) `m019_sem_model::bitemporal_model_matches_fold_live_at_and_as_of` (naive Retract first-live vs fold retract-all-live); (3) `m019_dur_a` x3 flaky only in the parallel workspace run. Honeycomb stays Proposed. No new capability.

## Locked decisions

- **D-KEY-LINEAGE:** `Correct`/`CorrectInterval` replacement facts carry the prior `delivery_key` and `claim_id` under the Correct's own `event_id`. A keyed retry is a duplicate while any fold fact with that `claim_id` and that `delivery_key` is not invalidated. Remint only when none is live. Payload comparison still uses the original minting event. Do not change fold `apply`.
- **D-NAIVE:** naive Retract mirrors fold: invalidate all live rows of the event id.
- **D-FLAKE:** investigate by running the three tests in the workspace run; fix by unique scratch dirs or a serialized group. Do not weaken assertions.
- RED first (the m013 pair already fails). Named M010-M017 oracle assertions stay. Bench <=10% on `ingest_plain.n` / `as_of.n`.

## Verifier kill-tests

- m013 keyed-correct pair, `m022_keyed_retry`, `m022_admission_pin` green.
- New: keyed Assert -> CorrectInterval -> Retract(ci id) -> same keyed Assert remints; same sequence without Retract appends nothing.
- A mutant that checks only the minting event id fails the m013 pair; a mutant that ignores liveness fails `m022_keyed_retry`.
- `cargo test --workspace --offline --no-fail-fast` zero failures, 3 consecutive runs.
