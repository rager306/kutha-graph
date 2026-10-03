# Requirements: Kutha

**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestones:** v0.11 / harness M018 "Wedge unblock" (Phases 45–48; paused after Phase 45 at the S02 decision gate) and v0.12 / harness M002 "Log durability protocol" (Phases 49–55; thawed 2026-10-03). Lease: `.kutha/STATE.md`.

## v0.11 Requirements

### Doc truth (Phase 45, S01)

- **DOC-06**: ADR-010/012/014/041/060, `semantic-gap-review.md`, `p0-spike-inventory.md` and `PROJECT.md` describe v0.10 behaviour (snapshot and WAL tmp+rename, jsonl SoT with covering WAL, `horns_by_premise`; sidecars still in place)
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

- **DIFF-01**: fold, CSR lease and clean rebuild agree on seeded random logs (no new dependency)
- **ORACLE-01**: I-F15 measures time or a bound that fails under the O(N) regression; I-F16 fails when rules do not re-derive
- **GATE-01**: bench gate (`ingest_plain.n`, `as_of.n`) is a governor row or a recorded controller step with a selftest

## v0.12 Requirements (M002, thawed; leased one slice at a time)

### Thaw map (Phase 49, S01)

- **THAW-01**: `.kutha/STATE.md` carries a per-item thaw map; RocksDB crate thaws in S03; Cypher (M003), HNSW (M005), six dictionaries (SIX-01), ADR-080/081, ADR-090, ADR-093 and Consensus 103+ keep their own gates
- **GOV-03**: the `freeze` governor check is split per dependency, each message names its real thaw gate, the tautological message check is gone, the workspace root `Cargo.toml` is scanned, and selftest plus pytest prove it

### SNAP-01 and fail-closed open (Phase 50, S02)

- **SNAP-01**: sidecars (`terms`, `quantum_outcomes`, `justifications`) replace via temp + sync + rename; a parent-directory fsync follows every rename; an unreadable or partial sidecar is treated as absent while an intact log opens; `check_tt_anchors` tolerates a log ahead of the snapshot
- **OPEN-01**: a failed WAL magic read or an empty WAL without jsonl never yields an empty log; a WAL error is not hidden behind jsonl
- **CRASH-01**: a failpoint after each persist step reopens to the last durable prefix or fails closed

### Rocks spike (Phase 51, S03)

- **ROCK-00**: `rocksdb` builds offline on this host in an isolated module; `EventId` to offset round-trips; not wired into `Runtime`; the governor permits the dependency only in the leased slice

### Log protocol (Phases 52–54, S04–S06)

- **APPEND-01**: `persist` appends framed records; the prefix bytes are unchanged; one `fdatasync` per quantum; torn tail dropped
- **COMMIT-01**: one durable encoding of the quantum outcome (commit frame or `Op::QuantumOutcome`), recorded in ADR-010/014 (decision gate)
- **SEG-01**: sealed segments, atomically replaced manifest, hash chain; recovery from manifest plus active segment; tampered sealed segment fails closed

### Rocks index lease (Phase 55, S07)

- **ROCK-01**: `EventId` to offset and `delivery_key` indexes in RocksDB are a lease rebuilt from the log; deleting the Rocks directory changes no answer; Rocks never stores an event payload, commit frame, disposition or term meaning
- **PERF-GATE**: bench re-run (`ingest_plain.n`, `as_of.n`, same host, vs previous tag) at every product slice that edits `crates/`

## Future Requirements (not leased)

### Deferred inside M002 horizon

- **WRITE-01**: `intern` admit path (WAL vs JSONL preference shipped in v0.10)
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
