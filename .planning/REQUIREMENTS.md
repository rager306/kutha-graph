# Requirements: Kutha

**Defined:** 2026-10-03
**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**Milestone:** v0.10 Docs honesty and replay integrity (harness lease M017)

## v0.10 Requirements

Source: 2026-10-02 audit (three subagents, CBM + GitNexus at `c702d71`, `rg` arbiter). Cells stay Proposed. Not M002 segments/Rocks. Run: `docs/process/gsd-auto-contract.md`.

### Living docs and governor (Phase 41, docs only — no `crates/`)

- [x] **DOC-02**: `.planning/PROJECT.md` carries no stale lease sentences (no M015 Active, no v0.08 overlay); it points at `.kutha/STATE.md` for lease and freeze
- [x] **GOV-02**: `cite-lease` covers `.planning/PROJECT.md`; a mutated stale `Active Milestone` token there fails the check
- [x] **DOC-03**: `semantic-gap-review.md` has a dated post-M016 addendum (F9/F16 closed; F13 remainder = FIXG-01; F1/F6 snapshot atomicity open and blocks M002); `invariants.yaml` I-F7/I-F9/I-F13/I-F16 no longer say `until M014/M015`
- [x] **DOC-04**: ADR-000/013/014/041/050/060/070 and README carry dated clarifications that match the code: newtype clocks (Op stays `u64`), three distinct "AS OF" cuts, Horn over live facts (not CSR), genesis-seeded relations, Evidence over stale body text; ADR-013 records the C01 decision as **open with options, none chosen**
- [x] **DOC-05**: One freeze table lives in `.kutha/STATE.md`; ROADMAP M016 Success says "live facts" not "CSR lease"; `legal-fixture-recon`, `semantic-contract-validation`, `p0-spike-inventory`, v0.06 audit, `WINDOWS.md`, `RETROSPECTIVE.md` carry a status banner; living M002 requirements below are falsifiable

### Replay integrity (Phase 42, product code)

- [x] **REPL-01**: Tampering a persisted `Op::TtAnchor` (wall or position) makes `replay_check` / `open` fail; named RED→GREEN oracle
- [x] **REPL-02**: `snapshot.json` and `events.wal` are replaced by temp-file + sync + rename; an injected abort before rename leaves the previous file intact; open prefers a WAL only when it is valid, else fails closed or falls back to intact `events.jsonl`
- [x] **REPL-03**: Outcome/justification sidecars never override the log: a sidecar that disagrees with log records fails closed on `open`
- [x] **REPL-04**: `snapshot.json` pins the fold index mode; `open` does not take it from the environment
- [x] **KNOWN-03**: `Runtime::as_known_on` rustdoc says TT-only; oracle `as_known_on ≠ live_at(tt, MAX)` on a closed interval; direct `known_at` equals a brute-force `is_known_at` filter

### Determinism and knowledge-cut characterization (Phase 43, tests first)

- [ ] **DET-01**: Seeded (no new dependency) shuffled-ingest and persist/open tests prove `replay(log).fingerprint == fold.fingerprint` and Eager == Lazy after persist/open
- [ ] **KNOWN-04**: A named characterization test records how Horn-derived supports behave against `as_known_on` (premise visible later than the anchor); result is written into ADR-013 Evidence as current behaviour. No behaviour change without a controller decision

### Ingest regression (Phase 44, product code)

- [ ] **PERF-01**: A same-host bisect names the commit(s) in M016 (Phases 37–40) that raised `ingest_plain.n` from ~108M ns (v0.08) to ~301M ns (post-M016) at N=10000; a scaling oracle (scan counts or ns ratio N vs 2N) is RED on `428f3ff`
- [ ] **PERF-02**: The regression is repaired without changing any named oracle assertion (FF5, FF6, M010–M016, M017); `ingest_plain.n` is within 10 percent of the v0.08 figure on the same host, or the residual is explained and accepted by the controller in the SUMMARY

## Future Requirements (not v0.10)

### M002 — log durability protocol (frozen until leased; v0.11 slot)

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes
- **COMMIT-01**: Quantum outcome as segment-framing commit marker vs `Op::QuantumOutcome` in the log (ADR-010/014 F11) — one encoding decision
- **SNAP-01**: Snapshot and sidecars remain leases; whatever v0.10 makes atomic is carried into segment framing
- **WRITE-01**: `intern` admit path; WAL vs JSONL open preference
- **IO-01** (intake only): kernel flusher/dirty-throttle on cloud block storage — `docs/architecture/m002-cloud-kernel-io-intake.md` (arXiv:2609.12597)
- **R3 exclusion**: do not re-ship "lease identity verified on open" (v0.04) as M002 identity

### Candidate v0.11 slots (need a controller decision)

- **CLOCK-01**: Type `Op` / `Event` / `Snapshot` clocks (`Vt`/`Tt`/`WallTime`); remove `u64::MAX` as "current TT" in facade and eval (F03, F14)
- **C01-01**: Application-end / sunset clause: pack stamp, fifth axis, or stop — ADR-013 D013-1 vs FIXG-01 verdict

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| Segments, hash chain, Rocks, Cypher, HNSW | Freeze; M002+ |
| Choosing a C01 option | Controller/product decision, not a docs-slice outcome |
| Typed `Op` clocks | Serialization change; v0.11 candidate |
| Demoting closed bodies out of `.kutha/ROADMAP.md` | Governor relations risk; separate decision |
| Committing or ignoring `.cursor/skills/gsd-*`, `.gsd/`, CE handoff dirs | Repository hygiene is the user's call |
| Promoting honeycomb to Accepted | L_map stays Proposed |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| DOC-02, DOC-03, DOC-04, DOC-05, GOV-02 | Phase 41 | Complete (41-VERIFICATION) |
| REPL-01, REPL-02, REPL-03, REPL-04, KNOWN-03 | Phase 42 | Complete (42-VERIFICATION) |
| DET-01, KNOWN-04 | Phase 43 | Pending |
| PERF-01, PERF-02 | Phase 44 | Pending |

**Coverage:** v0.10 requirements: 14 total; mapped: 14; unmapped: 0.
