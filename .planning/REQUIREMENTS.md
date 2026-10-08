# Requirements: Kutha

**Milestones:** v0.17 / harness M023 shipped 2026-10-08 (Phase 72 claim corrected). Next milestone unleased. Lease: `.kutha/STATE.md`.

## v0.17 Requirements (M023)

### Restore green (Phase 75, S01)

- [x] **GREEN-01**: `delivery_key_retry` short-circuits while any live fact of the keyed Assert's claim carries the key (follows `Correct`/`CorrectInterval`); remints only when none is live. `m013_write_path` keyed-correct pair and `m022_keyed_retry` green; new oracle: keyed retry after Retract of a CorrectInterval id
- [x] **GREEN-02**: naive model in `m019_sem_model` invalidates every live residual on Retract; a seed forces CorrectInterval-then-Retract
- [x] **GREEN-03**: `m019_dur_a` tests do not fail in the workspace run (isolated scratch or serialized); `cargo test --workspace --offline --no-fail-fast` zero failures on repeated runs; bench <=10%

### Close gate (Phase 76, S02)

- [x] **GATE-02**: 72-VERIFICATION, v0.16 audit, CHANGELOG correct the false verified claim; close checklist requires recorded full workspace tests and full `ci`; governor row proves it

### Living overlay (Phase 77, S03)

- [x] **DOC-TRUTH-01**: overlay and stale lines listed in the M023 S03 slice match HEAD; ADRs stay Proposed

### Close (Phase 78, S04)

- [x] **CLOSE-02**: M023 closed; Active None; audit note

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Fold identity decisions (Correct on multi-residual id, unknown id) | Next milestone (post-M022 roadmap M024) |
| Typed clocks, answer envelope, demo CLI | Later (M025) |
| Cypher / HNSW / legal pack / Pi Durable | Thaw map |
