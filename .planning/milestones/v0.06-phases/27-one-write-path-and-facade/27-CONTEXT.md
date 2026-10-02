# Phase 27 Context: One write path and facade

**Milestone:** v0.06 / harness M013 S06
**Requirements:** PIPE-01, PIPE-02, PIPE-03, PIPE-04
**Depends on:** Phase 26
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum F11, F12, F14; ADR-010 clarification 2026-10-01
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** One pipeline, implemented once: propose, admit, commit, receipt. `emit` becomes a thin wrapper; the early-return branches for allow, rule, and policy and the `append_meta` side path are deleted. Every typed record constructor (justification, action, rule, policy, genesis) calls the pipeline.
- **D-02:** Split `Runtime` by concern into modules under `crates/kutha-runtime/src/` (for example admission, pipeline, justification and action, provenance, hydrate). No source file in that directory exceeds 800 lines (`quantum.rs` is 1,488 today). No new crate, no repo-root `ports/adapters/domain` (ADR-022).
- **D-03:** The facade is the narrowed public surface of the runtime, not a second type: everything not in the PIPE-03 list becomes `pub(crate)`. PIPE-03 list: assert, retract, correct, derive (Behavior), admit and record action, `as_of` and `live_at`, persist and open, `fork_at`, `replay_check`.
- **D-04:** The consumer is `crates/kutha-runtime/examples/statute_as_of.rs` and `crates/kutha-runtime/tests/facade_statute.rs`. Both compile against the public API only (an integration test cannot see `pub(crate)`), run the statute-shaped scenario, and assert `as_of(2015) ≠ as_of(2021)` plus one admission and one action record.
- **D-05:** `QuantumOutcome` stays an `Op` until M002 (commit-marker framing is a durability decision, ADR-010 open question). Receipt semantics are unchanged.
- **D-06:** `kutha-tenant` and `tenant.rs` (`emit_chained`, `ingest_harness_jsonl`) keep working; the controller runs `uv run kutha-gov ci` at the end of this phase (H2/H4 observations go through them).
- **D-07:** Named oracles keep their assertions; renames go in the SUMMARY table. Re-run the Phase 22 harness: at most 10 percent regression in ingest and `as_of`.
- **D-08:** RED first: a test that drives only the facade fails to compile today (items are not reachable or not sufficient); and an oracle asserting that a justification record is rejected by the same admission as an Assert when its relation is unknown.

## Tool contract (both tools)

CBM: `get_architecture` on `kutha-runtime` once, `trace_path` inbound on `emit`, `record_justification`, `record_action`, `check_admission`; `search_graph` for `pub fn` on `Runtime`; `check_index_coverage` on every new module. GitNexus: `impact` upstream on `emit` (expect CRITICAL, 13 direct callers at last run), `context` on each moved method, `detect_changes` before commit. Record the `pub fn` count on `Runtime` before and after. The plan names which methods move to which module and who calls them.

## Verifier kill-tests

- The example and the integration test build with `cargo build --examples` and `cargo test --test facade_statute` and import nothing outside the public API.
- Emit an unknown relation as an Assert, a justification, and an action: all three are refused by the same rule.
- No file under `crates/kutha-runtime/src` is over 800 lines (`wc -l`).
- `rg 'fn append_meta' crates` is empty.
- `uv run kutha-gov ci` has 0 HIGH.

## Non-goals

Commit-marker framing, a new crate, a CLI, Cypher, rules.

## Harness

Active Milestone M013, Active Slice **S06** while this phase runs (controller leases it).
