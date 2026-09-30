# Phase 15 Context: Verify, persist, and time scale

**Milestone:** v0.04 / harness M012a S04 (+ S05 in this wave)
**Requirements:** DUR-01, DUR-02, DUR-03, TIME-01, TIME-02
**Depends on:** Phase 12 (log-native SoT); numeric order after Phase 14
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F6 subset, F7)

## Locked decisions

- **D-01:** `open` verifies snapshot identity against the log (`replay_check` or equivalent) and rejects a tampered snapshot (DUR-01).
- **D-02:** `persist` writes the log file atomically (rename-into-place or equivalent); crash mid-persist is not success (DUR-02).
- **D-03:** `Define` event ids are stable across persist/open for the same term set (DUR-03).
- **D-04:** VT and TT declare an explicit scale used by fixtures (TIME-01).
- **D-05:** Named tests document how fixture years/cuts map to that scale (TIME-02); full TT↔wall calendar map is optional.
- **D-06:** Do not regress LOG/REF/ING oracles. Honeycomb stays Proposed. No M012 dicts-as-facts. No M002 Rocks durability protocol (segmented WAL/hash chain stays M002).

## Spike anchors

- `crates/kutha-runtime/src/store.rs` — `persist`, `open`, snapshot path
- `crates/kutha-runtime/src/quantum.rs` — `replay_check`
- `crates/kutha-common/src/event.rs` — `ValidTime`, `TransactionTime`, `Op::Define`
- Bench note in semantic-gap-review F6 (tampered snapshot accepted today)

## Non-goals

- Fold-internal hot indexes (Phase 16 / S06)
- Rocks / segmented log durability protocol (M002)
- M012 dictionaries-as-facts

## Harness

- Active Milestone M012a, Active Slice **S04**, Phase H5, `L_delivery=M012a-S03-done`
- On Phase 15 close: mark ROADMAP S04 and S05 done; advance `L_delivery` to `M012a-S05-done` (or `M012a-S04-done` if S05 is folded into the same token — prefer `M012a-S05-done` when both checkboxes close)
- Do not clear S04 until Phase 15 verification passes
