# ADR-061: Fork-and-Diff

## Status

**Proposed** (Verify — named refs on the log; counterfactuals without CRDT merge)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Verify** (primary) · **Time** · **Space** · **Agent**
- Depends on: ADR-000 (D9), ADR-010, ADR-060
- Anticipates: ADR-091 (export a fork as capsule)

## Context

D9 Fork & diff is the counterfactual test class. ActiveGraph fork-at-offset is **replay from a cut**. This cell adds **named refs** so two agents can diverge without sharing tentative writes. Graph edit distance is the **diff algebra** on two folds, not MATCH and not ULTRA scores.

Grounding cards:

- `paper-graph-branch-fork` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-branch-fork.md`
- `paper-graph-edit-distance` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-edit-distance.md`

ForkBase/GitLake: Git-for-data / agent branches with atomic publish. CRIU/container fork is the wrong primitive. CRDT merge drops losers. Hindsight git-as-memory is a trap cousin. ChronoGraph is system-time versioning (012 cousin).

## Decision

### D061-1. A branch is a named overlay on the log

`ref → event-id` (plus optional overlay events). Queries compile with a branch id. P0 may be `main` only. Merge/publish is a **logged** commit/compensating event, not CRDT.

### D061-2. Fork-at-offset remains

Replay from any offset (060) stays. Named branches compose *on top*: two refs may share a prefix offset and hide tentative writes.

### D061-3. Diff is GED-class (or cheaper restriction), not cosine

Compare two folds/slices with an explicit distance/edit script. LLM narrative is not the diff. Exact GED is hard; honeycomb may restrict to labeled tree/DAG cuts or property-graph edit scripts as a lease.

### Clarification (2026-09-13): branch identity and counterfactual limits

Portable references must bind stable event/claim identity and branch ancestry, not a bare local `fact_seq`. Two branches can assign the same next sequence to different assertions; cross-branch correction must resolve the intended version or reject it. Publish requires explicit base/cut and conflict checks, not implicit last-writer-wins. Delivery keys (ADR-011) prevent retry from becoming another independent assertion. Exact merge encoding remains Proposed.

A fork changes recorded state, not the external world. Reusing an old tool response after changing its inputs is scenario playback, not evidence of a real counterfactual outcome; obtain or model new dependent observations and label that distinction. Fork, replay, and compensation must never silently reexecute external effects (ADR-062). P0 `fork_at` is a prefix experiment, not a merge or effect-isolation implementation.

**Hard separations:**

```text
Named branch           ≠  CRDT merge
Named branch           ≠  Snapshot placement (012)
GED diff               ≠  MATCH / ULTRA score
Agent experiment       ≠  Publish to main
Container/CRIU fork    ≠  Log ref
```

## Consequences

### Positive

- Pack authors and agents can experiment fail-closed.
- 090 can fork a practice overlay without mutating L_KB.

### Negative / risks

- Native microsecond clone is still research; COW leases (091) may be the packaging.

### Non-goals

- Vendoring Git/Iceberg. Blockchain merge.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| CRDT as merge | Losers vanish; not audit |
| Hindsight git memory | Trap |
| Only offset replay | Cannot isolate two writers’ tentatives |

## Open Research Questions

1. Overlay encoding vs copy-on-write leases.
2. Tractable GED subset for P1 tests.
3. Merge isolation vs TOKI (013).

## Related Decisions

- ADR-060, ADR-012, ADR-013, ADR-091
