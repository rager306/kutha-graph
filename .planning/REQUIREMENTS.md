# Requirements: Kutha

**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestones:** v0.13 / harness M019 shipped 2026-10-05 (archive `.planning/milestones/v0.13-REQUIREMENTS.md`). v0.12 / harness M002 shipped 2026-10-04 (archive `.planning/milestones/v0.12-REQUIREMENTS.md`). v0.11 / harness M018 "Wedge unblock" (Phases 45–48; paused after Phase 45 at the S02 decision gate). Lease: `.kutha/STATE.md`.

## v0.11 Requirements

### Doc truth (Phase 45, S01)

- **DOC-06** (Historical, v0.11 Phase 45): ADR-010/012/014/041/060, `semantic-gap-review.md`, `p0-spike-inventory.md` and `PROJECT.md` describe v0.10 behaviour (snapshot and WAL tmp+rename, jsonl SoT with covering WAL, `horns_by_premise`; sidecars still in place). Living SoT wording is DOC-08.
- **DOC-07**: ADR-000 D6 and ADR-010 D010-4 stop naming Rocks WAL as shipped; ADR-050 records five reserved facts ≠ six dictionaries ≠ harness dictionaries; ADR-013 states `as_of` is support-live, not derivation eligibility
- **REG-01**: one finding register — audit ids carry the `A-F` prefix everywhere; F1–F16 (gap review) keep their numbers
- **LANG-01**: ADR-000 spine and `docs/architecture/stca-guide.md` are English (meaning unchanged)
- **HYG-01**: CHANGELOG head, `.kutha/ROADMAP.md` closed-milestone wording, card count, `.gitignore` for scratch and vendored skill trees; one freeze table (STATE)

### Decisions (Phase 46, S02)

- **C01-01**: ADR-013 records one C01 option — pack stamp, fifth axis, or stop — with a named oracle or an explicit out-of-wedge note
- **HORN-KC-01**: Horn derivation vs `as_known_on` decided and pinned by a named test
- **ASOF-ELIG-01**: whether `as_of` hides a derived fact whose premise support was retracted is decided and pinned

### Horn closure (Phase 47, S03)

- **DET-02**: derived-event identity is deterministic from premise ids, rule pin and binding — or the UUID v7 choice is recorded with its replay consequence
- **HORN-CORR-01**: `Correct` and `CorrectInterval` re-derive; stale derived rows do not stay live
- **CASC-01**: every cascade hop passes `admit`; commit order is stable; unknown head or premise relation in a rule fails closed

### Differential tests (Phase 48, S04)

- [x] **DIFF-01**: fold, CSR lease and clean rebuild agree on seeded random logs (no new dependency) — **moved** to v0.13 Phase 62 / M019 S07 and closed as DIFF-02
- **ORACLE-01**: I-F15 measures time or a bound that fails under the O(N) regression; I-F16 fails when rules do not re-derive
- **GATE-01**: bench gate (`ingest_plain.n`, `as_of.n`) is a governor row or a recorded controller step with a selftest

## Future Requirements (not leased)

### Deferred after M002

- **WRITE-01**: intern admit vs segmented persist (`store::persist_until` / `Persister::append_segmented` vs legacy jsonl+WAL until `store::migrate`; Historical: "WAL vs JSONL preference shipped in v0.10")
- **IO-01** (intake only): kernel flusher/dirty-throttle on cloud block storage — `docs/architecture/m002-cloud-kernel-io-intake.md` (arXiv:2609.12597)
- **VAC-01**: vacuum policy and litigation hold (ADR-012)
- **R3 exclusion**: do not re-ship "lease identity verified on open" (v0.04) as M002 identity

### Candidate slots (need a controller decision)

- **CLOCK-01**: Type `Op` / `Event` / `Snapshot` clocks (`Vt`/`Tt`/`WallTime`); remove `u64::MAX` as "current TT" in facade and eval (A-F03, A-F14)
- **API-01**: close public bypasses of `emit` (`attach_outcomes`, `record_resume`, `pub mod store`, `derive` premise choice)
- **PERF-02**: remaining O(N) paths (`search_at`, `rule_hash_live_at`, `retract_derived`, `graph_len`)
- **SPIKE-LAW-01**: 1–3 real-text law-nexus quotes as `Op::Assert` with `as_of` / `as_known_on` oracles; no corpus copy; only after C01-01

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| Cypher (M003), HNSW (M005) | Own thaw gates; not M002 |
| RocksDB as a second SoT or event store | ADR-000 clarification: index lease only |
| Promoting honeycomb to Accepted | L_map stays Proposed |
