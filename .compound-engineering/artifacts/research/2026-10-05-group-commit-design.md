# GROUP-01: Typed/Dictionary group vs `committed_prefix`

**Date:** 2026-10-05. **Phase:** 60 / M019 S05. **Status:** design note (not an ADR). **Hard stop:** Event / Op / Snapshot / `KUTHASEG` frame change → stop; do not implement.

**Problem.** `commit::committed_prefix` (`commit.rs`) keeps through the last `Op::QuantumOutcome`, then keeps `SelfCommit` until the first `NeedsOutcome`. User quanta carry a trailing QO. Typed (`record_justification`) and Dictionary (`intern` / `emit_allow` / `emit_rule*`) groups do not. A crash mid-group leaves a readable SelfCommit prefix. That is Define-level atomicity, not group atomicity (ADR-010/014 Evidence 2026-10-04).

**Writer today.** `Persister::append` writes one frame per event, then one `sync_data`. POSIX `write` of several frames is not a group: a crash can leave complete frames 1..k of a k+1 group. Recovery already drops only a torn last frame (`scan_segment` / WAL recover). It cannot tell “group of 2 complete” from “group of 3 that died after 2” without extra metadata.

## Options

### (a) One frame-batch + one sync; torn batch absent via a group-length header

Write the group as one syscall, one sync. Recovery treats a torn batch as absent using a **group-length header** in the frame stream.

- **Failure modes.** Partial `write` of a multi-frame buffer still yields complete prefix frames. Without a header, recovery cannot drop those frames as “the batch”. With a header, a crash after header+prefix and before the last frame is defined, but the header **is** a frame-format change (`KUTHASEG` / `KUTHAWAL` v2). Tail guard (`last_frame_len` + last `EventId`) would need a new last-record shape. Named oracles (FF5/FF6, M010–M017, `m002_*`) assume v1 frames.
- **Format impact.** Event/Op/Snapshot untouched; **segment/WAL frame format changes**. D-03 hard stop.
- **Oracle.** Crash after k of n frames → open hides the whole group (not just a torn tail). Kill: keep the k complete frames.

**(a) without a format change** is not recovery: one `write`+`sync` is writer hygiene only. After crash, complete prefix frames stay visible. Not “(a)-style recovery”.

### (b) Trailing group-marker op

Append a SelfCommit (or new) op that closes the Typed/Dictionary group. Recovery keeps a group only when the marker is present.

- **Failure modes.** A new `Op` variant is an Op-schema change (ADR-000 D-locks, COMMIT-01 A, `op_commit_class` exhaustive match). Reusing `QuantumOutcome` for Dictionary/Typed collapses ReceiptMode and breaks “QO marks a User quantum” (F11 closed by option A). A reserved `kutha:*` Assert as a marker is still a log-schema convention and new writer paths; crash before the marker is the same hole.
- **Format impact.** **Op (and likely Event encoding) change**, or a silent second meaning for QO. D-03 hard stop.
- **Oracle.** Partial group without marker is hidden. Kill: treat marker-less SelfCommit suffix as committed.

### (c) Documented residual + characterization oracle

Leave `committed_prefix` as shipped (COMMIT-01 A). Pin today’s behaviour: a crash inside a Typed/Dictionary group can expose the SelfCommit prefix that reached disk; a `NeedsOutcome` still cuts the suffix (and drops later SelfCommit in that suffix).

- **Failure modes.** Partial dictionary/typed groups remain visible. Not a writer bug; the rule has no group boundary. Closing it later needs (a) or (b) and a format/schema lease.
- **Format impact.** None.
- **Oracle.** `mixed_typed_dict_partial_group_is_visible` (characterization). Kill: drop every event after the last QO (Define-only / `emit_allow` suffix must still be kept today).

## Verdict

**(c).** (a)-style recovery needs a group-length header (frame-format). (b) needs an Op/marker schema change. Both are D-03 hard stops. (a) without a format change cannot hide a complete mid-group prefix. Proceed: no `Event`/`Op`/`Snapshot`/frame edit; matrix row `DUR-mixed-typed-dict` → `characterization`; ADR-010/014 Evidence 2026-10-05 names the residual. WRITER-01 (advisory `kutha.lock`) is independent and does not close this hole.
