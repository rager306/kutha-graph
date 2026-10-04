# Requirements: Kutha

**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestones:** v0.12 / harness M002 shipped 2026-10-04 (archive `.planning/milestones/v0.12-REQUIREMENTS.md`). v0.11 / harness M018 "Wedge unblock" (Phases 45–48; paused after Phase 45 at the S02 decision gate). v0.13 / harness M019 "Post-M002 coherence" (Phases 56–62; planned, unleased). Lease: `.kutha/STATE.md`.

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

- **DIFF-01**: fold, CSR lease and clean rebuild agree on seeded random logs (no new dependency) — **moved** to v0.13 Phase 62 / M019 S07 (id stays here; work is DIFF-02)
- **ORACLE-01**: I-F15 measures time or a bound that fails under the O(N) regression; I-F16 fails when rules do not re-derive
- **GATE-01**: bench gate (`ingest_plain.n`, `as_of.n`) is a governor row or a recorded controller step with a selftest

## v0.13 Requirements (M019, planned, unleased)

### Docs honesty (Phase 56, S01)

- **DOC-08**: living docs, ADR Evidence and honeycomb comments match post-M002 encodings (roadmap audit §4 + ADR audit §4); closed M001–M017 ROADMAP bodies collapse to one-line + pointer; ADRs stay Proposed; `.kutha/STATE.md:3` is controller-owned at lease time

### Governor (Phases 57–58, S02–S03)

- **GOV-04**: new `cargo_dep_constraint` kind (`tomllib`) owns freeze-rocks / freeze-cypher-deps / freeze-hnsw-dep; governor-audit bypass forms are RED tests first
- **GOV-05**: `m002_*` oracles enrolled in honeycomb `evidence` and `observe_cargo.required` (plus `--features rocks-index` observe row)
- **GOV-06**: `artifact-budget` strict (`min_hits` 1 while a live phase exists; SUMMARY max 1); `id-map-cite` requires v0.12 / v0.13; selftest proves ledger / pointer kinds by their own mutation or an explicit skip with a reason
- **PAR-01**: independent checks and selftest proofs run in a process pool (`KUTHA_GOV_JOBS` / `--jobs`, default min(cpu, 8)); `--jobs 1` is the oracle (same findings, byte-equal fold); selftest copytree skips `.compound-engineering` and `.planning/milestones`

### Test strategy (Phase 59, S04)

- **TEST-01**: honeycomb ADR-023 (Proposed; conventions, with 020–022) + `docs/architecture/test-matrix.md` (invariant × layer × oracle × kill-mutation × status / owner-gate) + a `rust_test_asserts`-style governor row; owner-gated invariants are `blocked: decision`, not xfail

### Durability residuals (Phases 60–61, S05–S06)

- **WRITER-01**: single-writer advisory lock per store dir (`File::try_lock`; two writers fail closed; RED first)
- **GROUP-01**: Typed / Dictionary multi-event group vs `committed_prefix` decided in ADR-010 / 014; RED-first oracle; missing durability oracles from the code-tests audit §2 (mixed suffix, migrate × tail guard, active at exact frame end, wrong-role active)
- **ROCK-02**: Rocks index staleness beyond the tip is detected (index meta bound to manifest chain tip + count, or equivalent; RED first)
- **MEM-01**: `bytes_per_fact` 17218→25200 cause confirmed by measurement (dhat / heaptrack / counting allocator) and fixed or explained
- **CLEAN-01**: dead-code fate (`log_has_quantum_outcome`, `wal::append_events` `allow(dead_code)`, `rocks_spike`); `SegmentIndex` / `Persister` wiring decision recorded (default: not wired, reason). Bench gate applies on product slices.

### Semantic tests (Phase 62, S07)

- **DIFF-02**: seeded randomized differential fold vs CSR lease vs clean rebuild (the work pulled from DIFF-01; no new dependency; LCG like `m017_determinism.rs`)
- **SEM-02**: cascade order stable Eager vs Lazy; seeded determinism on derived triples; Correct of a Horn premise characterized; legal TR-02 gap recorded as `blocked: decision` in the matrix. No owner-gated behaviour changes.

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
