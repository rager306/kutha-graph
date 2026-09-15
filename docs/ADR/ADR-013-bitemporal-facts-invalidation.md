# ADR-013: Bi-temporal Facts & Invalidation

## Status

**Proposed** (Time/Query fact semantics — Graphiti *semantics* native, without Graphiti runtime)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Query** · **Verify** · **Agent**
- Depends on: ADR-000 (D4), ADR-010, ADR-011
- Anticipates: ADR-043 (interval index as an access path), ADR-050 (dicts use VT×TT), ADR-070 (AS OF surface)

## Context

ADR-000 D4 already requires valid-time × transaction-time on facts, with invalidation as behavior. Graphiti popularized *bitemporal fact edges* as a product; the matrix judged Graphiti **low optimality** as Kutha SoT. This cell takes the **semantics** and rejects the runtime.

Grounding cards:

- `paper-toki-contradiction-ops` — `.compound-engineering/artifacts/research/applicability/cards/paper-toki-contradiction-ops.md`
- `paper-memstrata-stale-fact` — `.compound-engineering/artifacts/research/applicability/cards/paper-memstrata-stale-fact.md`
- `paper-tgql-intervals` — `.compound-engineering/artifacts/research/applicability/cards/paper-tgql-intervals.md`
- `paper-temporal-interval-index` — `.compound-engineering/artifacts/research/applicability/cards/paper-temporal-interval-index.md`

TOKI: isolation-typed contradiction operators; losers stay in an audit row; LLM-as-judge on the write path is itself logged or it produces write-time anomalies. MemStrata: cosine cannot see contradiction (AUROC ~0.59); supersession is a deterministic (s,r,o) rule on a bi-temporal ledger. T-GQL: validity-interval property graphs and temporal *path* semantics — Neo4j is a host, not SoT. Interval indexes (Timeline Index, TIDE, MAP21, …) are the **access path** for valid-time; the log is ordered by transaction time.

## Decision

### D013-1. Native VT × TT on facts

Every fact/edge carries `valid_from` / `valid_to` (world) and `ingested_at` / `invalidated_at` (system), plus provenance. Core algebra is **two axes**. Legal five-clock stamps (ADR-090) map as **typed annotations**, not five peer algebraic axes.

### D013-2. Invalidation is typed; losers remain

Assert / retract / correct (ADR-010) plus TOKI-class contradiction operators. Isolation is declared at write time. Losers remain queryable as audit/belief-as-of until vacuum policy (ADR-012) applies. Silent overwrite is forbidden.

### D013-3. Supersession is deterministic, not cosine

An explicit correction or declared, versioned supersession policy retires the targeted claim/version/support. Equal endpoints or overlapping valid-time alone do not establish supersession: independent conflicting evidence must remain distinguishable. Embedding similarity and RAG cannot own stale-fact. LLM is not on the write path except as a logged proposal that still must validate.

### D013-4. Interval index is a lease; T-GQL is a language cousin

Valid-time overlap/contain/AS-OF uses an interval access path keyed by interned ids (ADR-011). The index is droppable. Temporal path algorithms (T-GQL family) sit on “which edges are live in `[t0,t1)`” — they are not the log. Query surface language is ADR-070.

**Hard separations:**

```text
Valid-time             ≠  Transaction-time
Invalidation           ≠  Delete
TOKI write contract    ≠  Chat heuristic / LWW-as-default
Cosine / RAG           ≠  Stale-fact detector
T-GQL / Neo4j host     ≠  Kutha SoT
Interval index         ≠  Snapshot placement (ADR-012)
Graphiti edges         ≠  Native fact semantics
```

## Consequences

### Positive

- Legal statutory QA can fail closed on superseded law.
- Dictionary entries and meta-prompt versions (050) reuse the same stamps.
- Planner (043) has a named valid-time access path.

### Negative / risks

- (s,r,o) keying vs free-text nodes is still a spike.
- TOKI isolation formalization before P0 may over-specify.

### Non-goals (this ADR)

- Implementing Graphiti, T-GQL on Neo4j, or a third clock.
- Cypher grammar (ADR-070).
- Vacuum of losers (ADR-012).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Graphiti as runtime | Trap; LLM write path; not Kutha SoT |
| Latest-state RAG | Cannot reconstruct prior belief; MemStrata fail |
| CRDT merge | Drops losers; not audited |
| Hindsight four-network memory | Agent memory product, not fact algebra |
| Valid_from fields as SoT | Prevents correction/provenance |

## Open Research Questions

1. Minimal TOKI operator subset for P1 (LWW vs await-confirm vs policy).
2. Fact key: interned (s,r,o) vs attributed hyperedge.
3. Interval index choice (Timeline vs B+ duration×endpoint) as a lease pack.
4. How legal five-clock stamps attach without becoming core axes.

## Related Decisions

- ADR-000 — D4
- ADR-010 — typed writes
- ADR-011 — interned ids
- ADR-012 — vacuum of history
- ADR-043 — planner may pick interval index
- ADR-090 — L_KB clocks as stamps

## Implementation note (2026-08-18)

P0 now has named `GraphFold::live_at` / `as_of` (no silent “now”) and FF5 on a statute-shaped fixture (`crates/kutha-runtime/tests/ff5_legal_pit.rs`). Status stays **Proposed** until a harness promotion packet lists this cell; governor still forbids bulk honeycomb Accepted.

## Semantic clarification (2026-09-13; Proposed)

- `live_at(tt, vt)` asks what was known at `tt` about `vt`. `as_of(vt)` uses current transaction knowledge (`tt = MAX`); it is not historical belief replay. FF5 currently compares two VT values, not the same VT before and after a late correction.
- Intervals are half-open. Current `Correct` replaces the entire targeted version at its new TT; it is **not** an interval-patch operator. A future partial correction of `[2015,2020)` within `[2010,infinity)` must explicitly preserve `[2010,2015)` and `[2020,infinity)` as residual versions, or reject the partial operation. Do not silently reinterpret today's API. Old TT queries must still see the old version.
- Withdrawing one source removes its support, not independent supports for the same claim. Derived eligibility must be reevaluated from remaining supports; withdrawal is not itself evidence of the opposite proposition. The P0 inverse behavior does not yet implement this maintenance.
- For unresolved conflict, a candidate policy-facing view is `(positive_supports, negative_supports)`: neither, positive only, negative only, or both. This is a proposed evidence summary, not an installed four-valued logic. Query completeness (`complete`, budget-limited, unavailable) is a separate axis; missing or truncated evidence cannot become a complete negative answer. Resolution/admission remains a versioned policy decision (ADR-051/093).
