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

## M023: Green HEAD and truth reset — ACTIVE (GSD v0.17)

**Source:** 2026-10-08 post-M022 audits (`.compound-engineering/artifacts/research/2026-10-08-post-m022-synthesis-context.md`). Plan: `.compound-engineering/artifacts/plans/2026-10-08-post-m022-roadmap-for-gsd-auto.md`. Process tag `v0.17`. HEAD `18151af` fails `cargo test --workspace`: two M022 keyed-retry regressions, one stale naive model after M021, three parallel-flaky `m019_dur_a` tests. M022 "verified" is false until this milestone closes. Honeycomb stays Proposed. Not Cypher/HNSW; no new capability.

- [x] **S01: Restore green** `risk:medium` `depends:[]`
  > After this: `delivery_key_retry` follows Correct/CorrectInterval replacements (live keyed support of the same claim short-circuits); m013 keyed-correct pair and `m022_keyed_retry` green; new oracle for keyed retry after Retract of a CorrectInterval id; naive model Retract invalidates all live residuals; `m019_dur_a` flakes isolated; `cargo test --workspace --no-fail-fast` zero failures; bench gate <=10%.
- [x] **S02: Close-gate honesty** `risk:low` `depends:[S01]`
  > After this: 72-VERIFICATION, v0.16 audit and CHANGELOG state the regression and fix; gsd-auto-contract close checklist requires a recorded full `cargo test --workspace --no-fail-fast` and full `ci`; a governor row proves it. No crates change.
- [x] **S03: Living overlay truth** `risk:low` `depends:[S02]`
  > After this: PROJECT, REQUIREMENTS, MILESTONES, this ROADMAP numbering, WINDOWS, test-matrix.md counts, legal-fixture-recon, semantic-gap-review, semantic-contract-validation, ADR-000/010/011 stale lines, p0-spike-inventory and honeycomb evidence match HEAD; ADRs stay Proposed. Docs/YAML only.
- [ ] **S04: Close** `risk:low` `depends:[S03]`
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

Do not start until STATE names them. M018–M022 CLOSED; **M023 active**. One lease at a time. M003+ stay unleased.

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
15. **M023** — ACTIVE (S01–S04, GSD v0.17). Detail above.
16. **M003** — Cypher skin over the already-correct AS OF cut (thaw: M003 leased).
17. **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
18. *(unnumbered)* Pack lifecycle and Cui allocation — only after two verticals run (rule of three).
16. **M005** — HNSW fence as retrieve-not-truth; exact AS OF must not be fakeable by ANN.
17. *(unnumbered)* Security — ABAC rewrite, grants @T, agent sandbox; required before any enterprise ship.
18. *(unnumbered)* Packaging, naming and license decision (ADR-092) before any publish.

P4 (Raft/multi-node, learned indexes) is not planned until single-node hybrid evidence exists.

## Thaw map

The per-item thaw map lives in `.kutha/STATE.md` § Thaw map. M002 is thawed; the other frozen items keep their own named gates. M013 thaw was scoped and freeze restored; do not treat it as a precedent.
