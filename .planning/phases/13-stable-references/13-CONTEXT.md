# Phase 13 Context: Stable references

**Milestone:** v0.04 / harness M012a S02
**Requirements:** REF-01, REF-02, REF-03
**Depends on:** Phase 12 (log-native SoT) — delivered
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F2)

## Locked decisions for this phase

- **D-01:** Retract and Correct (including CorrectInterval) target a stable EventId (or proposition id), not fold-local `fact_seq`.
- **D-02:** Justification cites use the same stable identity across fork / rebuild.
- **D-03:** A rebuilt fold that renumbers local seqs can still apply the same retract/cite payloads.
- **D-04:** Honeycomb stays **Proposed**. No M012 dictionaries-as-facts. No M002 Rocks.
- **D-05:** Do not regress Phase 12 log-native outcome/justification/resume behavior.

## Spike anchors (read before editing)

- `crates/kutha-common/src/event.rs` — `Op::Retract`, `Op::Correct`, `Op::CorrectInterval`, `Op::JustificationCite`, `EventId`
- `crates/kutha-runtime/src/fold.rs` — `Fact.seq`, apply paths for retract/correct
- `crates/kutha-runtime/src/quantum.rs` — justification cites, `check_admission`
- Phase 12 tests: `crates/kutha-runtime/tests/m012a_log_native.rs`

## Non-goals this phase

- Idempotent ingest / delivery keys (Phase 14 / S03)
- Verify-on-open / atomic persist / stable Define (Phase 15)
- Fold indexes (Phase 16)
- N-ary derivation / rule registry (M012)

## Harness

- Active Milestone M012a, Active Slice **S02**, Phase H5, `L_delivery=M012a-S01-done`
- Do not clear the S02 lease until Phase 13 SUMMARY + verification pass
