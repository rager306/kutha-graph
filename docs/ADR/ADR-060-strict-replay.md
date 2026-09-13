# ADR-060: Strict Replay

## Status

**Proposed** (Verify — D9 test class; fold-from-log is the proof, not a debugger sidecar)

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

`Runtime::replay_check` currently checks a fold fingerprint. It does not rerun behaviors or compare causal links and receipts. The existing passing check proves neither obligations 2/3 nor lease-independent semantic recovery. Missing code/cache/history yields an explicit unavailable verification result, not a live tool retry or successful replay.

**Hard separations:**

```text
Semantic replay        ≠  Storage WAL replay
Replay harness         ≠  Named branch (061)
CA cache hit           ≠  Re-call the LLM
Snapshot-shortened     ≠  Snapshot as SoT (012)
```

## Consequences

### Positive

- D9 becomes a cell with cards.
- 014 receipts have a consumer.

### Negative / risks

- P0 has fold replay checks; provenance and execution replay remain unimplemented contracts.

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
