# Phase 11: Semantic governor - Context

**Gathered:** 2026-09-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Make the governor check **meaning**, not needle presence: prove checks can fail (vacuity), prove named Rust tests actually assert, prove docs cite the live lease, prove honeycomb `evidence` resolves to real tests, prove ADR/lock references resolve, and lock in the Phase 9/10 results (AGENTS size + no lease tokens). Record product gaps F1–F8 as `deferred` invariants. Close H5.

**In scope:** SEM-01…SEM-08.

**Out of this phase:** any `crates/` edit; enforcing F1–F8 as product fixes; new FSM states other than the one `run_selftest` phase; Python `Check` subclasses (checks stay YAML rows; new behaviour is a *kind* in `scripts/kutha_gov/kinds.py`); editing `.kutha/STATE.md` (H5 lease already written in Phase 9); ADR body edits; leasing M012a/M012/M002.

</domain>

<decisions>
## Implementation Decisions

### SEM-01 — vacuity self-test (`kutha-gov selftest`)

- **D-V1:** A check is *non-vacuous* if (a) it passes on a faithful temporary copy of the tree (baseline) and (b) each declared/derived **mutation** applied to that copy makes it produce ≥1 HIGH. Never mutate the working tree. Copy once (excluding `.git`, `target`, `.venv`, `node_modules`, `__pycache__`, `.pytest_cache`, `.ruff_cache`, `.cbm*`, `.cursor/gsd-core`, `.cursor/skills`, `.claude`), apply a mutation, run the check, restore, repeat. A check whose baseline fails on the copy is reported `baseline-fail` (not silently skipped). — **Reversibility:** reversible

- **D-V2 (declarative, no per-check Python):** Optional `selftest:` block on a `checks.yaml` row: `mutations: [ {op: delete_file|remove_text|append_text|replace_regex, path, text|pattern, with} … ]` **or** `skip: "<reason>"`. Default mutations are **derived from the steps** for: `file_exists` (delete file), `file_contains` (remove every occurrence of one needle, case-aware; for `require: any` remove all needles), `file_absent` / `concat_absent` / `glob_absent` (append a forbidden needle), `file_equals` (append text), `concat_contains_any` (remove all needles from the concatenated paths), and the new kinds below (each defines its own derived mutation). Kinds that cannot be derived (`pointer_in_other_file`, `when_match_then_match`, `yaml_map_list`, `yaml_needles_in_glob`, `glob_paths_in_file`, `glob_none`, `markdown_heading_tag`, `git_path_implies`) need an explicit `selftest.mutations` or `selftest.skip` with a reason. A check with neither derivable steps nor a declaration is **unproven** and makes `selftest` exit non-zero (fail-closed). `git_path_implies` steps are declared `skip` (depends on the live git diff). — **Reversibility:** reversible

- **D-V3:** CLI `uv run kutha-gov selftest [--check ID]` prints one line per check (`OK|SKIP|UNPROVEN|VACUOUS|BASELINE-FAIL`, mutation count) and exits non-zero on anything except OK/SKIP-with-reason. An FSM kind `run_selftest` is added (META "Allowed FSM kinds" row + `fsm.py` + one `fsm.yaml` state/transition after `run_checks`) so `ci` runs it and records evidence `h5_selftest=<proved>/<total>`; `ci` prints the rung label `H5` when that evidence is present (today the label is derived in `__main__.py`). A pytest runs selftest against the real repo and against a synthetic vacuous check (e.g. `file_contains` with empty needles) that must be reported VACUOUS. — **Reversibility:** reversible

### SEM-02 — non-vacuous named tests (`rust_test_asserts`)

- **D-T1:** New check kind `rust_test_asserts`: params `path` or `glob`, `tests: [names]` (or `names_from_yaml`, see D-E1), optional `require_symbols: {test_name: [symbol,…]}`, optional `assert_prefixes` (default `["assert_"]` helper calls count). For each name: a `fn name` exists in the file(s); the attribute block directly above contains `#[test]` and no `#[ignore]`; the brace-matched body (comments and string literals stripped before matching braces) contains ≥1 of `assert!`, `assert_eq!`, `assert_ne!`, `debug_assert*!`, or a call to a function whose name starts with an allowed helper prefix; each `require_symbols` symbol appears in the body. Derived selftest mutation: replace the first `assert…!(` occurrence in the test body with `(` (or `#[ignore]` insertion) on the temp copy, so the check must go HIGH. — **Reversibility:** reversible

- **D-T2:** Convert every existing check that uses `"fn <name>"` needles (≈10 checks, ≈20 needle lines: `m010-semantic-open`, `m011-*`, `h4-membership-as-of`, `observe-required-fn`, …) to `rust_test_asserts`. If a currently named test turns out vacuous or unresolved, that is a **real finding**: record it in the SUMMARY and fix the *check* (correct name / symbols); do not edit `crates/` (report the product test gap instead). — **Reversibility:** reversible

### SEM-03 — lease citations equal STATE (`cite_equals`)

- **D-C1:** New kind `cite_equals`: `source: {path, pattern}` (one capture group, e.g. STATE `L_delivery=(\S+)`), `cites: [{glob|path, pattern}]` (each capture in each cited file must equal the source capture; zero cites is allowed, a mismatch is HIGH). Rows: `L_delivery`, `L_map`, `L_capability` tokens and `Active Milestone … (M\d{3})` phrasing cited in `AGENTS.md`, `README.md`, `STRATEGY.md`, `docs/process/*.md`, `docs/architecture/*.md`, `docs/ADR/README.md`. Historical docs that legitimately quote old values (CHANGELOG, `.planning/**`, review notes with an explicit "as of" marker) are **not** in the cite globs. Derived selftest mutation: append a wrong citation line to the first cited file. — **Reversibility:** reversible

### SEM-04 — evidence resolves (`names_from_yaml`)

- **D-E1:** `rust_test_asserts` also accepts `names_from_yaml: {path: .kutha/dictionaries/honeycomb.yaml, select: cells, field: evidence, glob: "crates/**/*.rs"}` (names resolved across the glob) and `require_evidence_when: {field: capability, equals: named}` so a cell claiming `capability: named` with empty evidence is HIGH. Every currently named evidence item must resolve to a non-vacuous test; unresolved names are findings (fix `honeycomb.yaml` evidence naming only if the test truly exists under another name; otherwise record as a product/evidence gap and downgrade nothing silently — report it). — **Reversibility:** reversible

### SEM-05 — references resolve (`refs_resolve`)

- **D-X1:** New kind `refs_resolve`: scan `paths|globs` for `ADR-(\d{3})` and `\bD([1-9]|10)\b` (lock ids; leading-zero forms like `D050-2` are not lock ids), require each ADR id to have a `docs/ADR/ADR-<id>-*.md` file (or be listed in `allow_dangling`, initially `[ADR-100]` for the frozen "do not open ADR-100" mentions) and each lock id to exist in `honeycomb.yaml` `locks`. Scope: `AGENTS.md`, `README.md`, `.kutha/**/*.md`, `.kutha/dictionaries/*.yaml`, `docs/process/*.md`. Range mentions like `ADR-010–093` resolve both endpoints. Derived selftest mutation: append a dangling `ADR-999` to the first scanned file. — **Reversibility:** reversible

### SEM-06 — AGENTS.md budget and tokens

- **D-B1:** New tiny kind `file_max_lines` (`path`, `max`); check `agents-lean` (in an invariants row, `disposition: check`) asserts `AGENTS.md` ≤ 110 lines (Phase 9 result: 100) and, with existing `file_absent`, forbids `L_delivery=`, `L_map=`, `L_capability=`, `M011 is active`. — **Reversibility:** reversible

### SEM-07 — deferred product invariants

- **D-D1:** Append eight `invariants.yaml` rows `I-F1-…` … `I-F8-…` (`disposition: deferred`, `until:` naming the target lease, e.g. "STATE names M012a"), `source: docs/architecture/semantic-gap-review.md` (note the ledger forbids the substring `docs/ADR/ADR-` in `invariants.yaml`). Claims are one sentence each in control-loop vocabulary ("Product outcomes/justifications are log records …"), not ADR copies and not enforced. Product-side symbols are not named as commitments. — **Reversibility:** reversible

### SEM-08 — tests, docs, changelog, close

- **D-Q1:** Each new kind and `selftest`/`run_selftest` gets a red-path pytest in `scripts/tests` (unittest style like `test_kutha_gov.py`); `uv run pytest` and `uv run kutha-gov py` (ruff + ty + pyrefly) stay green. — **Reversibility:** reversible
- **D-Q2:** `.kutha/META.md` "Allowed kinds" gets rows for every new kind (`rust_test_asserts`, `cite_equals`, `refs_resolve`, `file_max_lines`) and the FSM kind `run_selftest`; `docs/process/governor-intake.md` gains a short "What a green check proves" section (presence vs meaning; every check must be mutation-provable or carry a reason to skip). AGENTS.md Commands gets the one `selftest` line (stay ≤110). — **Reversibility:** reversible
- **D-Q3:** CHANGELOG: replace the Phase 9 and Phase 10 **pointer** stubs ("Narrative remains Phase 11 SEM-08") with real **Process** + **Trajectory** entries authored per `.cursor/skills/kutha-changelog/SKILL.md` (H5 lease, AGENTS diet, semantic-gap review + ADR clarifications + proposed order, semantic governor). No SemVer, no tag, no release. — **Reversibility:** reversible
- **D-Q4 (H5 close):** flip `- [ ] **H5**` to `- [x] **H5**` in `.kutha/ROADMAP.md` and retarget the `h4-lease` check needle accordingly (rename its description; keep the id); `docs/process/kutha-harness.md` marks H5 done. `.kutha/STATE.md` stays byte-stable. — **Reversibility:** reversible

### Governor and process inheritance

- **D-G1:** Each wave: `uv run kutha-gov ci` (0 HIGH, includes the new selftest phase once it lands), `uv run kutha-gov precommit`, `uv run pytest -q scripts/tests`, `uv run kutha-gov py`; SUMMARY carries an ≤8-line Trajectory excerpt. `docs-coupling` requires CHANGELOG in the diff for `.kutha/**`, `scripts/**`?, `docs/**` changes — follow the Phase 9/10 pattern until D-Q3 lands. — **Reversibility:** reversible
- **D-G2:** Freeze, Proposed honeycomb, three lifecycles, Active Slice None untouched. Product `crates/` untouched (`git diff --exit-code -- crates`). — **Reversibility:** reversible
- **D-G3:** Governor green ≠ ADR Accepted ≠ capability. The new checks make the harness more honest; they do not accept cells and must not be described as proving product correctness. — **Reversibility:** reversible

### Claude's Discretion

Module layout inside `scripts/kutha_gov` (a new `selftest.py` and small helpers are fine; keep `kinds.py` from growing unmanageably by adding a `rust_source.py` helper), exact YAML field names beyond those above, mutation implementation details, and plan/wave split — as long as: fail-closed defaults, no per-check Python, no working-tree mutation by `selftest`, checks stay YAML rows, and every new kind ships with a red-path test. Docs/Python only: no code-graph queries are required for `scripts/`; do not claim graph verification.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md` (Phase 11), `.planning/REQUIREMENTS.md` (SEM-01…08)
- `scripts/kutha_gov/{kinds,dictionary,protocol,fsm,__main__,honeycomb,observe}.py`, `scripts/tests/test_kutha_gov.py`, `scripts/tests/test_fsm.py`
- `.kutha/META.md` (kind allowlists), `.kutha/dictionaries/{checks,invariants,bridges,honeycomb,fsm}.yaml`
- `docs/process/governor-intake.md`, `docs/process/kutha-harness.md`
- `docs/architecture/semantic-gap-review.md` (F1–F8 wording for D-D1)
- `AGENTS.md`, `CHANGELOG.md`, `.cursor/skills/kutha-changelog/SKILL.md`
- `crates/kutha-runtime/tests/*.rs`, `crates/kutha-runtime/src/*.rs` (read-only: resolve named tests)

</canonical_refs>
