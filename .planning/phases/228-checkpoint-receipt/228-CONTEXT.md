# Phase 228 — checkpoint receipt (M154 S01, thin B1)

Source: controller scout 2026-10-22 (CBM `kutha-graph` + `rg` arbiter). Discuss is skipped; decisions below are locked for this phase.

## Goal

A sealed log prefix can be replaced by one **on-log checkpoint** plus a **receipt op**, so the sealed log never shrinks without a receipt and strict replay stays exact. This is ADR-012 layout compaction with a receipt. It is **not** vacuum (D012-4): no TT history is erased, loser facts stay (D013-2), `fingerprint` still hashes all facts including dead ones.

## Locked decisions

1. The checkpoint lives **on the log** (a `StoreBlob` of the prefix fold state, same shape as the existing `Snapshot` fold + dict strings) and is referenced by `Op::Checkpoint { first_event_id, last_event_id, dropped_count, fact_count, fingerprint_hex, blob_digest }`. `snapshot.json` stays a lease, never the checkpoint carrier (ADR-012 D012-1, ADR-000 D1).
2. `Op::Checkpoint` is a modern serde variant. Old readers fail open; this is a documented forward break (`docs/process/op-compat.md`). No serde aliases, no `remap_legacy_ops` entry, no new `op-wire.v1` meaning, `kutha-migrate` tables untouched. Do not run `migrate_op_wire` on a checkpointed store (it demotes segments).
3. `CommitClass::SelfCommit`; `op_affects_fold = true` (the checkpoint is the prefix fold); user `emit` of it is rejected like other meta ops (`reject_forged_meta` / `MetaOpRejected`).
4. Entry point: `store::checkpoint_sealed_prefix(dir)` under `kutha.lock`: rewrite to a new seq-1 segment chain (checkpoint blob + `Op::Checkpoint`, then the unchanged tail), `remove_obsolete_segments`, refresh the snapshot. Append via the store API, not `Runtime::emit`.
5. Exactness oracle: `open` Ok; `replay_check` Ok; `fingerprint` equal; `as_of` equal at several VT points; `as_known_on` equal at two TT walls; replay of the new log equals the original fold fingerprint.
6. Out of scope: expired-TT vacuum, legal hold, defined AS-OF failure, CSR/SEM/HNSW tiers, Rocks layout, Cypher, any change to the fingerprint contract, any harness code beyond one check or matrix row pinning this slice's invariant (moratorium).

## RED (record the failure before the change)

`crates/kutha-runtime/tests/b1_checkpoint_receipt.rs::checkpoint_sealed_prefix_preserves_fingerprint_and_pit`: fixture with `set_test_segment_limits(Some(4096), Some(3))` (see `m002_seg01.rs`), intern + `emit_allow` + N Asserts across at least two sealed files + one Retract + `emit_tt_anchor` at two walls + `persist`; then call `store::checkpoint_sealed_prefix`. On current code this does not compile (symbol missing); that is the recorded RED. Do **not** use "delete a sealed file then open fails" as the RED (it already passes).

## Known traps (from the scout)

- Exhaustive `Op` matches that fail to compile on a new variant: `op_commit_class`, `op_relation`, `op_affects_fold`, `GraphFold::apply`, `Event::digest_bytes`, `horn_behavior` skip list (`provenance.rs`), `Event::new` arms.
- `every_op_variant_is_classified` in `crates/kutha-runtime/tests/m002_commit01.rs` is a fixed-size table and will **not** fail to compile: add a row.
- Existing symbols to reuse: `Snapshot` (`snapshot.rs`), `persist_log` / `remove_obsolete_segments` (`segment.rs`), `GraphFold::from_snapshot` (`hydrate.rs`), `replay_check` (`store.rs`).
- CBM misses many Rust `Self::` edges; `rg` is the arbiter for callers. CBM inbound `Op` degree is usage, not calls.

## Gates

- Bench: `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline` pre and post on the same host; regression over 10 percent in `ingest_plain.n` or `as_of.n` is a controller stop. Cite numbers in the close CHANGELOG entry.
- Close: `cargo test --workspace --offline --no-fail-fast` exit 0 and `uv run kutha-gov ci --jobs 1` `run_high=0`, recorded in `228-VERIFICATION.md` (independent, fresh verifier).
- Artifact budget: at most two PLAN files of 150 lines or fewer, one SUMMARY of 60 lines or fewer, one VERIFICATION of 60 lines or fewer.
- Stop if an oracle assertion must change, a RED passes on current code, or ADR-000 D1-D10 blocks the design.
