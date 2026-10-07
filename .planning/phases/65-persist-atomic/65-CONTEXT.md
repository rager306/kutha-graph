# Phase 65: Persist atomic rewrite — Context

**Milestone:** v0.14 / harness M020 S03
**Requirements:** PA-01 segment tmp+rename, PA-02 crash-window oracle, PA-03 bench ≤10%
**Depends on:** Phase 64 closed; lease S03 Active
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

`persist_log` / `write_segment_file` must not truncate sealed or active segment paths in place. Write sibling `*.tmp`, `sync_all`, rename; `manifest.json` stays last via existing `write_manifest` (`replace_via_tmp`). Crash mid-rewrite leaves prior store openable or clean failure. Bench gate ≤10% on `ingest_plain.n` / `as_of.n` vs pre-change local baseline. Honeycomb stays **Proposed**. Do not edit `.kutha/STATE.md`. Do not write `65-VERIFICATION.md`.

## Locked decisions

- **D-01 (PA-01):** `write_segment_file` uses `store::replace_via_tmp` (path + `path` with `.tmp` suffix). Same path for `rewrite_segment_with_seal_trailer` and `write_test_segmented`.
- **D-02 (PA-02):** Test seam `abort_write_segment_before_rename` (commit=false) leaves dest bytes intact; `open` / `load_events` still succeed on a previously persisted dir. Successful re-persist changes segment inode (rename).
- **D-03 (PA-03):** After green tests, `cargo bench -p kutha-runtime --bench baseline` with `KUTHA_BENCH_N=10000`; stop if `ingest_plain.n` or `as_of.n` regress >10% vs the measurement taken immediately before the code change on the same host.
- **D-04:** Update characterization `write_segment_file_streams_frames` and `seg_manifest_tmp_sync_all_before_rename` counts for the extra `replace_via_tmp` calls. Product CHANGELOG. No ADR Accept / honeycomb restage. No Horn/fold (Phase 66).

## Deferred (not this phase)

CorrectInterval residuals, `live_triples`, Retract gate, oracle stub → 66. Process lease/bench contract → 67. Rollover append path (`persist_append` new segment create) — only if it shares `write_segment_file`; do not redesign append.

## Verifier kill-tests

- Source: `write_segment_file` body calls `replace_via_tmp` (no in-place `File::create` on dest).
- Re-persist changes `seg-*.log` inode.
- Abort before rename: dest unchanged; store still opens; fingerprint matches pre-abort.
- Live: targeted cargo tests green; bench ≤10% (record numbers in VERIFICATION).
- No `.kutha/STATE.md` edits in phase commits.

## Harness

Active Milestone M020, Active Slice **S03**.
