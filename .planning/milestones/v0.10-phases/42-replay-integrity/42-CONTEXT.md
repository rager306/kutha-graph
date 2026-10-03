# Phase 42 Context: Replay integrity

**Milestone:** v0.10 / harness M017 S02 (unleased until Phase 41 closes)
**Requirements:** REPL-01, REPL-02, REPL-03, REPL-04, KNOWN-03
**Depends on:** Phase 41
**Source:** 2026-10-02 audit findings F01, F02, F04, F06, F07, F08, F10 (code subagent, CBM + GitNexus, `rg` arbiter)
**Run rules:** `docs/process/gsd-auto-contract.md` (tool pair: CBM + GitNexus on product edits)

## Locked decisions

- **D-01 (RED first):** each REPL oracle fails on `c702d71` and the failure is recorded before the change. If a RED oracle passes, stop: the finding is wrong. **Exception (KNOWN-03):** the `as_known_on` / `known_at` oracles pin behaviour that is already correct (the defect is the rustdoc only), so they are expected GREEN on `c702d71` and are not a stop condition. **Hydrate trap (REPL-01):** `from_snapshot` rebuilds `tt_anchors` from the loaded log, so the anchor oracle must tamper the persisted copy and the check must compare against it; a fold-vs-replay comparison alone is tautological.
- **D-02 (anchors are replay-visible):** `Op::TtAnchor` stays out of the fact fingerprint's meaning, but `replay_check` / `open` must compare anchors (wall, position) between log and fold lease. Do not make anchors fold facts.
- **D-03 (atomic replace):** `snapshot.json` and `events.wal` use the existing temp + sync + rename pattern from `events.jsonl` (`store.rs`). Reuse the injected-abort helper style; no new dependency.
- **D-04 (open order):** WAL preferred only if it parses with valid magic and covers the log; otherwise fail closed or fall back to intact `events.jsonl`. The choice is recorded in the PLAN and ADR-010 Evidence.
- **D-05 (leases never win):** sidecars (`outcomes`, `justifications`, `terms`) are checked against log records on `open`; disagreement is an error, not a merge.
- **D-06 (index mode):** snapshot serializes `FoldIndexMode`; env var only sets the mode for a new fold.
- **D-07 (oracle preservation):** FF5, FF6, M010–M016 named tests keep assertions. Mechanical renames go in an old→new table in SUMMARY.
- **D-08 (non-goals):** typed `Op` clocks (F03/F14), Horn at knowledge cut (F05), segments/hash chain/manifest (M002), partial-interval Retract, blob digests in fingerprint (note only).
- **D-09:** Verifier ≠ executor; controller owns `.kutha/STATE.md`; benchmark re-run at close, over 10 percent regression in ingest or `as_of` is a stop.

## Blast radius (to fill in at plan time)

Symbols: `persist`, `open`, `finish_open`, `append_events`, `replay_check`, `GraphFold::deserialize`, `FoldIndexMode::from_env`, `Runtime::as_known_on`, `known_at`. Record CBM inbound, GitNexus `impact`, and `rg` counts; `rg` wins on disagreement.

## Verifier kill-tests

- Truncate WAL to magic-only with intact `events.jsonl`: `open` is Err or returns the full jsonl log, never an empty one.
- Rewrite one persisted `TtAnchor` wall: `replay_check` Err.
- Abort before snapshot/WAL rename: previous file byte-identical.
- Mismatched outcomes sidecar vs log outcomes: `open` Err.
- Persist under `KUTHA_FOLD_INDEX_MODE=eager`, open under `lazy`: same fingerprint, same `as_known_on`, mode from snapshot.
- `as_known_on(D)` ≠ `live_at(tt, u64::MAX)` on a closed VT interval.

## Harness

Active Milestone M017, Active Slice **S02** (when leased).
