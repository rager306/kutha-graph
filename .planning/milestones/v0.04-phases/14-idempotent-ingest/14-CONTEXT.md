# Phase 14 Context: Idempotent ingest

**Milestone:** v0.04 / harness M012a S03
**Requirements:** ING-01, ING-02, ING-03
**Depends on:** Phase 12 (log-native), Phase 13 (stable EventId refs)
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F3)

## Locked decisions for this phase

- **D-01:** Re-delivering an identical Assert with the same delivery key does not mint a second independent support.
- **D-02:** Claim/proposition identity is distinct from a support slot (documented + named tests).
- **D-03:** Conflict reporting does not require caller-only free polarity strings for the leased fixture path, **or** the remaining gap is documented as explicitly out of M012a.
- **D-04:** Honeycomb stays **Proposed**. No M012 dictionaries-as-facts. No M002 Rocks.
- **D-05:** Do not regress Phase 12 LOG or Phase 13 REF oracles.

## Spike anchors

- `crates/kutha-runtime/src/fold.rs` — `Fact.claim_id`, support slots
- `crates/kutha-runtime/src/quantum.rs` — `emit`, `conflict_report_at`
- `crates/kutha-common/src/event.rs` — `Op::Assert`, EventId
- Tests: `m011_claim_supports.rs`, `m012a_stable_refs.rs`, `m012a_log_native.rs`

## Non-goals

- Verify-on-open / atomic persist / stable Define (Phase 15)
- Fold indexes (Phase 16)
- Rule registry / n-ary derivation (M012)

## Harness

- Active Milestone M012a, Active Slice **S03**, Phase H5, `L_delivery=M012a-S02-done`
- Do not clear S03 until Phase 14 verification passes
