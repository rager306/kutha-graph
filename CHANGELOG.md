# Changelog

All notable changes to **this repository** are recorded here. This is project history, not GitHub Releases and not a Compound Engineering skill catalog.

Keep **product** (`crates/`) and **process** (harness) distinct. Dated entries may also use a **Trajectory** subsection so L_map / L_delivery / L_capability are not collapsed. New entries prefer Keep a Changelog groups (`Added` / `Changed` / `Fixed`) inside those plane headings.

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

