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
