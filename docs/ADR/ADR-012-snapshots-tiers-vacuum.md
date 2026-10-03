# ADR-012: Snapshots, Tiers, and Vacuum

## Status

**Proposed** (Time/Data placement vs policy GC of the log; not product storage runtime)

### Evidence (2026-10-02; Proposed)

`events.jsonl` persist replaces via same-directory temp file + `rename` (`store.rs` `replace_events_jsonl`). Quantum outcome and justification sidecars remain separate files (gap-review F1: log atomic, sidecars not). **2026-10-03 note:** snapshot/WAL in-place truncate is false; see Evidence 2026-10-03.

### Evidence (2026-10-03; Proposed)

M017 S02: `persist` replaces `snapshot.json` and `events.wal` via tmp+`sync_all`+rename (`replace_via_tmp`, `replace_snapshot`, `replace_events_wal`). `events.jsonl` is SoT; `events.wal` is a covering lease (`load_events_d04`, magic `KUTHAWAL`). `replay_check` calls `check_tt_anchors` (`provenance.rs`) so snapshot `TtAnchor` walls are compared to the log copy. `GraphFold::fingerprint` stays facts-only (`fold_query.rs`); `provenance_fingerprint` is a separate hash. Sidecars `terms.jsonl` / `quantum_outcomes.jsonl` / `justifications.jsonl` are written with `File::create` in place without sync or parent-dir fsync (`write_terms` / `write_outcomes` / `write_justifications`) — SNAP-01. No parent-directory fsync anywhere. Current defect (2026-10-03, rg arbiter; CBM `persist`→`encoded_log`, coverage `metadata_changed`): each `persist` rewrites the whole log (`store.rs` `encoded_log` / `replace_via_tmp`; `wal.rs` `write_wal`). M002 is thawed and planned as GSD v0.12 (slices S01–S07, `.kutha/ROADMAP.md` § M002): SNAP-01, parent-dir fsync, partial-sidecar-as-absent, `check_tt_anchors` log-ahead tolerance, and fail-closed WAL open are scheduled in S02; incremental append / O(N) persist rewrite in S04. Oracles: `wal_valid_covering_preferred`, `tampered_tt_anchor_wall_fails_replay_check`, `abort_snapshot_replace_keeps_previous_bytes`, `abort_wal_replace_keeps_previous_bytes`. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 50 / M002 S02; Proposed)

SNAP-01 closed by Phase 50: sidecars `terms.jsonl` / `quantum_outcomes.jsonl` / `justifications.jsonl` replace via tmp+`sync_all`+rename; `sync_dir` fsyncs the parent after every `replace_via_tmp` rename and once at end of `persist`; unparseable / mid-line / proper-prefix sidecars are absent (intact log opens; well-formed non-prefix still `InvalidData`); open-path `check_tt_anchors_range` allows log `TtAnchor`s with `ingested_at >= snap.next_tt` (`replay_check` stays exact); `recover_events` returns `InvalidData` ("truncated WAL header") when shorter than magic+version and `load_events_d04` does not mask non-`InvalidData` WAL I/O. Incremental append / O(N) persist rewrite remain S04; COMMIT-01 remains S05. Oracles: `m002_snap01.rs`. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 51 / M002 S03; Proposed)

RocksDB builds offline as an optional spike; no wiring. The ROCK-00 map is a droppable `EventId`→file-offset lease behind `rocks-index-spike`; jsonl remains SoT. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 52 / M002 S04; Proposed)

APPEND-01: incremental `Persister` append keeps jsonl/WAL prefix bytes; one log `sync_data` per non-checkpoint quantum; `checkpoint` / `close` refresh snapshot and sidecars via `replace_via_tmp` (logs not rewritten). Full `persist` still rewrites the whole log. O(N) persist rewrite is **closed for the append path**. Snapshot `open` applies log `Op::Define` events beyond `snap.dict_strings` in log order (`from_snapshot`); terms interned after the last checkpoint survive without a new snapshot. Persist/open parity is graph `EventId`s plus dictionary/`TermId`s, not byte equality of encoded logs. Oracles: `m002_append01.rs`. COMMIT-01 remains S05. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 53 / M002 S05; Proposed)

COMMIT-01 option A: a snapshot whose `log_offset` counts fold ops beyond the committed prefix fails closed (`InvalidData`). Pointer: ADR-010/014 2026-10-04. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 54-01 / M002 S06; Proposed)

`open` may load from `manifest.json` + `KUTHASEG` instead of jsonl/WAL; snapshot hydrate and `replay_check` are unchanged. Persist still writes jsonl+WAL. Pointer: ADR-010 2026-10-04 Phase 54-01. Status stays **Proposed**.

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Data** · **Packaging** · **Verify**
- Depends on: ADR-000 (D1, D5), ADR-010
- Anticipates: ADR-041 (CSR as aged picture), ADR-080 (who may vacuum), ADR-091 (cold capsules)

## Context

Replay-from-genesis is correct and too slow. This cell splits three nouns that papers keep mixing:

1. **Where a snapshot lives** (compaction / query-driven placement).
2. **How a hot picture is laid out** (adj-list → CSR aging; RAM vertices vs SSD edges).
3. **Whether history may be physically removed** (vacuum + legal hold).

Grounding cards:

- `paper-lsm-snapshot-compaction` — `.compound-engineering/artifacts/research/applicability/cards/paper-lsm-snapshot-compaction.md`
- `paper-bach-lsm-csr-bridge` — `.compound-engineering/artifacts/research/applicability/cards/paper-bach-lsm-csr-bridge.md`
- `paper-semi-external-graph` — `.compound-engineering/artifacts/research/applicability/cards/paper-semi-external-graph.md`
- `paper-event-log-vacuum-legal-hold` — `.compound-engineering/artifacts/research/applicability/cards/paper-event-log-vacuum-legal-hold.md`
- Adapter mapping (crate identifier, not a new card): `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md` (P-Temporal-Tensor)

LSM snapshots are a **compaction policy**: place them where queries land, not on a uniform wall clock. BACH ages adjacency lists into CSR inside LSM levels for mixed TP/AP. Semi-external graphs keep vertex state in RAM and stream edges from SSD — still a lease. Vacuum is **policy GC of the SoT**; legal hold pins ranges so vacuum cannot touch them. After vacuum, AS-OF of a removed state is a **defined failure**, not a silent hole.

## Decision

### D012-1. Snapshots are leases of the log, placed by query load

Snapshots shorten Yankin reconstruction. They are not SoT. Placement is a Control/pack policy (query-distribution clustering, time-aware compaction knobs). Compaction that **reorganizes layout** is allowed. Compaction that **drops losers silently** is TOKI-illegal (ADR-013).

### D012-2. Aging pipeline: mutable adj → frozen CSR (BACH-shaped)

Hot write-friendly adjacency at the top of the pipeline; AP-friendly CSR after aging (BACH). Samyama frozen CSR (ADR-041) is the in-process cousin of the same noun. LSM levels are not truth.

### D012-3. Tiers: in-RAM fold, SEM edge shards, cold archive

When edges do not fit RAM, **semi-external** placement (vertex state in RAM, edges on SSD) is a materialization tier, not a second log. Cold archive (later RVF/pack) is packaging. Crash recovery rebuilds vertex state from the **log**, not from a dirty edge cache.

### D012-4. Vacuum ≠ compaction; hold pins SoT

Vacuum physically removes expired transaction-time history under a **declared policy**, and logs that the drop happened. A **legal hold** is a grant-shaped fact: vacuum must fail closed on held ranges. LLM does not choose what to forget. P0 may be append-only-forever; honeycomb owns vacuum+hold as a pack.

### D012-5. Temporal vector tiering (Proposed adapter mapping)

Historical vector embeddings over diachronic entities (norm editions, agent memories) must not induce memory exhaustion (Profile B/C). When this cell is implemented, the named adapter identifier is `ruvector-temporal-tensor` (P-Temporal-Tensor in the RuVector adaptation note): Hot (8-bit), Warm (7/5-bit), and Cold (3-bit) with temporal segment delta reuse and frame-level random access decode. Like CSR/HNSW, tiered vector buffers are droppable leases derived from the log, not secondary sources of truth. This mapping does not add a Cargo dependency and does not authorize HNSW or M002.

**Hard separations:**

```text
Snapshot placement     ≠  Vacuum (physical erase)
Layout compaction      ≠  Deleting losers
SEM / SSD edge shards  ≠  Event log
Vector tiering lease   ≠  Authoritative fact history
Legal hold             ≠  LSM tombstone
Defined AS-OF failure  ≠  Silent hole after GC
```

### Clarification (2026-09-30; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F6): snapshots remain leases of the log (D012-1). Snapshot identity is the pair (log offset, fingerprint) and must be verified when that snapshot is opened. Persist of the log is atomic and append-only; a truncated or rewritten log file is not a legal open. Tampered snapshot bytes that disagree with the retained log fail closed on open, not only on later semantic replay. This subsection does not rewrite D012-1…D012-5 and does not mark this cell Accepted.

## Consequences

### Positive

- Legal PIT can stay cheap without Raphtory-in-RAM history.
- CSR aging has a named pipeline instead of “just compact Rocks.”
- GDPR/e-discovery have a place that is not “delete the fold.”

### Negative / risks

- Query-driven snapshot placement assumes a stable mix.
- Vacuum without 080/policy graph will be ad hoc.

### Non-goals (this ADR)

- Implementing LSM or GraphChi.
- ABAC of *who* may vacuum (ADR-080).
- Treating blockchain ADS as the archive (anchor only, ADR-014).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Uniform wall-clock snapshots | Wastes cold time; starves hot AS-OF |
| Compaction deletes history | Violates vacuum card + TOKI losers |
| SSD as SoT | SEM is a lease; log remains SoT |
| Agent “what to remember” as GC | LLM must not choose forget |

## Open Research Questions

1. P0: mmap/CSR-if-fits vs when to cut SEM shards.
2. Snapshot cadence as a pack policy vs engine default.
3. Vacuum query semantics: typed `Vacuumed` / `Held` outcomes (cousin of ADR-090 TR-06).
4. Schema vacuuming when historical schemata still interpret remaining facts.

## Related Decisions

- ADR-010 — log quantum
- ADR-013 — losers stay until vacuum policy says otherwise
- ADR-040/041 — leases and CSR
- ADR-042 — HNSW fence (vector access method; still frozen in STATE)
- ADR-080 — who may vacuum (later)
