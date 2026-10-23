# Phase 229 — checkpoint is a cut, not a hole (M155 S01)

Source: 2026-10-23 read-only audit (findings F1, F2, F3; CBM `kutha-graph` + `rg` arbiter) and the B2 scout §4. Discuss is skipped; decisions below are locked.

## Goal

Make the M154 checkpoint honest for its neighbours. A compacted prefix is an atomic cut: queries that need events inside it fail closed (ADR-012 D012-4 "defined failure, not a hole"); cascades that start from a prefix fact behave exactly like the un-checkpointed twin; the migrate tool refuses to demote a checkpointed store. No new Op, no thaw, no fingerprint-contract change, no vacuum (B1b stays open).

## Locked decisions

1. `Runtime::fork_at(n)` becomes fallible: `Result<Self, RuntimeError>` with a new typed variant (name `ForkCutUnavailable`, carrying `cut` and `log_len`, Display in English). Rules, with `c` = index of the `Op::Checkpoint` event in the current log (none if the log has no receipt):
   - `n == 0` Ok (empty fold, as today);
   - `1 <= n <= c` Err (cut inside the compacted prefix and its kept events);
   - `n > c` and `n <= log.len()` Ok (receipt included, result equals the original sealed-prefix state plus tail events up to `n`);
   - `n > log.len()` Err (today `EventLog::prefix` silently returns the full log).
   No reconstruction of interior cuts from `snapshot.json` or the checkpoint blob (that would fake SoT).
2. Existing `fork_at` callers get mechanical `.unwrap()`/`?` edits (about 26 hits, 15 files). Recorded as an old-to-new table in the SUMMARY. No assertion changes. The production caller `rollback_user_cascade` (`pipeline.rs`) captures `log.len()` before the failed cascade, so its cut is valid; use `expect` with that reason.
3. Horn cascade parity (F1, confirmed only by a RED): on a store reopened after `checkpoint_sealed_prefix`, a live `emit(Retract)` / `emit(Correct)` of a fact that was a Horn premise inside the compacted prefix must un-derive/re-derive exactly as in the un-checkpointed twin (same fingerprint and same live set, same `as_of`). The existing kill suite copies cascade events from the twin and never emits on the checkpointed runtime, so write the RED with real `emit`. Root cause is expected in `rebuild_log_indexes` (`pipeline.rs`) / `derived_by_cause` / `log_event` (`eval.rs`) walking only remaining `Op::Behavior` events. Fix by rebuilding the needed indexes from the restored facts (preferred) or, if that cannot be exact, fail closed with a typed error. If the RED passes on current code, the finding is wrong: record it, drop the item, and stop that item (not the phase).
4. `migrate_compat` / `migrate_op_wire` (`store.rs`, `kutha-migrate`): if the log contains `Op::Checkpoint`, return `Err(InvalidData)` with an English message, directory unchanged (no demote, no snapshot delete). Update `docs/process/op-compat.md` operator section (it currently says "demotes, answers unchanged"). The kill3 test that observes the demote behavior (`rg migrate_compat crates/kutha-runtime/tests/b1_checkpoint_kill3.rs`) asserts the old behavior: this is the ONE allowed assertion change; record it explicitly (old assertion, new assertion, reason) in the SUMMARY.
5. Out of scope: lineage/sidecar dangling ids (`caused_by`, sidecar cites), receipt authentication (F4: the receipt is a consistency check, not an authenticator; plain logs have none either), `StoreBlob` digest on open, `horn_behavior_reproducible` fail-open, an operator CLI for checkpoint, B2 diff, any harness code (moratorium; a Rust test pins each invariant).

## REDs (record the failure before the change)

- `crates/kutha-runtime/tests/b3_checkpoint_cut.rs` (new): `fork_at_interior_of_checkpointed_prefix_is_err` (persist, checkpoint, open, `c`, assert fork at `1..=c` and `len+1` is Err, `0` and `c+1..=len` Ok with `facts().len()` equal to the receipt `fact_count` at `c+1`), `fork_at_past_log_end_is_err_without_checkpoint`, `retract_prefix_horn_premise_after_checkpoint_matches_twin`, `migrate_compat_refuses_checkpointed_store_and_leaves_dir_unchanged`. Against current code the first compiles only if the signature stays `-> Self`: write the RED first against the current infallible signature using `std::panic::catch_unwind` free assertions (`assert_ne!(fork.fold().facts().len(), 0)` for the interior cut), record the failure, then change the signature and convert to `is_err`.
- Twin helper: build the same event sequence on an un-checkpointed store (see `b1_checkpoint_kill3.rs` for the twin pattern) and compare fingerprint plus live sets.

## Known traps

- `EventLog::prefix` (`log.rs`) uses `get(..n).unwrap_or(&self.events)`; fix at `fork_at`, not by changing `prefix` semantics for other callers unless `rg` shows none.
- `n` is an offset into the CURRENT log, not an original event index.
- Persist scratch dirs: new tests must use the RAII `PersistScratch` helper in `crates/kutha-runtime/tests/common/mod.rs` (no leaked dirs under `tmp/`).
- CBM misses many `Self::` edges; `rg` is the arbiter for callers. The index was refreshed on 2026-10-22; paths edited after that are `metadata_changed`.

## Gates

- Bench: `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline --offline` PRE and POST alone on the same host (note load; a sub-baseline value is noise); more than 10 percent regression in `ingest_plain.n` or `as_of.n` is a controller stop. Cite numbers in the close CHANGELOG entry.
- Close: `cargo test --workspace --offline --no-fail-fast` exit 0 and `uv run kutha-gov ci --jobs 1` `run_high=0` in `229-VERIFICATION.md` (independent, fresh verifier; passes only with all kill-tests green).
- Artifact budget: at most two PLAN files of 150 lines or fewer, one SUMMARY of 60 lines or fewer, one VERIFICATION of 60 lines or fewer.
- Stop if an oracle assertion other than the one named in decision 4 must change, the fork rules conflict with ADR-000 D1-D10, or the bench regresses.
