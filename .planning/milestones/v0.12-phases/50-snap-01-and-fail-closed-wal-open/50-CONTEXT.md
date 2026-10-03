# Phase 50 Context: SNAP-01 and fail-closed WAL open

**Milestone:** v0.12 / harness M002 S02
**Requirements:** SNAP-01, OPEN-01, CRASH-01
**Depends on:** Phase 49 (thaw map; shipped)
**Source:** 2026-10-03 re-verification at `f4af544`: provenance audit, durability design research, code-logic audit (CBM + `rg` arbiter)
**Run rules:** `docs/process/gsd-auto-contract.md` (RED first; oracle preservation; bench gate; CBM primary, GitNexus secondary outside M013)

## Locked decisions

- **D-01 (scope):** `crates/kutha-runtime/src/{store,wal,provenance,hydrate}.rs` and tests only. No segments, no manifest, no incremental append (Phase 52), no `rocksdb`, no new dependency, no change to the `Event` encoding, WAL format or file names.
- **D-02 (directory fsync):** add one helper that opens the directory and calls `sync_all`; call it after every `rename` in `replace_via_tmp` and once at the end of `persist`. A platform where opening a directory fails (non-Unix) returns Ok without syncing; record that in the helper's doc.
- **D-03 (sidecars atomic):** `terms.jsonl`, `quantum_outcomes.jsonl`, `justifications.jsonl` are written through `replace_via_tmp` (tmp name `<file>.tmp`, `sync_all`, rename, dir fsync). Persist order stays WAL, jsonl, terms, snapshot, outcomes, justifications.
- **D-04 (partial sidecar = absent):** a sidecar that does not parse, ends mid-line, or whose rows are a proper prefix of the log-derived rows (an older persist) is treated as absent; an intact log opens. Well-formed rows that are not a prefix of the log rows still fail with `InvalidData` (oracle `mismatched_outcomes_sidecar_fails_open` keeps its assertion). Same rule for `terms.jsonl` used as the last-resort dictionary: unparseable means absent, then the existing "cannot restore intern meanings" error applies.
- **D-05 (log ahead of snapshot):** `check_tt_anchors` on the open path compares snapshot-held anchors with log anchors whose `ingested_at < snap.next_tt`; log anchors at or after `next_tt` are tail and allowed. Missing, changed, or extra anchors inside the covered range still fail (hydrate trap; oracle `tampered_tt_anchor_wall_fails_replay_check` unchanged). `replay_check` on an in-memory runtime keeps exact equality.
- **D-06 (WAL open):** `recover_events` returns `InvalidData` ("truncated WAL header") when the file is shorter than magic+version, and propagates any non-`UnexpectedEof` read error; a torn tail record still ends the scan without error. `load_events_d04`: WAL `Err` with kind `InvalidData` plus readable jsonl falls back to jsonl (WAL is a lease; oracles `wal_header_only_*`, `wal_magic_only_*` unchanged); a WAL `Err` of any other kind is returned (a failing disk is not masked); WAL with no jsonl never yields an empty log.
- **D-07 (crash matrix):** add a `#[doc(hidden)] pub` seam that makes `persist` stop with an injected error after each named step (wal, jsonl, terms, snapshot, outcomes, justifications). For every step, over a prior persisted state A and a new state B, `open` returns a runtime whose graph log ids equal A's or B's (never anything else, never empty) and passes `replay_check`, or returns `InvalidData`; for steps after jsonl it must be Ok(B).
- **D-08 (RED first):** the RED commit contains the failing oracles and nothing else; record the failures in the SUMMARY. Then GREEN commits.
- **D-09:** Verifier ≠ executor. Only the controller edits `.kutha/STATE.md`, `.planning/STATE.md`, README lease row. Do not write `50-VERIFICATION.md`.

## Oracles (named, new tests in `crates/kutha-runtime/tests/m002_snap01.rs`)

`sidecar_truncated_mid_line_opens_from_log`, `sidecar_stale_prefix_opens_from_log`, `sidecar_unparseable_terms_is_absent`, `sidecar_spoof_not_prefix_still_fails`, `persist_leaves_no_tmp_files`, `log_ahead_of_snapshot_opens_with_tail_fold`, `extra_anchor_inside_snapshot_range_fails`, `wal_short_header_without_jsonl_is_err`, `wal_read_io_error_not_masked`, `crash_matrix_every_step_reopens_to_prefix`. Kill-mutations the verifier will try: sidecar via `File::create`, exact anchor equality, `Ok(vec![])` on short header, missing dir fsync call (counted through the helper), prefix rule dropped.

## Docs (same phase)

ADR-012/014/060 Evidence 2026-10-03: SNAP-01 closed by Phase 50 (sidecars atomic, dir fsync, anchor tolerance, fail-closed WAL open); incremental append and COMMIT-01 remain S04/S05. `semantic-gap-review.md` F1/F6 status line. `honeycomb.yaml` restage comment. `.planning/REQUIREMENTS.md` ticks nothing (controller). One CHANGELOG entry, product and process kept apart, in the final commit. Rustdoc on touched items.

## Bench gate

`crates/` is edited: before close run `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline` in an isolated `CARGO_TARGET_DIR` against `docs/architecture/benchmark-baseline.md` (Phase 44 numbers); a regression over 10 percent in `ingest_plain.n` or `as_of.n` is a stop. Persist is not benched here (Phase 52 adds it). Record the numbers in `benchmark-baseline.md`.

## Non-goals

Segments, hash chain, commit marker, incremental append, Rocks, typed clocks, C01, any change to fold/Horn semantics, `attach_outcomes`/API closures.

## Harness

Active Milestone M002, Active Slice **S02**.
