# Phase 30 Context: Write-path integrity hardening

**Milestone:** v0.06 / harness M013 S09 (tail)
**Requirements:** HARD-01, HARD-02, HARD-03, HARD-04
**Depends on:** Phase 29
**Source of gaps:** code and test review 2026-10-02 (findings F3, F5, F6, risks 7 and 8)
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** RED first for every requirement; each oracle fails on current code and the failure is recorded before the change. A RED that passes means the finding is wrong: stop.
- **D-02 (HARD-01):** `Runtime::emit` keeps its signature and call sites. Admit refuses Assert, Retract, Correct, and CorrectInterval on a reserved `kutha:` relation unless the write comes from a typed constructor (genesis allow, rule, policy, admission, action, justification). The mechanism is an internal write origin carried by the pipeline, not a new public API. Product `as_of` isolation of `kutha:` is unchanged.
- **D-03 (HARD-02):** `correct` and `correct_interval` copy the original `delivery_key` onto the replacement version. Oracle: keyed assert, correct, retry the same key; support count stays 1 and no second fact appears. Fingerprint rules for existing logs do not change.
- **D-04 (HARD-03):** One new test file runs FF5 shape, retract, correct, and `claim_supported_at` under `FoldIndexMode::Eager` and `Lazy` and compares results. Existing oracle assertions are not edited. If Lazy diverges, fix the Lazy query path, not the oracle.
- **D-05 (HARD-04):** `facade_statute.rs` asserts the exact live triples (not only inequality) after assert, correct, retract, persist/open, and `fork_at`, using only the public facade. The example keeps its output.
- **D-06 (non-goals):** Retract by `EventId` (a facade type change, M014 S01), typed clocks and `as_known_on`, `as_of` perf index (`vt_by_from` product-only), WAL and snapshot durability (M002), rule evaluation, `derive` auto-registering a rule, Cypher, HNSW, ADR-050 six dictionaries.
- **D-07:** Named oracles (FF5, FF6, M010, M011, M012a, M012, M013) keep assertions; mechanical renames go in an old-to-new table in the SUMMARY.
- **D-08:** Re-run the Phase 22 harness at the end. Compare with the Phase 27 delta in `benchmark-baseline.md`: ingest and `as_of` may not regress more than 10 percent against Phase 27 (the 10 percent over Phase 22 waiver is not renewed or widened).
- **D-09:** The verifier is a different model from the executor.

## Tool contract (both tools)

CBM: `trace_path` inbound on `emit`, `admit`, `correct`, `correct_interval`; `search_graph` for `delivery_key` writers; `check_index_coverage` on each edited file. GitNexus: `impact` upstream on `emit` and `GraphFold::apply` (expect callers missing under tests; `rg` is the arbiter), `context` on moved methods, `detect_changes` before commit. Record both counts and the `rg` count in the plan.

## Verifier kill-tests

- `emit` of an Assert on `kutha:allows` from outside the typed constructors is refused; a typed `emit_allow` still works; FF6 and the genesis oracles stay green.
- Keyed assert, correct, same-key retry: support count 1 under both modes.
- Delete the Lazy path's index warm-up in a scratch copy: the dual-mode oracle fails.
- Replace one `assert_eq!` in `facade_statute.rs` with `assert_ne!` of the wrong triple in a scratch copy: it fails.
- `uv run kutha-gov ci` has 0 HIGH (full, not `--skip-cargo`).

## Non-goals

See D-06.

## Harness

Active Milestone M013, Active Slice **S09** while this phase runs (controller leases it after S08 closes).
