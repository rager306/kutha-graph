# Phase 230 — fork diff (M156 S01, B2)

Source: B2 scout 2026-10-23 (CBM + `rg`), M155 results (`fork_at` is fallible). Discuss is skipped; decisions below are locked.

## Goal

ADR-061 reduced to a fact-set diff: a pure read-side function that says exactly how two folds differ, usable on forks (counterfactuals) and on stores that were checkpointed. It is a query, not a log write: no Op, no receipt, no wire change. Not named refs (D061-1), not graph-edit-distance (D061-3), not merge. Honeycomb stays Proposed.

## Locked decisions

1. API (in `fold_query.rs`, re-exported from `lib.rs`): `pub fn diff_folds(a: &GraphFold, b: &GraphFold) -> FoldDiff`, plus `GraphFold::diff(&self, other)` as a thin wrapper. `FoldDiff { added: Vec<Fact>, removed: Vec<Fact>, changed: Vec<FactChange> }` with `FactChange { before: Fact, after: Fact }`; direction: `diff_folds(a, b)` is what turns `a` into `b`. Names may be adjusted if `rg` shows a clash; keep the semantics.
2. Row identity is the minting event id plus the valid-time window `(event_id, valid_from, valid_to)`, never `seq` (fold-local, ADR-061 clarification). Keys are NOT guaranteed unique (CI multi-residual facts share an event id; derived heads from two minting paths share one id, found in M155): identity is key plus rank among rows with an equal key, ranked by a canonical total order over all other fields (object, relation, subject, claim id, delivery key, polarity, ingested_at, invalidated_at). `changed` = same (key, rank) pair whose rows differ in any field except `seq`; `added` = only in `b`; `removed` = only in `a`. All three vectors are sorted by (key, rank) and deterministic (no hash-map iteration order).
3. Domain: the whole fold picture `GraphFold::facts()` including dead/loser rows (ADR-013: losers stay), both clocks on the row. `as_of` / `live_at` / `known_at` are oracles after applying a diff, not the diff domain.
4. Totality oracle: a test helper `apply_diff(rows_a, &diff) -> Vec<Row>` and the property `normalize(apply_diff(a, diff)) == normalize(b)` as a multiset of rows modulo `seq`, for every pair in the fixtures. Also `diff(a,a)` empty; `diff(a,b)` and `diff(b,a)` are mirror images (added<->removed, changed before<->after); transitivity is not required.
5. Fork integration, no new fork API: counterfactual fixtures use `Runtime::fork_at(cut)` (fallible after M155). On a checkpointed store an interior cut returns `ForkCutUnavailable`; the diff slice must not try to work around it. A fork at `c+1..=len` of a checkpointed store must diff exactly like the same cut of the un-checkpointed twin once ids are normalised.
6. Out of scope: any Op or log write, named refs/branches, merge, GED, semantic as_of-level diff, an operator CLI, Verus proofs, harness code (moratorium; Rust tests pin the invariants), a diff of dictionaries/rules/blobs (facts only; state this in the doc).

## RED (record the failure before the change)

`crates/kutha-runtime/tests/b4_fork_diff.rs` (new). Add a stub `diff_folds` returning an empty `FoldDiff` first so the file compiles and fails semantically. Tests (names fixed): `diff_folds_aa_is_empty`, `diff_folds_mirror_symmetry`, `fork_diff_exact_on_counterfactual_fixture` (the roadmap falsifier: intern + Asserts A-B, A-B..., `cut = log.len()`, then Assert B-C, a Retract of A-B, a CorrectInterval; `diff_folds(&fork_at(cut), &live)` has exactly the facts minted after the cut as `added` and the invalidated row as `changed`, keyed and sorted), `diff_fold_vs_fork_at_matches_post_cut_mints`, `apply_diff_reproduces_other_side_modulo_seq` (also compared through `as_of` at several valid times and `as_known_on` at two walls), `diff_handles_equal_keys_ci_residuals_and_multipath_heads`, `diff_of_checkpointed_fork_matches_plain_twin` (fork at `c+1..=len`; interior cut is Err). Use the RAII `PersistScratch` helper in `crates/kutha-runtime/tests/common/mod.rs`; leave no dirs under `tmp/`.

## Known traps

- `Fact` is the fold row (`fold.rs`); `facts()` is `fold.rs:552-554`; `fingerprint` hashes facts only (`fold_query.rs`). Do not change the fingerprint contract or any hot path.
- Minted ids differ between independent runs (UUID v7): twin comparisons normalise ids or use the same log prefix on both sides.
- Rank order must be total and independent of insertion order; test with a shuffled copy of the facts.
- CBM misses many `Self::` edges; `rg` is the arbiter. Index refreshed 2026-10-23.

## Gates

- Bench: `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` PRE and POST on the same host with `taskset -c 0`, load recorded (a foreign `/root/prime-agent` cargo may run); more than 10 percent regression in `ingest_plain.n` or `as_of.n` is a stop. Cite numbers in the close CHANGELOG entry.
- Close: `cargo test --workspace --offline --no-fail-fast` exit 0 and `uv run kutha-gov ci --jobs 1` `run_high=0` in `230-VERIFICATION.md` (independent, fresh verifier). Kill-test files stay under about 60 s each.
- Artifact budget: at most two PLAN files of 150 lines or fewer, one SUMMARY of 60 lines or fewer, one VERIFICATION of 60 lines or fewer.
- Docs at close (English): ADR-061 Evidence (fact-set diff shipped; named refs and GED still open; Proposed), `docs/architecture/semantic-gap-review.md` F8 note, `docs/architecture/p0-spike-inventory.md` line, `stca-guide.md` fork/diff note, test-matrix rows plus paragraph, CHANGELOG Product entry.
- Stop if the diff needs a new Op or touches the fingerprint contract, a design conflict with ADR-000 D1-D10 appears, or the bench regresses.
