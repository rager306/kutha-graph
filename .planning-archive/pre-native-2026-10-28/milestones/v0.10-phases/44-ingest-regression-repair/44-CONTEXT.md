# Phase 44 Context: Ingest regression repair

**Milestone:** v0.10 / harness M017 S04 (unleased until Phase 43 closes)
**Requirements:** PERF-01, PERF-02
**Depends on:** Phase 43
**Source:** Phase 42 controller bisect, same host, N=10000 Eager: `ingest_plain.n` v0.06 106M ns, v0.08 108M, `428f3ff` (post-M016) 301M, HEAD after Phase 42 281–303M. See `docs/architecture/benchmark-baseline.md` § Phase 42.
**Run rules:** `docs/process/gsd-auto-contract.md` (tool pair CBM + GitNexus on product edits)

## Locked decisions

- **D-01 (bisect first):** run `cargo bench -p kutha-runtime --offline --bench baseline` (`KUTHA_BENCH_N=10000`) in throwaway worktrees under `tmp/` at the M016 commits (Phases 37–40: AST hash, evaluator, policy eval, golden fixture) and name the first commit with a >10 percent jump. Record numbers in SUMMARY. Remove worktrees after.
- **D-02 (hypothesis, not fact):** the Horn evaluator or policy evaluation scans `fold.facts()` (`eval.rs` `live_horns` / `search_at`) or re-hashes rule bodies on every assert/retract (O(N) per ingest). Verify with CBM `trace_path` outbound from the ingest path and `rg`; do not edit before the bisect agrees.
- **D-03 (RED first):** a scaling oracle, in the style of the M013 S03 fold-index oracle, fails on `428f3ff`: either a scan counter on the ingest path or a ratio of ingest time at N and 2N below a documented bound. Time-based oracles use a generous ratio and are run with `--test-threads=1`.
- **D-04 (no behaviour change):** derived supports, `inverse_knows`, multi-hop and n-ary eligibility, retraction maintenance, replay and policy decisions stay identical. FF5, FF6, M010–M017 oracles keep every assertion; any changed assertion is a STOP.
- **D-05 (fix shape):** prefer an index or cache keyed by the rule/premise relation over caching results across facts; no new dependency; no new public API unless the plan proves it is needed.
- **D-06 (accept path):** if the cost is inherent (rules now evaluated on ingest by design), the SUMMARY states the per-fact cost, the controller decides whether to accept it, and the baseline note is re-stated with the new reference. This is a controller decision, not an executor one.
- **D-07:** Verifier ≠ executor; controller owns `.kutha/STATE.md`; benchmark re-run at close.

## Verifier kill-tests

- Re-run the bench in a clean worktree at the phase's first and last commit; ingest at HEAD is within 10 percent of v0.08 or the SUMMARY records a controller-accepted residual.
- A deliberately re-introduced full scan on the ingest path fails the scaling oracle.
- `cargo test --workspace --offline` named oracle count unchanged.

## Non-goals

Typed `Op` clocks; M002; changing Horn semantics; benches beyond `baseline`.

## Harness

Active Milestone M017, Active Slice **S04** (when leased).
