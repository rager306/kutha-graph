# Phase 12 Context: Log-native SoT

**Milestone:** v0.04 / harness M012a S01
**Requirements:** LOG-01, LOG-02, LOG-03
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F1)

## Locked decisions for this phase

- Event log remains SoT (D1). Outcome / justification / resume durability must survive discarding sidecar files.
- Sidecar files (`quantum_outcomes.jsonl`, `justifications.jsonl`, etc.) become leases only after log-native records exist.
- `provenance_fingerprint` (or successor) must mix those log-native bytes, not only `Op::Behavior` rows.
- Honeycomb cells stay **Proposed**. No M012 dictionaries-as-facts. No M002 Rocks.

## Spike anchors (read before editing)

- `crates/kutha-runtime/src/store.rs` — `OUTCOMES_REL` / `JUSTIFICATIONS_REL`, persist order
- `crates/kutha-runtime/src/quantum.rs` — `Justification`, `attach_justifications`, `record_justification`, `provenance_fingerprint`
- Existing M011 e2e / quantum outcome tests under `crates/kutha-runtime/tests/`

## Non-goals this phase

- Stable EventId retract targets (Phase 13 / S02)
- Idempotent ingest (Phase 14 / S03)
- Verify-on-open / atomic persist / stable Define (Phase 15)
- Fold indexes (Phase 16)
- Full ADR-014 Accepted promotion

## Harness

- Active Milestone M012a, Active Slice **S01**, Phase H5, `L_delivery=M012a-leased`
- Do not clear the S01 lease until Phase 12 SUMMARY + verification pass
