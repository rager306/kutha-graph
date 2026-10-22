# Kutha execution roadmap

Non-authoritative delivery pyramid. Architecture SoT remains `docs/ADR/`. Honeycomb is a **map**, not this list.

**Vision:** Event-sourced bi-temporal graph engine; LLM proposes; log and dictionaries own audited truth. Wedge: legal temporal agents.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP. Controller protocol lives in the same file.

## M001: Legal PIT steel thread — CLOSED

**Closed:** S01–S03 delivered. Historical lease was `L_delivery=M001-closed`. Archive: `.planning/milestones/v0.01-*`. Not product Accepted.

- [x] **S01: Named AS OF cut + fixture**
- [x] **S02: CSR agrees with the cut**
- [x] **S03: Relation allowlist fail-closed**

## M010: Semantic recovery without snapshot lease — CLOSED

**Closed:** S01–S03 delivered. Historical lease was `L_delivery=M010-closed`. Archive: `.planning/milestones/`. Not product Accepted.

- [x] **S01: Open recovers intern meanings**
- [x] **S02: Logged term-definition ops**
- [x] **S03: Live intern appends Define**

## M011: Claim / support identity — CLOSED

**Closed:** S01–S08 delivered. Archive: `.planning/milestones/v0.02-*`. Not product Accepted.

- [x] **S01: Portable claim_id on facts**
- [x] **S02: Unknown claim fail-closed + replay lineage**
- [x] **S03: Thin P→Q derivation eligibility**
- [x] **S04: Partial correction with residual intervals**
- [x] **S05: Persisted quantum outcome and continuation**
- [x] **S06: Typed CSR lease keeps labels and support multiplicity**
- [x] **S07: Provenance and rule-version check apart from state replay**
- [x] **S08: End-to-end candidate fixture**

## M012a: Single-log SoT and stable references — CLOSED

**Closed:** S01–S06 delivered. Archive: `.planning/milestones/v0.04-*`. Closing F1–F3, F6–F8. Not product Accepted.

- [x] **S01: Log-native outcomes and justifications**
- [x] **S02: Stable references for retract and cites**
- [x] **S03: Idempotent ingest and proposition polarity**
- [x] **S04: Verify-on-open, atomic persist, stable Define**
- [x] **S05: Declared time scale**
- [x] **S06: Fold-internal indexes for hot reads**

## Harness dogfood (process plane, not M00x)

- [x] **H0** — dictionary FSM + file trajectory.
- [x] **H1** — `kutha-gov ci` observes named FF tests via `cargo test` (evidence, not SoT).
- [x] **H2** — harness JSONL ingested onto the Kutha log; query AS OF last process status.
- [x] **H3** — fail-closed process writes through a relation allowlist.
- [x] **H4** — process-relation membership editions logged as snapshots and queryable AS OF a prior tenant cut (tip YAML stays the admit lease; full ADR-090 ontology stays frozen).
- [x] **H5** — semantic governor + lean agent context (harness/docs only)

## M012: Dictionaries as facts — CLOSED

**Closed:** S01–S05 delivered. Archive: `.planning/milestones/v0.05-*`. Closing F4/F5. Historical (superseded 2026-10-04): "Freeze still blocks … M002". Not product Accepted.

- [x] **S01: Relation allowlist as log facts**
- [x] **S02: Rule registry (`rule_version` = definition hash)**
- [x] **S03: Admission status and policy version as meta-facts**
- [x] **S04: Thin action record**
- [x] **S05: Multi-hop / n-ary derivation eligibility**

## M013: Honest baseline and one write path — CLOSED (GSD v0.06)

**Closed:** S01–S09 delivered 2026-10-02. Process tag `v0.06`. Archive: `.planning/milestones/v0.06-*`. Historical (superseded 2026-10-04): "Still frozen: … M002". Not product Accepted.

- [x] **S01: Benchmark baseline and size target**
- [x] **S02: Legal fixture reconnaissance**
- [x] **S03: Fold indexes for ingest**
- [x] **S04: Genesis log**
- [x] **S05: Dictionary entries as facts**
- [x] **S06: One write path and facade**
- [x] **S07: Independent verification and artifact budget**
- [x] **S08: Close-out honesty — documents and governor**
- [x] **S09: Write-path integrity hardening**

## M014: Typed clocks and as-known-on — CLOSED (GSD v0.07)

**Closed:** S01–S03 delivered 2026-10-02. Process tag `v0.07`. Archive: `.planning/milestones/v0.07-*`.

- [x] **S01: Clock newtypes**
- [x] **S02: `tt_anchor` facts and `as_known_on`**
- [x] **S03: Fixture migration**

## M015: Known-on honesty and M014 residuals — CLOSED (GSD v0.08)

**Closed:** S01–S02 delivered 2026-10-02. Process tag `v0.08`. Archive: `.planning/milestones/v0.08-*`.

- [x] **S01: Known-on cut and anchor admission**
- [x] **S02: Living docs and governor honesty**

## M016: Test hardening + executable rules — CLOSED (GSD v0.09)

**Closed:** S01–S05 delivered. Process tag `v0.09`. Archive: `.planning/milestones/v0.09-*`. Honeycomb stays Proposed.

- [x] **S01: Test hardening T2–T6**
- [x] **S02: Rule AST and hash**
- [x] **S03: Evaluator and support counting**
- [x] **S04: Policy evaluation and execution replay**
- [x] **S05: Legal golden fixture and verdict**

## M017: Docs honesty and replay integrity — CLOSED (GSD v0.10)

**Closed:** S01–S04 delivered 2026-10-03. Process tag `v0.10`. Archive: `.planning/milestones/v0.10-*`. Residuals in `.planning/milestones/v0.10-MILESTONE-AUDIT.md`.

- [x] **S01: Living docs and governor honesty**
- [x] **S02: Replay integrity**
- [x] **S03: Determinism and knowledge-cut characterization**
- [x] **S04: Ingest regression repair**

## M018: Wedge unblock — CLOSED (GSD v0.11)

**Closed:** 2026-10-06. GSD Phases 45–48, S01–S04. Independent VERIFICATIONs including `48-VERIFICATION.md`. Crate SemVer stays 0.0.0. Honeycomb stays Proposed. Not a late `v0.11` process tag after `v0.13`.

**Source:** 2026-10-03 post-v0.10 review at `d230ab2` (five read-only audits, CBM + GitNexus, `rg` arbiter): trajectory, ADR vs code, tests, docs, code logic. The wedge (norm AS OF a date) is blocked by an undecided C01 and by Horn semantics, not by Rocks. Not M002 segments or Rocks; not typed `Op` clocks; not ADR Accepted.

**Success:** living docs and ADR Evidence agree with the code (one F-register, English spine); ADR-013 records one C01 option and the Horn-vs-knowledge-cut rule, with a named oracle; derived facts have deterministic identity and follow `Correct`; every cascade hop is admitted. Fold / CSR / rebuild differential (DIFF-01) moved to M019 S07.

**S02–S04 shipped 2026-10-06.** M002 CLOSED (2026-10-04). M019 CLOSED (2026-10-05). DIFF-01 moved to M019 S07.

- [x] **S01: Doc truth round 2** `risk:low` `depends:[]`
  > After this: ADR-010/012/014/041/060 Evidence and `semantic-gap-review.md` match v0.10 code (snapshot and WAL atomic, `horns_by_premise`, sidecars still in place); audit ids carry the `A-F` prefix; ADR-000 D6 and ADR-010 D010-4 no longer name Rocks WAL as shipped; ADR-013 states `as_of` is support-live, not eligibility; ADR-000 and `stca-guide.md` spine is English; CHANGELOG head, `PROJECT.md`, `.kutha/ROADMAP.md` closed-milestone wording and `.gitignore` hygiene are fixed; no file under `crates/` changes.
- [x] **S02: C01 and knowledge-cut decision** `risk:high` `depends:[S01]` `gate:controller+user decision`
  > After this: ADR-013 records one C01 option (pack stamp, fifth axis, or stop) with a named oracle or an explicit out-of-wedge note; the Horn-vs-`as_known_on` rule (HORN-KC-01) and the `as_of` vs derivation-eligibility rule are decided; if behaviour changes, the oracle is RED first and `m017_known_on_horn.rs` is inverted or re-pinned by the decision.
- [x] **S03: Horn determinism and closure** `risk:high` `depends:[S02]`
  > After this: two ingests of the same ops give the same derived-event identity (or the decision to keep UUID v7 is recorded with its replay consequence); `Correct` and `CorrectInterval` re-derive; every cascade hop passes `admit` and commits in a stable order; the unknown head or premise relation in a rule fails closed; named oracles unchanged.
- [x] **S04: Differential tests and oracle tightening** `risk:medium` `depends:[S03]`
  > After this: I-F15 and I-F16 oracles measure what they claim; the bench gate is a recorded controller step or a governor row. DIFF-01 (fold, CSR and clean rebuild on seeded random logs) moved to M019 S07. OPEN-01 (failed WAL magic read, empty WAL, WAL error hidden behind jsonl) and the crash seam between jsonl and sidecars remain on M002 S02.

## M002: Log durability protocol — CLOSED (GSD v0.12)

**Closed:** 2026-10-04. Process tag `v0.12`. S01–S07 complete; seven independent VERIFICATIONs. Crate SemVer stays 0.0.0. Residuals in `.planning/milestones/v0.12-MILESTONE-AUDIT.md`.

**Thawed 2026-10-03** then shipped. The freeze came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it; the owner asked for M002 to be re-verified and run under GSD. Provenance and the per-item thaw map: `.kutha/STATE.md` § Thaw map. ADR-000 D1–D10 are not a lease gate: D6 names a Rust+RocksDB event store, D8.1 asks for log segmentation. S06 moved the physical encoding to a manifest plus sealed segments; the event log stays SoT and `events.jsonl` is an on-demand export (owner acknowledgement still needed for that wording).

**Source:** 2026-10-03 re-verification at `f4af544` (provenance audit, governor freeze map, durability design research; CBM + GitNexus, `rg` arbiter). Not M003 Cypher; not M005 HNSW; not ADR Accepted.

**Success:** SNAP-01 sidecar tmp+sync+rename and parent-dir fsync; fail-closed WAL open; framed append (no O(N) rewrite of the prefix); commit marker option A (`Op::QuantumOutcome`); sealed segments with a cross-segment SHA-256 chain and atomic manifest; droppable Rocks index lease (`EventId` and `delivery_key` locators, not wired into `Runtime`). Residuals: Typed/Dictionary partial group, no multi-writer lock, tip-only index staleness, line-based `freeze-rocks` bypass forms, `rocks_spike` still on jsonl, O(N) `open`, RSS `bytes_per_fact` 17218→25200 B at Phase 50. The event log stays SoT (D1).

- [x] **S01: Thaw map and freeze governor honesty** `risk:low` `depends:[]`
  > After this: the blanket "freeze until M002 lease" is replaced by a per-item thaw map (RocksDB crate: S03; Cypher: M003; HNSW: M005; six dictionaries: SIX-01; ADR-080/081, ADR-090, ADR-093, Consensus 103+: their own named gate); the `freeze` governor check no longer claims to read STATE (split per dependency, each message names its real thaw gate) and its selftest and pytest prove it; no file under `crates/` changes.
- [x] **S02: SNAP-01 and fail-closed WAL open** `risk:medium` `depends:[S01]`
  > After this: sidecars replace via tmp+sync+rename and every rename is followed by a parent-directory fsync; an unreadable or partial sidecar is treated as absent while an intact log still opens; `check_tt_anchors` tolerates a log ahead of the snapshot; a failed WAL magic read or an empty WAL without jsonl never yields an empty log; a WAL error is not hidden behind jsonl; a crash-matrix test (failpoint after each persist step) reopens to the last durable prefix or fails closed.
- [x] **S03: RocksDB build spike** `risk:low` `depends:[S01]`
  > After this: a thin isolated module compiles `rocksdb` offline on this host and round-trips `EventId` to file offset; it is not wired into `Runtime`; the governor allows the dependency only in the leased slice; build prerequisites are recorded; if it cannot build, the failure and the fallback (pure-Rust index) are written down.
- [x] **S04: Incremental framed append** `risk:high` `depends:[S02]`
  > After this: a second `persist` appends length+CRC framed records and does not rewrite the prefix bytes; one `fdatasync` per quantum; a crash mid-append drops only the incomplete tail; replay parity with the jsonl path; bench gate (`ingest_plain.n`, `as_of.n`) within 10 percent.
- [x] **S05: Commit marker decision (COMMIT-01)** `risk:high` `depends:[S04]` `gate:controller+user decision`
  > After this: one durable encoding of the quantum outcome is chosen (commit frame in the segment, or `Op::QuantumOutcome` kept) and recorded in ADR-010/014; a crash between payload events and the commit leaves no uncommitted prefix visible; named oracles unchanged.
- [x] **S06: Segments, manifest and hash chain** `risk:high` `depends:[S04,S05]`
  > After this: sealed segments with an atomically replaced manifest; recovery from manifest plus the active segment; a flipped byte or swapped records in a sealed segment fails open with `InvalidData`; the jsonl export is no longer the SoT.
- [x] **S07: Droppable RocksDB index lease** `risk:medium` `depends:[S03,S06]`
  > After this: `EventId` to offset (and `delivery_key`) indexes live in RocksDB as a lease rebuilt from the log; deleting the Rocks directory changes no answer; Rocks never stores an event payload, a commit frame, a disposition or a term meaning.

IO-01 (cloud kernel writeback soak) stays an intake note; vacuum and litigation hold stay outside M002.

## M019: Post-M002 coherence: docs, governor, test strategy, durability residuals — CLOSED (GSD v0.13)

**Closed:** 2026-10-05. Process tag `v0.13` (GSD process tag, not a product release). S01–S07 complete; seven independent VERIFICATIONs (Phase 62 passed-with-findings). Crate SemVer stays 0.0.0. Residuals in `.planning/milestones/v0.13-MILESTONE-AUDIT.md`. v0.11 (M018 S02–S04, phases 46–48) stays paused at the S02 decision gate. One lease at a time. ADRs stay Proposed. Not M003 Cypher; not M005 HNSW; not a C01 / Horn verdict; not ADR Accepted.

**Source:** 2026-10-04 post-M002 audits at `36fedb9` / tag `v0.12` (CBM `root-kutha-graph`, GitNexus `kutha-graph`, `rg` arbiter): `.compound-engineering/artifacts/research/2026-10-04-post-m002-audit-roadmap.md`, `2026-10-04-post-m002-audit-adr.md`, `2026-10-04-post-m002-audit-code-tests.md`, `2026-10-04-post-m002-audit-governor.md`. Owner request: docs correction first, then governor hardening plus SMP, test-strategy cell and matrix, durability residuals, semantic test expansion — all through GSD auto.

**Success:** living docs, ADR Evidence and honeycomb comments match post-M002 encodings (ADRs remain Proposed); freeze-dep checks parse Cargo TOML; M002 oracles are enrolled as evidence and observed; independent governor checks can run in parallel with `--jobs 1` as the byte-equal oracle; a Proposed conventions cell plus `docs/architecture/test-matrix.md` names every oracle; single-writer lock and mixed Typed/Dictionary group have RED-then-GREEN oracles; Rocks index staleness beyond the tip is detected; `bytes_per_fact` 17218→25200 is measured and fixed or explained; dead-code fate is recorded; DIFF-01 plus semantic characterizations land without owner-gated behaviour changes. Not whole-file persist rewrite; not O(N) `open`; not IO-01 / VAC-01.

- [x] **S01: Docs honesty round 3** `risk:low` `depends:[]`
  > After this: the living-doc corrections in the roadmap audit §4 and the ADR audit §4 are applied (docs / YAML comments only; no crate or `scripts/` edit): ADR-000 D1 / D6 / line 184; ADR-010 D010-4 and F11 option A closed with B/C as an explicit residual; historical markers on pre-SEG-01 Evidence in ADR-010 / 012 / 014 / 060; ADR-012 D012-5 “does not authorize M002” struck; ADR-014 D014-3 cousin and F11; `docs/ADR/README.md` Rocks paragraph (unwired `rocks-index` spike vs unaccepted Rocks event store); `m002-cloud-kernel-io-intake.md` status; `p0-spike-inventory.md` Rocks row; `PROJECT.md` D6 / Validated persist picture; `REQUIREMENTS.md` DOC-06 historical and WRITE-01 rewritten as intern-admit vs segmented persist; `.planning/codebase/*` superseded banner; `STRATEGY.md` date and Rocks sentence; honeycomb.yaml ADR-010 / 012 evidence / capability honesty for the M002 oracles (Status stays Proposed); STATE / CHANGELOG “tag `v0.12` is for the controller to create” wording (CHANGELOG here; `.kutha/STATE.md:3` Product-SoT sentence is **controller-owned** and is edited only at lease time — this slice only lists it); closed M001–M017 bodies in this ROADMAP collapse to one-line + archive pointer (`.planning/milestones/*/NN-VERIFICATION.md` archives untouched). ADRs stay Proposed.
- [x] **S02: Governor hardening** `risk:medium` `depends:[S01]`
  > After this: a new `cargo_dep_constraint` kind (stdlib `tomllib`; YAML row + one kind, not a Check subclass; META / intake `disposition: kind`) owns `freeze-rocks` / `freeze-cypher-deps` / `freeze-hnsw-dep`; bypass forms from the governor audit (`[dependencies.rocksdb]`, `rocksdb="0.22"`, `rocksdb = "0.22" # optional = true`, workspace `[workspace.dependencies.rocksdb]`) are RED tests first; `m002_*` oracles are enrolled in honeycomb `evidence` and `observe_cargo.required` (plus an `observe_cargo` row with `--features rocks-index`); `artifact-budget` is strict (`min_hits` 1 while a live `.planning/phases/*` exists; SUMMARY `max_files_per_dir` 1); `id-map-cite` requires `v0.12` / `v0.13`; selftest proves ledger / pointer kinds by their own mutation or declares an explicit skip with a reason.
- [x] **S03: Governor SMP** `risk:medium` `depends:[S02]`
  > After this: independent checks and selftest proofs run in a process pool (`KUTHA_GOV_JOBS` / `--jobs`, default = min(cpu, 8)); `observe_cargo` may overlap selftest where the FSM allows; findings and event-log order stay deterministic (results merged in check order); `--jobs 1` stays the oracle (same findings, byte-equal fold). Selftest `copytree` skips `.compound-engineering` and `.planning/milestones` so proofs cannot depend on those archives. Measured: `ci` default jobs about 54 percent faster than `--jobs 1` (39.7 s → 18.4 s warm); selftest −25 percent; precommit −10 percent; cold `ci` 167 s.
- [x] **S04: Test strategy and oracle matrix** `risk:medium` `depends:[S03]`
  > After this: a new honeycomb cell **023** (test strategy and oracle matrix; Proposed; sits with 020–022 conventions) and `docs/architecture/test-matrix.md` (invariant × layer × oracle × kill-mutation × status / owner-gate, seeded from the code-tests audit §2–3) exist; a governor row in `rust_test_asserts` style proves every oracle named in the matrix exists and that owner-gated invariants are marked `blocked: decision` rather than xfail; layering rules are written (characterization vs decision oracle; cfg-feature tests need an FSM observe row; durability invariants require a mutation-kill). Status stays Proposed. Landed: ADR-023 Proposed; `test-matrix.yaml` 67 rows (killed 34, gap 19, blocked 6, spec-only 4, characterization 3, gate 1) plus the human page; kind `test_matrix` / check `test-matrix` (65 checks); independent verification 12/12.
- [x] **S05: Durability residuals A** `risk:high` `depends:[S04]`
  > After this: a single-writer advisory lock per store dir (`File::try_lock`; Rust 1.99; two writers fail closed; tests RED first); Typed / Dictionary multi-event group vs `committed_prefix` has a design decision recorded in ADR-010 / 014 and a RED-first oracle; missing oracles from the code-tests audit §2 land (mixed suffix; migrate × tail guard; active at exact frame end; wrong-role active in the manifest). Named M010–M017 oracle assertions do not change. Landed: advisory single-writer `kutha.lock` (`File::try_lock`, WouldBlock, readers unlocked, dead-process safe, panic releases, listings ignore it); Persister poisoned after a fail-closed append; GROUP-01 verdict (c) documented residual with no format change; matrix 67 rows (killed 46, characterization 4, gap 6, blocked 6, spec-only 4, gate 1); 4 lock oracles were RED on the old code; bench gate PASS; independent verification 13/13.
- [x] **S06: Durability residuals B** `risk:high` `depends:[S05]`
  > After this: Rocks index staleness beyond the tip is detected cheaply (bind index meta to the manifest chain tip + count, or equivalent; RED first); the `bytes_per_fact` 17218→25200 rise is confirmed by a measurement (dhat / heaptrack or a counting allocator) and fixed or explained; dead-code cleanup (`log_has_quantum_outcome`, `wal::append_events` `allow(dead_code)`, `rocks_spike` fate: keep as spike or delete with an ADR note); a decision is recorded whether `SegmentIndex` / `Persister` get wired into `Runtime` / `kutha-tenant` (default: record “not wired, reason”). Bench gate applies. Landed: ROCK-02 always-on `m:sealed`, opt-in `verify_full()`, `m:v` 2 (mid-poison only via `verify_full`/`get_event`; `m:v` shadowed by `m:sealed`); DUR-export-race green pins on the Phase 60 lock; MEM-01 counting allocator + streaming frames + `split_events` by ref + `drop(encoded_log)` (persist peak N=10000 56.7 MB → 28.7 MB; persisted bytes identical; `bytes_per_fact` residual not shown as an engine lower bound); `log_has_quantum_outcome` deleted; `wal::append_events` and `rocks_spike` kept; wiring not done (reasons in ADR-010); matrix 67 rows (killed 48, characterization 5, gap 3, blocked 6, spec-only 4, gate 1); bench gate PASS on controller re-measure (executor `open` +10.10% did not reproduce); independent verification 8/8.
- [x] **S07: Semantic and logic test expansion** `risk:medium` `depends:[S06]`
  > After this: seeded randomized differential fold vs CSR lease vs clean rebuild is green (DIFF-01, pulled from M018 S04; hand-rolled seeded LCG like `m017_determinism.rs`; no new dependency); cascade order is stable Eager vs Lazy; derived triples are seed-deterministic; Correct of a Horn premise is characterized (current behaviour pinned and labeled characterization); the legal TR-02 gap is recorded as `blocked: decision` in the matrix. No owner-gated behaviour changes. Landed: seeded LCG oracles (`m019_sem_diff.rs`, `m019_sem_model.rs`, `m019_sem_meta.rs`; ≥64 seeds; `KUTHA_SEEDS` may raise) for fold vs typed/untyped CSR vs persist+open (legacy/segmented/migrate), naive ADR-013 list-scan vs `GraphFold`, cascade Eager vs Lazy, metamorphic persist/tamper/replay, Horn Correct characterization; `TermDictionary::is_empty` only `src/` change (no bench gate); matrix 72 rows (killed 55, characterization 6, gap 0, blocked 6, spec-only 4, gate 1); isolated 5.1+5.3+0.09 s; `KUTHA_SEEDS=256` green; `kutha-gov ci` 0 HIGH / 1 LOW / 65 checks; independent verification 11/11 PASS-WITH-FINDINGS then re-verified; final fix `511e2a5`. Owner-gated still blocked: C01/FIXG-01, HORN-KC-01, ASOF-ELIG-01, DET-02, TR-02, HORN-CORR-01.

IO-01 and VAC-01 stay intake. Whole-file `persist` and O(N) `open` stay residuals outside this milestone.

## M020: Audit correction — CLOSED (GSD v0.14)

**Closed:** 2026-10-07. Source: `.compound-engineering/artifacts/reviews/2026-10-07-whole-repo-audit/` plus parent re-verification `.compound-engineering/artifacts/plans/2026-10-07-audit-correction-plan.md`. Process tag `v0.14` (GSD process tag, not a product release). S01–S05 complete; five independent VERIFICATIONs. Residuals in `.planning/milestones/v0.14-MILESTONE-AUDIT.md`. Crate SemVer stays 0.0.0. Honeycomb stays Proposed. Not Cypher/HNSW.

- [x] **S01: Hygiene** `risk:low` `depends:[]`
  > After this: `cargo fmt --check` green; dead `line_at` removed; stale Next-action / ROADMAP pointers fixed; `ce-skills-ru` handoff override committed or dropped; no product behaviour change.
- [x] **S02: Harness fail-closed** `risk:medium` `depends:[S01]`
  > After this: nameless dictionary row, missing cite file, bad JSONL line, `min_phase` without NN-, empty heading glob are findings (not silent pass); each has a RED selftest mutation first.
- [x] **S03: Persist atomic rewrite** `risk:high` `depends:[S02]`
  > After this: `persist_log` / seal rewrite write `*.tmp`, fsync, rename; manifest last; failpoint/crash-window oracle; bench gate ≤10% on `ingest_plain.n` / `as_of.n`.
- [x] **S04: Horn and fold residuals** `risk:high` `depends:[S03]`
  > After this: CorrectInterval prefix/suffix re-derive; co-live-safe `live_triples`; Retract admit gate for reserved/`kutha:*`; real fingerprint oracle replaces `open_retract_reemit` stub; named pins; bench gate ≤10%.
- [x] **S05: Process contract** `risk:low` `depends:[S04]`
  > After this: lease = STATE+CHANGELOG only (codified); product close records bench; post-verify VERIFICATION edits require re-verify; M020 closed.

## M151: observe phase_ms wall honesty — OPEN (GSD v1.45)

Dogfood follow-on to M150: under jobs>1 overlap, `phase_ms observe_cargo=` measures Future join-wait (often ~0) after cargo already finished in the pipeline — lies next to real `elapsed_ms=` on findings. Thin slice: record cargo wall time in the overlap pack and use it for `phase_ms`. Honeycomb stays Proposed. Budget 103→104.

- [ ] **S01: observe phase_ms wall** `risk:low` `depends:[]`
  > After this: matrix `SEM-m151-observe-phase-ms` killed. Phase 225.

## M150: observe-fail elapsed + stderr tail — CLOSED (GSD v1.44)

Dogfood follow-on to M148/M149: bare `cargo test exit 101` findings look like a no-op when `phase_ms observe_cargo=0` races, and hide compile/stderr. Thin slice: `observe-fail` includes `elapsed_ms=N` and a truncated output tail (`KUTHA_GOV_OBSERVE_FAIL_TAIL`, default 240). Honeycomb stays Proposed. Budget 102→103.

- [x] **S01: observe-fail detail** `risk:low` `depends:[]`
  > After this: matrix `SEM-m150-observe-fail-detail` killed. Phase 224. Landed.

## M149: partial cargo observe → incomplete retry — CLOSED (GSD v1.43)

Dogfood follow-on to M148: parallel/cold cargo can exit 101 after some required tests print `ok`, so M145's "no verdict" incomplete gate misses and M146 never retries — N x `observe-missing` instead. Thin slice: non-zero exit + missing required + no FAILED among required → one `observe-incomplete` (retryable). Real FAILED stays per-name. Honeycomb stays Proposed. Budget 101→102.

- [x] **S01: partial-run incomplete honesty** `risk:low` `depends:[]`
  > After this: matrix `SEM-m149-partial-observe-incomplete` killed. Phase 223. Landed.

## M148: ci summary observe_retried label — CLOSED (GSD v1.42)

Dogfood follow-on to M147: LOW `observe-retried` is easy to miss in findings noise. Thin slice: harness summary always prints `observe_retried=N` next to `run_high=` / `run_low=`. Label contract via pytest (not file_contains needles). Honeycomb stays Proposed. Budget 100→101.

- [x] **S01: observe_retried summary label** `risk:low` `depends:[]`
  > After this: matrix `SEM-m148-observe-retried-summary` killed. Phase 222. Landed.

## M147: observe-retried LOW + cargo pack HIGH-only — CLOSED (GSD v1.41)

Dogfood follow-on to M146: `_apply_cargo_pack` treats any finding as `cargo=fail`, so a LOW retry breadcrumb would lie. Thin slice: pack status fails only on HIGH; successful incomplete→ok retry emits LOW `observe-retried`. Honeycomb stays Proposed. Budget 99→100.

- [x] **S01: observe-retried breadcrumb** `risk:low` `depends:[]`
  > After this: matrix `SEM-m147-observe-retried` killed. Phase 221. Landed.

## M146: cargo observe retry on incomplete — CLOSED (GSD v1.40)

Dogfood follow-on to M145: `observe-incomplete` still fails the quantum; a single automatic retry (warm/race) recovers without manual re-run. Thin slice: `KUTHA_GOV_CARGO_OBSERVE_RETRIES` (default 1) under the same flock; discard first-attempt findings when retry succeeds. Honeycomb stays Proposed. Budget 98→99.

- [x] **S01: retry on observe-incomplete** `risk:low` `depends:[]`
  > After this: matrix `SEM-m146-observe-retry` killed. Phase 220. Landed.

## M145: observe-incomplete honesty (cargo infra fail) — CLOSED (GSD v1.39)

Dogfood residual (M143/M144 closeout): cargo exit 101 with no test lines mints N× `observe-missing` HIGH beside one `observe-fail`, drowning the infra signal. Thin slice: when `cargo_exit != 0` and no required name has ok/FAILED, one `observe-incomplete` instead of N missing. Honeycomb stays Proposed. Budget 97→98.

- [x] **S01: observe-incomplete collapse** `risk:low` `depends:[]`
  > After this: matrix `SEM-m145-observe-incomplete` killed. Phase 219. Landed.

## M144: git-auto-base summary labels behavioral (de-text-couple) — CLOSED (GSD v1.38)

CE residual after M137–M143: `git-auto-base` still pins exact `run_high=` / `tip_high=` f-string needles in `__main__.py` that duplicate `test_cmd_ci_prints_run_high_and_tip_high`. Substring `run_high=` also matches `this_run_high=` rewords. Thin slice: keep merge-base wiring + both pytest names; drop label file_contains. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: git-auto-base label de-text-couple** `risk:low` `depends:[]`
  > After this: matrix `SEM-m144-git-auto-base-behavioral` killed. Phase 218. Landed.

## M143: skip-cargo-log-marker behavioral (de-text-couple) — CLOSED (GSD v1.37)

CE residual after M137–M142: skip-cargo-log-marker still pins CE #38 comment and assert-snippet needles that rot independently of tenant=skipped fold tip behavior. Thin slice: keep fsm wiring + pytest name; drop comment/assert snippets. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: skip-cargo-log-marker de-text-couple** `risk:low` `depends:[]`
  > After this: matrix `SEM-m143-skip-cargo-marker-behavioral` killed. Phase 217. Landed.

## M142: observe-skipped-breadcrumb behavioral (de-text-couple) — CLOSED (GSD v1.36)

CE residual after M137–M141: observe-skipped-breadcrumb still pins assert-snippet needles (`category == "observe-skipped"`, `prior-high`) that rot independently of FSM behavior. Thin slice: keep fsm wiring + pytest name; drop assert snippets. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: observe-skipped de-text-couple** `risk:low` `depends:[]`
  > After this: matrix `SEM-m142-observe-skipped-behavioral` killed. Phase 216. Landed.

## M141: parallel-context-flags behavioral (de-text-couple) — CLOSED (GSD v1.35)

CE residual after M137/M140: parallel-context-flags still pins docstring/comment and assert-snippet needles that rot independently of `_eval_check_worker` behavior. Thin slice: keep Context wiring needles + pytest name; drop comment/`skip_emit=True`/`a.md,b.md` text pins. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: parallel-context-flags de-text-couple** `risk:low` `depends:[]`
  > After this: matrix `SEM-m141-parallel-context-behavioral` killed. Phase 215. Landed.

## M140: agent-surface hint behavioral (de-text-couple) — CLOSED (GSD v1.34)

CE residual after M137/M138: agent-surface still pins exact `__main__.py` hint strings that duplicate CLI reject pytest. Thin slice: drop hint needles; keep parser wiring; soften stderr oracles to surface redirect not exact `hint:` template. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: agent-surface hint de-text-couple** `risk:low` `depends:[]`
  > After this: matrix `SEM-m140-agent-surface-hint-behavioral` killed. Phase 214. Landed.

## M139: multi-premise retract premise-fact index — CLOSED (GSD v1.33)

M136 made single-premise retract near-linear; multi-premise Horns still walk the whole same-rel `behaviors_by_premise_rel` bucket in phase 2 → O(N²) `retract_examine` for N independent chains. Thin slice: droppable premise-EventId index + scaling oracle. Honeycomb stays Proposed. Budget stays 97. No Op schema change (lease only).

- [x] **S01: premise-fact retract index** `risk:med` `depends:[]`
  > After this: matrix `SEM-m139-retract-multiprem-fact-index` killed. Phase 213. Landed.

## M138: kinds-family-split behavioral (de-text-couple) — CLOSED (GSD v1.32)

CE 2026-10-15 residual: kinds-split text needles rot independently of behavior. Thin slice: import/dispatch pytest oracle; drop `RUNNERS: dict[str, Runner]` needle; selftest mutates RUNNERS key. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: kinds-family-split dispatch oracle** `risk:low` `depends:[]`
  > After this: matrix `SEM-m138-kinds-family-behavioral` killed. Phase 212. Landed.

## M137: parallel-check-error-parity behavioral (de-text-couple) — CLOSED (GSD v1.31)

CE 2026-10-15 residual: text-coupled governor oracles (parallel-check-error-parity file_contains needles) rot independently of behavior. Thin slice: direct `_eval_check_worker` oracle + drop docstring/assert needles; keep `return eval_check(check, ctx)`. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: behavioral worker check-error oracle** `risk:low` `depends:[]`
  > After this: matrix `SEM-m137-parallel-error-behavioral` killed. Phase 211. Landed.

## M136: retract N-derived N-retracts scaling — CLOSED (GSD v1.30)

CE 2026-10-15 testing gap: no oracle for N derived + N retracts asserting sublinear `retract_examine` growth (same-rel sibling second-phase scan was quadratic for single-premise Horns). Thin slice: skip single-premise in phase 2 + scaling oracle. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: retract scaling oracle** `risk:low` `depends:[]`
  > After this: matrix `SEM-m136-retract-n-scaling` killed. Phase 210. Landed.

## M135: migrate_pins unlock window between steps — CLOSED (GSD v1.29)

CE 2026-10-15 residual: `migrate_pins` / `migrate_compat` drop and reacquire `kutha.lock` between chained steps; concurrent operator may interleave but converges via markers — untested. Thin slice: no parent lock source oracle + deterministic gap WouldBlock + converge; pin-compat docs. Honeycomb stays Proposed. Budget stays 97.

- [x] **S01: migrate_pins unlock-window oracles** `risk:low` `depends:[]`
  > After this: matrix `SEM-m135-migrate-pins-unlock-window` killed. Phase 209. Landed.

## M134: pin-cite.v1 delete/rerun honesty — CLOSED (GSD v1.28)

CE 2026-10-15 residual: after `pin-cite.v1` is stamped, evidence added later never remaps orphans on rerun; operator must delete the marker — confirm in operator docs, not only code comments. Thin slice: pin-compat.md sentence + needle + rematerialize oracle. Honeycomb stays Proposed. Budget stays 97 (extend `pin-compat-policy`).

- [x] **S01: pin-cite delete marker honesty** `risk:low` `depends:[]`
  > After this: matrix `SEM-m134-pin-cite-delete-rerun` killed. Phase 208. Landed.

## M133: migrate_compat op-wire-before-pins order — CLOSED (GSD v1.27)

CE 2026-10-15 residual: SEM-m076 remapped-count + markers stay green if `migrate_compat` swapped pin-then-op-wire. Thin slice: source-order oracle that `migrate_op_wire` precedes `migrate_pins`. Honeycomb stays Proposed.

- [x] **S01: migrate_compat call-order oracle** `risk:low` `depends:[]`
  > After this: matrix `SEM-m133-migrate-compat-order` killed. Phase 207. Landed.

## M132: reconcile tip truncate sync_all — CLOSED (GSD v1.26)

CE 2026-10-15 residual: `reconcile_uncommitted_tip` truncates with `set_len` + `sync_data`; on platforms where fdatasync does not persist size, a power loss can resurrect the suffix. Thin slice: `sync_all` after each truncate `set_len`. Honeycomb stays Proposed. `L_delivery=M132-closed`.

- [x] **S01: sync_all after reconcile set_len** `risk:low` `depends:[]`
  > After this: matrix `SEM-m132-reconcile-sync-all` killed. Phase 206. Landed.

## M131: yaml_map_list empty-select fail-closed — CLOSED (GSD v1.25)

CE 2026-10-15 residual: empty `yaml_map_list` select list vacuously passes require/unique/cross-ref. Thin slice: HIGH when select is an empty list (parity with `yaml_needles_in_glob` / test-matrix). Honeycomb stays Proposed. `L_delivery=M131-closed`.

- [x] **S01: nonempty yaml_map_list select** `risk:low` `depends:[]`
  > After this: empty `items: []` is HIGH; matrix `SEM-m131-yaml-map-list-nonempty` killed. Phase 205. Landed.

## M130: selftest parallel submit serial backstop — CLOSED (GSD v1.24)

CE 2026-10-15 residual: `_run_selftest_parallel` dict-comprehension submit has no serial backstop on submit failure (unlike `run_checks_parallel`). Thin slice: submit one-by-one + leftover serial. Honeycomb stays Proposed. `L_delivery=M130-closed`.

- [x] **S01: selftest submit serial backstop** `risk:low` `depends:[]`
  > After this: FailAfterFirst submit still returns a row per check; matrix `SEM-m130-selftest-submit-backstop` killed. Phase 204. Landed.

## M129: yaml_needles live dogfood — CLOSED (GSD v1.23)

`yaml_needles_in_glob` is DERIVABLE (M125) but unused in live checks. Thin slice: dictionary check that FSM `observe_cargo` / `observe_rocks` required names appear as `fn <name>` under crates. Complements `observe-required-fn` (asserting `#[test]`). Honeycomb stays Proposed. `L_delivery=M129-closed`.

- [x] **S01: yaml-needles-live check** `risk:low` `depends:[]`
  > After this: live kind user; budget 95; matrix `SEM-m129-yaml-needles-live` killed. Phase 203. Landed.

## M128: Horn multi-combination oracle — CLOSED (GSD v1.22)

CE 2026-10-15 residual risk: no oracle that two disjoint live windows of the same premise relation overlapping one trigger mint two heads (`search_at_all` already enumerates). Thin slice: kill-grade cargo oracle + matrix. Honeycomb stays Proposed. `L_delivery=M128-closed`.

- [x] **S01: multi-combination Horn oracle** `risk:low` `depends:[]`
  > After this: two disjoint AB residuals + open BC yield two AC heads; matrix `SEM-m128-horn-multi-combination` killed. Phase 202. Landed.

## M127: selftest.py line ceiling — CLOSED (GSD v1.21)

`kinds-lean` pins the kinds kernel; `selftest.py` (~951 lines) is the next largest governor module without a ceiling. Thin slice: `file_max_lines` pin at 1000. Honeycomb stays Proposed.

- [x] **S01: selftest-lean file_max_lines** `risk:low` `depends:[]`
  > After this: matrix `SEM-m127-selftest-lean` killed. Phase 201. Landed.

## M126: kinds.py line ceiling — CLOSED (GSD v1.20)

After the kinds-family split, `kinds.py` was ~2292 lines and has regrown (~2431). Thin slice: `file_max_lines` pin (like `agents-lean`) so further kernel growth fails closed. Honeycomb stays Proposed.

- [x] **S01: kinds-lean file_max_lines** `risk:low` `depends:[]`
  > After this: matrix `SEM-m126-kinds-lean` killed. Phase 200. Landed.

## M125: yaml_needles selftest derive + DERIVABLE completeness — CLOSED (GSD v1.19)

`yaml_needles_in_glob` has zero live dictionary users and was not DERIVABLE (CE 2026-10-15 residual). Thin slice: derive + assert ALLOWED ⊆ DERIVABLE ∪ {git_path_implies}. Honeycomb stays Proposed.

- [x] **S01: derive yaml_needles + DERIVABLE completeness** `risk:low` `depends:[]`
  > After this: synthetic selftest OK; matrix `SEM-m125-yaml-needles-derive` killed. Phase 199. Landed.

## M124: glob_regex_absent selftest derive — CLOSED (GSD v1.18)

M123 landed `glob_regex_absent` but selftest only proved it via explicit YAML mutations. Thin slice: DERIVABLE derive + mutation target so a check with only that kind is not UNPROVEN. Honeycomb stays Proposed.

- [x] **S01: derive glob_regex_absent in selftest** `risk:low` `depends:[]`
  > After this: synthetic selftest OK without explicit mutations; matrix `SEM-m124-glob-regex-derive` killed. Phase 198. Landed.

## M123: fixed-tmp join-after-tmp dogfood — CLOSED (GSD v1.17)

`Path::new("tmp/…")` is forbidden, but `.join("tmp").join("literal")` still slips (sem `p62-sem`). Thin slice: kind `glob_regex_absent` + unique leaf without a fixed middle segment. Honeycomb stays Proposed.

- [x] **S01: glob_regex_absent for join(tmp).join(str)** `risk:low` `depends:[]`
  > After this: no `.join("tmp").join("` under crates; matrix `SEM-m123-fixed-tmp-join` killed. Phase 197. Landed.

## M122: fixed-tmp persist-dir dogfood — CLOSED (GSD v1.16)

`Path::new("tmp/…")` in crates raced under parallel Cargo (example + m026). Thin slice: governor `glob_absent` needle + per-call dirs for the remaining hits. Honeycomb stays Proposed.

- [x] **S01: fixed-tmp-persist-dir check + unique remaining dirs** `risk:low` `depends:[]`
  > After this: no `Path::new("tmp/` under crates; matrix `SEM-m122-fixed-tmp-persist-dir` killed. Phase 196. Landed.

## M121: unique facade-statute persist dir — CLOSED (GSD v1.15)

`statute_as_of_via_facade` persisted to a fixed `tmp/p27-facade-statute`. Same race as M116–M120 under parallel Cargo. Thin slice: a per-call directory (pid, nanos, sequence). Honeycomb stays Proposed.

- [x] **S01: unique persist dir for facade statute open** `risk:low` `depends:[]`
  > After this: two calls do not share a path; the fixed `tmp/p27-facade-statute` name is gone; matrix `SEM-m121-facade-statute-store-dir` killed. Phase 195. Landed.

## M120: unique fork known-on persist dir — CLOSED (GSD v1.14)

`fork_at_before_later_anchor_differs_and_persist_open` persisted to a fixed `tmp/p36-02-fork`. Same race as M116–M119 under parallel Cargo. Thin slice: a per-call directory (pid, nanos, sequence). Honeycomb stays Proposed.

- [x] **S01: unique persist dir for fork known-on open** `risk:low` `depends:[]`
  > After this: two calls do not share a path; the fixed `tmp/p36-02-fork` name is gone; matrix `SEM-m120-fork-known-on-store-dir` killed. Phase 194. Landed.

## M119: unique anchor-admit persist dir — CLOSED (GSD v1.13)

`monotone_unique_anchors_persist_open` persisted to a fixed `tmp/p34-02-store`. Same race as M116–M118 under parallel Cargo. Thin slice: a per-call directory (pid, nanos, sequence). Honeycomb stays Proposed.

- [x] **S01: unique persist dir for monotone anchors open** `risk:low` `depends:[]`
  > After this: two calls do not share a path; the fixed `tmp/p34-02-store` name is gone; matrix `SEM-m119-anchor-admit-store-dir` killed. Phase 193. Landed.

## M118: unique closed-VT known-on persist dir — CLOSED (GSD v1.12)

`closed_vt_known_on_persist_open` persisted to a fixed `tmp/p34-01-store`. Same race as M116/M117 under parallel Cargo. Thin slice: a per-call directory (pid, nanos, sequence). Honeycomb stays Proposed.

- [x] **S01: unique persist dir for closed-VT known-on open** `risk:low` `depends:[]`
  > After this: two calls do not share a path; the fixed `tmp/p34-01-store` name is gone; matrix `SEM-m118-closed-vt-known-on-store-dir` killed. Phase 192. Landed.

## M117: unique quantum known-on persist dir — CLOSED (GSD v1.11)

`as_known_on_survives_persist_open_and_matches_fold` persisted to a fixed `tmp/p32-01-store`. Same race as M116 under parallel Cargo. Thin slice: a per-call directory (pid, nanos, sequence). Honeycomb stays Proposed.

- [x] **S01: unique persist dir for quantum known-on open** `risk:low` `depends:[]`
  > After this: two calls do not share a path; the fixed `tmp/p32-01-store` name is gone; matrix `SEM-m117-quantum-known-on-store-dir` killed. Phase 191. Landed.

## M116: unique known-on persist dir — CLOSED (GSD v1.10)

`as_known_on_persist_open_preserves_answers` persisted to a fixed `tmp/p32-02-store`. A second Cargo process could delete that directory mid-write (`StoreIo` ENOENT). Thin slice: a per-call directory (pid, nanos, sequence), same pattern as `m019_dur_a`. Honeycomb stays Proposed.

- [x] **S01: unique persist dir for as_known_on open** `risk:low` `depends:[]`
  > After this: two calls do not share a path; the fixed `tmp/p32-02-store` name is gone; matrix `SEM-m116-known-on-store-dir` killed. Phase 190. Landed.

## M115: empty --check on ci — CLOSED (GSD v1.09)

`ci --check ""` is falsy after join, so the reject guard skips and the full quantum runs. Thin slice: a present `--check` on `ci` exits 2 even when the value is empty. Honeycomb stays Proposed.

- [x] **S01: reject empty --check on ci** `risk:low` `depends:[]`
  > After this: matrix `SEM-m115-empty-check-on-ci` killed. Phase 189. Landed.

## M114: cargo observe flock — CLOSED (GSD v1.08)

Parallel `cargo bench` and `kutha-gov ci` raced on the same `target/` and produced false `observe_cargo` exit 101 (M113 session). Thin slice: exclusive flock around observe cargo test+build; wait from `.env`. Honeycomb stays Proposed.

- [x] **S01: flock observe_cargo on target dir** `risk:low` `depends:[]`
  > After this: matrix `SEM-m114-cargo-observe-lock` killed. Phase 188. Landed.

## M113: selftest mutation static substring — CLOSED (GSD v1.07)

M112 rejects substring-preserving `replace_regex` only when selftest runs. Authors can land a vacuous mutation and stay green until `ci`'s selftest phase. Thin slice: `get_checks` fail-closes at load (LoadError alongside survivors). Honeycomb stays Proposed.

- [x] **S01: dictionary load rejects substring-preserving replace_regex** `risk:low` `depends:[]`
  > After this: matrix `SEM-m113-selftest-mutation-static` killed. Phase 187. Landed.

## M112: selftest substring trap — CLOSED (GSD v1.06)

`replace_regex` with `needle_REMOVED` still matched `needle` and classified VACUOUS. Selftest now fails closed as UNPROVEN before apply; intake/META document the rule. Honeycomb stays Proposed.

- [x] **S01: reject substring-preserving replace_regex** `risk:low` `depends:[]`
  > After this: matrix `SEM-m112-selftest-substring` killed. Phase 186. Landed.

## M111: hot-path lease dogfood + unindex no fallback — CLOSED (GSD v1.05)

Process: enroll governor `hot-path-lease-honesty` so README/code needles for `tt_by_ingest` and live-only rebuild cannot drift (budget 87). Product: drop `unindex_behavior` full-bucket else branches; reverse pointers are authoritative. Honeycomb stays Proposed.

- [x] **S01: dogfood check + no unindex fallback** `risk:low` `depends:[]`
  > After this: matrix `SEM-m111-hot-path-dogfood` / `SEM-m111-unindex-no-fallback` killed. Phase 185. Landed.

## M110: rebuild indexes only live Behaviors — CLOSED (GSD v1.04)

`rebuild_log_indexes` put every Behavior id into `derived_by_cause` / `cause_of_behavior`, including Correct/Retract dead heads. After persist+open, retract of the premise examined the dead child again. Live-only rebuild keeps the lease honest. Honeycomb stays Proposed.

- [x] **S01: live-only derived index on rebuild** `risk:med` `depends:[]`
  > After this: open after Correct of Behavior then Retract premise examines 0; matrix `SEM-m110-rebuild-live-derived` killed. Phase 184. Landed.

## M109: known_at ingest index — CLOSED (GSD v1.03)

Warm `known_at` / `as_known_on` walked every fact after the TT cut. Hot map `tt_by_ingest` skips facts ingested after the cut. Lazy cold path unchanged (AUD-08). Honeycomb stays Proposed.

- [x] **S01: tt_by_ingest for known_at** `risk:low` `depends:[]`
  > After this: examine counts stay constant vs late decoys at D=32 and D=128; matrix `SEM-m109-known-at-ingest` killed. Phase 183. Landed.

## M108: live behaviors are a set — CLOSED (GSD v1.02)

Unindex of one Behavior walked every live id (`retain` over the vector). The set is a `FxHashSet`: removal bumps the examine counter once. Honeycomb stays Proposed.

- [x] **S01: O(1) live removal** `risk:low` `depends:[]`
  > After this: correct of one Behavior examines a constant live-set count at D=32 and D=128; matrix `SEM-m108-unindex-live-set` killed. Phase 182. Landed.

## M107: unindex skips live-behavior vector — CLOSED (GSD v1.01)

`unindex_behavior` walked every live Behavior id on Retract/Correct of an ordinary Assert. A non-behavior is absent from `cause_of_behavior` and `premise_rels_of_behavior`, so the walk is skipped. Honeycomb stays Proposed.

- [x] **S01: skip live vector** `risk:low` `depends:[]`
  > After this: correct of a non-behavior examines 0 live-behavior entries at D=32 and D=128; matrix `SEM-m107-unindex-live-skip` killed. Phase 181. Landed.

## M106: open restores fold-index lease — CLOSED (GSD v1.00)

`finish_open` replayed under the process env and copied only `FoldIndexMode`. A Lazy snapshot opened while env was eager kept warm maps; an Eager snapshot opened while env was lazy scanned. Honeycomb stays Proposed.

- [x] **S01: snapshot lease warmth** `risk:med` `depends:[]`
  > After this: open calls `set_index_mode` so a Lazy snapshot still full-scans and an Eager snapshot keeps the warm range; matrix `SEM-m106-open-index-lease` killed. Phase 180. Landed.

## M105: audit residuals — CLOSED (GSD v0.99)

2026-10-18 audit: policy-pin scan, premise-rel unindex, duplicate Define, Horn combination windows, replay VT coverage. Honeycomb stays Proposed. Not a hard FSM.

- [x] **S01: residual honesty** `risk:med` `depends:[]`
  > After this: newest policy pin is one row visit; unindex of a non-behavior does not scan premise buckets; matrix `SEM-m105-policy-pin-newest` and `SEM-m105-unindex-premise-pointer` killed. Phases 176–179. Landed.

## M104: unindex cause reverse pointer — CLOSED (GSD v0.98)

CE 2026-10-15 residual: `unindex_behavior` scans every `derived_by_cause` bucket. Store the cause per Behavior id and remove from that one bucket. Honeycomb stays Proposed.

- [x] **S01: reverse cause pointer** `risk:med` `depends:[]`
  > After this: Correct of one Behavior examines a constant number of cause buckets as live D grows; matrix `SEM-m104-unindex-cause-pointer` killed. Phase 175. Landed.

## M103: duplicate Define keeps first id — CLOSED (GSD v0.97)

CE 2026-10-15 residual: `TermDictionary::from_strings` last-wins on a repeated name, so a later duplicate Define steals the id. Honeycomb stays Proposed.

- [x] **S01: keep-first intern map** `risk:low` `depends:[]`
  > After this: a repeated name resolves to the earliest index; matrix `SEM-m103-define-keep-first` killed. Phase 174. Landed.

## M102: migrate verify drops segments — CLOSED (GSD v0.96)

CE 2026-10-15 residual: failed migrate verify deletes `manifest.json` and leaves the generation-unique `seg-*.log` files beside the legacy store. Honeycomb stays Proposed.

- [x] **S01: discard segment files on verify failure** `risk:med` `depends:[]`
  > After this: `migrate_with_verify_fault` leaves no `seg-*.log`; legacy jsonl/WAL still open. Matrix `SEM-m102-migrate-verify-segments` killed. Phase 173. Landed.

## M101: Horn pin overlap — CLOSED (GSD v0.95)

CE 2026-10-15 residual: `live_horns` keeps a pin only when it is live at the trigger `valid_from`. A pin that starts later is skipped even when the conjunction with the premise window is non-empty. Honeycomb stays Proposed.

- [x] **S01: overlap-aware rule pin** `risk:med` `depends:[]`
  > After this: a pin window that starts after the trigger still mints the head on the intersection; matrix `SEM-m101-horn-pin-overlap` killed. Phase 172. Landed.

## M100: Correct unindexes Behavior — CLOSED (GSD v0.94)

CE 2026-10-15 residual: `append_indexed` drops a Behavior on Retract only. Correct and CorrectInterval invalidate the Behavior fact but leave it in `live_behaviors`, `derived_by_cause`, and `behaviors_by_premise_rel`, so later retract scans still examine the dead head. Honeycomb stays Proposed.

- [x] **S01: unindex Behavior on Correct** `risk:med` `depends:[]`
  > After this: Correct/CorrectInterval of a Behavior removes it from the three leases; matrix `SEM-m100-correct-unindex-behavior` killed. Phase 171. Landed.

## M099: CE 2026-10-15 regression enrolment — CLOSED (GSD v0.93)


Whole-review lock: every 2026-10-15 finding stays tied to a living oracle, plus the testing gaps the report still listed (empty `--check`, yaml_map_list scalar rows, multi-check exclusivity, migrate_compat call order). `L_delivery=M099-closed`.

- [x] **S01: regression map + gap oracles** `risk:low` `depends:[]`
  > After this: dropping a mapped oracle or restoring those four gap shapes fails a named test. Phase 170. Landed.

## M098: kinds-split matrix oracle — CLOSED (GSD v0.92)


CE whole-repo (2026-10-15 HEAD) finding #5 (report-only): `SEM-m054-id-kinds-split` names oracle `test-matrix`, which does not observe kinds.py family placement. Decision: enroll a dictionary `file_contains` check (parallel-check-error-parity pattern) and point the matrix row at it. `L_delivery=M098-closed`.

- [x] **S01: kinds-family-split check** `risk:low` `depends:[]`
  > After this: matrix oracle is `kinds-family-split`; re-inlining families fails that check. Phase 169. Landed.

## M097: Horn overlap-aware premise discovery — CLOSED (GSD v0.91)

CE whole-repo (2026-10-15 HEAD) finding #4: multi-premise Horn discovery is point-gated on trigger `valid_from`, so a later-starting overlapping premise is missed order-dependently. Design call: **overlap-aware** (VT-overlap candidates + existing conjunct_vt) over retry-at-candidate-starts. Thin slice + order-independence oracle. `L_delivery=M097-closed`.

- [x] **S01: overlap-aware search_at** `risk:med` `depends:[]`
  > After this: AB[2015,2020)+BC[2010,∞) either insert order yields AC[2015,2020); matrix `SEM-m097-horn-overlap-aware` killed. Phase 168. Landed.

## M096: bench rss_before at sample — CLOSED (GSD v0.90)

CE whole-repo (2026-10-15 HEAD) finding #2: `rss_before` is taken before all 6 factor runtimes while `rss_after` / denominator use only the dedicated sample (n), inflating `bytes_per_fact` ~15x. Thin slice: move `rss_before` to immediately before sample alloc + oracle. `L_delivery=M096-closed`.

- [x] **S01: rss_before after factor loops** `risk:low` `depends:[]`
  > After this: `bytes_per_fact` delta brackets only the dedicated sample; matrix `SEM-m096-bench-rss-before-sample` killed. Phase 167. Landed.

## M095: retract second-phase indexed scan — CLOSED (GSD v0.89)

CE whole-repo (2026-10-15 HEAD) finding #3: after the indexed direct-children phase, every retract/correct still clones and walks the full `live_behaviors` vector with `horn_body_matches_except` per entry (O(N*D)). Thin slice: index the second phase by premise/body so only candidate behaviors are re-checked + examine-count oracle. `L_delivery=M095-closed`.

- [x] **S01: index second-phase retract scan** `risk:med` `depends:[]`
  > After this: second-phase retract/correct examines only behaviors that could mention the dead premise; matrix `SEM-m095-retract-second-phase-index` killed. Phase 166. Landed.

## M094: dictionary loader fail-closed — CLOSED (GSD v0.88)

CE whole-repo (2026-10-15 HEAD) findings #6–#8: checks.yaml loader silently skips scalar rows, overwrites duplicate ids, and runs stepless/scalar-steps checks as vacuous OK. Thin slice: LoadError alongside survivors (CE #20 pattern) + oracles. `L_delivery=M094-closed`.

- [x] **S01: scalar/dup/stepless load errors** `risk:low` `depends:[]`
  > After this: non-mapping rows, duplicate ids, and empty/non-list/scalar-only steps yield LoadErrorCheck HIGH; matrix `SEM-m094-dict-load-fail-closed` killed. Phase 165. Landed.

## M093: kutha-migrate CLI contract — CLOSED (GSD v0.87)

CE whole-repo (2026-10-15 HEAD) finding #9: `--help` exits 2 on stderr; extra positionals silently ignored; missing dir stays exit 2 (CLI must not create). Thin slice: help→stdout exit 0; argc>2 reject; document existing-dir. `L_delivery=M093-closed`.

- [x] **S01: --help exit 0 + reject extra args** `risk:low` `depends:[]`
  > After this: `--help`/`-h` exit 0 on stdout; argc>2 usage+exit 2; matrix `SEM-m093-migrate-cli-contract` killed. Phase 164. Landed.

## M092: migrate reports dropped_tail — CLOSED (GSD v0.86)

CE whole-repo (2026-10-15 HEAD) finding #13: `migrate_op_wire` / `migrate_pin_wire` / `migrate_pin_cite` drop torn jsonl/WAL/segment tails with no count, unlike `migrate().dropped_uncommitted`. Thin slice: surface `dropped_tail` on the three reports (+ oracle). `L_delivery=M092-closed`.

- [x] **S01: dropped_tail on Op/Pin migrate reports** `risk:low` `depends:[]`
  > After this: torn source tails increment `dropped_tail` on OpWire/PinWire/PinCite reports; matrix `SEM-m092-migrate-dropped-tail` killed. Phase 163. Landed.

## M091: migrate_pin_cite drops stale justifications sidecar — CLOSED (GSD v0.85)

CE whole-repo (2026-10-15 HEAD) finding #1: `migrate_pin_cite` remaps JustificationCite pins in the log but removes only `snapshot.json`, leaving a non-empty `justifications.jsonl` lease that fails `sidecar_or_stale_prefix` on reopen. Thin slice: drop the regenerable sidecar (+ RED oracle with non-empty sidecar). `L_delivery=M091-closed`.

- [x] **S01: drop justifications.jsonl in migrate_pin_cite** `risk:medium` `depends:[]`
  > After this: migrate+open succeeds when pre-migrate persist wrote cite rows; matrix `SEM-m091-pin-cite-justifications` killed. Phase 162. Landed.

## M090: --skip-cargo reject redirect hint — CLOSED (GSD v0.84)

Process debt after M089: CLI rejects `--skip-cargo` outside `ci` (exit 2), but stderr only names the rule — agents still guess the next command. Thin slice: one-line redirect hint; reject oracle. `L_delivery=M090-closed`.

- [x] **S01: stderr hint on invalid --skip-cargo** `risk:low` `depends:[]`
  > After this: invalid `--skip-cargo` prints `hint: kutha-gov ci --skip-cargo`; matrix `SEM-m090-skip-cargo-reject-hint` killed. Phase 161. Landed.

## M089: --check reject redirect hint — CLOSED (GSD v0.83)

Process debt after M088: CLI rejects `--check` outside json|precommit|selftest (exit 2), but stderr only names the rule — agents still guess the next command. Thin slice: one-line redirect hint; extend reject oracle. `L_delivery=M089-closed`.

- [x] **S01: stderr hint on invalid --check** `risk:low` `depends:[]`
  > After this: invalid `--check` prints `hint: kutha-gov {json|precommit|selftest} --check ID[,ID…]`; matrix `SEM-m089-check-reject-hint` killed. Phase 160. Landed.

## M088: AGENTS --check not for ci — CLOSED (GSD v0.82)

Process debt after M087: CLI rejects `ci --check`, and harness names three surfaces, but AGENTS Flags line groups `ci` with `--check` ("where noted") so agents still try selective checks on full FSM. Thin slice: AGENTS + agent-surface + CLI reject oracle. `L_delivery=M088-closed`.

- [x] **S01: AGENTS says --check excludes ci** `risk:low` `depends:[]`
  > After this: AGENTS Flags state `--check` only for json|precommit|selftest; `ci --check` still exit 2; matrix `SEM-m088-agents-check-not-ci` killed. Phase 159. Landed.

## M087: harness --check surface discoverability — CLOSED (GSD v0.81)

Process debt after M086: AGENTS and CLI accept `--check` on json/precommit/selftest (repeatable + comma), but `kutha-harness.md` still says only json/precommit. Thin slice: harness prose + dogfood/agent-surface pins. `L_delivery=M087-closed`.

- [x] **S01: harness documents three --check surfaces** `risk:low` `depends:[]`
  > After this: `kutha-harness.md` names json/precommit/selftest `--check` (repeatable + comma); matrix `SEM-m087-harness-check-surfaces` killed. Phase 158. Landed.

## M086: selftest --check append oracle — CLOSED (GSD v0.80)

Process debt after M085: json/precommit have repeated `--check` oracles; `selftest` shares `_join_check_flags` but has no multi-select append pin. Thin slice: pytest + agent-surface. `L_delivery=M086-closed`.

- [x] **S01: selftest repeated --check runs both** `risk:low` `depends:[]`
  > After this: `test_selftest_repeated_check_flags_runs_both`; matrix `SEM-m086-selftest-check-append` killed. Phase 157. Landed.

## M085: json --check append oracle — CLOSED (GSD v0.79)

Process debt after M079: repeated `--check` works for precommit, but `kutha-gov json` has no oracle that append selects both rows (same `_join_check_flags` path). Thin slice: pytest + agent-surface. `L_delivery=M085-closed`.

- [x] **S01: json repeated --check runs both** `risk:low` `depends:[]`
  > After this: `test_json_repeated_check_flags_runs_both`; matrix `SEM-m085-json-check-append` killed. Phase 156. Landed.

## M084: .env.example built migrate path — CLOSED (GSD v0.78)

Process debt after M083: META/AGENTS/harness cite built migrate, but `.env.example` only documents `KUTHA_TENANT_BIN`. Thin slice: comment the observe-built path + optional `KUTHA_MIGRATE_DIR`; toolchain pins it. `L_delivery=M084-closed`.

- [x] **S01: .env.example migrate discoverability** `risk:low` `depends:[]`
  > After this: `.env.example` names `debug/kutha-migrate`; matrix `SEM-m084-env-migrate-path` killed. Phase 155. Landed.

## M083: Harness CutPort migrate-bin — CLOSED (GSD v0.77)

Process debt after M082: AGENTS/op/pin cite built migrate, but `kutha-harness.md` Space table only lists tenant-bin CutPort. Thin slice: add migrate-bin row; dogfood pins it. `L_delivery=M083-closed`.

- [x] **S01: CutPort migrate-bin row** `risk:low` `depends:[]`
  > After this: harness Space table names migrate-bin + debug/kutha-migrate; matrix `SEM-m083-harness-migrate-cutport` killed. Phase 154. Landed.

## M082: Process notes built migrate path — CLOSED (GSD v0.76)

Process debt after M081: AGENTS prefers built `kutha-migrate`, but `op-compat.md` / `pin-compat.md` still show bare CLI form. Thin slice: both process notes cite `debug/kutha-migrate`; policy checks pin it. `L_delivery=M082-closed`.

- [x] **S01: op-compat + pin-compat built path** `risk:low` `depends:[]`
  > After this: both notes name `debug/kutha-migrate`; matrix `SEM-m082-migrate-process-notes` killed. Phase 153. Landed.

## M081: Built migrate bin discoverability — CLOSED (GSD v0.75)

Process debt after M080: observe builds `kutha-migrate`, but AGENTS still documents `cargo run` (second compile). Thin slice: agent-surface prefers `$CARGO_TARGET_DIR/debug/kutha-migrate`. `L_delivery=M081-closed`.

- [x] **S01: AGENTS/META prefer built migrate** `risk:low` `depends:[]`
  > After this: no `cargo run … kutha-migrate` in AGENTS; matrix `SEM-m081-migrate-bin-path` killed. Phase 152. Landed.

## M080: Observe build both operator bins — CLOSED (GSD v0.74)

Process debt after M078: `kutha-migrate` exists but `observe_cargo` build only compiles `kutha-tenant`, so post-ci operators still need a separate build. Thin slice: one `cargo build` with both `--bin` flags; extend `tenant-bin` oracle. `L_delivery=M080-closed`.

- [x] **S01: build kutha-tenant + kutha-migrate** `risk:low` `depends:[]`
  > After this: fsm + observe default build both bins; matrix `SEM-m080-migrate-build` killed. Phase 151. Landed.

## M079: Repeatable --check flags — CLOSED (GSD v0.73)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.73`. Honeycomb stays Proposed.

Process debt after M077: comma-lists work, but repeating `--check a --check b` still keeps only the last flag (argparse default). Thin slice: `action=append` then join into existing `_parse_check_ids` (reuse). `L_delivery=M079-closed`.

- [x] **S01: --check action=append** `risk:low` `depends:[]`
  > After this: repeated `--check` flags union; comma form unchanged; matrix `SEM-m079-check-append` killed. Phase 150. Landed.

## M078: Compat migrate operator CLI — CLOSED (GSD v0.72)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.72`. Honeycomb stays Proposed.

Process debt after M076/M077: `migrate_compat` is library-only; operators have no bin entry (unlike `kutha-tenant`). Thin slice: `kutha-migrate` calls existing `store::migrate_compat` on a directory path / `KUTHA_MIGRATE_DIR`. `L_delivery=M078-closed`.

- [x] **S01: kutha-migrate bin → migrate_compat** `risk:low` `depends:[]`
  > After this: one binary runs Op-wire then pins; prints report; matrix `SEM-m078-migrate-cli` killed. Phase 149. Landed.

## M077: Multi-check CLI select — CLOSED (GSD v0.71)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.71`. Honeycomb stays Proposed.

Process debt after M076 retro: `kutha-gov --check` takes a single id; repeating `--check` keeps only the last (argparse), so agents drop intended oracles. Thin slice: comma-separated ids through `_select_checks` / selftest (dictionary lookup; no hardcoded check names). `L_delivery=M077-closed`.

- [x] **S01: comma-separated --check ids** `risk:low` `depends:[]`
  > After this: `--check a,b` runs both; unknown/empty fail-closed; AGENTS + agent-surface updated; matrix `SEM-m077-multi-check` killed. Phase 148. Landed.

## M076: Compat migrate sequence facade — CLOSED (GSD v0.70)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.70`. Honeycomb stays Proposed.

Process debt after M075: pin migrate requires modern Event JSON, but Op-wire and pin migrate stay separate entrypoints; operators can run `migrate_pins` while meta-op envelopes remain. Thin slice: `store::migrate_compat` = `migrate_op_wire` then `migrate_pins` (reuse). `L_delivery=M076-closed`.

- [x] **S01: migrate_compat op-wire then pins** `risk:low` `depends:[]`
  > After this: one entry remaps meta-ops then pins; stamps op-wire + pin markers; matrix `SEM-m076-migrate-compat` killed. Phase 147. Landed.

## M075: Pin migrate sequence facade — CLOSED (GSD v0.69)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.69`. Honeycomb stays Proposed.

Process debt after M072/M074: pin-wire and pin-cite are separate directory rewrites; operators can run cite without wire (or forget the order). Thin slice: `store::migrate_pins` calls existing `migrate_pin_wire` then `migrate_pin_cite` (reuse; no new remap logic). `L_delivery=M075-closed`.

- [x] **S01: migrate_pins wire-then-cite** `risk:low` `depends:[]`
  > After this: one entry remaps Behavior then JustificationCite; stamps both markers; matrix `SEM-m075-migrate-pins` killed. Phase 146. Landed.

## M074: Agent surface discoverability — CLOSED (GSD v0.68)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.68`. Honeycomb stays Proposed.

Process debt after M073 (CE whole-repo agent_native_gaps W1/O1): gsd-auto-contract tool-pair table still labels CBM as `root-kutha-graph` while SKILL.md canonical is `kutha-graph`; AGENTS.md Commands omit live CLI flags (`--jobs`, `--skip-cargo`, `json`, `--check`). Thin slice: docs needles + governor check; no crates. `L_delivery=M074-closed`.

- [x] **S01: CBM name + AGENTS CLI flags** `risk:low` `depends:[]`
  > After this: tool pair prefers `kutha-graph`; AGENTS lists parallel/skip/json/check; matrix `SEM-m074-agent-surface` killed. Phase 145. Landed.

## M073: Git auto clean-tree coupling window — CLOSED (GSD v0.67)

**Closed:** S01 delivered 2026-10-14. Process tag `v0.67`. Honeycomb stays Proposed.

Process debt after M072: on a clean worktree `git_against=auto` falls back to `HEAD~1`, so `docs-coupling` falsely HIGHs when the last commit touched `checks.yaml` without `invariants.yaml`/`bridges.yaml` even though the branch range is coupled. Thin slice: clean-tree auto uses upstream merge-base…HEAD; `ci` summary separates run HIGH from fold tip. `L_delivery=M073-closed`.

- [x] **S01: auto merge-base + ci tip labels** `risk:low` `depends:[]`
  > After this: clean-tree `ci` docs-coupling uses branch window; summary shows `run_high` vs fold `tip_high`; matrix `SEM-m073-git-auto-base` killed. Phase 144. Landed.

## M072: Pin-cite JustificationCite migrate — CLOSED (GSD v0.66)

**Closed:** S01 delivered 2026-10-13. Process tag `v0.66`. Honeycomb stays Proposed.

Process debt after M071: Behavior pins migrate, but JustificationCite `rule_version` can still hold main-era UTF-8 ident digests. Thin slice: explicit `migrate_pin_cite` + `pin-cite.v1`; remap cites when pin equals legacy domain+UTF-8 for a rule ident evidenced in the same log (Behavior.name / rule-ident Define). `L_delivery=M072-closed`.

- [x] **S01: migrate_pin_cite JustificationCite rule_version** `risk:medium` `depends:[]`
  > After this: jsonl/WAL/segmented dirs rewrite legacy cite pins to v1 when name is evidenced; matrix `SEM-m072-pin-cite` killed. Phase 143. Landed.

## M071: Pin-wire Behavior migrate — CLOSED (GSD v0.65)

**Closed:** S01 delivered 2026-10-13. Process tag `v0.65`. Honeycomb stays Proposed.

Process debt after M051/M070: pin-compat documents rotation but no directory rewrite for main-era UTF-8 ident pins on Behavior. Thin slice: explicit `migrate_pin_wire` + `pin-wire.v1`; remap only when name is rule ident and pin equals legacy domain+UTF-8. `L_delivery=M071-closed`.

- [x] **S01: migrate_pin_wire Behavior rule_version** `risk:medium` `depends:[]`
  > After this: jsonl/WAL/segmented dirs rewrite legacy Behavior pins to v1; matrix `SEM-m071-pin-wire` killed. Phase 142. Landed.

## M070: Op-wire demote orphan cleanup — CLOSED (GSD v0.64)

**Closed:** S01 delivered 2026-10-13. Process tag `v0.64`. Honeycomb stays Proposed.

Process debt after M069: segmented demote deletes only manifest-named segments; generation-unique orphan `seg-*.log` can remain. Thin slice: glob-clean `seg-*.log` after demote; correct stale G4 residual cites (#26 ≠ clock facade). `L_delivery=M070-closed`.

- [x] **S01: orphan seg cleanup + G4 doc honesty** `risk:low` `depends:[]`
  > After this: demote leaves no `seg-*.log`; matrix `SEM-m070-op-wire-orphan-seg` killed; op-compat/pin-compat residual text honest. Phase 141. Landed.

## M069: Op wire segmented migrate — CLOSED (GSD v0.63)

**Closed:** S01 delivered 2026-10-13. Process tag `v0.63`. Honeycomb stays Proposed.

Process debt after M068: segmented dirs (`manifest.json`) still rejected by `migrate_op_wire`. Thin slice: recover frame JSON Values from sealed+active segments, remap, materialize `events.jsonl`, drop segments/manifest, stamp `op-wire.v1`. Open stays fail-open. `L_delivery=M069-closed`.

- [x] **S01: segmented migrate_op_wire** `risk:medium` `depends:[]`
  > After this: segmented dirs with meta-op frames rewrite; matrix `SEM-m069-op-wire-seg` killed. Phase 140. Landed. Op-wire layouts complete (jsonl / WAL / segmented).

## M068: Op wire WAL-only migrate — CLOSED (GSD v0.62)

**Closed:** S01 delivered 2026-10-13. Process tag `v0.62`. Honeycomb stays Proposed.

Process debt after M067: WAL-only legacy dirs still deferred. Thin slice: `migrate_op_wire` reads WAL frame payloads as JSON Values, remaps, materializes `events.jsonl`, stamps `op-wire.v1`. Segmented dirs stay deferred. `L_delivery=M068-closed`.

- [x] **S01: WAL-only migrate_op_wire** `risk:medium` `depends:[]`
  > After this: WAL-only dirs with meta-op frames rewrite; matrix `SEM-m068-op-wire-wal` killed. Phase 139. Landed. Segmented dirs deferred.

## M067: Op wire directory migrate — CLOSED (GSD v0.61)

**Closed:** S01 delivered 2026-10-13. Process tag `v0.61`. Honeycomb stays Proposed.

Process debt after M066: five meta-op remappers exist but no directory rewrite. Thin slice: explicit `store::migrate_op_wire` on legacy jsonl dirs, stamp `op-wire.v1`, open stays fail-open. `L_delivery=M067-closed`.

- [x] **S01: migrate_op_wire + format marker** `risk:medium` `depends:[]`
  > After this: legacy jsonl with meta-ops rewrites via remap helpers; marker makes the step idempotent; matrix `SEM-m067-op-wire-dir` killed. Phase 138. Landed. Segmented/WAL-only deferred.

## M066: Op wire migrate (RecordAction) — CLOSED (GSD v0.60)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.60`. Honeycomb stays Proposed.

Process debt after M065: `RecordAction` still deferred. Thin slice remaps legacy action JSON to a single `kutha:action` Assert (same shape as `record_action`). Open stays fail-open. `L_delivery=M066-closed`.

- [x] **S01: RecordAction → kutha:action Assert** `risk:medium` `depends:[]`
  > After this: `record_action_to_assert` via caller intern; matrix `SEM-m066-op-migrate-action` killed. Phase 137. Landed. Directory rewrite stays deferred.

## M065: Op wire migrate (RecordAdmission) — CLOSED (GSD v0.59)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.59`. Honeycomb stays Proposed.

Process debt after M064: `RecordAdmission` still deferred. Thin slice remaps legacy admission JSON to a single `kutha:admission` Assert (same shape as `record_justification`). Open stays fail-open. `L_delivery=M065-closed`.

- [x] **S01: RecordAdmission → kutha:admission Assert** `risk:medium` `depends:[]`
  > After this: `record_admission_to_assert` via caller intern; matrix `SEM-m065-op-migrate-admission` killed. Phase 136. Landed. RecordAction stays deferred.

## M064: Op wire migrate (PinPolicy) — CLOSED (GSD v0.58)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.58`. Honeycomb stays Proposed.

Process debt after M063: `PinPolicy` still deferred. Thin slice expands legacy policy JSON to `StoreBlob` + `kutha:policy` Assert (same shape as `emit_policy`). `L_delivery=M064-closed`.

- [x] **S01: PinPolicy → StoreBlob+Assert** `risk:medium` `depends:[]`
  > After this: `pin_policy_to_ops` via `policy_version_hash`; matrix `SEM-m064-op-migrate-policy` killed. Phase 135. Landed. RecordAdmission/RecordAction stay deferred.

## M063: Op wire migrate (RegisterRule) — CLOSED (GSD v0.57)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.57`. Honeycomb stays Proposed.

Process debt after M062: `RegisterRule` still deferred. Thin slice expands legacy rule JSON to `StoreBlob` + `kutha:rule` Assert (same shape as `emit_rule`). `L_delivery=M063-closed`.

- [x] **S01: RegisterRule → StoreBlob+Assert** `risk:medium` `depends:[]`
  > After this: `remap_legacy_ops` handles RegisterRule via `RuleAst::named`; matrix `SEM-m063-op-migrate-rule` killed. Phase 134. Landed. PinPolicy/RecordAdmission/RecordAction stay deferred.

## M062: Op wire migrate (AllowRelation) — CLOSED (GSD v0.56)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.56`. Honeycomb stays Proposed.

Owner authorized CE #1 follow-on: explicit migrate off main-era dictionary meta-ops. Thin first slice remaps `AllowRelation` only; `open` stays fail-open (no silent serde aliases). `L_delivery=M062-closed`.

- [x] **S01: AllowRelation → Assert remap** `risk:medium` `depends:[]`
  > After this: `kutha_common::op_migrate` remaps legacy AllowRelation JSON via caller intern; m050 fail-open oracles kept; matrix `SEM-m062-op-migrate-allow` killed. Phase 133. Landed. Other four meta-ops stay deferred.

## M061: Live env budget floor — CLOSED (GSD v0.55)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.55`. Honeycomb stays Proposed.

Process debt after M060: `budget_covers_checks` and `budget-template-align` cover fsm/template, but a live `.env` below fsm still truncates only on `ci` (precommit runs the full dictionary). `L_delivery=M061-closed`.

- [x] **S01: env_int_at_least kind** `risk:low` `depends:[]`
  > After this: kind `env_int_at_least`; check `env-budget-floor`; matrix `SEM-m061-env-budget-floor` killed. Phase 132. Landed.

## M060: Budget covers check count — CLOSED (GSD v0.54)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.54`. Honeycomb stays Proposed.

Process debt after M059: template/env alignment did not prove `defaults.budget` covers the dictionary; short budget was only caught by ci truncation or a pytest assert. `L_delivery=M060-closed`.

- [x] **S01: budget_covers_checks kind** `risk:low` `depends:[]`
  > After this: kind `budget_covers_checks`; check `budget-covers-checks`; matrix `SEM-m060-budget-covers` killed. Phase 131. Landed.

## M059: Budget template alignment — CLOSED (GSD v0.53)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.53`. Honeycomb stays Proposed.

Process debt after M058: `.env.example` could lag `fsm.yaml` defaults.budget and silently truncate the check tail under Cui. `L_delivery=M059-closed`.

- [x] **S01: cite_equals fsm budget ↔ .env.example** `risk:low` `depends:[]`
  > After this: check `budget-template-align` via existing `cite_equals`; matrix `SEM-m059-budget-template` killed. Phase 130. Landed.

## M058: Disk-space preflight — CLOSED (GSD v0.52)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.52`. Honeycomb stays Proposed.

Process debt after M057: sessions hit 100% disk mid-wave; governor had no dictionary gate before heavy cargo. `L_delivery=M058-closed`.

- [x] **S01: disk_free kind + check** `risk:low` `depends:[]`
  > After this: kind `disk_free` with YAML `min_free_gib`; check `disk-free`; selftest derives absurd threshold; matrix `SEM-m058-disk-free` killed. Phase 129. Landed (also relocated Phase 65/66 bench markers off gitignored `tmp/`).

## M057: Lease-diff ROADMAP coupling — CLOSED (GSD v0.51)

**Closed:** S01 delivered 2026-10-12. Process tag `v0.51`. Honeycomb stays Proposed.

Process debt after M056: lease-bundle honesty was documented and trajectory-checked, but `docs-coupling` still allowed a STATE+CHANGELOG+README diff without `.kutha/ROADMAP.md`. `L_delivery=M057-closed`.

- [x] **S01: STATE diff implies ROADMAP** `risk:low` `depends:[]`
  > After this: `docs-coupling` HIGH when `.kutha/STATE.md` changes without `.kutha/ROADMAP.md`; matrix `SEM-m057-lease-roadmap` killed. Phase 128. Landed.

## M056: Advisory residual close — CLOSED (GSD v0.50)

**Closed:** S01–S05 delivered 2026-10-12. Process tag `v0.50`. Honeycomb stays Proposed.

Process debt first (lease-bundle vs trajectory), then persist heal #29/#30, then eligibility #26/#28, then hygiene. `L_delivery=M056-closed`.

- [x] **S01: Lease-bundle honesty** `risk:low` `depends:[]`
  > After this: gsd-auto lease commit includes ROADMAP open checkbox + README status so trajectory/state-readme stay green; matrix SEM-m056-lease-bundle killed. Phase 123. Landed.
- [x] **S02: Persist heal residuals** `risk:high` `depends:[S01]`
  > After this: short orphan next-segment header recreated on heal (#29); migrate verify-fail removes manifest then sync_dir (#30). Oracles `m056_persist_heal`. Phase 124. Landed.
- [x] **S03: Delivery-key same-relation** `risk:medium` `depends:[S02]`
  > After this: keyed retry requires same-relation live support (#26). Phase 125. Landed.
- [x] **S04: NormView derived Correct eligibility** `risk:medium` `depends:[S03]`
  > After this: Correct of derived head re-validates replacement vs rule (#28). Phase 126. Landed.
- [x] **S05: Hygiene** `risk:low` `depends:[S04]`
  > After this: VERIFICATION uv-only (#35); statute fixture single-language (#36). Phase 127. Landed.

## M055: Append tip rollback (CE #6) — CLOSED (GSD v0.49)

**Closed:** S01 delivered 2026-10-11. Process tag `v0.49`. Honeycomb stays Proposed.

- [x] **S01: Tip rollback on post-write Other** `risk:high` `depends:[]`
  > After this: post-write Other restores durable memory cursor and leaves crash-shaped suffix; next append rewinds before write (no duplicate EventIds); Other keeps kutha.lock; poison only after committed rollover. Oracles `m055_append_tip_rollback`. Matrix `SEM-gap-persist-poison` killed. Landed: Phase 122.

## M054: CE whole-repo residual gaps — CLOSED (GSD v0.48)

**Closed:** S01–S05 delivered 2026-10-11. Process tag `v0.48`. Honeycomb stays Proposed. #6 closed under M055.

- [x] **S01: Horn soundness** `risk:high` `depends:[]`
  > After this: Correct/CI-caused re-fire eligible; non-trigger retract drops derived; head VT = premise×pin conjunction; full-cover dedup + uncovered residuals; no duplicate derived Retract. Oracles `m054_horn_soundness`. Matrix SEM-m054-horn-* killed. Landed: Phase 117.
- [x] **S02: Persist durability** `risk:high` `depends:[]`
  > After this: torn empty active healed on open_for_append; generation-unique segment names before manifest swap. #6 deferred to M055. Oracles `m054_persist_durability`. Landed: Phase 118.
- [x] **S03: Governor honesty** `risk:medium` `depends:[]`
  > After this: PLAN budget on CONTEXT; matrix schema/rows fail-closed; pool at-most-once. Matrix SEM-m054-gov-* killed. Landed: Phase 119.
- [x] **S04: Scaling and bench honesty** `risk:medium` `depends:[]`
  > After this: RSS sample while dedicated runtime alive; timed as_of validated; retract/Horn/append/pin indexes constant in N. Oracles `m054_scaling_bench`. Matrix SEM-m054-scale-* killed. Landed: Phase 120.
- [x] **S05: ID contract and structure** `risk:medium` `depends:[]`
  > After this: UUID v8 derived EventIds; shrink SHA; SEM + kinds family splits. Oracles `m054_event_id` / `m054_id_structure`. Matrix SEM-m054-id-* killed. Landed: Phase 121.

## M053: CE whole-repo regression enrolment — CLOSED (GSD v0.47)

**Closed:** S01 delivered 2026-10-11. Process tag `v0.47`. Honeycomb stays Proposed.

- [x] **S01: Enrol review regressions and gap map** `risk:low` `depends:[]`
  > After this: `m053_ce_review_regressions` pins advisor withdrawals + five legacy meta-ops; matrix killed rows + M054 gap classes; `ce-review-regression-map.md`. Landed: Phase 116.

## M052: CE whole-repo wave 1 — CLOSED (GSD v0.46)

**Closed:** S01–S04 delivered 2026-10-11. Process tag `v0.46`. Honeycomb stays Proposed. Whole-repo #1 demoted (`advisor-correction.md`).

- [x] **S01: Triage correction** `risk:low` `depends:[]`
  > After this: advisor-correction demotes #1; fix-first starts at #8; Phase 112 closed. Landed: `advisor-correction.md`; `112-VERIFICATION.md`.
- [x] **S02: Cascade atomicity** `risk:high` `depends:[S01]`
  > After this: later-hop rule errors leave fold/log unchanged; `m052_cascade_atomic`; Phase 113. Landed: `SEM-m052-cascade-atomic`; `113-VERIFICATION.md`.
- [x] **S03: Lock before heal** `risk:high` `depends:[S02]`
  > After this: attach locks before heal/write; Phase 114. Landed: `SEM-m052-lock-before-heal`; `114-VERIFICATION.md`.
- [x] **S04: Snapshot-free open replay** `risk:high` `depends:[S03]`
  > After this: all open paths run replay_check; Phase 115. Landed: `SEM-m052-open-replay`; `115-VERIFICATION.md`.

## M051: Pin scheme v1 stability vectors — CLOSED (GSD v0.45)

**Closed:** S01 delivered 2026-10-11. Process tag `v0.45`. Honeycomb stays Proposed. CE review P1 #24.

- [x] **S01: Document pin scheme v1; hardcoded stability vectors** `risk:low` `depends:[]`
  > After this: `docs/process/pin-compat.md` records documented rotation; `m051_pin_compat` oracles; FSM budget 76. UTF-8→v1 Behavior migrate deferred. No Cypher/HNSW. Landed: `pin-compat-policy`; `111-VERIFICATION.md`.

## M050: Op compat documented break — CLOSED (GSD v0.44)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.44`. Honeycomb stays Proposed. CE review P1 #14.

- [x] **S01: Document Op wire break; pin fail-open for legacy meta-ops** `risk:low` `depends:[]`
  > After this: `docs/process/op-compat.md` records documented-break policy; `m050_op_compat` oracles; FSM budget 75. Serde TermId remap deferred. No Cypher/HNSW. Landed: `op-compat-policy`; `110-VERIFICATION.md`.

## M049: skip-cargo log marker — CLOSED (GSD v0.43)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.43`. Honeycomb stays Proposed. CE review P2 #38.

- [x] **S01: skip_cargo emit records tenant=skipped for fold tip** `risk:low` `depends:[]`
  > After this: emit_tenant skip path appends allowlisted `tenant=skipped`; fold tip is not a stale full-evidence tenant; FSM budget 74. RED oracles first. No Cypher/HNSW. Landed: `skip-cargo-log-marker`; `109-VERIFICATION.md`.

## M048: parallel Context flags — CLOSED (GSD v0.42)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.42`. Honeycomb stays Proposed. CE review P2 #48.

- [x] **S01: Worker Context carries skip_emit and changed_paths** `risk:low` `depends:[]`
  > After this: `_eval_check_worker` payload includes both fields; FSM budget 73. RED oracles first. No Cypher/HNSW. Landed: `parallel-context-flags`; `108-VERIFICATION.md`.

## M047: observe-skipped breadcrumb — CLOSED (GSD v0.41)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.41`. Honeycomb stays Proposed. CE review P2 #39.

- [x] **S01: Suppressed observe_cargo leaves LOW observe-skipped** `risk:low` `depends:[]`
  > After this: `_note_observe_skipped` on skip/prior-high/overlap-after-high; observation uses allowlisted `cargo`; FSM budget 72. RED oracles first. No Cypher/HNSW. Landed: `observe-skipped-breadcrumb`; `107-VERIFICATION.md`.

## M046: parallel check-error parity — CLOSED (GSD v0.40)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.40`. Honeycomb stays Proposed. CE review P2 #23.

- [x] **S01: check.run exceptions are check-error under jobs=1 and jobs>1** `risk:low` `depends:[]`
  > After this: `_eval_check_worker` reuses `eval_check`; `worker-error` is transport-only; FSM budget 71. RED oracles first. No Cypher/HNSW. Landed: `parallel-check-error-parity`; `106-VERIFICATION.md`.

## M045: glob_absent comment policy — CLOSED (GSD v0.39)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.39`. Honeycomb stays Proposed. CE review P2 #22.

- [x] **S01: One comment policy with or without unless_line_contains** `risk:low` `depends:[]`
  > After this: `glob_absent` skips `#` and `//` comment lines for needles whether or not `unless_line_contains` is set. RED oracles first. No Cypher/HNSW. Landed: `glob-absent-comment-policy`; `105-VERIFICATION.md`.

## M044: nameless dict retain — CLOSED (GSD v0.38)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.38`. Honeycomb stays Proposed. CE review P2 #20.

- [x] **S01: Nameless check id keeps prior valid checks** `risk:low` `depends:[]`
  > After this: `get_checks` continues past rows without `id`; `LoadErrorCheck` cites row index alongside survivors. RED oracles first. No Cypher/HNSW. Landed: `nameless-dict-retain`; `104-VERIFICATION.md`.

## M043: FSM quantum honest name — CLOSED (GSD v0.37)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.37`. Honeycomb stays Proposed. CE review P1 #17.

- [x] **S01: skip_cargo quantum is scheduling-only; live asserts cargo/tenant/ff5** `risk:low` `depends:[]`
  > After this: pytest skip quantum named `test_ci_quantum_scheduling_reaches_ok_with_skips`; lying name forbidden; live observations via `KUTHA_LIVE_CARGO=1`. RED oracles first. No Cypher/HNSW. Landed: `fsm-quantum-honest-name`; `103-VERIFICATION.md`.

## M042: m028×layouts + shrink corpus — CLOSED (GSD v0.36)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.36`. Honeycomb stays Proposed. ADR-023 Clarification Phase 102.

- [x] **S01: Dict retract on all PersistLayout + corpus roundtrip** `risk:low` `depends:[]`
  > After this: trailing emit_retract_dict revoked after open on Legacy/Segmented/Rollover/Migrated; `save_shrunk_to`/`load_shrunk`. RED oracles first. No Cypher/HNSW. Landed: `m042_m028_layouts`; `102-VERIFICATION.md`.

## M041: persist-trace identity + shrink gate — CLOSED (GSD v0.35)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.35`. Honeycomb stays Proposed. ADR-023 Clarification Phase 101.

- [x] **S01: Open EventId identity across layouts + shrink ddmin** `risk:low` `depends:[]`
  > After this: `expected_open_event_ids` vs layouts; `candidate_deps_hold` + `shrink_events_ddmin`; Rollover seam reject ≠ oracle fail. RED oracles first. No Cypher/HNSW. Landed: `m041_persist_trace_identity`; `101-VERIFICATION.md`.

## M040: vacuous planning gates — CLOSED (GSD v0.34)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.34`. Honeycomb stays Proposed. CE review P1 #16.

- [x] **S01: Live CONTEXT without VERIFICATION is HIGH** `risk:low` `depends:[]`
  > After this: `verification-independent` drops `allow_empty`; content steps gate on CONTEXT via `when_glob_exists`; idle without CONTEXT skips. RED oracles first. No Cypher/HNSW. Landed: `_when_glob_idle`; `100-VERIFICATION.md`.

## M039: stub CI honest name — CLOSED (GSD v0.33)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.33`. Honeycomb stays Proposed. CE review P1 #7.

- [x] **S01: Stub cargo quantum is not live-tree green** `risk:low` `depends:[]`
  > After this: pytest stub quantum named `test_ci_fsm_reaches_ok_with_stubbed_observe`; lying name forbidden; live observe is `kutha-gov ci` or `KUTHA_LIVE_CARGO=1`. RED oracles first. No Cypher/HNSW. Landed: `stub-ci-honest-name`; `99-VERIFICATION.md`.

## M038: policy delivery-key namespace — CLOSED (GSD v0.32)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.32`. Honeycomb stays Proposed. CE review P2 #29.

- [x] **S01: Relation-namespaced delivery-key retry** `risk:high` `depends:[]`
  > After this: `by_delivery_key` keys `(relation, delivery_key)`; identical `emit_policy` after `record_justification` is idempotent; admission keeps policy pin. RED oracles first. No Cypher/HNSW. Landed: `m038_policy_delivery_namespace.rs`; ADR-050 Clarification Phase 98; `98-VERIFICATION.md`.

## M037: norm_view claim-lineage eligibility — CLOSED (GSD v0.31)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.31`. Honeycomb stays Proposed. CE review P1 #13.

- [x] **S01: Claim-lineage eligibility in norm_view** `risk:high` `depends:[]`
  > After this: `norm_view` gates on `event_by_id(claim_id) is Op::Behavior`, not minting event type; Correct/CI of ineligible derived stay Abstain. RED oracles first. No Cypher/HNSW. Landed: `m037_norm_view_claim_lineage.rs`; ADR-013 Clarification Phase 97; `97-VERIFICATION.md`.

## M036: VT-aware Horn dedup — CLOSED (GSD v0.30)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.30`. Honeycomb stays Proposed. CE review P1 #11.

- [x] **S01: VT-aware Horn live-triple dedup** `risk:high` `depends:[]`
  > After this: `has_live_triple*` checks `vt_intersect` overlap; disjoint VT windows mint separate derived inverses. RED oracles first. No Cypher/HNSW. Landed: `m036_vt_aware_dedup.rs`; ADR-013 Clarification Phase 96; `96-VERIFICATION.md`.

## M035: Replay dict bind — CLOSED (GSD v0.29)

**Closed:** S01 delivered 2026-10-10. Process tag `v0.29`. Honeycomb stays Proposed. CE review P1 #10.

- [x] **S01: Query-ready replay binds reserved dict** `risk:high` `depends:[]`
  > After this: `replay_with_dict` + `replay_check` call `bind_reserved`; bare `replay` stays fingerprint-only. RED oracles first. No Cypher/HNSW. Landed: `m035_replay_dict_bind.rs`; ADR-060 Clarification Phase 95; `95-VERIFICATION.md`.

## M034: Correct Horn re-fire — CLOSED (GSD v0.28)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.28`. Honeycomb stays Proposed. CE review P1 #9.

- [x] **S01: Correct follow-ons except-set + fire order** `risk:high` `depends:[]`
  > After this: `Op::Correct` mirrors CorrectInterval (`horn_fires_except`, fires-then-retracts); same-triple Correct keeps derived inverse. RED oracles first. No Cypher/HNSW. Landed: `m034_correct_refire.rs`; ADR-013 Clarification Phase 94; `94-VERIFICATION.md`.

## M033: CI admission VT — CLOSED (GSD v0.27)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.27`. Honeycomb stays Proposed. CE review P1 #8.

- [x] **S01: Admission any CI residual at row.vt** `risk:high` `depends:[]`
  > After this: `check_admission_row` uses `live_fact_indices_by_event_id` so patch/suffix VT admits; retracted CI stays `stale_support`. RED oracles first. No Cypher/HNSW. Landed: `m033_ci_admission_vt.rs`; ADR-013 Clarification Phase 93; `93-VERIFICATION.md`.

## M032: Derive live premise — CLOSED (GSD v0.26)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.26`. Honeycomb stays Proposed. CE review P1 #4.

- [x] **S01: Filter invalidated premises in derive()** `risk:high` `depends:[]`
  > After this: `derive()` premise find requires `invalidated_at.is_none()` (same as retract/correct); sole retract → UnknownFact; later retract leaves prior live Assert as caused_by. RED oracles first. No Cypher/HNSW. Landed: `m032_derive_live_premise.rs`; ADR-013 Clarification Phase 92; `92-VERIFICATION.md`.

## M031: WAL-tail heal — CLOSED (GSD v0.25)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.25`. Honeycomb stays Proposed. CE review #30.

- [x] **S01: Heal jsonl-ahead-of-WAL on open_for_append** `risk:high` `depends:[]`
  > After this: Jsonl failpoint (covering jsonl, short WAL, dropped==0) no longer wedges attach; heal rewrites via `persist_unlocked` then attaches. RED oracles first. No Cypher/HNSW. Landed: `legacy_attach_needs_rewrite`; `m031_wal_tail_heal.rs`; ADR-010 Clarification Phase 91; `91-VERIFICATION.md`.

## M030: Writer lock retention — CLOSED (GSD v0.24)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.24`. Honeycomb stays Proposed. CE review P1 #2.

- [x] **S01: Retain kutha.lock on non-poison errors** `risk:high` `depends:[]`
  > After this: `ErrorKind::Other` append/checkpoint errors keep `kutha.lock` while the `Persister` lives; fail-closed `InvalidData` still poisons and releases. RED oracles first. No Cypher/HNSW. Landed: `release_lock_if_poisoned`; `m030_writer_lock.rs`; heal tests drop crashed writer; ADR-010 Clarification Phase 90; `90-VERIFICATION.md`.

## M029: Interrupted rollover seal heal — CLOSED (GSD v0.23)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.23`. Honeycomb stays Proposed. CE review P1 #3.

- [x] **S01: Finish seal trailer on attach** `risk:high` `depends:[]`
  > After this: crash after rollover seal-trailer sync no longer wedges `open_for_append`/`attach`; heal promotes the sealed active and opens an empty next segment. RED oracles first. No Cypher/HNSW. Landed: `heal_interrupted_active_seal`; `m029_rollover_heal.rs`; ADR-012 Clarification Phase 89; `89-VERIFICATION.md`.

## M028: Dictionary retract on open — CLOSED (GSD v0.22)

**Closed:** S01 delivered 2026-10-09. Process tag `v0.22`. Honeycomb stays Proposed. CE review P0 #1.

- [x] **S01: Trailing dict Retract SelfCommit** `risk:high` `depends:[]`
  > After this: Retract of a `kutha:*` Assert is SelfCommit in `committed_prefix`; persist→open keeps trailing `emit_retract_dict`; revoked allow stays revoked. RED oracles first. No Cypher/HNSW. Landed: `m028_dict_retract.rs`; ADR-014 Clarification Phase 88; `88-VERIFICATION.md`.

## M027: Trajectory honesty — CLOSED (GSD v0.21)

**Closed:** 2026-10-09. Process tag `v0.21`. S01 complete. Audit: `.planning/milestones/v0.21-MILESTONE-AUDIT.md`. Honeycomb stays Proposed. No `crates/` change.

- [x] **S01: Characterization kills + idle ACTIVE** `risk:low` `depends:[]`
  > After this: `killed` rows whose `kill_mutation` is characterization-shaped (mirror/alias/honor-system/env-floor) are HIGH; ROADMAP must not mark a heading active while Active Milestone is None; Later-list active token matches STATE. Reuse `test_matrix` / `glob_absent` / `cite_equals` (no new Python class).

## M026: Test strength — CLOSED (GSD v0.20)

**Closed:** 2026-10-09. Source: `.compound-engineering/artifacts/plans/2026-10-08-post-m022-roadmap-for-gsd-auto.md`. Process tag `v0.20`. S01–S02 complete. Audit: `.planning/milestones/v0.20-MILESTONE-AUDIT.md`. Honeycomb stays Proposed. No product behavior change unless an oracle exposes a bug.

- [x] **S01: Generators and missing oracles** `risk:medium` `depends:[]`
  > After this: `ingest_mix` Coverage reaches Retract-before-Assert, Correct/Retract of derived Behavior ids, overlapping Asserts, keyed retry, empty log; oracles G-C04/C05/C11; every new oracle has a kill_mutation.
- [x] **S02: Matrix honesty** `risk:low` `depends:[S01]`
  > After this: honor-system and tautology rows re-labelled or given real mutants; optional cargo-mutants note is verifier evidence, not a gate.

## M025: Wedge answer surface — CLOSED (GSD v0.19)

**Closed:** 2026-10-09. Source: `.compound-engineering/artifacts/plans/2026-10-08-post-m022-roadmap-for-gsd-auto.md`. Process tag `v0.19`. S01–S04 complete. Audit: `.planning/milestones/v0.19-MILESTONE-AUDIT.md`. Honeycomb stays Proposed. Not Cypher/HNSW; not ADR-090 pack thaw.

- [x] **S01: `as_known_on` VT decision** `risk:medium` `gate:controller decision` `depends:[]`
  > After this: ADR-013 states `as_known_on` is TT-only by design (G-A08/G-C03); pin oracles `m025_as_known_on_vt`; `semantic-contract-validation.md` agrees.
- [x] **S02: Clock-safe facade** `risk:high` `depends:[S01]`
  > After this: `as_of` takes `ValidTime` and `as_known_on` takes `WallTime` at public call sites; bare `u64` and wrong-role clocks fail compile (trybuild `ui_clock_facade`); `Op` storage stays `u64`; no fifth axis; bench ≤10%.
- [x] **S03: Legal answer envelope** `risk:high` `depends:[S02]`
  > After this: `norm_view(vt, wall)` returns triples with eligibility and typed non-success; live ∩ eligible is the facade's job; oracles include retracted-premise and missing-anchor.
- [x] **S04: Demo binary and real-text fixtures** `risk:medium` `depends:[S03]`
  > After this: thin bin answers "norm in force on D, as known on W" over fixture ops; 2–3 real-text statute fixtures as test data only; `I-legal-pack-not-next` stays.

## M024: Fold identity and write semantics — CLOSED (GSD v0.18)

**Closed:** 2026-10-08. Source: `.compound-engineering/artifacts/plans/2026-10-08-post-m022-roadmap-for-gsd-auto.md`. Process tag `v0.18`. S01–S02 complete. Audit: `.planning/milestones/v0.18-MILESTONE-AUDIT.md`. Honeycomb stays Proposed. Not Cypher/HNSW.

- [x] **S01: Fold identity** `risk:high` `depends:[]`
  > After this: live-gated Retract/Correct; Correct multi-residual → one replacement; CorrectInterval multi fail-closed; ADR-013 Clarification; oracles `m024_fold_identity`; bench ≤10%.
- [x] **S02: Write-path atomicity** `risk:medium` `depends:[S01]`
  > After this: intern Define crash window characterized (SelfCommit); D014-1 digest `[u8;32]` pinned; event_ids width leak + GROUP-01 residual; ADR-014 Clarification; oracles `m024_write_path`.

## M023: Green HEAD and truth reset — CLOSED (GSD v0.17)

**Closed:** 2026-10-08. Plan: `.compound-engineering/artifacts/plans/2026-10-08-post-m022-roadmap-for-gsd-auto.md`. Process tag `v0.17`. S01–S04 complete. Audit: `.planning/milestones/v0.17-MILESTONE-AUDIT.md`. Honeycomb stays Proposed. Corrected false M022 Phase 72 verified claim.

- [x] **S01: Restore green** `risk:medium` `depends:[]`
  > After this: `delivery_key_retry` follows Correct/CorrectInterval replacements (live keyed support of the same claim short-circuits); m013 keyed-correct pair and `m022_keyed_retry` green; new oracle for keyed retry after Retract of a CorrectInterval id; naive model Retract invalidates all live residuals; `m019_dur_a` flakes isolated; `cargo test --workspace --no-fail-fast` zero failures; bench gate <=10%.
- [x] **S02: Close-gate honesty** `risk:low` `depends:[S01]`
  > After this: 72-VERIFICATION, v0.16 audit and CHANGELOG state the regression and fix; gsd-auto-contract close checklist requires a recorded full `cargo test --workspace --no-fail-fast` and full `ci`; a governor row proves it. No crates change.
- [x] **S03: Living overlay truth** `risk:low` `depends:[S02]`
  > After this: PROJECT, REQUIREMENTS, MILESTONES, this ROADMAP numbering, WINDOWS, test-matrix.md counts, legal-fixture-recon, semantic-gap-review, semantic-contract-validation, ADR-000/010/011 stale lines, p0-spike-inventory and honeycomb evidence match HEAD; ADRs stay Proposed. Docs/YAML only.
- [x] **S04: Close** `risk:low` `depends:[S03]`
  > After this: M023 closed; Active None; audit note; full `ci` and workspace tests recorded.

## M022: Audit residual honesty — CLOSED (GSD v0.16)

**Closed:** 2026-10-08. Plan: `.compound-engineering/artifacts/plans/2026-10-08-m022-audit-residuals.md`. Process tag `v0.16`. S01–S04 complete. Audit: `.planning/milestones/v0.16-MILESTONE-AUDIT.md`. Honeycomb stays Proposed.

- [x] **S01: Audit honesty** `risk:low` `depends:[]`
  > After this: v0.14 audit strikes Retract CI residual; triage table landed.
- [x] **S02: Keyed retry live** `risk:high` `depends:[S01]`
  > After this: delivery_key_retry requires live fact; RED first; bench ≤10%; admission EventId pin.
- [x] **S03: Harness emit isolate** `risk:medium` `depends:[S02]`
  > After this: Context.skip_emit; observe without overlap_with at jobs>1 is HIGH.
- [x] **S04: Close** `risk:low` `depends:[S03]`
  > After this: M022 closed; Active None; audit note.

## M021: Wedge decisions — CLOSED (GSD v0.15)

**Closed:** 2026-10-07. Plan: `.compound-engineering/artifacts/plans/2026-10-07-m021-wedge-decisions.md`. Process tag `v0.15`. S01–S03 complete; independent VERIFICATIONs 68–70. Audit: `.planning/milestones/v0.15-MILESTONE-AUDIT.md`. Honeycomb stays Proposed. Not Cypher/HNSW.

- [x] **S01: Wedge decisions** `risk:medium` `depends:[]`
  > After this: TR-02 three-moments pin; C01 appliesTo Assert spike; SEM-c01/horn-kc/asof/tr02 `killed`; ADR-013/090 Evidence; fold unchanged. Landed: `m021_wedge_decisions.rs`; ADR Evidence 2026-10-07; matrix killed; independent `68-VERIFICATION.md`.
- [x] **S02: Retract all live** `risk:high` `depends:[S01]`
  > After this: Retract(event_id) clears all live residuals for that id; RED first; bench ≤10%. Landed: fold Retract-all-live; `m021_retract_all_live.rs`; bench gate PASS; `69-VERIFICATION.md`.
- [x] **S03: Close** `risk:low` `depends:[S02]`
  > After this: M021 closed; Active None; audit note.

## Later milestones (not active)

Do not start until STATE names them. M018–M042 CLOSED. One lease at a time. M003+ stay unleased.

Proposed order (long horizon, non-authoritative; sequence not calendar; re-derive each step with `/gsd-new-milestone` when leased). Ids are given only to the nearest steps; later waypoints stay unnumbered until a lease creates them (inflation guard).

1. **M011** — CLOSED (S01–S08). Historical record above.
2. **M012a** — CLOSED (S01–S06). Detail above.
3. **M012** — CLOSED (S01–S05). Detail above.
4. **M013** — CLOSED (S01–S09, GSD v0.06 tagged). Detail above.
5. **M014** — CLOSED. Typed clocks and as-known-on. Detail above.
6. **M015** — CLOSED. Known-on honesty and M014 residuals (v0.08). Detail above.
7. **M016** — CLOSED (S01–S05, GSD v0.09 tagged). Detail above.
8. **M017** — CLOSED (S01–S04, GSD v0.10 tagged). Detail above.
9. **M018** — CLOSED (S01–S04, GSD v0.11). Detail above.
10. **M002** — CLOSED (S01–S07, GSD v0.12). Detail above. Vacuum policy and litigation hold stay in this horizon, not as the milestone identity. **Intake (2026-10-02):** cloud kernel writeback / dirty-throttle stalls — `docs/architecture/m002-cloud-kernel-io-intake.md` (arXiv:2609.12597 / SteelDB companion).
11. **M019** — CLOSED (S01–S07, GSD v0.13). Detail above.
12. **M020** — CLOSED (S01–S05, GSD v0.14). Detail above.
13. **M021** — CLOSED (S01–S03, GSD v0.15). Detail above.
14. **M022** — CLOSED (S01–S04, GSD v0.16). Detail above.
15. **M023** — CLOSED (S01–S04, GSD v0.17). Detail above.
16. **M024** — CLOSED (S01–S02, GSD v0.18). Detail above.
17. **M025** — CLOSED (S01–S04, GSD v0.19). Detail above.
18. **M026** — CLOSED (S01–S02, GSD v0.20). Detail above.
19. **M027** — CLOSED (S01, GSD v0.21). Detail above.
20. **M028** — CLOSED (S01, GSD v0.22). Detail above.
21. **M029** — CLOSED (S01, GSD v0.23). Detail above.
22. **M030** — CLOSED (S01, GSD v0.24). Detail above.
23. **M031** — CLOSED (S01, GSD v0.25). Detail above.
24. **M032** — CLOSED (S01, GSD v0.26). Detail above.
25. **M033** — CLOSED (S01, GSD v0.27). Detail above.
26. **M034** — CLOSED (S01, GSD v0.28). Detail above.
27. **M035** — CLOSED (S01, GSD v0.29). Detail above.
28. **M036** — CLOSED (S01, GSD v0.30). Detail above.
29. **M037** — CLOSED (S01, GSD v0.31). Detail above.
30. **M038** — CLOSED (S01, GSD v0.32). Detail above.
31. **M039** — CLOSED (S01, GSD v0.33). Detail above.
32. **M040** — CLOSED (S01, GSD v0.34). Detail above.
33. **M041** — CLOSED (S01, GSD v0.35). Detail above.
34. **M042** — CLOSED (S01, GSD v0.36). Detail above.
35. **M003** — Cypher skin over the already-correct AS OF cut (thaw: M003 leased).
17. **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
18. *(unnumbered)* Pack lifecycle and Cui allocation — only after two verticals run (rule of three).
16. **M005** — HNSW fence as retrieve-not-truth; exact AS OF must not be fakeable by ANN.
17. *(unnumbered)* Security — ABAC rewrite, grants @T, agent sandbox; required before any enterprise ship.
18. *(unnumbered)* Packaging, naming and license decision (ADR-092) before any publish.

P4 (Raft/multi-node, learned indexes) is not planned until single-node hybrid evidence exists.

## Thaw map

The per-item thaw map lives in `.kutha/STATE.md` § Thaw map. M002 is thawed; the other frozen items keep their own named gates. M013 thaw was scoped and freeze restored; do not treat it as a precedent.
