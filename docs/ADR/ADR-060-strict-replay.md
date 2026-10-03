# ADR-060: Strict Replay

## Status

**Proposed** (Verify — D9 test class; fold-from-log is the proof, not a debugger sidecar)

### Evidence (2026-10-02; Proposed)

Behavior lineage and provenance oracles remain (`replay_rejects_behavior_without_prior_cause`, provenance fingerprint tests). Full execution replay and log-native quantum outcomes are still partial (sidecars; gap-review F1/F9). **2026-10-03 note:** Horn execution replay shipped in M016; ADS receipts and `Unavailable` did not — see Evidence 2026-10-03.

### Evidence (2026-10-03; Proposed)

Shipped: lineage (`Op::Behavior.caused_by` must name an earlier event) and Horn execution replay (`Runtime::replay_check` → `horn_behavior_reproducible` in `eval.rs`, called from `provenance.rs`). `replay_check` also calls `check_tt_anchors`: snapshot-held `TtAnchor` walls are compared to the log-derived map (oracle `tampered_tt_anchor_wall_fails_replay_check`). `GraphFold::fingerprint` stays facts-only (`fold_query.rs`); `provenance_fingerprint` is a separate hash. Persist of `snapshot.json` / `events.wal` is tmp+`sync_all`+rename (`replace_via_tmp`). Not shipped: D014-1 cryptographic ADS/authenticator receipts (`QuantumReceipt` is SHA-256 of event digests), an `Unavailable` outcome, and sidecar atomicity (gap-review F1: `write_terms` / `write_outcomes` / `write_justifications` still `File::create` in place — SNAP-01). No parent-directory fsync. M002 is thawed and planned as GSD v0.12 (slices S01–S07, `.kutha/ROADMAP.md` § M002): SNAP-01, parent-dir fsync, partial-sidecar-as-absent, `check_tt_anchors` log-ahead tolerance, and fail-closed WAL open are scheduled in S02. This subsection does not rewrite D060-1…D060-3.

### Evidence (2026-10-03, Phase 50 / M002 S02; Proposed)

SNAP-01 closed by Phase 50: sidecar atomic replace + parent-dir fsync; open-path `check_tt_anchors_range` keyed on `snap.next_tt` (`replay_check` exact equality unchanged); fail-closed WAL header and non-`InvalidData` I/O. ADS receipts and `Unavailable` remain unshipped. This subsection does not rewrite D060-1…D060-3. Status stays **Proposed**.

### Evidence (2026-10-03, Phase 52 / M002 S04; Proposed)

Open after incremental append tail-folds a stale snapshot (`replay_check` still exact). Torn jsonl tails without `\n` are dropped; complete malformed jsonl lines still fail `InvalidData`. ADS receipts and `Unavailable` remain unshipped. This subsection does not rewrite D060-1…D060-3. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 53 / M002 S05; Proposed)

COMMIT-01 option A trims the uncommitted suffix before `replay_check`. Pointer: ADR-010/014 2026-10-04. This subsection does not rewrite D060-1…D060-3. Status stays **Proposed**.

### Evidence (2026-10-04, Phase 54-01 / M002 S06; Proposed)

Segmented `open` still runs `committed_prefix` then `replay_check`. Pointer: ADR-010 2026-10-04 Phase 54-01. This subsection does not rewrite D060-1…D060-3. Status stays **Proposed**.

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Verify** (primary) · **Time** · **Agent**
- Depends on: ADR-000 (D9), ADR-010, ADR-014
- Anticipates: ADR-061 (named branches), ADR-012 (snapshots shorten replay)

## Context

D9 already names Strict Replay with `ReplayDivergenceError`. ActiveGraph: cheap fork/replay at any event. Rocks WAL: crash reconstruction of bytes ≠ semantic replay. Yankin: reconstruction cost envelopes. R3: transaction-granularity record/replay/retroaction of *modified code* over a trace — existence proof; do not clone an RDBMS interceptor as Kutha core.

Grounding cards:

- `paper-activegraph-log-is-sot` — `.compound-engineering/artifacts/research/applicability/cards/paper-activegraph-log-is-sot.md`
- `rocksdb-wal-recovery` — `.compound-engineering/artifacts/research/applicability/cards/rocksdb-wal-recovery.md`
- `paper-r3-record-replay-retroaction` — `.compound-engineering/artifacts/research/applicability/cards/paper-r3-record-replay-retroaction.md`
- `paper-yankin-event-sourced-query` — `.compound-engineering/artifacts/research/applicability/cards/paper-yankin-event-sourced-query.md`

## Decision

### D060-1. Semantic replay is fold(log, CA-cache)

State replay folds recorded events; execution replay additionally reruns the same versioned behaviors against recorded content-addressed inputs. The proposed strict contract requires matching state and the declared deterministic receipt/evidence fields. Divergence is a test failure, not a “best effort.” LLM and external effect endpoints are not called during strict replay.

### D060-2. WAL recovery is a cousin, not the harness

Rocks `WALRecoveryMode` proves storage durability. Strict replay proves **Kutha semantics**. Both are required; they are not interchangeable.

### D060-3. Retroaction is optional on top of replay

R3-style “what if this behavior were different” over a recorded cut belongs after 061 refs exist. Cut grain is **quantum / event offset**, not thread interleavings.

### Clarification (2026-09-13): three independent verification obligations

1. **State replay:** apply recorded events and compare the fold, including resolved term meanings when claiming semantic recovery (ADR-011).
2. **Provenance/integrity verification:** validate event references, dependency links, input digests, and quantum outcomes. Mutating a cause while keeping the triple unchanged must not pass this check.
3. **Execution replay:** reproduce derivations with pinned code/rules, policy versions, input cuts, cached observations, and controlled nondeterminism. Declare which IDs/timestamps are recorded inputs and which fields are compared; never assume fresh UUIDs or signatures reproduce byte-for-byte.

`Runtime::replay_check` currently checks a fold fingerprint **and** that every `Op::Behavior.caused_by` names an earlier event in the same log (M011 S02). Historical (pre-M016): it still does not rerun behaviors or compare receipts. As of M016, Horn-pinned Behaviors are re-derived via `horn_behavior_reproducible` (obligation 3 for Horn; see Evidence 2026-10-03). An explicit unavailable outcome for missing code/cache/history remains a **proposed** requirement: today's API returns `Ok(rebuilt)` or `RuntimeError::ReplayDivergence` / `BrokenLineage` / `ExecutionReplayFailed` (and I/O errors from `store::open`).

### Clarification (2026-09-30; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F1): provenance/integrity (obligation 2) validates quantum outcomes as log records, not as an extra source-of-truth file. Sidecar bytes are leases; omitting them from a fingerprint must not hide a SoT gap. State replay (obligation 1) and execution replay (obligation 3) are unchanged. This subsection does not rewrite D060-1…D060-3 and does not mark this cell Accepted.

**Hard separations:**

```text
Semantic replay        ≠  Storage WAL replay
Replay harness         ≠  Named branch (061)
CA cache hit           ≠  Re-call the LLM
Snapshot-shortened     ≠  Snapshot as SoT (012)
```

### Clarification (2026-10-01; Proposed)

Target contract, from docs/architecture/semantic-gap-review.md (F9, F16): execution replay is re-derivation. Registered rules are executed over the premises named in the log; the result is compared with the derived facts the log contains. A derived fact that no registered rule can reproduce is a replay failure. Today's causal-reference and rule-hash checks (obligations 1 and 2) remain necessary and are not sufficient for this. This subsection does not rewrite D060-1…D060-4 and does not mark this cell Accepted.

## Consequences

### Positive

- D9 becomes a cell with cards.
- 014 receipts have a consumer.

### Negative / risks

- P0 has fold fingerprint plus lineage; Horn execution replay (re-running pinned rules in `replay_check` via `horn_behavior_reproducible`) shipped in M016. Cryptographic ADS receipts and an `Unavailable` outcome remain unimplemented.

### Non-goals

- Implementing the suite. Blockchain as replay log.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Testcontainers-only proof | D9 wants replay as first-class |
| R3 interceptor as core | Wrong SoT |
| ChronoGraph system-time as replay | Snapshot cousin of 012 |

## Open Research Questions

1. Canonical `ReplayDivergenceError` payload (offset, receipt mismatch).
2. How much snapshot shortening 012 is allowed before the test is weak.
3. Recording overhead for always-on CA cache.

## Related Decisions

- ADR-010, ADR-012, ADR-014, ADR-061
