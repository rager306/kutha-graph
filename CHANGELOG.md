# Changelog

All notable changes to **this repository** are recorded here. This is project history, not GitHub Releases and not a Compound Engineering skill catalog.

Keep **product** (`crates/`) and **process** (harness) distinct. Dated entries may also use a **Trajectory** subsection so L_map / L_delivery / L_capability are not collapsed. New entries prefer Keep a Changelog groups (`Added` / `Changed` / `Fixed`) inside those plane headings.

## 2026-10-03 — Product: SNAP-01 atomic sidecars and fail-closed WAL open

### Product

### Fixed

- `persist` writes `terms.jsonl` / `quantum_outcomes.jsonl` / `justifications.jsonl` through tmp+`sync_all`+rename; parent-directory fsync after each rename and at end of persist. Unparseable, mid-line, or proper-prefix sidecars are treated as absent; well-formed non-prefix still fails `InvalidData`.
- Open-path `check_tt_anchors_range` allows log `TtAnchor`s with `ingested_at >= snap.next_tt`; `replay_check` stays exact. `recover_events` returns `InvalidData` ("truncated WAL header") when shorter than magic+version; `load_events_d04` does not mask non-`InvalidData` WAL I/O. Crash-matrix failpoint seam `persist_with_failpoint`. Oracles: `crates/kutha-runtime/tests/m002_snap01.rs`.

### Trajectory

- ADR-012/014/060 dated Evidence 2026-10-03 Phase 50 (SNAP-01 closed in evidence; incremental append S04; COMMIT-01 S05). `semantic-gap-review.md` F1/F6 status. `honeycomb.yaml` restaged (capability and `freeze_as` unchanged). Cells stay **Proposed**.
- Bench gate vs `7e31784` (`KUTHA_BENCH_N=10000`): ingest_plain.n +15.9% / +23.7% — **STOP**; as_of.n +2.5% / +11.3%. Controller must not close on this gate.

## 2026-10-03 — Process: lease M002 S02 (GSD Phase 50)

### Trajectory

- Active Milestone **M002**, Active Slice **S02** (`L_delivery=M002-S01-done`): SNAP-01, OPEN-01, CRASH-01. Product code in `crates/kutha-runtime` only; RocksDB stays banned until Phase 51; honeycomb stays Proposed.

## 2026-10-03 — Process: thaw map and freeze governor honesty (M002 S01)

### Process

### Changed

- Governor: check `freeze` is split into `freeze-rocks` (Phase 51 / S03), `freeze-cypher-deps` (M003) and `freeze-hnsw-dep` (M005); each scans `crates/*/Cargo.toml` and the workspace root `Cargo.toml`; no message claims to read STATE; the tautological message check is gone; `bridges.yaml` has three rows; pytest and selftest cover each id. YAML only, no kernel change (the `unless:` extension is Phase 51's first task).
- Docs: `AGENTS.md`, `README.md`, `PROJECT.md`, `kutha-harness.md`, `governor-intake.md`, `gsd-auto-contract.md`, codebase maps and ADR-000/010/012/014/050/060/070 point to `.kutha/STATE.md` § Thaw map instead of "freeze until M002 lease"; ADR-010/012/014/060 Evidence schedules SNAP-01, fail-closed WAL open, incremental append, COMMIT-01 and the Rocks index lease on M002 S02–S07; ADR-010 and ADR-012 also record the O(N) `persist` rewrite as the current defect. `honeycomb.yaml` restaged (capability and `freeze_as` unchanged).

### Trajectory

- No `crates/` change. **M002 S01 closed** (`L_delivery=M002-S01-done`, Active Slice **None**); Phase 49 verified independently (11/11) after two reopens (CHANGELOG duplicate and overclaim, harness doc omitted the workspace `Cargo.toml`, authorship wording, graphiti/falkordb test). Honeycomb stays Proposed.

## 2026-10-03 — Process: thaw M002 and plan GSD v0.12; lease S01

### Process

### Added

- Harness milestone **M002** thawed and planned as GSD **v0.12** (Phases 49–55, slices S01–S07: thaw map and governor honesty; SNAP-01 with fail-closed WAL open; RocksDB build spike; incremental framed append; commit-marker decision; segments, manifest and hash chain; droppable RocksDB index lease). Requirements in `.planning/REQUIREMENTS.md`; Phase 49 CONTEXT under `.planning/phases/49-thaw-map-and-freeze-governor-honesty/`. Re-verification at `f4af544` (provenance, governor freeze map, durability design research) found that the freeze list and its "until explicit M002 lease" gate came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored), which the owner says he did not decide, and not from ADR-000 D1–D10; the real defects are the O(N) `persist` rewrite, in-place sidecar writes, missing parent-directory fsync, exact-equality `check_tt_anchors`, and fail-open WAL magic reads.

### Changed

- `.kutha/STATE.md` § Freeze becomes § Thaw map: RocksDB crate thaws at Phase 51; Cypher (M003), HNSW (M005), six dictionaries (SIX-01), ADR-080/081, ADR-090, ADR-093 and Consensus 103+ keep their own gates. M018 is paused after S01 at the S02 decision gate; OPEN-01 and the crash seam moved from M018 S04 to M002 S02.

### Trajectory

- Active Milestone **M002**, Active Slice **S01** (`L_delivery=M018-S01-done`). No `crates/` change in S01; the `rocksdb` dependency stays banned by the governor until Phase 51. Honeycomb stays Proposed.

## 2026-10-03 — Process: doc truth round 2 (M018 S01)

### Process

### Changed

- ADR-010/012/013/014/041/050/060 and `docs/ADR/README.md` carry dated 2026-10-03 clarifications: `snapshot.json` and `events.wal` replace via tmp+sync+rename, `events.jsonl` is SoT with `KUTHAWAL` as covering lease, `replay_check` compares `TtAnchor` (`check_tt_anchors`), `live_horns` is lease-first (`horns_by_premise`); sidecars stay in-place `File::create` (SNAP-01, M002). ADR-013 states `as_of` is support-live (derivation eligibility is `derivation_eligible_at`), Horn fires at `TT=MAX`, `Correct` does not re-derive, and a missing `event_id` target is a silent no-op; none of these is decided. `honeycomb.yaml` restaged (capability unchanged).
- `semantic-gap-review.md`, `p0-spike-inventory.md`, `semantic-contract-validation.md`, `PROJECT.md`, `.kutha/ROADMAP.md`, `gsd-auto-contract.md`: v0.10 reality, audit ids `A-F01`–`A-F14` separate from gap-review `F1`–`F16`, A-F05 stays open, one freeze table (STATE), applicability card count 164 (163 matrix + M002 intake card).
- `ADR-000` and `docs/architecture/stca-guide.md` translated to English (no decision text changed; ADR-000 gains a D3 pointer and a D6 clarification).
- `.gitignore` ignores crate scratch and vendored agent trees (`crates/**/tmp/`, `.gsd/`, `.claude/skills/`, `.cursor/skills/gsd-*/`, GSD install state, hooks, agents).

### Trajectory

- No `crates/` change. Honeycomb cells stay Proposed. **M018 S01 closed** (`L_delivery=M018-S01-done`, Active Slice **None**); Phase 45 verified independently 5/5 after one reopen (stale F1/F6 claims in `semantic-gap-review.md` marked historical with a current status). S02 is a decision gate and is not leased.

## 2026-10-03 — Process: plan M018 (GSD v0.11) and lease S01

### Process

### Added

- Harness milestone **M018** "Wedge unblock" (GSD v0.11, Phases 45–48) from the 2026-10-03 post-v0.10 review (five read-only audits at `d230ab2`): S01 doc truth round 2, S02 C01 and knowledge-cut decision (decision gate), S03 Horn determinism and closure, S04 fail-closed open and differential tests. Requirements in `.planning/REQUIREMENTS.md`; Phase 45 CONTEXT under `.planning/phases/45-doc-truth-round-2/`. M002 durability moves to the v0.12 slot, still frozen.

### Trajectory

- **M017 is CLOSED** (tag `v0.10`); the product S04 entries below are historical and carry no live lease. Active Milestone **M018**, Active Slice **S01** (documents only, no `crates/` change). Audit ids F01–F14 of 2026-10-02 are written `A-F01`–`A-F14` from here on; older entries keep the original spelling. Honeycomb stays Proposed.

## 2026-10-03 — Product: Horn premise-relation ingest lease (M017 S04)

### Product

### Fixed

- Droppable `GraphFold` lease `horns_by_premise` (premise relation name → live Horn slots, `serde skip`, not fingerprint). Indexed on `kutha:rule` push and dropped on retract, including Lazy, like `allows_by_name`. `horn_fires` looks up the asserted relation instead of scanning `facts()` in `live_horns`. Extra `relatedTo` ingest no longer pays O(N) Horn visits. No new crate or public API. Same-host Eager `KUTHA_BENCH_N=10000`: `ingest_plain.n` 111283177 / 105460301 ns vs 44-01 v0.08 115322845 (within 10%; `as_of.n` 401528 / 290851).

### Trajectory

- Lease and honeycomb cells unchanged. Active Milestone **M017**, Active Slice **S04**.

## 2026-10-03 — Product: ingest Horn-visit RED oracle (M017 S04)

### Product

### Added

- `GraphFold` `horn_examine` counter (`serde skip`, not fingerprint) with `reset_horn_examine_count` / `horn_examine_count`. `live_horns` and `search_at` bump once per row. Integration oracle `extra_relatedto_horn_visits_constant_in_n` in `crates/kutha-runtime/tests/m017_ingest_scaling.rs` (RED on HEAD: visits grow with N). No ingest-path repair.

### Trajectory

- Lease and honeycomb cells unchanged. Active Milestone **M017**, Active Slice **S04**.

## 2026-10-03 — Product: KNOWN-04 fixture hygiene

### Product

### Changed

- Shared `seed_statute_between_anchors` in `crates/kutha-runtime/tests/common/mod.rs` (m014 and m016 known-on hardening keep every assertion). FIXG-01 drops `C01.contains` quote-membership tautology; `RuleTerm::constant` none and VT vs known-on remain. `leased_pin_and_derive` not merged (arity and live_count asserts diverge).

## 2026-10-03 — Product: KNOWN-04 Horn vs as_known_on (M017 S03)

### Product

### Added

- Characterization `known04_horn_fires_when_premise_hidden_on_early_cut` in `crates/kutha-runtime/tests/m017_known_on_horn.rs`: two-premise Horn mints derived `relatedTo(A,C)` at `TT = MAX` while `as_known_on(2014)` hides A–B and the derived row; `as_known_on(2020)` shows A–B. No eval or known-on behaviour change.

### Trajectory

- ADR-013 Evidence (2026-10-03) records current F05 behaviour; decision pending. Honeycomb restage comment and evidence name; cell stays **Proposed**. Active Milestone **M017**, Active Slice **S03**.

## 2026-10-03 — Product: DET-01 seeded determinism (M017 S03)

### Product

### Added

- Seeded LCG properties in `crates/kutha-runtime/tests/m017_determinism.rs`: all listed seeds (`1, 7, 13, 42, 99, 12345, 0xA5A5_A5A5, 0xC0FFEE`, N=24) prove `replay(log).fingerprint == fold.fingerprint`, Eager vs Lazy equality on sorted `as_of` / `as_known_on` / `live_at` triples (not fingerprints), persist+open round trip under `tmp/p43-01-*`, and identical-log delivery-key retry. No new Cargo test crates.

### Trajectory

- Lease and honeycomb cells unchanged. Active Milestone **M017**, Active Slice **S03**.

## 2026-10-03 — Product: replay integrity (M017 S02)

### Product

### Fixed

- `open` prefers a covering WAL, falls back to intact `events.jsonl`, and fails closed when event ids diverge. Tampered persisted `TtAnchor` walls fail `replay_check` / `open`. `snapshot.json` and `events.wal` replace via temp + sync + rename. A present outcomes or justifications sidecar must equal log rows; an absent sidecar stays Ok. Snapshot JSON pins `FoldIndexMode` (missing field → Eager).

### Changed

- `Runtime::as_known_on` rustdoc is TT-only (`GraphFold::known_at`), not `live_at(tt, MAX)`.

### Trajectory

- ADR-010 Evidence (2026-10-03) records the D-04 open policy. Honeycomb comment restage only; cell stays **Proposed**. Active Milestone **M017**, Active Slice **S02**.

## 2026-10-03 — Trajectory: M017 CLOSED (v0.10 archived)

### Trajectory

- Archive Phases 41–44 under `milestones/v0.10-*`; `L_delivery=M017-closed`, Active Milestone **None**. Process tag `v0.10`; crate SemVer stays 0.0.0. Ingest regression from M016 repaired (+6.3% vs v0.08). Bench gate at product close added to the GSD auto contract. Carried residuals: `v0.10-MILESTONE-AUDIT.md`.

## 2026-10-03 — Process: M017 S04 leased (ingest regression)

### Process

- Lease M017-S04 for GSD Phase 44 (PERF-01, PERF-02).

### Trajectory

- Active Milestone **M017**, Active Slice **S04**. `L_delivery=M017-S03-done`.

## 2026-10-03 — Trajectory: M017 S03 CLOSED (Phase 43 verified)

### Trajectory

- M017 S03 closed (`L_delivery=M017-S03-done`), Active Slice **None**; independent kill-tests `m017_verify43_killtests.rs` pass. Next: Phase 44 (ingest regression repair).

## 2026-10-03 — Process: M017 S03 leased (determinism)

### Process

- Lease M017-S03 for GSD Phase 43 (DET-01, KNOWN-04). Phase 44 unleased.

### Trajectory

- Active Milestone **M017**, Active Slice **S03**. `L_delivery=M017-S02-done`.

## 2026-10-03 — Trajectory: M017 S02 CLOSED (Phase 42 verified)

### Trajectory

- M017 S02 closed (`L_delivery=M017-S02-done`), Active Slice **None**. Phase 44 (ingest regression repair, S04) authored; Phase 43 next.

## 2026-10-03 — Product: M017 S02 verification kill-tests and ingest finding

### Product

- Independent kill-tests `crates/kutha-runtime/tests/m017_verify_killtests.rs` (anchor tamper, truncated WAL, id gap, spurious sidecar, env flip, abort-before-rename, closed-VT known-on) all pass against Phase 42.

### Trajectory

- Same-host bench bisect: ingest_plain.n 106M (v0.06), 108M (v0.08), 301M (post-M016): M016 tripled ingest, Phase 42 does not move it. Scheduled as M017 S04 / Phase 44.

## 2026-10-03 — Process: M017 S02 leased (replay integrity)

### Process

- Lease M017-S02 for GSD Phase 42 (REPL-01..04, KNOWN-03). Phases 43 unleased. No crate change in this entry.

### Trajectory

- Active Milestone **M017**, Active Slice **S02**. `L_delivery=M017-S01-done`.

## 2026-10-03 — Process: M017 S01 living docs and cite-lease

### Process

- GSD v0.10 / M017 authored; S01 leased (living docs and governor honesty). `.planning/PROJECT.md` cites `.kutha/STATE.md` for lease and freeze; `cite-lease` covers PROJECT (stale `Active Milestone` token fails). Semantic-gap addendum 2026-10-03, I-F7/I-F9/I-F13/I-F16 restaged, architecture and planning banners, ROADMAP M016 Success (Horn over live facts) and Freeze pointer. ADR-000/013/014/041/050/060/070 and README carry 2026-10-03 clarifications (honeycomb restage line added). `idle-delivery-closed` selftest mutation now bites both idle and leased STATE; the cite-lease Python test allows PROJECT.md.

### Trajectory

- M017 S01 closed (`L_delivery=M017-S01-done`), Active Slice **None**; independent verification `41-VERIFICATION.md` passed. Honeycomb stays **Proposed**. No `crates/` change.

## 2026-10-02 — Process: M002 intake note (kernel I/O stalls)

### Process

- Captured arXiv:2609.12597 / SteelDB companion for frozen M002: `docs/architecture/m002-cloud-kernel-io-intake.md` and applicability card `paper-kernel-io-dirty-throttle-cloud-oltp`. No lease; freeze unchanged.

## 2026-10-02 — Trajectory: M016 CLOSED (v0.09 archived)

### Trajectory

- `L_delivery=M016-closed`. Active Milestone **None**. Process tag `v0.09`. Crate SemVer stays `0.0.0`. Honeycomb stays **Proposed**. Next: lease M002 before durability work.

### Product

- FIXG-01 fail-closed: C01 sunset is not a Horn; ADR-013 Evidence dated. No fifth clock.

## 2026-10-02 — Product: C01 sunset fail-closed FIXG-01 verdict (Phase 40)

### Product

### Added

- Named oracle `fixg01_c01_sunset_inexpressible_as_horn`: C01 quote; Horn cannot hold a calendar sunset; `valid_to` folding is VT not a transitional rule.

### Changed

- ADR-013 Evidence (2026-10-02): fail-closed — two axes + current RuleAst cannot express application-end without a fifth clock. Cell stays **Proposed**. No five clocks invented.

### Trajectory

- Cells remain **Proposed**. M016 S05 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S05 leased

### Trajectory

- Active Slice **S05** (GSD Phase 40, Legal golden fixture). `L_delivery=M016-S04-done`.

## 2026-10-02 — Trajectory: M016 S04 CLOSED (Phase 39)

### Trajectory

- Phase 39 verified; `L_delivery=M016-S04-done`. Next: lease S05 (Legal golden fixture).

### Product

- Live policy blob `deny` fails admission; Horn execution replay fails empty-pin inverse.

## 2026-10-02 — Product: independent RULE-03/04 kill-tests (Phase 39)

### Product

### Added

- Independent kill-test: live `deny` fails `check_admission`; prior admission AS OF stays true; empty-pin inverse fails `replay_check` with unchanged fingerprint.

### Trajectory

- Cells remain **Proposed**. M016 S04 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: policy body eval and Horn execution replay (Phase 39)

### Product

### Added

- Live policy blob `deny` fails `check_admission` (`reason: policy`); other live bodies including `leased-policy` still admit. Prior admission remains queryable AS OF after a later pin.
- `replay_check` re-derives Horn-pinned Behaviors at `ingested_at`; empty-pin inverse fails closed without changing the facts-only fingerprint.

### Trajectory

- Cells remain **Proposed**. M016 S04 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S04 leased

### Trajectory

- Active Slice **S04** (GSD Phase 39, Policy + execution replay). `L_delivery=M016-S03-done`.

## 2026-10-02 — Trajectory: M016 S03 CLOSED (Phase 38)

### Trajectory

- Phase 38 verified; `L_delivery=M016-S03-done`. Next: lease S04 (Policy + execution replay).

### Product

- Horn evaluator; genesis inverse_knows; n-ary Horn; premise retract drops Horn-derived supports.

## 2026-10-02 — Product: independent RULE-02 kill-tests (Phase 38)

### Product

### Added

- Independent kill-test: inverse pin is genesis Horn hash; relatedTo does not mint inverse; retract of knows drops live knownBy.

### Trajectory

- Cells remain **Proposed**. M016 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: n-ary Horn and derived retract (Phase 38)

### Product

### Added

- Named test: two-premise Horn fires only when both premises are live; retracting one drops the derived support.

### Trajectory

- Cells remain **Proposed**. M016 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: Horn evaluator replaces inverse_knows cascade (Phase 38)

### Product

### Changed

- `knows` Assert derives `inverse_knows` from a genesis Horn body (`knownBy(Y,X) :- knows(X,Y)`). The pin is the canonical body hash. The hard-coded `follow_ons` special-case is gone.

### Trajectory

- Cells remain **Proposed**. M016 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S03 leased

### Trajectory

- Active Slice **S03** (GSD Phase 38, Rule evaluator). `L_delivery=M016-S02-done`.

## 2026-10-02 — Trajectory: M016 S02 CLOSED (Phase 37)

### Trajectory

- Phase 37 verified; `L_delivery=M016-S02-done`. Next: lease S03 (Rule evaluator).

### Product

- Canonical Named/Horn `RuleAst`; `rule_version` is body hash; non-canonical bodies fail closed.

## 2026-10-02 — Product: independent RULE-01 kill-tests (Phase 37)

### Product

### Added

- Independent kill-tests: big-endian length and truncation are not canonical; crafted Horn is rejected; `StoreBlob` JSON with a Named pin does not append.

### Trajectory

- Cells remain **Proposed**. M016 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: rule body hash is rule_version (Phase 37)

### Product

### Changed

- `rule_definition_hash` of an identifier is the canonical `Named` body hash. `emit_rule` stores tagged AST bytes, not raw UTF-8. Non-canonical names fail closed (`UnknownRuleVersion`). `blob_digest_matches` accepts a rule-domain digest only for exact-canonical bodies.

### Added

- Named tests: JSON/`emit_rule` reject; stored blob `from_canonical` equals `Named`; Horn `emit_rule_ast` pin equals body hash.

### Trajectory

- Cells remain **Proposed**. M016 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: rule AST canonical encoding (Phase 37)

### Product

### Added

- Bounded positive `RuleAst` (`Named`, `Horn`) with tagged little-endian canonical bytes. Body hash is SHA-256 of `kutha-rule-def` plus those bytes. JSON, trailing junk, and raw UTF-8 names are not canonical. Evaluation is not in this slice.

### Trajectory

- Cells remain **Proposed**. M016 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: test hardening T2–T6 (Phase 36)

### Product

### Added

- `kutha-common` trybuild UI crate proves `TransactionTime` and `WallTime` cannot be passed where `ValidTime` is required (`E0308`). The rustdoc `compile_fail` example stays as a secondary check.
- Named test: stored `JustificationCite` `tt`/`vt` stay distinct; a swapped-field clone is not equal to the stored row.
- Named tests: Eager vs Lazy `as_known_on`; `fork_at` before a later `TtAnchor`; mid-cut snapshot plus tail Assert/`TtAnchor`; `GraphFold::replay` known-on.

### Changed

- `snapshot_plus_tail_matches_full_replay` uses a mid-cut snapshot, a non-empty Assert+`TtAnchor` tail, and `as_known_on` (fingerprint remains facts-only).

### Trajectory

- Cells remain **Proposed**. M016 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Process: living docs and governor honesty (Phase 35)

### Process

### Changed

- Living ROADMAP, PROJECT, semantic-gap, p0-spike, and legal-fixture-recon match M014 ship plus the Phase 34 known-on residual.
- `observe_cargo.required` includes `true_on_d_differs_from_known_on_d`. Artifact-budget globs use `min_hits: 1` while live phases exist; id-map-cite needles include v0.07 and v0.08.

### Product

### Changed

- Comment only: TT-to-wall is M014 `tt_anchor` / M015 known-on honesty, not out of M012a S05.

### Trajectory

- ADR-013 Evidence records F13 clock/anchor ship and `as_known_on`; the five-clock question stays open for M016. Honeycomb ADR-013 evidence restaged. Cells remain **Proposed**. No SemVer tag.

## 2026-10-02 — Product: fail-closed TtAnchor admit (Phase 34)

### Product

### Fixed

- `emit_tt_anchor` rejects a duplicate wall or a wall earlier than the max prior wall (`RuntimeError::TtAnchorRejected`). The log and fold stay unchanged. Monotone unique walls still resolve greatest-anchor-≤D after persist/open.

### Trajectory

- Cells remain **Proposed**. M015 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: known-on closed VT (Phase 34)

### Product

### Fixed

- `as_known_on` evaluates TT-only product at the resolved anchor (`known_at` / `is_known_at`). Facts with finite `valid_to` stay visible at a knowledge cut. `as_of` still uses `live_at(MAX, vt)`; `is_live_at` is unchanged.

### Trajectory

- Cells remain **Proposed**. M015 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: fixture clock wraps (Phase 33)

### Product

### Changed

- FF5 public clock cuts wrap `ValidTime::from` / `TransactionTime::MAX`; `Op` `valid_from`/`valid_to` stay `u64`. Oracle assertion predicates unchanged.
- M011/M012/M013 fixtures and FF6 `live_count` wrap the same constructors at `as_of` / `live_at` / CSR / justification / derivation / admission cuts.

### Trajectory

- Cells remain **Proposed**. M014 S03 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: tt_anchor and as_known_on (Phase 32)

### Product

### Added

- `Op::TtAnchor { wall: u64 }` maps `Event.ingested_at` (transaction time) to a wall date. The fold keeps a skip-serialized `BTreeMap` lease rebuilt from the log; no `Fact` is minted.
- `Runtime::as_known_on` / `GraphFold::as_known_on` take `impl Into<WallTime>` and return `Option` of product triples: greatest wall at or before the date, then `live_at(tt, u64::MAX)`. `None` means no such anchor (miss, not empty-as-of).
- Named test `true_on_d_differs_from_known_on_d` (`crates/kutha-runtime/tests/m014_as_known_on.rs`): `as_of(2015)` sees a statute that `as_known_on(2015)` does not; `as_known_on(2020)` sees it after the later anchor.

### Trajectory

- Cells remain **Proposed**. M014 S02 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: typed clock newtypes (Phase 31)

### Product

### Added

- `ValidTime`, `TransactionTime`, and `WallTime` are distinct `#[serde(transparent)]` tuple structs (`Vt` / `Tt` aliases). Mixing clocks without conversion does not type-check (`compile_fail` doctest on `ValidTime`). `Op` clock fields and `Event.ingested_at` stay `u64` so named oracle literals compile.

### Changed

- `GraphFold` query cuts (`as_of`, `live_at`, dictionary/admission/policy pins, `live_supports`, `claim_supported_at`) take `impl Into` clocks and compare `as_u64` against `Fact` integer fields. `as_of(2015u64)` and `as_of(ValidTime::from(2015))` type-check; `as_of(TransactionTime::from(1))` does not.
- `Runtime::as_of` / `live_at` match the fold (`impl Into` clocks). Write helpers (`assert` / `retract` / `correct` / `derive` / `emit_allow` / `emit_rule` / `emit_policy`) take `u64` so named oracle literals still compile.
- `Runtime::csr_lease_at` / `typed_csr_lease_at`, `CsrLease::from_fold` / `TypedCsrLease::from_fold`, and `Materializer::build` take `impl Into` clocks and convert with `as_u64` before the integer CSR picture.
- Justification/action/admission/policy/derivation/conflict cuts take `impl Into` clocks; `Op::JustificationCite` `tt`/`vt` stay `u64`.
- TIME-01 fixture year bindings wrap `ValidTime::from(...)`; `assert_eq!(y2015, 2015)` bodies unchanged.

### Trajectory

- Cells remain **Proposed**. M014 S01 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M016 S02 leased

### Trajectory

- Active Slice **S02** (GSD Phase 37, Rule AST). `L_delivery=M016-S01-done`.

## 2026-10-02 — Trajectory: M016 S01 CLOSED (Phase 36)

### Trajectory

- Phase 36 verified; `L_delivery=M016-S01-done`. Next: lease S02 (Rule AST).

### Product

- trybuild E0308 clock mix; JustificationCite tt/vt oracle; known-on Eager/Lazy/fork/snapshot+tail/replay hardening tests.

## 2026-10-02 — Trajectory: M016 S01 leased

### Trajectory

- Active Milestone **M016**, Active Slice **S01** (GSD Phase 36, T2–T6). `L_delivery=M015-closed`.

## 2026-10-02 — Process: v0.09 milestone authored

### Process

- GSD v0.09 / harness M016: Phase 36 test-hardening T2–T6, then Phases 37–40 rules (F9/F16). Not leased yet.

## 2026-10-02 — Trajectory: M015 CLOSED (GSD v0.08 archive and tag)

### Trajectory

- M015 S01–S02 delivered; Active Milestone cleared.
- Governor: artifact-budget `min_hits` returned to 0 for empty live phases between milestones. Archive `.planning/milestones/v0.08-*`; process tag `v0.08`. Crate SemVer stays `0.0.0`.

### Product

- TT-only `as_known_on` (closed VT survives); fail-closed `TtAnchor` walls; ANK required in governor.

## 2026-10-02 — Trajectory: M015 S02 leased

### Trajectory

- Active Slice **S02** (GSD Phase 35). `L_delivery=M015-S01-done`.

## 2026-10-02 — Trajectory: M015 S01 CLOSED (Phase 34)

### Trajectory

- Phase 34 verified; Active Slice cleared. `L_delivery=M015-S01-done`. Next: lease S02 alone.
- Governor: `idle-delivery-closed` selftest forces Milestone None + non-closed `L_delivery` (works leased or idle).

### Product

- `as_known_on` uses TT-only `known_at` (closed VT intervals survive); duplicate/backwards `TtAnchor` fail closed at admit.

## 2026-10-02 — Trajectory: M015 S01 leased

### Trajectory

- Active Milestone **M015**, Active Slice **S01** (GSD Phase 34). `L_delivery=M014-closed`. `README.md` shows M015.

## 2026-10-02 — Process: v0.08 milestone authored

### Process

- GSD v0.08 / harness M015: known-on honesty and M014 residuals (phases 34–35). Executable rules renumbered to **M016** / v0.09. Id map updated. Not leased yet.

## 2026-10-02 — Trajectory: M014 CLOSED (GSD v0.07 archive and tag)

### Trajectory

- M014 S01–S03 delivered; Active Milestone cleared. GSD archive under `.planning/milestones/v0.07-*`; process tag `v0.07`. Crate SemVer stays `0.0.0`.
- Governor: `artifact-budget` / `verification-independent` selftest skip when live phases empty; `idle-delivery-closed` mutates `L_delivery`; `state-readme` selftest mutates Phase only at idle.

### Product

- F13: typed clocks, `tt_anchor`/`as_known_on`, fixture clock wraps. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: M014 S03 leased

### Trajectory

- Active Slice **S03** (GSD Phase 33, fixture migration). `L_delivery=M014-S02-done`.

## 2026-10-02 — Trajectory: M014 S02 CLOSED (Phase 32)

### Trajectory

- Phase 32 verified; Active Slice cleared. `L_delivery=M014-S02-done`. Next: lease S03 alone.

### Product

- `Op::TtAnchor` + `as_known_on(WallTime)`; named true-on-D vs known-on-D; FF5 unchanged and green.

## 2026-10-02 — Trajectory: M014 S02 leased

### Trajectory

- Active Slice **S02** (GSD Phase 32, `tt_anchor` / `as_known_on`). `L_delivery=M014-S01-done`. `README.md` shows M014 S02 active.

## 2026-10-02 — Trajectory: M014 S01 CLOSED (Phase 31)

### Trajectory

- Phase 31 clock newtypes verified; Active Slice cleared. `L_delivery=M014-S01-done`. Next: lease S02 alone.
- Governor selftest `state-readme` mutation pattern updated to `M014` (was `M013`).

### Product

- Distinct `ValidTime` / `TransactionTime` / `WallTime` newtypes; query cuts take `Into`; `compile_fail` mix proof; named oracles green (TIME-01 wraps only).

## 2026-10-02 — Trajectory: M014 S01 leased

### Trajectory

- Active Milestone **M014**, Active Slice **S01** (GSD Phase 31, clock newtypes). `L_delivery=M013-closed`. `README.md` shows Active Milestone **M014**. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: M013 CLOSED (GSD v0.06 archive and tag)

### Trajectory

- M013 S01–S09 delivered; Active Milestone cleared. GSD archive under `.planning/milestones/v0.06-*`; process tag `v0.06`. Crate SemVer stays `0.0.0`. Residual audit debt acknowledged. Cells remain **Proposed**.

### Process

- Governor: `artifact-budget` `min_hits` 0 so an empty live `.planning/phases/` is allowed between milestones; `verification-independent` accepts `allow_empty` on its globs. `L_delivery=M013-closed` with Active Milestone None.
- Id map: v0.06 → shipped; v0.07 (M014, phases 31–33) is next and needs a lease.

## 2026-10-02 — Product: facade exact triples and dual-mode bench (Phase 30-02)

### Product

### Added

- `facade_statute`: `assert_eq!` on sorted live triples after assert, correct, retract, persist/open, and empty `fork_at(0)` product picture (HARD-04).

### Changed

- `benchmark-baseline.md`: Phase 30 bench delta vs Phase 27 after-lease (ingest +5.6%, as_of faster).

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: dual-mode FF5 oracle (Phase 30-02)

### Product

### Added

- `m013_dual_mode`: Eager vs Lazy agreement on FF5 `as_of`, `live_at`, retract, correct, and `claim_supported_at` without warming Lazy leases before reads (HARD-03).

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: write-path admit origin and keyed correct identity

### Product

### Changed

- User `emit` cannot Assert, Behavior, Correct, or CorrectInterval on reserved `kutha:` relations (`MetaOpRejected`); dictionary constructors (`emit_allow` / rule / policy) and typed justification/action paths unchanged; user Retract of dictionary facts still admitted (D-07 oracles).
- `GraphFold::apply` copies the prior fact `delivery_key` onto Correct and CorrectInterval replacement rows so replacement identity survives keyed correction; keyed ingest retry idempotence remains the log `by_delivery_key` map on the minting Assert (ING-01). Snapshots written before this change whose logs contain keyed Correct/CorrectInterval will fail `replay_check` fingerprint until rebuilt (research stage; no migration).
- Phase 30 bench `as_of` delta vs Phase 27 in `benchmark-baseline.md` is single-run variance, not a query-path change in this slice.

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Product: Phase 30-01 RED oracles (write-path integrity)

### Product

### Added

- `m013_write_path` RED tests: public `emit` Assert on `kutha:allows` must be refused (HARD-01); keyed `Correct` / `CorrectInterval` must preserve `delivery_key` through retry (HARD-02). Implementation follows in the same phase.

### Trajectory

- Cells remain **Proposed**. M013 S09 lease unchanged; no SemVer tag.

## 2026-10-02 — Trajectory: M013 S09 closed

### Trajectory

- Phase 30 verified independently (`30-VERIFICATION` passed, Grok 4.6 xhigh). Active Slice cleared; `L_delivery=M013-S09-done`; M013 S01–S09 delivered. No archive, no SemVer tag. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: M013 S09 leased

### Trajectory

- `L_delivery=M013-S08-done`, Active Slice **S09** (GSD Phase 30, write-path integrity hardening; HARD-01..04). Cells remain **Proposed**; no archive, no SemVer tag.

## 2026-10-02 — Trajectory: M013 S08 closed

### Trajectory

- Phase 29 verified independently (`29-VERIFICATION` passed, Grok 4.6 xhigh). Active Slice cleared; `L_delivery=M013-S08-done`; S09 (Phase 30) unleased. M013 stays open; no archive, no SemVer tag. Cells remain **Proposed**.

### Process

- Governor: budget truncation and empty budget globs fail closed; verification independence covers phases 22+; 60 checks within budget 64.

## 2026-10-02 — Process: Phase 29 review follow-ups

### Process

### Fixed

- `frontmatter_paths_exist` rejects paths that resolve outside the repo and an empty `read_only_inputs` list (HIGH); quoted entries are unquoted. Two tests pin both.
- `i-f11-one-write-path` pins `require_symbols`; invariants comment now says only deferred rows are unenforced; META `file_contains` row names `glob` / `min_phase`.

## 2026-10-02 — Process: honor-system verify and process doc alignment

### Process

### Changed

- `kutha-harness.md` and `.kutha/META.md` document `ci` as run_checks → run_selftest → observe_cargo; `--skip-cargo` leaves selftest on; `glob_max_lines` `min_hits` zero-match rule.
- `codex-subagents.md` Cursor role map for phases 22–30; `governor-intake.md` current I-cui-budget / glob / frontmatter examples.
- `gsd-auto-contract.md` states model-family independence is honor-system, not governor-provable.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Trajectory: post-M013 addendum and ADR evidence notes

### Trajectory

### Changed

- Dated Evidence/Clarification blocks on ADR-000, 010–014, 040–041, 050, 060–061 (Status stays **Proposed**; lock bodies unchanged).
- `semantic-gap-review.md` post-M013 addendum: F10–F12, F14–F15 closed with named oracles; F1/F7–F9/F13/F16 open; FF5 scope at TT max.
- `honeycomb.yaml` M013 test names restaged (`statute_as_of_via_facade`, genesis, ingest/fold oracles).

### Process

- `docs/ADR/README.md` status vocabulary note (map/delivery/capability vs Accepted).

## 2026-10-02 — Process: align v0.06 bookkeeping with 28-VERIFICATION

### Process

### Changed

- BASE/REC requirement checkboxes and traceability marked complete (evidence: 22-VERIFICATION, 23-VERIFICATION, `legal-fixture-recon.md`).
- `v0.06-MILESTONE-AUDIT.md` YAML and body agree: Phase 28 passed; PROC-02 satisfied; `gaps_found` only for audit-tail and tracking debt.
- `PROJECT.md` and `.kutha/ROADMAP.md` cite `L_delivery=M013-S07-done` and M013 S08–S09 tail without closing the milestone.

### Trajectory

- M013 stays leased until S09; no archive or tag. Cells remain **Proposed**.

## 2026-10-02 — Process: restage shipped I-F oracles and require M013 tests

### Process

### Added

- Six M013 `observe_cargo.required` test names; eleven `i-f*` `rust_test_asserts` checks restage shipped semantic-gap rows (I-F1–F6, F10–F12, F14–F15).

### Changed

- I-F7/I-F8 `until` strings name M014 / remaining fork-diff honestly; I-F9, F13, F16 stay deferred.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Process: glob verification independence and listed input paths

### Process

### Added

- `file_contains` supports `glob` and `min_phase`; new `frontmatter_paths_exist` kind requires listed `read_only_inputs` files to exist under the repo root.
- `verification-independent` uses phase-22+ globs instead of per-phase hardcoded paths; removed non-repo path from `23-VERIFICATION.md`.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Process: fail closed on check-budget truncation and empty budget globs

### Process

### Added

- FSM `run_checks` emits HIGH `budget` when any check is skipped under the configured budget; ledger row `I-cui-budget` records the `run_checks` kind obligation.
- `glob_max_lines` optional `min_hits` (default 0); artifact-budget steps require at least one phase-22+ match per glob.

### Trajectory

- No lease change. Cells remain **Proposed**.

## 2026-10-02 — Process: M013 tail S08–S09 from the audit and project review

### Process

### Added

- `.kutha/ROADMAP.md` M013 tail: S08 close-out honesty (documents and governor) and S09 write-path integrity hardening. GSD Phases 29 and 30 with CONTEXT; requirements DOC, GOV, HARD in `.planning/REQUIREMENTS.md`.

### Changed

- `docs/process/gsd-auto-contract.md` § Id map: v0.06 is Phases 22–30 and slices S01–S09; tool pair applies to Phase 30 (product code), CBM only for Phase 29. M013 stays leased until S09 closes. No archive, no SemVer tag.

### Trajectory

- No lease change in this commit. Active Slice **None**; S08 is leased in the next commit alone. Cells remain **Proposed**.

## 2026-10-02 — Process: ci --skip-cargo and phase timings

### Process

### Added

- `uv run kutha-gov ci --skip-cargo` skips `observe_cargo` and `emit_tenant` (selftest still runs); a HIGH from checks also skips cargo and tenant after selftest. The ci report prints `observe: cargo skipped (not evidence)` and per-phase `phase_ms`. `--skip-cargo` is rejected for commands other than `ci`. Not closeout evidence; one full `ci` at close (`docs/process/kutha-harness.md`).
- `AGENTS.md`: one short `rg` vs `ast-grep` note (text/regex arbiter vs AST). Still under the `agents-lean` line budget.

### Trajectory

- No lease change. Active Slice **None**. Cells remain **Proposed**.

## 2026-10-02 — Process: FSM check budget above the check count

### Process

### Fixed

- `defaults.budget` and `KUTHA_GOV_BUDGET` were 48 while the governor had 49 checks after Phase 28, so the default `ci` quantum silently truncated the tail check (`version-freeze`) and `test_ci_quantum_reaches_ok_on_this_tree` / `test_cui_budget_truncates_tail_checks_when_v_is_short` failed. Budget is now 64 (headroom). Fail-closed on any truncation is planned in Phase 29.

### Trajectory

- No lease change. Active Slice **None**. Cells remain **Proposed**.

## 2026-10-02 — Process: close M013 S07 after independent verification

### Process

### Changed

- Phase 28 independent verification passed (`28-VERIFICATION.md`); PROC-02 closed. Active Slice cleared; `L_delivery=M013-S07-done`. M013 S01–S07 delivered.

### Trajectory

- Phase 28 / S07 closed. Cells remain **Proposed**. Do not start M014 until leased. No SemVer tag.

## 2026-10-02 — Process: v0.06 milestone audit and PROC checkboxes

### Process

### Added

- `.planning/v0.06-MILESTONE-AUDIT.md` from gsd-audit-milestone; status `gaps_found` because `28-VERIFICATION.md` is not written in this wave (D-08).

### Changed

- REQUIREMENTS PROC-01, PROC-03, and PROC-04 checked; PROC-02 left open pending independent verify-work.

### Trajectory

- No lease close, no SemVer tag, no `state.milestone-switch`. Cells remain **Proposed**. S07 stays leased.

## 2026-10-02 — Process: Id map cite from governor and ROADMAPs

### Process

### Added

- Invariant `I-id-map` and check `id-map-cite`: the Id map table stays in `docs/process/gsd-auto-contract.md`; `.planning/ROADMAP.md` and `.kutha/ROADMAP.md` link to § Id map and must not restate the table header.

### Trajectory

- No lease or honeycomb change in this commit. Cells remain **Proposed**.

## 2026-10-02 — Process: remaining artifact budgets and verification independence

### Process

### Added

- `artifact-budget` steps for SUMMARY (max 60, n≤2), VERIFICATION (max 60, n=1), and CONTEXT (max 80, n=1) from phase 22.
- Check `verification-independent` and invariant `I-verification-independent`: `glob_absent` of `-PLAN.md` / `-SUMMARY.md` on VERIFICATION files; `file_contains` independence keys on phases 22–27.

### Trajectory

- No lease or honeycomb change in this commit. Cells remain **Proposed**.

## 2026-10-02 — Process: glob_max_lines and PLAN artifact budget

### Process

### Added

- Kind `glob_max_lines`: glob matches counted with `wc -l`, optional `min_phase` (phases 22+), optional `max_files_per_dir`. Excess lines are HIGH `max-lines`; too many files in one phase directory are HIGH `max-files`. Derived selftest mutation appends newlines to the first sorted match.
- Check `artifact-budget` (PLAN step only) and invariant `I-artifact-budget`. Unknown kind stays fail-closed HIGH.

### Trajectory

- No lease or honeycomb change in this commit. Cells remain **Proposed**.

## 2026-10-02 — Product: O(1) kutha:allows admit lease

### Product

### Changed

- Fold maintains a droppable `kutha:allows` hot lease (`allows_by_name`) rebuilt with other fold leases. `relation_allowed_at` is O(allows) per admit instead of collecting all live facts. Eager/Lazy still applies to product maps; product `as_of` excludes `kutha:`; `dictionary_as_of` unchanged. Named D-07 expects and `Op::AllowRelation` absence unchanged.
- Phase 22 bench at N=10000 after the lease: ingest_plain.n 120267790 ns (+26.1% vs Phase 22, was +2168%); as_of.n 511233 ns (single-shot fold walk; was +1566%).

### Trajectory

- No harness lease change. Active Slice **S06**. Cells remain **Proposed**.

## 2026-10-02 — Product: public facade and write-path oracles

### Product

### Added

- Public Runtime facade: `assert` / `retract` / `correct` / `derive` (internable names), `as_of` / `live_at`, `persist` / `open`. Statute example and `facade_statute` compile against that surface; `as_of(2015) ≠ as_of(2021)` plus one admission and one action.
- `m013_write_path` unknown-relation Assert is `UnknownRelation`; justification/action go through pipeline receipt (`outcome_records` grows); no ghost-relation fact.

### Changed

- FF5 named `as_of` reads `Runtime::as_of` (same triples as `fold().as_of`). `emit` stays public so m012/ff6/bench call sites and expects are unchanged (D-07).

### Process

- Governor `file_contains` needles for `live_support_count` / `claim_supported_at`, `derivation_eligible_at`, `provenance_fingerprint`, and `check_admission` / `conflict_report_at` follow the 27-02 module split (`fold_query.rs`, `provenance.rs`, `admission.rs`). Same symbols; named cargo expects unchanged.

### Trajectory

- S06 still Active Slice. Phase 22 bench re-run: ingest_plain.n +2168%, as_of.n +1566% vs the Phase 22 table — D-07 ≤10% **STOP** (documented in `docs/architecture/benchmark-baseline.md`). Cells remain **Proposed**.

## 2026-10-02 — Product: one write pipeline and runtime module split

### Product

### Added

- `Runtime::pipeline` / `pipeline_kind` / `pipeline_events`: one propose→admit→commit→receipt path. `emit` is a thin wrapper that still rejects public forging of `JustificationCite`, `QuantumOutcome`, `kutha:admission`, and `kutha:action` (`MetaOpRejected`).
- Runtime modules under `crates/kutha-runtime/src/`: `pipeline.rs`, `admission.rs`, `justification.rs`, `hydrate.rs`, `provenance.rs`, `fold_query.rs`, `quantum_invariants.rs`. Every `src/*.rs` is ≤800 lines. No new crate.

### Changed

- `emit_allow` / `emit_rule` / `emit_policy`, `record_justification` / `record_action` / `record_resume`, and user `emit` all call the pipeline. The `append_meta` side path is deleted. Dictionary constructors stay receipt-quiet so named m012 log-growth expects hold; justification/action buffer a receipt without an extra log `QuantumOutcome`.
- `GraphFold` query surface (`as_of`, `live_at`, dictionary/registry cuts, fingerprint) lives in `fold_query.rs`; apply/replay/hot maps stay in `fold.rs`.

### Trajectory

- No lease change. Active Slice **S06**. Cells remain **Proposed**. Facade methods and Phase 22 bench delta are the next 27-02 commit.

## 2026-10-01 — Product: RED facade statute consumer

### Product

### Added

- Integration test `facade_statute::statute_as_of_via_facade` and example `statute_as_of`: D-08 RED (PIPE-03/04) — facade-only consumer (`Runtime` / `RuntimeError`; D-03 names `assert` / `retract` / `correct` / `derive` / `as_of` / `live_at` / `persist` / `open`) does not compile today. One admission and one action record. Wiring is 27-02.
- Integration test `m013_write_path::unknown_relation_rejected_for_assert_justification_and_action`: D-08 RED (PIPE-01) — Assert unknown-relation is fail-closed; justification/action skip the same admit/receipt (`outcome_records` does not grow). Wiring is 27-02.

### Trajectory

- No lease change. Active Slice **S06**. Cells remain **Proposed**.

## 2026-10-01 — Product: EventId admissions and green dictionary oracles

### Product

### Changed

- `justification_id` is the `EventId` of the `kutha:admission` Assert. Named oracles (`m012_*`, `m012a_*`, `m011_*`, `ff5`, `ff6`) call `emit_allow` / `emit_rule` / `emit_policy`; 26-01 tests read `dictionary_as_of` (product `as_of` stays kutha-free). Retract of an admission or genesis allow EventId versions the later cut; `fork_at` still admits.
- Provenance mix hashes dictionary Asserts (former AllowRelation/RegisterRule/PinPolicy/RecordAdmission/RecordAction) plus `StoreBlob`.

### Trajectory

- Phase 26 / S05 closed: independent verification passed; `L_delivery=M013-S05-done`; Active Slice **None**. Cells remain **Proposed**. Do not start Phase 27 until S06 leased.

## 2026-10-01 — Product: dictionary entries as kutha facts and StoreBlob


### Product

### Added

- `Op::StoreBlob { digest, bytes }` — content-addressed body on the log (store-once; digest is SHA-256 hex of bytes). Used for `kutha:rule` / `kutha:policy` payloads; digest is `rule_definition_hash` / `policy_version_hash`.
- Reserved relation constants `KUTHA_ALLOWS`, `KUTHA_RULE`, `KUTHA_POLICY`, `KUTHA_ADMISSION`, `KUTHA_ACTION` and `KUTHA_RESERVED_RELS`.
- `Fold::dictionary_as_of` / `dictionary_live_at` — live dictionary facts including `kutha:` (D-05). Product `as_of` / `live_at` exclude `kutha:`.
- Runtime helpers `emit_allow`, `emit_rule`, `emit_policy` (silent intern + `StoreBlob` + dictionary Assert). `intern` marks reserved `kutha:` names so `admit` skips the product allowlist for those relations. `Retract` is fact-only (`EventId`).

### Changed

- Removed `Op::AllowRelation`, `RegisterRule`, `PinPolicy`, `RecordAdmission`, `RecordAction` and the five fold side tables. Dictionary rows are ordinary bi-temporal Facts with those reserved relations.
- Genesis emits `kutha:allows` Asserts (`valid_from` 0) after silent intern of reserved names plus YAML, instead of `AllowRelation`.
- `check_admission` reads the fold and log (no RAM justification buffer). `record_justification` returns the `kutha:admission` Assert `EventId` as `justification_id`. Public `emit` of `kutha:admission` / `kutha:action` Assert or `JustificationCite` is `MetaOpRejected`.

### Trajectory

- No lease change. Active Slice **S05**. Cells remain **Proposed**.

## 2026-10-01 — Product: RED dictionary entries on as_of


### Product

### Added

- Integration test `m013_dictionary_facts::admission_and_policy_visible_on_as_of_after_later_pin` (D-08 RED): after `PinPolicy` + `record_justification` + a later `PinPolicy`, `fold.as_of(2010)` must contain interned `kutha:policy` and `kutha:admission` triples (same walk as FF5). Fails today: those rows live in fold side tables. Wiring is 26-02.
- Integration test `m013_dictionary_facts::genesis_allow_visible_on_as_of` (D-08 RED): `Runtime::default()` then intern `kutha:allows`; `fold.as_of(0)` must contain that relation. Fails today: genesis is a non-fact allow-entry. Do not intern extra YAML names.

### Trajectory

- No lease change. Active Slice **S05**. Cells remain **Proposed**.

## 2026-10-01 — Product: genesis AllowRelation on new log

### Product

### Added

- `Runtime::new` seeds one `Op::AllowRelation` per YAML name (`valid_from` 0) on an empty log. Names are not interned. Sorted for a stable provenance mix.
- Genesis oracles `stripped_genesis_rejects_product_relation` and `persist_open_replay_keeps_genesis` (stripped log fail-closed; persist/open keeps YAML names; genesis payload tamper fails `provenance_check`).

### Changed

- `admit` consults `fold.relation_allowed_at` only. Runtime no longer holds a YAML `HashSet`. `from_snapshot` / `from_dict_and_events` / `fork_at` do not read relations YAML (D-01, D-02, D-03).
- M010 `intern_appends_define_for_new_terms_only` length proxies use genesis prefix `G` (Define-silence unchanged).

### Trajectory

- Phase 25 / S04 closed: independent verification passed; `L_delivery=M013-S04-done`; Active Slice **None**. Cells remain **Proposed**. Do not start Phase 26 until S05 leased.

## 2026-10-01 — Product: RED genesis log oracles

### Product

### Added

- Integration tests `m013_genesis_log` (`retract_seeded_related_to_fails_closed_after`, `open_after_relations_path_missing_still_emits`): D-07 / GEN-03 and D-03 / GEN-01 RED — retract YAML-seeded `relatedTo` then Assert must fail closed; persist then `open` with `KUTHA_RELATIONS_PATH` pointing at a missing file must still emit `inForceAs` (currently fails closed because `open` reloads YAML). Wiring is 25-02.

### Trajectory

- No lease change. Active Slice **S04**. Cells remain **Proposed**.

## 2026-10-01 — Process: productivity from log audit

### Process

### Added

- `kutha-gov ci --skip-cargo`: keep dictionary + H5 selftest + emit_log; skip cargo/tenant subprocess for harness iteration; fail-fast skips the same after HIGH; `phase_ms` on stdout (`docs/process/kutha-harness.md`).
- Gate stacking + reindex discipline in `docs/process/gsd-auto-contract.md`.
- Plan: `.compound-engineering/artifacts/plans/2026-10-01-process-productivity-from-log-audit.md`.

### Changed

- `.planning/config.json`: `skip_discuss`, no worktrees/Nyquist/UI/AI-integration phases, `git.create_tag=false`; keep `plan_check` + `verifier`.
- `.planning/WINDOWS.md`: waive 1/3/5/6; fixed 8; leave 2/4/7/9 open.
- Phase 23 `23-VERIFICATION.md` trimmed to ≤60 lines (artifact budget).

### Trajectory

- No lease change. Item deferred: hanging ideation `http.server` / unauth MCP (explain-only; no kill, no MCP edit).

## 2026-10-01 — Product: M013 S03 fold ingest indexes (pending D-06)

### Product

### Added

- Fold/Runtime ingest index leases (`fact_by_event`, `live_triples`, `by_event_id`, `by_delivery_key`) and `ingest_examine` visit counter; D-03 scan sites use indexes (Phase 24 / IDX-01, IDX-02).
- Integration test `m013_fold_indexes` (`green_ingest_visits_constant_in_n`, fork/open checks).

### Trajectory

- Active Slice **S03**; `L_delivery=M013-S02-done` until S03 closes.
- D-06 absolute ingest wall-time vs Phase 22 exceeded 10% on two runs (+17…+28%); documented in `benchmark-baseline.md` Phase 24 delta. Controller decision required before closing S03.
- Reserve spike (`reserve_ingest_indexes`, small Vec caps): still +21…+28% ingest; capacity alone does not clear D-06.
- Hybrid spike (`FxHashMap` + `SmallVec<[usize;1]>` + reserve; Samyama/Pogocache-inspired): ingest still +21…+28% vs Phase 22; visit oracle green. See `benchmark-baseline.md` hybrid section.
- `FoldIndexMode` config (Eager = accept write-cost default; Lazy = thin write / lazy lease via `KUTHA_FOLD_INDEX_MODE=lazy|thin`). API: `Runtime::with_index_mode` / `set_index_mode` / `ensure_fold_indexes`. Tests: `m013_fold_index_mode`. Env template: `.env.example` (`KUTHA_FOLD_INDEX_MODE=eager`); see `benchmark-baseline.md` § Fold index mode.
- Phase 24 / S03 closed: independent verification passed (IDX-03 with controller D-06 override); `L_delivery=M013-S03-done`.

## 2026-10-01 — Process: M013 S02 legal fixture reconnaissance

### Process

### Added

- `docs/architecture/legal-fixture-recon.md`: Proposed review mapping law-nexus fixtures to the current two-axis API (Phase 23 / REC-01..03).

### Trajectory

- Active Slice cleared; `L_delivery=M013-S02-done`. Independent verification passed (`23-VERIFICATION.md`). Note `legal-fixture-recon` has 12 QUOTE-INDEX cases including sunset C01.

## 2026-10-01 — Product: M013 S01 benchmark baseline

### Product

### Added

- `crates/kutha-runtime/benches/baseline.rs` (`[[bench]] harness = false`): relatedTo plain and keyed ingest at N/2N/4N with live_count, fingerprint, claim_supported_at, late retract, 27 sorted metric keys, and fail-closed `KUTHA_BENCH_SABOTAGE=drop_one` (Phase 22 / BASE-01, BASE-03).

### Fixed

- Baseline bench scratch dirs resolve under repo-root `tmp/` via `CARGO_MANIFEST_DIR` (cargo package cwd no longer leaves untracked `crates/kutha-runtime/tmp/`).

### Trajectory

- `L_delivery=M013-S01-done` after Phase 22 verify; Active Slice cleared before S02 lease. Independent verification passed (`22-VERIFICATION.md`).
- `docs/architecture/benchmark-baseline.md`: measured default-N run, ratios, persist/open at three sizes, proposed target N 1000000 and memory budget 13674000000.

### Process

### Added

- Phase 22 / M013-S01 baseline note under `docs/architecture/` (not an ADR); governor docs-coupling satisfied by this CHANGELOG entry.

## 2026-10-01 — Post-M012 review: roadmap reset and M013 lease

### Process

### Added

- `docs/process/gsd-auto-contract.md`: id map (GSD version, harness milestone, ADR), controller protocol for lease-per-phase GSD runs, the CBM plus GitNexus tool pair with measured limits, an independent-verification gate, refactor safety rules, and a planning-artifact budget.
- Ledger rows `I-F9`…`I-F16` in `.kutha/dictionaries/invariants.yaml` (deferred, not enforced).

### Changed

- `.kutha/ROADMAP.md`: M013 (baseline, legal fixture reconnaissance, fold indexes, genesis log, dictionary entries as facts, one write path and facade, independent verification) leased; M014 (typed clocks) and M015 (executable rules and the real-text legal golden fixture) proposed; M002 re-scoped to segmentation, hash chain, manifest, and commit-marker framing, gated by the M015 verdict.
- Code-graph rules: GitNexus runs together with CBM during M013 refactor phases only.
- v0.06 Phases 22 and 23 are planned and plan-checked (two rounds, independent checker); Phases 24–28 carry CONTEXT only and are planned against the code the previous phase leaves. Executors commit per task; the final plan's commit carries the CHANGELOG entry.

### Trajectory

- `docs/architecture/semantic-gap-review.md` gains an addendum (F9–F16, roadmap findings R1–R6, remedies). ADR-010, 011, 013, 014, 040, 041, 050, 051, 060, and 090 gain dated Proposed clarifications or open questions; `honeycomb.yaml` restaged. No cell is Accepted and no lock is rewritten.
- `L_delivery=M013-leased` at milestone open (Active Slice was cleared until S01). Freeze holds except the thawed M013 subset named in STATE. M014, M015, and M002 need their own lease.

## 2026-10-01 — Product: M012 S05 multi-hop derivation (M012 closed)

### Product

- Two-hop `derivation_eligible_at` walk and residual `inverse_knows` cascade are green under named DER oracles (`m012_multi_hop`).

### Process

- Governor DER observe + Phase 21 closed. Harness ROADMAP marks M012 **CLOSED**.

### Trajectory

- `L_delivery=M012-S05-done`; Active Slice **None**. Active Milestone still **M012** until operator clears or leases the next wave. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Process: governor observes DER-01..03

### Process

### Added

- Governor bridge `B-m012-s05` / `m012-s05-multi-hop` and three `observe_cargo.required` names for DER-01..03 (`two_hop_derivation_eligible_when_chain_live`, `two_hop_ineligible_when_root_assert_retracted`, `inverse_knows_cascade_is_residual_spike`). ALL, RULE, ADM, ACT, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 evidence lists include the DER names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S05**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S05 two-hop derivation eligibility

### Product

### Added

- `derivation_eligible_at` walks `caused_by` for a named two-hop hashed fixture. Retracting the root Assert drops hop2 eligibility while the hop2 fact may stay live. persist/open reconstructs that picture from the log.
- `inverse_knows` remains the residual automatic cascade (empty pin, knows-only) with a named guard. Hashed user Behavior chains are the leased follow-on.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S05**. Freeze until M002 unchanged. Not ADR-050 six dictionaries. Not a MATCH compiler or provenance polynomials.

## 2026-10-01 — Process: lease M012 Active Slice S05

### Process

- Active Slice **S05** (multi-hop / n-ary derivation eligibility). Active Milestone stays **M012**; `L_delivery=M012-S04-done` until S05 closes. Last M012 product slice.

### Trajectory

- GSD Phase 21 may edit product crates for DER-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen. Full provenance polynomials stay out of scope.

## 2026-10-01 — Product: M012 S04 thin action record

### Product

- `Op::RecordAction` binds resolved arguments to admission + policy version.
- Action + admission remain queryable AS OF a prior cut after a later policy pin. Named oracles in `m012_action_record`.

### Process

- Governor ACT observe + Phase 20 closed.

### Trajectory

- `L_delivery=M012-S04-done`; Active Slice **None** until S05 leased. Active Milestone stays M012. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes ACT-01..02

### Process

### Added

- Governor bridge `B-m012-s04` / `m012-s04-action-record` and two `observe_cargo.required` names for ACT-01..02 (`action_binds_args_to_admission_and_policy`, `action_and_admission_as_of_prior_cut_after_policy_change`). ALL, RULE, ADM, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-051 evidence lists include the ACT names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S04 thin Action record

### Product

### Added

- Thin Action is `Op::RecordAction` log facts that bind resolved justification arguments to the admission decision and policy version. Named AS OF-after-later-policy oracle. persist/open reconstructs action-entries from the log.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged. Not ADR-050 six dictionaries. Not n-ary derivation.

## 2026-10-01 — Process: lease M012 Active Slice S04

### Process

- Active Slice **S04** (thin action record). Active Milestone stays **M012**; `L_delivery=M012-S03-done` until S04 closes.

### Trajectory

- GSD Phase 20 may edit product crates for ACT-01..02. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes ADM-01..03

### Process

### Added

- Governor bridge `B-m012-s03` / `m012-s03-admission-facts` and three `observe_cargo.required` names for ADM-01..03 (`admission_status_queryable_as_of_cut`, `admission_cites_pinned_policy_version`, `record_justification_invokes_check_admission`). ALL, RULE, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-050 evidence lists include the ADM names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S03 admission and policy meta-facts

### Product

### Added

- Admission status is `Op::RecordAdmission` log facts queryable AS OF a cut. Policy version is `Op::PinPolicy`; `record_justification` invokes `check_admission` and cites the live pin. Retract of those EventIds versions the later cut; persist/open reconstructs entries from the log. Named oracles in `m012_admission_facts`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged. Not ADR-050 six dictionaries. Not a thin Action record.

## 2026-10-01 — Process: lease M012 Active Slice S03

### Process

- Active Slice **S03** (admission status and policy version as meta-facts). Active Milestone stays **M012**; `L_delivery=M012-S02-done` until S03 closes.

### Trajectory

- GSD Phase 19 may edit product crates for ADM-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes RULE-01..03

### Process

### Added

- Governor bridge `B-m012-s02` / `m012-s02-rule-registry` and three `observe_cargo.required` names for RULE-01..03 (`register_rule_pins_behavior_to_definition_hash`, `unknown_or_mismatched_rule_version_fails_closed`, `free_string_rule_version_does_not_append`). ALL, FF6, LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 evidence lists include the RULE names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: M012 S02 rule registry as log facts

### Product

### Added

- Rule definitions are `Op::RegisterRule` log facts. `Behavior.rule_version` equals `rule_definition_hash` of the registered body. Unknown, mismatched, and free-string pins fail closed on emit and eligibility. Retract of a registry EventId versions the later cut; persist/open reconstructs the hash from the log. Named oracles in `m012_rule_registry`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged. Not ADR-050 six dictionaries.

## 2026-10-01 — Process: lease M012 Active Slice S02

### Process

- Active Slice **S02** (rule registry; `rule_version` = definition hash). Active Milestone stays **M012**; `L_delivery=M012-S01-done` until S02 closes.

### Trajectory

- GSD Phase 18 may edit product crates for RULE-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Product: M012 S01 allowlist as log facts

### Product

- `Op::AllowRelation` appends versioned allowlist facts; `Runtime::admit` consults the fold cut (YAML seed is fallback only).
- Retract by EventId versions the allowlist; persist/open reconstructs YAML-absent names from the log. Named oracles in `m012_allowlist_facts`.

### Process

- Governor `m012-s01-allowlist-facts` + FSM observe for three ALL tests. GSD Phase 17 closed.

### Trajectory

- `L_delivery=M012-S01-done`; Active Slice **None** until S02 leased. Active Milestone stays M012. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: governor observes ALL-01..03

### Process

### Added

- Governor bridge `B-m012-s01` / `m012-s01-allowlist-facts` and three `observe_cargo.required` names for ALL-01..03 (`allow_relation_appends_as_log_fact`, `retract_allow_relation_drops_admit_at_later_cut`, `admit_consults_fold_allowlist_not_yaml_alone`). FF6 `ff6_unknown_relation_does_not_append` stays required. LOG, REF, ING, DUR, TIME, and HOT names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 evidence lists include the ALL names. ADR-050 map stays Proposed and delivery frozen. Active Slice remains **S01**. Freeze until M002 unchanged.

## 2026-10-01 — Product: allowlist entries as log facts

### Product

### Added

- Relation allowlist entries are `Op::AllowRelation` log facts. `Runtime::admit` consults the fold cut for a live name; YAML/env remains a seed for names never logged. Unknown relations and empty allow-names still fail closed. Retract of an allow-entry EventId versions the later cut. Named oracles in `m012_allowlist_facts`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S01**. Freeze until M002 unchanged. Not ADR-050 six dictionaries.

## 2026-10-01 — Process: lease M012 Active Slice S01

### Process

- Active Slice **S01** (relation allowlist as log facts). Active Milestone stays **M012**; `L_delivery=M012-leased` until S01 closes.

### Trajectory

- GSD Phase 17 may edit product crates for ALL-01..03. Six dictionaries / Cypher / HNSW / M002 stay frozen.

## 2026-10-01 — Process: lease M012 dictionaries as facts

### Process

- Harness lease: Active Milestone **M012**; Active Slice **None**; `L_delivery=M012-leased`. Phase stays **H5**.
- Thawed subset named on STATE/ROADMAP: versioned relation-allowlist entries, rule registry (`rule_version` = definition hash), admission status + policy version as meta-facts, thin action record, multi-hop derivation eligibility. ADR-050 six dictionaries, Cypher/HNSW, legal pack, M002 remain frozen.

### Trajectory

- Product plane may land F4/F5 under M012. Governor green is not ADR Accepted. Honeycomb stays Proposed.

## 2026-10-01 — Product: M012a S06 hot indexes (M012a closed)

### Product

- `GraphFold` indexes facts by `valid_from` / `claim_id`; `as_of` / `claim_supported_at` skip non-overlapping facts (HOT-01).
- Untyped CSR builds via `CsrMaterializer`; CSR remains a droppable lease (HOT-02/03). Named oracles in `m012a_hot_indexes`.

### Process

- Governor `m012a-s06-hot-indexes` + FSM observe for three HOT tests. GSD Phase 16 closed. Harness ROADMAP marks M012a **CLOSED**.

### Trajectory

- `L_delivery=M012a-S06-done`; Active Slice **None**. Active Milestone still **M012a** until operator clears or leases M012. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Process: governor observes HOT-01..03

### Process

### Added

- Governor bridge `B-m012a-s06` / `m012a-s06-hot-indexes` and three `observe_cargo.required` names for HOT-01..03 (`as_of_and_claim_supported_at_skip_non_overlapping_facts`, `discard_csr_lease_and_snapshot_rebuild_keeps_as_of`, `csr_lease_at_builds_via_materializer`). LOG, REF, ING, DUR, and TIME names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-040 and ADR-041 evidence lists include the HOT names. Active Slice remains **S06**. Freeze until M002 unchanged. Not a GED-class fork-diff API.

## 2026-10-01 — Product: droppable CSR via Materializer

### Product

### Changed

- Untyped `csr_lease_at` builds through `CsrMaterializer` then unloads. `from_fold` consumes the fold live-fact iterator. Dropping the lease does not change `as_of` or fingerprint. Indexes and CSR stay in-memory leases, not snapshot SoT. Typed CSR remains `TypedCsrLease::from_fold` (named spike limit).

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S06**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Product: fold-internal hot as_of and claim_supported_at

### Product

### Added

- Fold-internal VT and claim maps so `as_of` / `claim_supported_at` skip non-overlapping facts. Answers match a brute-force `is_live_at` filter of `facts()`. Maps skip-serialize, rebuild from facts after snapshot open, and are not fingerprint input.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S06**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S06 for Phase 16

### Process

- Active Slice **S06** (fold-internal hot indexes for `as_of` / `claim_supported_at`). Active Milestone stays **M012a**; `L_delivery=M012a-S05-done` until S06 closes.

### Trajectory

- GSD Phase 16 may edit product crates for HOT-01..03. Last M012a slice. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S04+S05 verify, persist, time scale

### Product

- `open` verifies snapshot via `replay_check` (tampered snapshot → `InvalidData`).
- `persist` replaces `events.jsonl` atomically (temp + rename). `Define` ids are name-stable (`Event::stable_define`).
- `TimeScale::{YearCe, LogSequence}` declared; fixture oracles map years and TT as log sequence.

### Process

- Governor `m012a-s04-verify-persist` / `m012a-s05-time-scale` + FSM observe for five DUR/TIME tests. GSD Phase 15 closed.

### Trajectory

- `L_delivery=M012a-S05-done`; Active Slice **None** until S06 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Process: governor observes DUR-01..03 and TIME-01..02

### Process

### Added

- Governor bridges `B-m012a-s04` / `m012a-s04-verify-persist` and `B-m012a-s05` / `m012a-s05-time-scale`, plus five `observe_cargo.required` names for the DUR and TIME oracles. LOG, REF, and ING names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-012 and ADR-013 evidence lists include the new names. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: declared YearCe valid-time and log-sequence transaction-time

### Product

### Added

- `TimeScale::{YearCe, LogSequence}` with `VALID_TIME_SCALE` and `TRANSACTION_TIME_SCALE`. Fixture integers 2015/2017/2021 are Gregorian years CE; `Event.ingested_at` is log sequence, not wall-clock. Named tests in `m012a_time_scale`. No TT-to-wall calendar map.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: verify-on-open, atomic jsonl persist, stable Define ids

### Product

### Added

- `open` verifies snapshot identity against the log (`replay_check`) and rejects a tampered snapshot as `InvalidData` / `ReplayDivergenceError`.
- Dictionary `Op::Define` ids are name-stable (SHA-256 of the ASCII prefix `kutha-define-id` plus the term bytes) across persist/open/persist. Live `intern()` still mints UUID v7.

### Changed

- `persist` replaces `events.jsonl` by writing a same-directory temp file, syncing, then renaming. A failed temp write leaves the previous complete jsonl in place.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S04 for Phase 15

### Process

- Active Slice **S04** (verify-on-open, atomic persist, stable Define). Phase 15 GSD wave also covers S05 declared time scale. Active Milestone stays **M012a**; `L_delivery=M012a-S03-done` until Phase 15 closes.

### Trajectory

- GSD Phase 15 may edit product crates for DUR-01..03 and TIME-01..02. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S03 idempotent ingest

### Product

- Assert carries optional `delivery_key` and `polarity`; identical keyed Assert does not mint a second support.
- `conflict_report_at` partitions by stored polarity. Named oracles in `m012a_idempotent_ingest`.

### Process

- Governor `m012a-s03-idempotent-ingest` + FSM observe for three ING tests. GSD Phase 14 closed.

### Trajectory

- `L_delivery=M012a-S03-done`; Active Slice **None** until S04 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Process: governor observes ING-01..03

### Process

### Added

- Governor bridge `B-m012a-s03` / check `m012a-s03-idempotent-ingest` and three `observe_cargo.required` names for the ING-01..03 oracles. LOG and REF names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-013 evidence lists include the new names. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: claim vs support identities

### Product

### Added

- Named ING-02 oracle `claim_id_distinct_from_support_slot`. `Op::Assert` and `Fact` rustdoc name claim (proposition), minting EventId (support slot), and `delivery_key` (retry) as three identities.

### Changed

- `conflict_report_at` takes claim, transaction-time, and valid-time only and partitions live supports by stored `Fact.polarity`. Correct/CorrectInterval residuals copy polarity; an object-changing patch row flips Positive/Negative. Leased e2e fixture Asserts store Positive.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: delivery-key Assert ingest

### Product

### Added

- `Op::Assert` carries an optional `delivery_key` and `SupportPolarity` (default none). Re-emitting an identical Assert with the same non-empty key returns the original `EventId` and does not append a second support.
- Same delivery key with a different Assert payload fails closed (`DeliveryKeyConflict`) and does not append. Persist/open still honors the durable key.

### Changed

- Fold `Fact` copies `delivery_key` and polarity; both mix into the event digest and fold fingerprint.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S03

### Process

- Active Slice **S03** (idempotent ingest / delivery keys; claim ≠ support slot). Active Milestone stays **M012a**; `L_delivery=M012a-S02-done` until S03 closes.

### Trajectory

- GSD Phase 14 may edit product crates for ING-01..03. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S02 stable references

### Product

- Retract, Correct, and CorrectInterval target minting `EventId` (`Fact.event_id`); fold-local `seq` stays a lease.
- Justification cites and admission key on EventId across fork/rebuild. Named oracles in `m012a_stable_refs`.

### Process

- Governor `m012a-s02-stable-refs` + FSM observe for three REF tests. GSD Phase 13 closed.

### Trajectory

- `L_delivery=M012a-S02-done`; Active Slice **None** until S03 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Product + Process: rebuilt-fold EventId oracle

### Product

### Added

- Named REF-03 oracle `rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads`: dropping a filler Assert renumbers fold-local seqs; the same Retract and JustificationCite Events still bind.

### Process

### Added

- Governor bridge `B-m012a-s02` / check `m012a-s02-stable-refs` and three `observe_cargo.required` names for the REF-01..03 oracles. LOG names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-061 evidence lists include the new names. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: EventId justification cites

### Product

### Changed

- `Op::JustificationCite` and `check_admission` name source supports by minting `EventId` only. Fold-local seq is no longer in the durable cite payload.
- `fork_at` of a prefix that contains the cite still admits after the parent retracts that EventId.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: minting EventId retract and correct

### Product

### Changed

- `Op::Retract`, `Op::Correct`, and `Op::CorrectInterval` target the minting `EventId` stored on `Fact.event_id`. Fold-local `seq` remains a lease index (CSR / conflict report), not the operator key.
- `Runtime::emit` returns `UnknownFact { event_id }` and does not append when no Fact was minted by that id. Interval patches that do not intersect still fail closed as `IntervalPatchRejected { event_id }`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged. Not ADR-011 Accepted.

## 2026-10-01 — Process: lease M012a S02

### Process

- Active Slice **S02** (stable EventId / proposition refs for Retract, Correct, and justification cites). Active Milestone stays **M012a**; `L_delivery=M012a-S01-done` until S02 closes.

### Trajectory

- GSD Phase 13 may edit product crates for REF-01..03. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-09-30 — Product: M012a S01 log-native SoT

### Product

- Quantum outcomes, justification cites, and resume persist as log records (`Op::QuantumOutcome`, `Op::JustificationCite`); sidecars are droppable leases.
- `provenance_fingerprint` mixes those log-native bytes (`kutha-prov-log-native`). Named oracles in `m012a_log_native`.

### Process

- Governor check `m012a-s01-log-native` + FSM observe names for the three LOG tests. GSD Phase 12 closed.

### Trajectory

- `L_delivery=M012a-S01-done`; Active Slice **None** until S02 leased. Active Milestone stays M012a. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Log-native SoT

### Product

### Added

- `Op::QuantumOutcome` fold-noop events on the product log. After persist, deleting `quantum_outcomes.jsonl` still reconstructs Zero / Partial / Full (`discard_outcomes_sidecar_keeps_reconstructible_disposition`).
- `Op::JustificationCite` fold-noop events. After persist, deleting `justifications.jsonl` still reconstructs cites and `check_admission` (`discard_justifications_sidecar_keeps_admission_and_resume`). Resume is a log-native `Op::QuantumOutcome` with `disposition: resume`.

### Changed

- Quantum outcome and justification SoT is the event log. Sidecar JSONL files remain written leases; open hydrates from log Events when they exist and does not let an empty sidecar wipe them.
- `provenance_fingerprint` domain tag is `kutha-prov-log-native`; the mix includes Behavior plus log-native outcome and justification Event bytes (`provenance_fingerprint_moves_when_log_native_record_bytes_change`).

### Process

### Added

- Governor bridge `B-m012a-s01` / check `m012a-s01-log-native` and three `observe_cargo.required` names for the LOG-01..03 oracles.

### Trajectory

- Honeycomb stays **Proposed**. Harness lease remains M012a / S01. Freeze until explicit M002 is unchanged. Not ADR-014 Accepted.

## 2026-09-30 — Process: lease M012a

### Process

- Harness lease: Active Milestone **M012a**; Active Slice **S01** (log-native outcomes/justifications); `L_delivery=M012a-leased`. Phase stays **H5**. Freeze until explicit M002 unchanged.
- Governor Active Milestone patterns accept letter-suffix ids (`M012a`). `.kutha/ROADMAP.md` marks M011 CLOSED and M012a ACTIVE with S01–S06 checkboxes.

### Trajectory

- Product plane may land single-log SoT + stable-reference work (F1–F3, F6 verify/atomic/Define, F7, F8). F4/F5 wait for M012. Rocks / Cypher / HNSW / legal pack stay frozen. Honeycomb stays Proposed.

## 2026-09-30 — Process: semantic governor

### Process

### Added

- `kutha-gov selftest` copies the tree once, mutates only that copy, and fails closed on UNPROVEN or VACUOUS. FSM `run_selftest` records `h5_selftest`; `ci` labels H5 from that evidence.
- Kinds `rust_test_asserts`, `cite_equals`, `refs_resolve`, and `file_max_lines`. Named `fn`-needle checks and honeycomb evidence resolve through asserting `#[test]` bodies. `cite-lease` also matches `L_map=`, `L_capability=`, and Active Milestone M### phrasing to STATE.
- Intake section “What a green check proves”: presence is not meaning; every check is mutation-provable or carries a skip reason. META allowlists the new kinds and `run_selftest`. `AGENTS.md` Commands includes `uv run kutha-gov selftest` (line budget ≤110, no lease tokens).

### Changed

- `run_checks` default budget and `.env.example` are 48 so `ci` cannot truncate Cui slices (32 would drop checks).
- Control-loop rows `I-F1-outcomes` … `I-F8-hot-reads` record F1–F8 as `disposition: deferred` (`until` names M012a / M012 / M002). Recorded, not enforced.
- ROADMAP H5 checkbox is checked; `h4-lease` requires the done form without promoting ADR-090. Dogfood ladder marks H5 **Done**. It remains the current rung (no H6).

### Fixed

- Live-repo selftest found no vacuous named test and no vacuous YAML row (`docs-coupling` skips: depends on the live git diff).
- Harness tests no longer leak `/tmp/kutha-selftest-mini-*` and `/tmp/kutha-rust-test-*` directories (cleanup registered with `atexit`).
- Harness scratch is repo-local: `scripts/tests/conftest.py` and `kutha-gov selftest` create temp trees under gitignored `tmp/` instead of system `/tmp`; selftest's tree copy skips root `tmp/`.

### Trajectory

- Operator Phase stays **H5**, now closed on the dogfood ladder. Active Milestone stays M011; Active Slice None; freeze and honeycomb **Proposed** unchanged. Governor green is not product correctness, not ADR Accepted, and not L_capability. F1–F8 are product gaps deferred to unleased M012a (F4/F5 until STATE names M012; F6 durability protocol until STATE names M002).

## 2026-09-30 — Process: semantic-gap review and Proposed honeycomb notes

### Process

- Architecture note `docs/architecture/semantic-gap-review.md` traces F1–F8 onto the honeycomb (per-cell verdicts, D1–D10 lock assessment). It is not an ADR and not a lease.
- Twelve honeycomb ADRs received dated 2026-09-30 Proposed clarifications and/or open questions. Status stays Proposed. ADR README points at the review.
- Later-milestones proposed order (non-authoritative): M012a (single-log SoT and stable refs) → M012 (dictionaries as facts) → M002 (log durability protocol first). Not leased.

### Trajectory

- Honeycomb stays Proposed. No cell promoted. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Process: H5 operator lease and AGENTS diet

### Process

- Operator lease Phase named **H5** (semantic governor + lean agent context). Dogfood ladder and `h4-lease` / `dogfood` needles name H5 without promoting ADR-090.
- `AGENTS.md` diet: 194 lines down to about 100 durable operating rules. Volatile notes relocated (P0 spike inventory, Leapfrog, applicability Agent notes, README Layout). Stale S03 live-lease caption replaced with a STATE pointer.

### Trajectory

- Operator Phase is **H5**; Active Milestone stays M011; Active Slice None; freeze and honeycomb Proposed unchanged. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: end-to-end candidate fixture

### Product

### Added

- Durable `justifications.jsonl` sidecar (`JUSTIFICATIONS_REL`) beside outcomes; `record_justification` / `check_admission` fail-closed on stale `source_fact_seqs`, ineligible derivation, or `rule_version` mismatch. Missing sidecar loads empty (cannot admit).
- `conflict_report_at` reports `positive_supports` and `negative_supports` without picking a winner.
- Named tests `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, `e2e_incremental_matches_reconstruct_after_discarding_leases` (FIX-01, FIX-02, FIX-03).

### Process

- FSM `observe_cargo.required` plus `m011-e2e` / `B-m011-e2e` needles for the three named FIX tests (GATE-01).
- Harness lease advanced: **S08** delivered (`L_delivery=M011-S08-done`); Active Slice **None**. M011 tail (S04–S08) closed. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S08 delivered** (`L_delivery=M011-S08-done`); Active Slice cleared. ADR-013, ADR-011, ADR-012, and ADR-040 remain Proposed (not Accepted, not L_capability). Evidence names the FIX oracles `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, and `e2e_incremental_matches_reconstruct_after_discarding_leases`. Green governor is not ADR Accepted and not L_capability. Conflict report is not four-valued logic.

## 2026-09-30 — Process: lease Active Slice S08 after S07 delivery

### Process

- Harness lease advanced: **S07** remains delivered (`L_delivery=M011-S07-done`); Active Slice **S08** (end-to-end candidate fixture — semantic-contract observations 1–5 at named cuts). Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S08**; honeycomb cells stay Proposed (not Accepted, not L_capability). S07 provenance tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: provenance check apart from state replay

### Product

### Added

- `Op::Behavior.rule_version` (`#[serde(default)]`); fold still ignores lineage (`name` / `caused_by` / `rule_version`). `Runtime::provenance_fingerprint` / `provenance_check` sit beside unchanged `replay_check` (ADR-060 obligation 2; state replay stays obligation 1).
- Named test `provenance_detects_caused_by_swap_when_state_fingerprint_matches` (PROV-01): swap `caused_by` among valid priors; state fingerprint matches; provenance digest moves; both `replay_check` Ok.
- Named test `provenance_detects_rule_version_change_when_state_fingerprint_matches` (PROV-02): change only `rule_version` (`r1` → `r2`); state fingerprint matches; provenance digest moves; no execution replay.

### Process

- FSM `observe_cargo.required` plus `m011-provenance` / `B-m011-provenance` needles for named PROV-01 and PROV-02 tests (GATE-01).
- Harness lease advanced: **S07** delivered (`L_delivery=M011-S07-done`); Active Slice **None**. S08 remains unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S07 delivered** (`L_delivery=M011-S07-done`); Active Slice cleared until an explicit S08 lease. ADR-060 and ADR-011 honeycomb evidence names `provenance_detects_caused_by_swap_when_state_fingerprint_matches` and `provenance_detects_rule_version_change_when_state_fingerprint_matches`; map stays Proposed on both (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability. `replay_check` Ok is not execution replay.

## 2026-09-30 — Process: lease Active Slice S07 after S06 delivery

### Process

- Harness lease advanced: **S06** remains delivered (`L_delivery=M011-S06-done`); Active Slice **S07** (provenance and rule-version check apart from state replay). S08 remains unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S07**; ADR-040 and ADR-060 remain Proposed (not Accepted, not L_capability). S06 typed-CSR tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: typed CSR lease (labels + support multiplicity)

### Product

- `TypedEdge` / `TypedCsrLease` beside untyped `CsrLease`; `Runtime::typed_csr_lease_at` + `edges_out` project one edge per live Fact (`relation`, `object`, `claim_id`, `fact_seq`). Untyped `CsrLease::from_fold` object-only dedup unchanged.
- Named test `typed_csr_preserves_relation_labels_and_support_multiplicity` (CSR-01): ≥2 relations and ≥2 supports visible on typed edges; untyped neighbors stay a singleton object.
- Named test `untyped_csr_neighbor_set_and_ff5_still_hold` (CSR-02): untyped neighbor-set + FF3-style drop/rebuild; existing FF5 statute observe stays green.

### Process

- FSM `observe_cargo.required` plus `m011-typed-csr` / `B-m011-typed-csr` needles for named CSR-01 and CSR-02 tests (GATE-01).
- Harness lease advanced: **S06** delivered (`L_delivery=M011-S06-done`); Active Slice **None**. S07–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S06 delivered** (`L_delivery=M011-S06-done`); Active Slice cleared until an explicit S07+ lease. ADR-040 and ADR-041 honeycomb evidence names `typed_csr_preserves_relation_labels_and_support_multiplicity` and `untyped_csr_neighbor_set_and_ff5_still_hold`; map stays Proposed on both (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Process: lease Active Slice S06 after S05 delivery

### Process

- Harness lease advanced: **S05** remains delivered (`L_delivery=M011-S05-done`); Active Slice **S06** (typed CSR lease keeps labels and support multiplicity). S07–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S06**; ADR-014 remains Proposed (not Accepted, not L_capability). S05 outcome tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-29 — Product: persisted quantum outcome sidecar

### Product

- Authoritative `quantum_outcomes.jsonl` sidecar beside the log (written by `store::persist` after events; loaded on every `store::open` path). Not a droppable lease — discarding `snapshot.json` keeps outcome history (D-O1 / ADR-010).
- Dispositions `Zero` / `Partial` / `Full` from abort × `events_in_quantum`; `Runtime::record_resume` appends an explicit `Resume` row (open never auto-resumes). Call `Ok(QuantumOutcome)` remains not completion proof (ADR-014).
- Named test `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open` (OUT-01): budgets 0/1/2 via `Runtime::new(0|1|default)` survive persist→open as Zero/Partial/Full.
- Named test `crash_after_prefix_has_no_terminal_success_until_explicit_resume` (OUT-02): after a committed prefix without a Full row, open invents no terminal success until `record_resume`.

### Process

- FSM `observe_cargo.required` plus `m011-quantum-outcome` / `B-m011-quantum-outcome` needles for named OUT-01 and OUT-02 tests (GATE-01).
- Harness lease advanced: **S05** delivered (`L_delivery=M011-S05-done`); Active Slice cleared. S06–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S05 delivered** (`L_delivery=M011-S05-done`); Active Slice cleared until an explicit S06+ lease. ADR-014 honeycomb evidence names the OUT-01/OUT-02 tests; map stays Proposed (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.

### Product

- `Op::CorrectInterval { fact_seq, object, patch_from, patch_to }`: explicit interval patch on a live fact. Fold invalidates the original row (VT bounds unchanged) and pushes prefix residual, clipped replacement, and suffix residual sharing `claim_id`.
- Named test `interval_patch_leaves_vt_2012_and_2021_residuals` (CORR-01): after patch `[2015, 2020)` on `[2010, ∞)`, `as_of(2012)` and `as_of(2021)` keep source `a` as `P`; interior `as_of(2017)` is `P-prime`.
- Named test `whole_version_correct_does_not_invent_residuals` (CORR-02): whole-version `Op::Correct` with narrower VT `[2015, 2020)` on `[2010, ∞)` invents no live residual of `P` at VT 2012 or 2021; interior `as_of(2017)` is the replacement only.
- Fail-closed interval-patch writes: unknown `fact_seq` (`UnknownFact`); non-intersecting (including half-open touching endpoints), inverted, and not-live targets (`IntervalPatchRejected`) do not append.

### Process

- FSM `observe_cargo.required` plus `m011-partial-correction` / `B-m011-partial-correction` needles for named CORR-01 and CORR-02 tests (GATE-01).
- Harness lease advanced: **S04** delivered (`L_delivery=M011-S04-done`); Active Slice **S05** (persisted quantum outcome and continuation). S06–S08 remain unleased. Not ADR Accepted; not closed delivery.

### Trajectory

- Active Slice is **S05**; ADR-013 remains Proposed (not Accepted, not L_capability). S04 residual tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-16 — Product + Process: M011 S03 thin P→Q eligibility

### Product

- `Runtime::derivation_eligible_at`: Behavior-derived claim `Q` is eligible at a TT×VT cut iff `Q` is still live and its premise claim (from `caused_by`) still has a live support.
- Named test `derived_q_loses_eligibility_when_last_premise_support_withdrawn`: two supports for `P`; retract one → `Q` stays eligible; retract last → eligibility drops; historical `Q` and earlier TT cut remain.

### Process

- Lease `L_delivery=M011-S03-done`. FSM observe + `m011-claim-supports` needles cover S03.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record the thin oracle. Cells remain Proposed; full provenance polynomials and M002 freeze unchanged.

## 2026-09-16 — Process: session handoff after M011 S02

### Process

- Durable handoff at `.compound-engineering/artifacts/handoffs/m011-s02-cbm-mcp.md`. Project `.cursor/mcp.json` points Cursor at local `codebase-memory-mcp` stdio (agent sessions were missing user MCP servers).

## 2026-09-16 — Product + Process: M011 S02 unknown claim and replay lineage

### Product

- `emit` rejects `Assert.claim` that never appeared as `Fact.claim_id` (`UnknownClaim`; log does not grow).
- `replay_check` rejects `Behavior.caused_by` that is not an earlier event id (`BrokenLineage`). Named tests `unknown_claim_does_not_append`, `replay_rejects_behavior_without_prior_cause`.

### Process

- Lease `L_delivery=M011-S02-done`. FSM observe + `m011-claim-supports` needles cover S02.

### Trajectory

- ADR-060 / semantic-contract-validation record causal-ref check. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-16 — Product + Process: M011 S01 independent claim supports

### Product

- `Op::Assert.claim` / `Fact.claim_id`: absent claim opens a new claim (`event.id`); `Some(id)` attaches another support. Fold helpers `live_support_count` / `claim_supported_at`. Correct inherits `claim_id`.
- Named test `retracting_one_support_leaves_claim_supported`.

### Process

- Leased M011; S01 delivered (`L_delivery=M011-S01-done`). Governor `m011-claim-supports` + FSM observe required.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record S01. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-16 — Product + Process: M010 S03 live intern Define; M010 closed

### Product

- `Runtime::intern` appends `Op::Define` for newly created terms; bootstrap `knows`/`knownBy` stay silent. `graph_len()` and graph-oriented `snapshot.log_offset`; `from_snapshot` strips Define before applying the tail.
- Named test `intern_appends_define_for_new_terms_only`. Allowlist tests assert graph growth via `graph_len`.

### Process

- M010 S03 delivered then milestone closed (`L_delivery=M010-closed`). FSM observe requires the live-intern test; `m010-semantic-open` cites S03 needles.
- `pointer_in_other_file` supports `require_template` so an Active Slice must remain unchecked on ROADMAP (blocks done-but-leased drift).

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record S03. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Product + Process: M010 S02 logged term definitions

### Product

- Persist encodes `Op::Define` (dictionary order) on the durable event stream. `open` recovers intern meanings without `snapshot.json` and without `terms.jsonl`. Snapshot open strips Define so log_offset still matches the in-memory graph log.
- Named test `open_without_snapshot_or_terms_file_recovers_from_define_ops`.

### Process

- Lease moved to M010 S02. Governor observe accepts `module::tests::name` cargo lines. `m010-semantic-open` cites the Define recovery test.

### Trajectory

- ADR-011 / semantic-contract-validation record S02. ADR-011 honeycomb evidence names the new test. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Product + Process: lease M010 S01 semantic recovery open

### Product

- `persist` retains authoritative `terms.jsonl`; `open` recovers fold + intern meanings without `snapshot.json`. Named test `open_without_snapshot_recovers_intern_meanings`.

### Process

- STATE/ROADMAP/README lease M010 S01. Governor `m010-semantic-open` + FSM observe required. Bridge cites store.rs.

### Trajectory

- ADR-011 and semantic-contract-validation note M010 S01 limit (companion terms file; event-logged term ops deferred to S02). Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Process: live H4 membership emit on CI quanta

### Process

- `emit_log` syncs tip process-relation membership into `.kutha/events.jsonl` when the tip set differs from the last logged `allows` edition (H4 dogfood no longer test-only).
- kutha-changelog skill example lease text updated past H3→H4.

### Trajectory

- Cells and freeze unchanged. Active Milestone remains None; no M002 lease.

## 2026-09-15 — Process: post-M001 lease debt wave 1

### Process

- Added governor kind `when_match_then_match` and check `idle-delivery-closed` so Active Milestone None requires `L_delivery=*-closed`.
- Retargeted freeze finding text to the live M002 lease gate; extended `docs-coupling` so STATE and architecture-note diffs require CHANGELOG.
- Indexed `kutha-graph` in GitNexus for session blast-radius queries.

### Trajectory

- Refreshed `docs/architecture/semantic-contract-validation.md` lease wording to Phase H4 / M001-closed (no Active Milestone). Cells and freeze unchanged.

## 2026-09-15 — Process: close M001 milestone

### Process

- Cleared Active Milestone after S01–S03 delivery (`Active Milestone: None`). ROADMAP marks M001 **CLOSED**. Freeze and H4 phase unchanged.

### Trajectory

- `L_delivery=M001-closed`. L_map stays Proposed; L_capability stays ff5-green. No M002 lease.

## 2026-09-15 — Product + Process: H4 process overlay dogfood

### Product

- Tenant ingest maps `process.relations` / `allows` snapshot editions onto `processAllows` and chains intervals. AS OF a prior cut still sees a membership set that later editions dropped. One FF6 name; not ADR-050's six kinds.

### Process

- Tip YAML stays the admit lease. Reserved process relation `allows` logs one sorted membership snapshot per edition. Pytest proves tip-without-`status` rejects `status` without editing the repo tip. Governor `h4-lease` / `h4-membership-as-of` and FSM `observe_cargo.required` name the H4 evidence.
- CodeRabbit follow-ups: reject comma-bearing process relation names (CSV membership ambiguity); `pointer_in_other_file` names the real `other` path; `load_map` rejects non-mapping cells; ROADMAP H4 wording matches shipped overlay scope.

### Trajectory

- ADR-042/060 wording clarified (predicate filter language; `replay_check` unavailable outcome marked proposed). Lease still Phase H4; freeze unchanged.

### Trajectory

- Cells remain **Proposed**. ADR-090 ontology stays frozen. Delivery lease is **H4** overlay dogfood, not a legal pack.

## 2026-09-15 — Process: H4 session handoff after architecture POV

### Process

- Session continuity: `.compound-engineering/artifacts/handoffs/h4-after-architecture-pov.md` records the architecture POV (accept lifecycle self-correction; reject Stages 1–4 as delivery order) and the next CE path: H4 overlay brainstorm/plan, not a legal pack.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: durable CE artifacts stay under docs_root

### Process

- Standing rule: session-surviving Compound Engineering writes (plans, handoffs, sweep state) go under `.compound-engineering/artifacts/`, not `/tmp`. `ce-handoff` must use `.compound-engineering/artifacts/handoffs/<topic>.md`; the skill's default managed store is OS-evictable.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: CE skill routing for Russian chat

### Process

- Always-on Cursor rule `.cursor/rules/ce-skills-ru.mdc` maps Russian verbs (коммит, changelog, PR, план, …) onto Compound Engineering skills and `kutha-changelog`. Plugin-cache `description` fields stay English; do not patch them. Git user-rule remains safety-only; commit workflow is `ce-commit`.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: changelog skill (not a release bumper)

### Process

- Adapted baoyu `release-skills` into `.cursor/skills/kutha-changelog/`: dated `CHANGELOG.md` on product / process / Trajectory planes. Versions stay `0.0.0`. Commit handoff is **ce-commit**; this skill does not tag, `gh release`, or push.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: L_map compact honeycomb index

### Added

- Compact L_map index `.kutha/dictionaries/honeycomb.yaml` (`kutha-map-honeycomb/v1`): D1–D10 plus all opened ADR cells, with orthogonal `map` / `delivery` / `capability` stages and `depends_on` / `locks` / `evidence` links.
- `kutha-gov map` (optional cell id, neighborhood dump, `--format json`) so agents can load the graph without a second markdown coverage table.
- Governor check `honeycomb-ledger`, FSM state `load_honeycomb`, kind `glob_paths_in_file`, and `yaml_map_list` list/path/embed/glob steps. ADR diffs must include the index (`docs-coupling`).

### Process

- The map dictionary is a fourth intake surface: governor validates shape and links; it does not accept cells, enforce HNSW/Cypher, or treat the index as a delivery backlog.
- Control-loop claims stay in `invariants.yaml`; product freeze/tests stay in `bridges.yaml`. Honeycomb rows are not YAML checks.

### Trajectory

- Cells remain **Proposed**. Delivery annotations (`spike` / `frozen` / `map-only`) and named FF evidence do not promote L_map or L_capability. Lease stays H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: governor intake split and docs entry

### Added

- Root `README.md` and `CLAUDE.md` (`@AGENTS.md`) as the human / Claude Code entry points.
- Harness `kutha-gov precommit` (dictionary checks only: no cargo quantum, no JSONL) plus `--check ID`, `git_path_implies` coupling, and `.pre-commit-config.yaml`. Neighbor surfaces: daily-archive `--check-only`, law-nexus `--check`; not a baoyu release bumper.
- Control-loop ledger `.kutha/dictionaries/invariants.yaml` and bridge ledger `.kutha/dictionaries/bridges.yaml` (`docs/process/governor-intake.md`). Kind `yaml_map_list` keeps each check id in exactly one ledger.

### Changed

- RuVector adapter citations in ADR-012/014/042/052 and the CE adaptation note: in-repo cards + `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`; no machine-local vendor paths. Mapping does not lift the HNSW freeze.

### Process

- Pre-commit profile is FAST-ONLY: ruff on `scripts/` and `kutha-gov precommit`. Product `cargo test` remains `kutha-gov ci`. Versions stay `0.0.0`. CHANGELOG remains plane-dated history, not GitHub Releases.
- Control loop and Kutha requirements stay partitioned: honeycomb / fitness live in ADRs, STATE, and crates tests; `map-only` and `capability` are not process dispositions. Bridges may cite product freeze or named tests; they do not copy L_map or L_capability.

### Trajectory

- Cells remain **Proposed**. Product crates untouched. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay).

## 2026-09-13 — Architecture: ruVector adapter mapping (L_map)

### Research & ADRs

- Mapped five named adapter crates onto already-open honeycomb cells via `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`. Literature bound stays **163 cards**; no new matrix rows.
- **P-Temporal-Tensor** → ADR-012 D012-5 (vector tiering lease ≠ fact history).
- **P-Retrieval-Receipt** → ADR-014 D014-5 (`paper-constant-size-evidence`; write quantum receipt ≠ read evidence). `ruvector-proof-gate` remains a D014-2 cousin, not a new decision.
- **P-HNSW** → ADR-042 D042-2/3 citing existing cards `ruvector-hnsw-delete-repair`, `paper-acorn-predicate-subgraph`, `paper-navix-filtered-hnsw`. Mapping does not lift the HNSW freeze.
- **P-Agent-Memory** → ADR-052 D052-4 (coherence score ≠ fact validity).
- ADRs cite in-repo cards and the adaptation note. Machine-local vendor paths are not architecture SoT.

### Trajectory

- Cells remain **Proposed**. Product crates untouched. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). This mapping does not authorize HNSW, Cypher, or M002.

## 2026-08-18 — Wave 6: same-second tenant AS OF + one cargo compile per quantum

### Product

- Tenant ingest reports `last_valid_from` as the **emitted** cut after monotonic chaining. Two status rows with the same unix second no longer query AS OF a closed window (`as_of_match=0`).
- Test `h2_same_second_status_as_of_uses_emitted_cut` (TDD).

### Process

- `observe_cargo` optional `build:` compiles `kutha-tenant` once after `cargo test`. `emit_tenant` executes that binary (`KUTHA_TENANT_BIN`, else `$CARGO_TARGET_DIR/debug/kutha-tenant`). No second `cargo run`.
- Checks `plane-mix-dicts` and `tenant-bin`: process/product relation schemas stay distinct; FSM must not `cargo run` the tenant.
- `kutha-gov py` treats ty warnings as errors (`--error-on-warning`).

### Trajectory

- Phase **H3**. Next: H4 waits on ADR-090 overlay (do not start a legal pack). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 5: H3 process allowlist + one source for FF names

### Process

- Fail-closed process writes: `.kutha/dictionaries/relations.yaml` (`schema: kutha-harness-relations/v1`). Unknown relation does not append to `.kutha/events.jsonl`.
- FSM loads `relations.yaml` before checks. Override: `KUTHA_HARNESS_RELATIONS_PATH`.
- Named FF tests are CLI **evidence**, not process relations (aligned with H2 tenant mapping of `cargo` only).
- Kind `yaml_needles_in_glob`: FSM `observe_cargo.required` must appear as `fn` in `crates/**/*.rs` (one list, not a copy in checks.yaml).
- `.cbmignore` excludes ideation `mermaid.min.js` from the code index.

### Trajectory

- Phase **H3**. Next: H4 waits on ADR-090 overlay (do not start a legal pack). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 4: H2 tenant ingest + unnamed CSR cut

### Product

- Removed `Runtime::csr_lease()` (silent valid-time `0`). Callers name a cut with `csr_lease_at(tt, vt)`.
- H2 tenant: `ingest_harness_jsonl` maps process `harness.run`/`status` → `runStatus` and `harness.observe`/`cargo` → `observed`, with monotonic `valid_to` windows. Same log, two process times, different live status (fixture).
- Binary `kutha-tenant` persists the tenant picture under `KUTHA_TENANT_DIR` (default `.kutha/tenant`, gitignored).
- Allowlist rows `runStatus` and `observed` are delivery facts on the FF6 list, not a second dictionary kind (ADR-050 note).

### Process

- FSM kind `emit_tenant` after `emit_log`: run `kutha-tenant`, require `as_of(last)` to match last status.
- Fold pictures `last_cargo` and `last_tenant`.
- Check `unnamed-csr` forbids bringing `csr_lease()` back.

### Trajectory

- Phase **H2**. Next: H3 (process-plane allowlist). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 3: H1 cargo observation as evidence

### Process (harness)

- FSM kind `observe_cargo`: after dictionary checks, run `cargo test --workspace --offline` and require named FF tests (`ff5_…`, `ff6_…`) to appear as `... ok`.
- Cargo output is **evidence**, not product SoT. Success is recorded as `harness.observe` triples; missing/FAILED tests are HIGH and still walk emit/fold so the log exists.
- `emit_log` counts FSM HIGH findings (not only dictionary check highs).
- Timeout from `KUTHA_GOV_CARGO_TIMEOUT_SEC` then `fsm.yaml` `timeout_sec`.
- Static check `observe-required-fn` keeps required names aligned with Rust `fn` lines.
- Split glued FSM tests (`glob_none` vs budget-from-env).

### Trajectory

- Phase **H1**. Next: H2 (harness facts on the Kutha log). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 2: FF6 relation allowlist + harness hygiene

### Product

- Fail-closed writes: `Runtime::emit` admits only relations listed in `crates/kutha-runtime/dictionaries/relations.yaml` (`schema: kutha-relations/v1`). Unknown relation → `RuntimeError::UnknownRelation`; log does not grow.
- Path override: `KUTHA_RELATIONS_PATH`. Cascade default: `KUTHA_MAX_CASCADE` (also from repo `.env`).
- FF6 tests: `crates/kutha-runtime/tests/ff6_allowlist.rs`.

### Process

- Kind `glob_none`: forbidden globs (setuptools `*.egg-info` must not live in the tree).
- Check `relation-allowlist` guards the product dictionary.
- FSM transitions must use `event:` (test rejects YAML 1.1 boolean keys).

### Trajectory

- M001 S03 done. Active slice **None**. Next: H1 (cargo test as harness evidence). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 1: meta-prompt FSM + Legal PIT FF5

### Process (harness)

- `kutha-gov ci` walks a dictionary FSM (`.kutha/dictionaries/fsm.yaml`): constitution → checks → emit → fold → `ok`|`fail`.
- Unknown FSM kind or missing transition fails closed (`unknown-fsm-kind` / `unknown-transition`).
- Budget and warn-fail come from `.env` (`KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`), then `fsm.yaml` defaults; CLI flags win. Template: `.env.example`.
- Trajectory also requires **Active Slice** to exist on ROADMAP.
- YAML transition field is `event:` (not `on:` — YAML 1.1 would coerce `on` to boolean `true`).

### Product (kutha-runtime)

- Named cuts: `GraphFold::live_at(tt, vt)`, `GraphFold::as_of(vt)` (no “now” default), `Runtime::csr_lease_at(tt, vt)`.
- FF5: same statute-shaped log, `as_of(2015) ≠ as_of(2021)`; CSR neighbors disagree the same way.
- FF3: dropping a CSR lease does not change log length or fold fingerprint.
- Fixture note: `crates/kutha-runtime/tests/fixtures/legal_pit.md`.

### Trajectory

- M001 S01 and S02 done. Active slice is S03 (FF6 relation-allowlist stub). Honeycomb ADRs stay Proposed.

