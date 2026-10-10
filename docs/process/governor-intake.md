# Governor intake (control loop → check)

Process contract, not an ADR. Companion: `.kutha/META.md`.

This is the fail-closed analog of ADR-050 D050-2 on the **harness** plane: the LLM may propose a check; the dictionaries validate; silence is not a requirement. It does not implement product dictionary entities.

Frozen-dep checks (`freeze-rocks`, `freeze-cypher-deps`, `freeze-hnsw-dep`) scan `crates/*/Cargo.toml` and the workspace `Cargo.toml`; they do not read STATE. `.kutha/STATE.md` § Thaw map documents the named gates. Honeycomb `freeze_as` strings must appear in STATE (`honeycomb-ledger`).

Three surfaces stay distinct. Mixing them in one ledger is a plane collapse.

| Surface | Owns | Lives in | Governor role |
|---------|------|----------|----------------|
| **Control loop** | Trajectory, META, FSM, docs coupling, plane-mix, honeycomb `freeze_as` in STATE | `.kutha/dictionaries/invariants.yaml` | Interpreter may enforce |
| **Bridges** | A process check that *cites* a product freeze or named test | `.kutha/dictionaries/bridges.yaml` | Cite only; do not copy L_map / L_capability |
| **Map** | Honeycomb cells, D1–D10, stages, depends_on | `.kutha/dictionaries/honeycomb.yaml` | Validate + `kutha-gov map`; does not accept cells |

ADR bodies stay narrative. `honeycomb.yaml` is the compact L_map index. `map` / `delivery` / `capability` stay orthogonal. Dump for context: `uv run kutha-gov map` (optional cell id; `--format json`).

## Principle

A **control-loop** “must” becomes a governor requirement only after it has a **disposition** in the process ledger. A **bridge** becomes a governor requirement only after it has a `cites` row. A honeycomb cell is tracked in `honeycomb.yaml`, not as a YAML check. Adding a check without a home in exactly one of invariants or bridges, or a Python `Check` subclass, is out of protocol. Silence is not a requirement. Kutha requirements stay in the map index + ADR bodies + named tests.

```text
control-loop claim (STATE / META / process doc)
  → invariants.yaml row
      disposition ∈ {deferred | check | kind}
  → if check: append checks.yaml using an allowed kind

bridge (cite product freeze or named test; do not copy its state machine)
  → bridges.yaml row (id, claim, cites, check)
  → append checks.yaml using an allowed kind

map cell (ADR opened or stage/edge changed)
  → honeycomb.yaml row (map, delivery, capability, depends_on, locks, evidence)
  → ADR Status remains the narrative twin; governor matches map to **Status**
  → not a YAML check; not Accepted
```

Governor green ≠ ADR Accepted ≠ capability. A ledger row does not promote a honeycomb cell. Bridges may cite; they may not copy state machines.

## Control-loop dispositions (closed)

| Disposition | Means | Required extra fields |
|-------------|--------|------------------------|
| `deferred` | Process rung not yet on; STATE must name it | `until` |
| `check` | `kutha-gov` enforces this control-loop claim now | `check` (id in `checks.yaml`) |
| `kind` | Existing kinds cannot express it (last responsible moment) | `kind` (name to add to META) |

Unknown disposition → HIGH (`invariants-ledger`). `map-only` and `capability` are **not** process dispositions: L_map is ADRs; L_capability is crates tests / FSM `required` evidence.

## Bridges

Required fields: `id`, `claim`, `cites`, `check`. No `disposition`. A bridge always names a live check and a product path it cites (STATE Thaw map, a crate test, or a product dictionary). Frozen-dep bridges cite STATE; the checks themselves scan Cargo.toml. Unknown extra lifecycle copied from an ADR → that belongs in the ADR, not here.

A check id appears in **exactly one** ledger: invariants (`disposition: check`) or bridges (`check:`), then in `checks.yaml`. Overlap → HIGH.

## Intake steps

1. Decide the surface. Product cell → ADR file + `honeycomb.yaml`. Control loop → invariants.yaml. A fence that cites product without owning it → bridges.yaml.
2. Control loop: append `.kutha/dictionaries/invariants.yaml` with `id`, `claim`, `source`, `disposition`.
3. If `disposition: check`, append `.kutha/dictionaries/checks.yaml` using a kind from META. Same git diff must include the ledger row (`docs-coupling`).
4. Bridge: append `.kutha/dictionaries/bridges.yaml` and the YAML check. Do not copy honeycomb Status or fitness into the bridge row.
5. Map: append `.kutha/dictionaries/honeycomb.yaml`. Stages: `map` (ADR Status) · `delivery` (`map-only`/`frozen`/`spike`) · `capability` (`none`/`named`). Links: `depends_on`, `locks`, `evidence`. Run `uv run kutha-gov map`.
6. If no allowed kind fits, set `disposition: kind` on the **control-loop** ledger, then change META + `kinds.py` + a test. Do not add `scripts/kutha_gov/checks/*.py`.
7. Run `uv run kutha-gov explain <check-id>` and `uv run kutha-gov precommit --check invariants-ledger` (or `--check honeycomb-ledger`).
8. Record the process change under CHANGELOG **Process** (not as a GitHub Release).

## What a green check proves

Presence is not meaning. A needle that still matches after the claimed property is gone is a vacuous pass. `kutha-gov selftest` (and `ci`'s `run_selftest`) proves meaning by mutating a **copy** of the tree: each check must go HIGH under a declared or derived mutation, or carry a specific `selftest.skip` reason. Unproven (neither mutation nor skip reason) fails closed. Skip is not a silent pass.

Selftest classifies every dictionary check as `derived-by-own-mutation`, `explicit-mutation`, or `explicit-skip` (reason required). `git_path_implies` is skip because it depends on the live git diff. A check whose only HIGH derivation is `file_exists` deletion is `file-exists-only` (LOW: not proof of sibling ledger/pointer steps). New kinds (`yaml_map_list`, `pointer_in_other_file`, `markdown_heading_tag`, `glob_none`, `glob_paths_in_file`, `when_match_then_match`) derive a mutation that breaks that kind's own rule, or the YAML row declares skip/mutations.

A green governor is **not** ADR **Accepted**, not L_capability, and not product correctness. Checks keep the harness honest about its own dictionaries, citations, and named-test evidence. They do not close F1–F8, accept honeycomb cells, or lease M012a.

## Parallel execution

`ci`, `selftest`, and `precommit` accept `--jobs N` (env `KUTHA_GOV_JOBS`). Precedence: flag > env > default `min(CPU, 8)`. `1` is serial. `0` or a non-integer is a contract error (exit 2).

Independent dictionary checks and selftest proofs run in a process pool (`spawn`). Results merge in dictionary check order, never completion order. `--jobs 1` is the oracle: findings (id, severity, message, order), the exit code, and JSONL rows (ignoring timing fields such as `phase_ms`, timestamps, and run ids) match any `N ≥ 2`.

`observe_cargo` / `observe_rocks` may overlap `run_checks` + `run_selftest` when the FSM row has `overlap_with:` and jobs > 1. Emit, tenant, and fold stay serial at the end. A worker exception, a pool breakage, or a timeout is a HIGH `worker-error` (check id + exception text), never a silent skip. A crashed pool falls back to serial for unfinished checks and records a LOW `pool-fallback`.

## Example intake rows (current)

Control-loop kind obligation (Phase 29 GOV-01):

```yaml
# invariants.yaml
- id: I-cui-budget
  claim: "FSM run_checks must not silently truncate the checks dictionary when budget is below the check count"
  source: "docs/process/governor-intake.md"
  disposition: kind
  kind: run_checks
```

TOML freeze kind (Phase 57 GOV-04). No existing kind parses Cargo tables, so this is `disposition: kind` — not a `Check` subclass and not `scripts/kutha_gov/checks/*.py`.

```yaml
# invariants.yaml
- id: I-cargo-dep-constraint
  claim: "Frozen Cargo dependency checks parse TOML tables (not line grep)"
  source: "docs/process/governor-intake.md"
  disposition: kind
  kind: cargo_dep_constraint
```

`cargo_dep_constraint` (stdlib `tomllib` only) takes a Cargo.toml glob and `rules` mapping crate names to `forbid` or `require_optional`. It walks `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`, `[target.*.dependencies]`, `[workspace.dependencies]`, table form `[dependencies.NAME]`, and `package =` renames. Comments never match. A missing or unparsable TOML file is HIGH; a glob with zero matches is not. `freeze-rocks` / `freeze-cypher-deps` / `freeze-hnsw-dep` keep their ids and messages; they now use this kind.

Test-matrix kind (Phase 59 TEST-01). `yaml_map_list` cannot prove (iii)–(v) without theatre: ROADMAP slice owners are `M019-S05` while the file writes `## M019` plus `**S05:`; `killed` × `kill_mutation != none` is a cross-field rule; `feature:<observe>` is a prefix lookup into `fsm.yaml` states of kind `observe_cargo`. Disposition is `kind`.

```yaml
# invariants.yaml
- id: I-test-matrix-kind
  claim: "Test-matrix consistency cannot be expressed as yaml_map_list"
  source: "docs/process/governor-intake.md"
  disposition: kind
  kind: test_matrix
```

`test_matrix` reads `docs/architecture/test-matrix.yaml`. Product `oracle` names must be asserting `#[test]` fns (not `#[ignore]`); harness names must be check ids; `blocked` owners are the decision allowlist (`C01`, `HORN-KC-01`, `HORN-CORR-01`, `ASOF-ELIG-01`, `TR-02`, `DET-02`, `FIXG-01`); gap and other slice owners must appear as `M###-S##` under that ROADMAP heading; `unowned` is rejected. Green means the map is consistent, not that the product is tested. Do not add the matrix to `observe_cargo.required`.

Artifact-budget glob with non-vacuous match (`artifact-budget` in `checks.yaml`):

```yaml
- kind: glob_max_lines
  glob: .planning/phases/*-*/*-PLAN.md
  min_phase: 22
  min_hits: 1
  max: 150
  max_files_per_dir: 2
```

`min_hits` is idle vs live: `0` when `.planning/phases/` is empty after a milestone archive, so an empty glob is not a vacuous HIGH. When a live phase CONTEXT exists, set `min_hits: 1` (kind option `when_glob_exists`) so a missing SUMMARY/VERIFICATION/CONTEXT glob is HIGH again. PLAN `min_hits` is gated on the PLAN glob itself: this lease has CONTEXT without a PLAN file, so requiring a PLAN would be a false HIGH. `verification-independent` must not use `allow_empty`: gate content steps with `when_glob_exists` on CONTEXT so idle skips, and live CONTEXT without VERIFICATION is HIGH (CE review P1 #16).

`observe_rocks` (FSM kind `observe_cargo`) runs `cargo test -p kutha-runtime --features rocks-index,rocks-index-spike --test m002_rock01 --test m002_rock00 --offline` with `timeout_sec: 420`. Measured 2026-10-04 on this host with a fresh `CARGO_TARGET_DIR`: cold ROCK-01 279.8s (<6 min bound, so the row stays on default `ci`), warm ROCK-01 1.4s, ROCK-00 after the shared compile 8.1s. Not `optional`. Do not enable those features in crate Cargo.toml.

Verification independence (`verification-independent` check): `file_contains` with `glob` + `min_phase` on `NN-VERIFICATION.md`, plus `frontmatter_paths_exist` on `read_only_inputs` (paths must exist; plan/summary artifacts forbidden).

## What these ledgers are not

- Not a clone of law-nexus ADR frontmatter, D098 tags, or `document-freshness-triggers.json` catalogs.
- Not reverse-audit of every ADR sentence (daily-archive `adr_rules.yaml`).
- Not Keep a Changelog / SemVer / release-plz.
- Not H4-as-legal-pack, ADR-090 ontology, or ADR-050 six product dictionaries.
- Not a second honeycomb markdown: the compact index is `honeycomb.yaml`; cells stay **Proposed** in `docs/ADR/` until the cell is in the running engine.
