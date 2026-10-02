# Phase 24 Context: Fold indexes for ingest

**Milestone:** v0.06 / harness M013 S03
**Requirements:** IDX-01, IDX-02, IDX-03
**Depends on:** Phase 22 (baseline harness)
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum F15; ADR-040 clarification 2026-10-01
**Run rules:** `docs/process/gsd-auto-contract.md` (RED first, oracle preservation, tool pair)

## Locked decisions

- **D-01:** Indexes are leases: derived from the log, rebuilt on `open`, `from_snapshot`, `from_dict_and_events`, and `fork_at`, never persisted as truth. No log or snapshot format change; no public API change.
- **D-02:** Reuse what exists. The fold already has `fact_index_by_event_id` and `claim_facts`; expose them `pub(crate)`. Add two: delivery key → event, and `EventId` → log position.
- **D-03:** Scan sites to remove (re-locate by symbol; lines as of 2026-10-01): `delivery_key_retry` (`self.log.iter()`), `admit_claim` (`facts().iter().any(claim_id)`), `emit` Retract/Correct known-fact checks (three `facts().iter()` calls), `follow_ons` existence check, `derivation_eligible_at_walk` and `check_admission` (`self.log.iter().find(id)`).
- **D-04:** RED first: a scaling oracle (pattern of `tests/m012a_hot_indexes.rs`: count events and facts visited, not wall time) fails on current code at N, 2N, 4N and passes after: visited count per op is constant in N.
- **D-05:** Named oracles keep their assertions. Allowed edits are accessor visibility and helper renames, listed in the SUMMARY.
- **D-06:** Re-run the Phase 22 harness after the change; ingest and `as_of` regress at most 10 percent, and the delta is recorded.

## Tool contract (both tools, phase edits product code)

- CBM: `search_graph` and inbound `trace_path` on `delivery_key_retry`, `admit_claim`, `follow_ons`, `emit`, `derivation_eligible_at_walk`, `check_admission`; `check_index_coverage` on `quantum.rs` and `fold.rs`; after edit `detect_changes`.
- GitNexus: `list_repos` (reindex with `gitnexus analyze --index-only` if behind, controller only), `impact` upstream on the same symbols, `detect_changes` before commit.
- Record in the PLAN: CBM fan-in, GitNexus count, and `rg 'self\.log\.iter\(\)|facts\(\)\.iter\(\)' crates/kutha-runtime/src` count before and after. `rg` is the arbiter (GitNexus misses `crates/*/tests/`).

## Verifier kill-tests

- Emit the same keyed Assert twice at N, 2N, 4N: one fact, one support, and visited count does not grow.
- `open` a persisted log, then retract and re-emit: indexes agree with a clean replay (fingerprint equal).
- `fork_at` prefix then emit on the fork: the parent’s indexes are untouched.
- `rg` shows no `self.log.iter().find` or `facts().iter().any` on the listed paths.

## Non-goals

New features, Rocks, changing fold semantics, optimizing reads (already done in v0.04 Phase 16).

## Harness

Active Milestone M013, Active Slice **S03** while this phase runs (controller leases it).
