# ADR-010: Event Log & Reactive Runtime Quantum

## Status

**Proposed** (Time-band physics — log as SoT and one emit→idle quantum; not product runtime)

### Evidence (2026-10-02; Proposed)

Historical (superseded by the 2026-10-04 blocks below): M013 routes Assert/Retract/Correct/Behavior and dictionary constructors through `pipeline` (`pipeline.rs`). `rg append_meta crates/` is empty. Oracle: `unknown_relation_rejected_for_assert_justification_and_action` (`m013_write_path.rs`).

### Evidence (2026-10-03; Proposed)

Historical (superseded by the 2026-10-04 blocks below): D-04 open policy (M017 S02): jsonl is SoT; WAL is a lease. Prefer WAL only when recover yields magic `KUTHAWAL` + version 1 and recovered ids cover jsonl ids in order (WAL may be longer). Header-only, magic-only, empty, short, or recover-err plus readable jsonl → jsonl (never an empty log). Both parse and neither sequence covers the other → InvalidData. Snapshot pins `tt_anchors` (replay-visible, not fingerprint) and `FoldIndexMode`. `persist` replaces `snapshot.json` and `events.wal` via tmp+`sync_all`+rename (`replace_via_tmp`, `replace_snapshot`, `replace_events_wal`). Oracle: `m017_replay_integrity.rs` (`wal_valid_covering_preferred`, `tampered_tt_anchor_wall_fails_replay_check`, `abort_snapshot_replace_keeps_previous_bytes`, `abort_wal_replace_keeps_previous_bytes`). Current defect (2026-10-03, rg arbiter; CBM `persist`→`encoded_log`, coverage `metadata_changed`): each `persist` rewrites the whole log (`store.rs` `encoded_log` / `replace_via_tmp`; `wal.rs` `write_wal`). M002 is thawed and planned as GSD v0.12 (slices S01–S07, `.kutha/ROADMAP.md` § M002); incremental append / O(N) persist rewrite is scheduled in S04. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 51 / M002 S03; Proposed)

Historical (superseded by the 2026-10-04 blocks below): RocksDB builds offline as an optional spike; no wiring. Feature `rocks-index-spike` on `kutha-runtime` (`rocksdb` 0.22.0, `librocksdb-sys` 0.16.0+8.10.0, `default-features = false`) holds an `EventId`→`events.jsonl` offset map in `rocks_spike`. Default workspace tests do not compile RocksDB. Not wired into `Runtime`, persist, fold, or hydrate. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 52 / M002 S04; Proposed)

Historical (superseded by the 2026-10-04 blocks below for the default writer): APPEND-01: `store::Persister` (`attach` / `append` / `checkpoint` / `close`) writes only events beyond the durable tail: JSONL `write_all` then WAL length+CRC32 frames, then one WAL `sync_data` (commit point). Prefix bytes of `events.jsonl` / `events.wal` are not rewritten. A torn jsonl last line without `\n` is dropped; a complete malformed line still fails `InvalidData`. Full `persist` remains the atomic rewrite. The O(N) persist rewrite defect is **closed for the append path**. Oracles: `m002_append01.rs`. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 53 / M002 S05; Proposed)

COMMIT-01 option A: `Op::QuantumOutcome` stays the User-quantum commit marker (no WAL/jsonl format change, no env switch). `store::open` applies `commit::committed_prefix` after load and before sidecar load, hydrate, and `replay_check`. Self-commit: `Define`, `TtAnchor`, `StoreBlob`, `JustificationCite`, `QuantumOutcome`, dictionary `kutha:*` Assert. NeedsOutcome: product `Assert`/`Retract`/`Correct`/`CorrectInterval`/`Behavior`. F11 is closed by option A (`Op::QuantumOutcome` + `committed_prefix`). Options B/C (commit frame / quantum envelope) stay as an explicit residual (not shipped; S06 shipped segments, not commit frames). Residual: Typed/Dictionary multi-event groups have no outcome by design, so a crash inside such a group can still expose a partial group (`Define`-level atomicity only; ADR-014). Oracles: `m002_commit01.rs`. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 54-01 / M002 S06; Proposed)

Historical (superseded by the 2026-10-04 blocks below): SEG-01 reader: when `manifest.json` is present, `open` loads `KUTHASEG` v1 sealed files (full verify: magic, trailer, count/first/last/bytes/SHA-256 chain, stitch order) then the active segment (torn last frame dropped; a seal trailer on a still-named-active file ends the stream). `events.jsonl` / `events.wal` are not read. Absent manifest keeps `load_events_d04`. Then `committed_prefix` → hydrate/`replay_check` unchanged. `persist` / `Persister` still write jsonl+WAL. Oracles: `m002_seg01.rs`. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 54-02 / M002 S06; Proposed)

Empty and already-segmented dirs persist `KUTHASEG` + manifest. `Persister` appends to the named active file only (one `sync_data` per quantum; rollover trailer then header then manifest flip). Checkpoint does not regenerate jsonl. `open_for_append` heals with a full segmented persist. Legacy jsonl+WAL writer unchanged. Oracles: `m002_seg01.rs`. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 54-03 / M002 S06; Proposed)

SEG-01: `store::migrate` loads `load_events_d04` then `committed_prefix`, writes segments + manifest, verifies reopen (ids + fold fingerprint), leaves `events.jsonl` as an export, removes `events.wal` only after that flip. A verify failpoint that corrupts a freshly written segment returns `InvalidData` and unflips the manifest so the legacy store stays intact. `store::export_jsonl` is on-demand (`replace_via_tmp`); `open` never reads jsonl when a manifest exists. Header is 44 bytes (magic + v1 + previous-segment chain tip; H0 only for the first segment). Open checks sealed+active sequence contiguity, then each sealed file (missing, chain link, CRC, trailer, count, bytes, hash chain, manifest fields) and that the active header links to the last sealed tip. Tamper oracles use those distinct needles. This does **not** stop an attacker who rewrites every file including the manifest and recomputes hashes; the manifest tip is not externally anchored. Residuals: Typed/Dictionary group hole (COMMIT-01); no multi-writer lock; WAL duplication gone only for segmented dirs; `rocks_spike` still indexes jsonl offsets until S07. Oracles: `m002_seg01.rs`. Status stays **Proposed**. D1 event log = SoT unchanged.

### Evidence (2026-10-04, Phase 55 / M002 S07; Proposed)

ROCK-01: optional feature `rocks-index` opens a droppable `SegmentIndex` (`rocks_index.rs`) beside the segmented store. Rocks holds locators and ids only: `e:` + EventId → (seq, offset, frame_len) BE 16 bytes; `d:` + UTF-8 delivery_key → last Assert EventId with that key (same last-wins rule as Runtime `by_delivery_key`; Retract/Correct do not remove); `m:v` u32be=1; `m:tip` 88 bytes = sealed chain hex + count + last EventId. Payload, commit frames, dispositions, term meanings, and the hash chain stay out of Rocks. Build/catch_up read the verified segmented log (committed prefix only); legacy dirs fail `InvalidData` "run store::migrate first". `open_or_rebuild` drops and rebuilds when `m:tip` mismatches or the tip EventId frame fails CRC/id. `get_event` re-reads the segment frame. Deleting the Rocks directory changes no log answer. Not wired into `Runtime`, `store::open`, `Persister`, fold, or hydrate. Phase 51 `rocks-index-spike` / jsonl `rocks_spike` stay. Oracles: `m002_rock01.rs` (feature `rocks-index`; default `cargo test` does not run them). Status stays **Proposed**.

### Evidence (2026-10-05, Phase 60 / M019 S05; Proposed)

WRITER-01: advisory `File::try_lock` on `kutha.lock` inside the store directory (local filesystem only; not multi-host / NFS coordination). Taken by `Persister::attach`, `open_for_append`, `store::persist`, `store::migrate`, and `store::export_jsonl`. Readers (`store::open`) take no lock. Second writer fails closed (`ErrorKind::WouldBlock`, message names `kutha.lock`). A lock file left by a dead process does not block (advisory; dies with the process). Panic/unwind Drop unlocks. GROUP-01 verdict (c): no Event/Op/Snapshot/frame-format change; a crashed Typed/Dictionary group can still expose a SelfCommit prefix (`Define`-level atomicity). That residual is pinned as characterization, not closed. Residuals still open: no external chain anchor; `open` O(N); `Persister` not wired into `kutha-tenant`. Oracles: `m019_dur_a.rs`. Status stays **Proposed**.

### Current encoding (HEAD, 2026-10-05; Proposed)

The semantic log is the event sequence (D1); the physical encoding is sealed `KUTHASEG` segments plus `manifest.json` for new and migrated directories, or legacy `events.jsonl` + `events.wal` until `store::migrate`; `events.jsonl` after migrate is an unread export. Rocks is an optional unwired locator index (`rocks-index`), never the event store. Tokio is not in the workspace. `store::persist_until` writes jsonl+WAL only when `writes_legacy_layout`; else `segment::persist_log` rewrites `KUTHASEG` + `manifest.json`. Writers take `kutha.lock` (advisory, per directory, local FS). `Persister::append_segmented` is O(batch); full `persist()` is still O(N); `open` is O(N) (`load_log_events` + hydrate). `store::open` trims with `commit::committed_prefix`. `rocks_index::SegmentIndex` has no inbound product callers (CBM). `Persister` is not wired into `kutha-tenant` (`store::persist` only). Residuals: Typed/Dictionary group has no outcome (GROUP-01 (c)); no external chain anchor; `open` O(N); `Persister` not in `kutha-tenant`.

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Composition** · **Verify** · **Agent**
- Depends on: ADR-000 (D1, D2, D9), ADR-001, ADR-002
- Anticipates: ADR-011 (lean events), ADR-012 (snapshots/vacuum), ADR-014 (budgets & receipts), ADR-040 (leases of the fold), ADR-060/061 (replay/fork)

## Context

Kutha’s spine already locks **the append-only event log as source of truth** and **the graph as a deterministic fold** (ADR-000 D1/D2). Honeycomb still lacked a cell that names the *runtime quantum* and the crash cousin without turning vendor stores into SoT.

Grounding cards (SoT for evidence, not for decisions):

- `paper-activegraph-log-is-sot` — `.compound-engineering/artifacts/research/applicability/cards/paper-activegraph-log-is-sot.md`
- `paper-yankin-event-sourced-query` — `.compound-engineering/artifacts/research/applicability/cards/paper-yankin-event-sourced-query.md`
- `paper-tgms-operators` — `.compound-engineering/artifacts/research/applicability/cards/paper-tgms-operators.md`
- `rocksdb-wal-recovery` — `.compound-engineering/artifacts/research/applicability/cards/rocksdb-wal-recovery.md`

ActiveGraph inverts agent stacks: log first, working graph = deterministic projection, behaviors emit events. Yankin names query *mechanisms* over event-sourced state (reconstruction, temporal, cross-stream, retroactive replay) with cost envelopes — not a second SoT. TGMS supplies typed write operators with the LLM **outside** the trust boundary. RocksDB WAL is durability/crash recovery of *storage*, not the semantic log.

We need this cell so later ADRs can cite one quantum instead of restating D1.

## Decision

### D010-1. Log is SoT; fold is a picture

The append-only event log is the only source of temporal truth. The property graph, CSR, HNSW, interval indexes, named-graph slices, and receipt projections are **droppable leases** of a fold at a log offset. The authoritative inputs needed to reconstruct meaning and quantum outcomes must survive dropping these leases (ADR-011/014). Unloading a lease must not change history. LLM compiles or proposes; it never writes truth.

### D010-2. Runtime quantum = emit → cascade → idle

One quantum is: admit an event (or a validated patch) → append to the log → project into subscribed leases → trigger relation/behavior reactions \(B\) / \(R_B\) → cascade until idle (or until a budget bound from ADR-014 fires). No component instructs another except **via** logged events. Direct mutate of the fold is forbidden.

### Clarification (2026-09-13): idle is a scoped completion claim

For deterministic positive rules over a finite fact domain, saturation uses `X[n+1] = X[n] union T(X[n])` with monotone `T` and duplicate suppression. Completion means a fixed point for the declared rule set and input cut, not merely an empty worker queue. Fresh-entity generation requires an explicit bound; negation requires a declared evaluation discipline such as stratification. External tool effects are not pure rules (ADR-062).

Budget exhaustion is incomplete evaluation, not saturation or proof of absence. This is a **Proposed** contract: the P0 inverse-`knows` behavior is not a general rule engine. See ADR-014 for partial-quantum outcomes.

### Open question (2026-09-30)

Open, from docs/architecture/semantic-gap-review.md (F1, F6): D010-1 already requires that authoritative inputs survive dropping leases. Which record kinds (outcomes, justifications, resume) are log events versus retained objects is not decided here. D010-4 (WAL as crash cousin) is unchanged. This question does not rewrite D010-1…D010-4 and does not mark this cell Accepted.

### D010-3. Typed writes, not free MERGE-as-truth

Write surface is a closed operator family in the TGMS direction: **assert / retract / correct** (plus later schema/dict events). Invalidation is a behavior, not silent overwrite (deepened in ADR-013). Query-over-log uses Yankin mechanism groups; AS OF / replay / reconstruction are named cuts with cost envelopes, not marketing synonyms.

### D010-4. WAL is the crash cousin, not the semantic log

RocksDB (or equivalent) WAL + `WALRecoveryMode` is how bytes survive a crash. Semantic replay is fold-from-log. A recovered LSM is a materialization of storage WAL; it is not permission to treat LSM levels as SoT. Historical (superseded by Clarification 2026-10-04 Phase 56 above): Pointer (2026-10-03): shipped persist is `events.jsonl` SoT plus covering `KUTHAWAL` lease (`load_events_d04`); Rocks is an M002 index option (thawed 2026-10-03; scheduled S07, build spike S03) — Clarification (2026-10-03).
Historical pointer (2026-10-04): superseded for the physical encoding by SEG-01 (manifest + segments); the event log is still the SoT.

### Clarification (2026-10-04, Phase 56 / M019 S01; Proposed)

The D010-4 "RocksDB WALRecoveryMode" and 2026-10-03 "events.jsonl is SoT plus covering KUTHAWAL" sentences are **superseded** as current physical-SoT claims. Crash cousin at HEAD is sealed-segment `sync_data` / `KUTHAWAL` on a legacy dir (`writes_legacy_layout`). Physical encoding: SEG-01 (`manifest.json` + `KUTHASEG`) for new and migrated dirs; legacy jsonl+WAL until `store::migrate`. Rocks is an optional unwired locator (`rocks-index` / `rocks_index::SegmentIndex`), never the event store. This subsection does not rewrite D010-1…D010-4 lock titles and does not mark this cell Accepted.

**Hard separations (must fail closed if collapsed):**

```text
Event log              ≠  Fold / working graph
Semantic replay        ≠  Storage WAL recovery
Typed assert/retract   ≠  LLM narrative write
Quantum (emit→idle)    ≠  Workflow-engine DAG
Lease at log offset    ≠  Second database
```

### Clarification (2026-10-01; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F11): every record kind (assert, retract, correct, behavior, definition, dictionary entry, admission, action) enters the log through one path: propose, admit, commit, receipt. A kind-specific early return or a side append without admission is a spike shortcut, not the contract. This subsection does not rewrite D010-1…D010-4 and does not mark this cell Accepted.

### Open question (2026-10-01)

Historical (superseded by Evidence 2026-10-04 Phase 53 and Clarification 2026-10-04 Phase 56): the quantum outcome is currently an `Op` that roughly triples the log (121k events for 40k asserts at the 2026-09-30 bench). F11 is **closed by option A** (`Op::QuantumOutcome` + `commit::committed_prefix`). Options B/C stay as an explicit residual (not shipped). Typed/Dictionary groups still have no outcome. This question does not rewrite D010-1…D010-4 and does not mark this cell Accepted.

### Clarification (2026-10-03; Proposed)

Historical (superseded by the Current encoding (HEAD) block above): Implemented store: `events.jsonl` is SoT; `events.wal` is a covering lease (`store.rs` `load_events_d04`, `wal.rs` magic `KUTHAWAL` version 1). Persist atomicity for `snapshot.json` and `events.wal` closed in M017 S02 (`replace_via_tmp` / `replace_snapshot` / `replace_events_wal`). Rocks `WALRecoveryMode` remains the literature cousin and an M002 index option, not the shipped crash path. Commit-marker vs `Op::QuantumOutcome` encoding is COMMIT-01 (gap-review F11). Sidecar `File::create` without parent-dir fsync is SNAP-01. M002 is thawed and planned as GSD v0.12 (slices S01–S07, `.kutha/ROADMAP.md` § M002): SNAP-01, parent-dir fsync, partial-sidecar-as-absent, `check_tt_anchors` log-ahead tolerance, and fail-closed WAL open are scheduled in S02; COMMIT-01 in S05 (decision gate); Rocks as index lease in S07 (build spike S03). This subsection does not rewrite D010-1…D010-4 and does not mark this cell Accepted.
Pointer (2026-10-04): superseded for the physical encoding by SEG-01 (manifest + segments); the event log is still the SoT.

## Consequences

### Positive

- Replay, fork-at-offset, and lineage have a single spine.
- Materializer ADRs (040+) can mount without competing for SoT.
- Agent control (050) has a place to append *after* validate.

### Negative / risks

- Naive “replay from genesis every query” is too slow — needs ADR-012 snapshots as *placement*, not as truth.
- Unbounded cascades without ADR-014 budgets become storms.

### Non-goals (this ADR)

- Choosing a mandatory crate API or shipping `src/`.
- Defining lean event fields (ADR-011) or vacuum policy (ADR-012).
- Making Graphiti / ActiveGraph / RocksDB the product SoT.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Graphiti fact-edges as SoT | Trap cluster; LLM on write path; clocks collapse (`graphiti-bitemporal-fact-edges`) |
| Samyama / Neo4j / Falkor as SoT | Fast pictures, not the log; violates D1/D5 |
| CRDT merge as truth | Eventual merge drops losers; not audited SoT (`paper-crdt-graph-eventual`) |
| Geo-Raft / blockchain ledger as SoT | Replication or ADS anchor ≠ semantic log (`paper-geo-raft-wan`, `paper-blockchain-graph-ads`) |
| Hard FSM as the quantum | Rejected by D3; quantum is log+behaviors, not a per-vertical statechart |

## Open Research Questions

1. Bound of one quantum: event count vs wall time vs Cui envelope (ADR-014) — falsifiable spike, not a crate name.
2. Yankin cost envelopes for graph-shaped reconstruction vs snapshot-shortened replay (JOB-class later).
3. Operator IR for assert/retract/correct vs Cypher write subset (ADR-070).
4. Content-addressed cache keys for LLM/tool calls inside a quantum (D6) — required for cheap replay; encoding in ADR-011/014.

## Related Decisions

- ADR-000 — D1 log SoT, D2 reactive quantum, D9 tests = replay/fork
- ADR-001 — vision: log is truth; materializations are pictures
- ADR-002 — STCA time axis
- ADR-011 — lean event schema
- ADR-012 — snapshots vs vacuum
- ADR-014 — cascade budgets and quantum receipts
