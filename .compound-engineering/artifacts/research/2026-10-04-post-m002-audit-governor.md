# Post-M002 governor audit (2026-10-04)

Read-only. HEAD `36fedb9`. CBM `root-kutha-graph` (gen 2026-10-04T14:36:49Z, cited paths `no_recorded_issue`/`metadata_match`). GitNexus `kutha-graph` at the same commit. Fast CLI: `kutha-gov list` (64 checks), `fsm`, `explain`. Did not run `ci` / `selftest` / pytest / `index_repository`. Last reported ci: 0 HIGH, 1 LOW. Rule: governor green ≠ ADR Accepted ≠ capability.

## 1. Real invariants vs theatre; does selftest prove fail?

**Protect a real tree/process invariant (still presence, not meaning):**
- Freeze trio `freeze-rocks` / `freeze-cypher-deps` / `freeze-hnsw-dep` (`checks.yaml:74–124`) — Cargo text, not STATE. Bypass: see §2.
- Ledgers: `invariants-ledger` / `bridges-ledger` / `honeycomb-ledger` (`yaml_map_list` + `glob_paths_in_file` + evidence `rust_test_asserts`, `checks.yaml:812–1104`). Schema/partition/ADR-path/map-status are real; slogan `file_contains` steps on the same checks are not.
- `adr-status` (`markdown_heading_tag` + `forbid_accepted_prefix`, :52–72): honeycomb ADR cannot claim Accepted.
- `idle-delivery-closed` + `cite-lease` + `docs-coupling` (`git_path_implies`): lease token and diff-owns-docs. `cite-lease` ignores CHANGELOG and `.planning/` except PROJECT.md (`:1116–1117`).
- `observe-required-fn` + `m010*` / `m011*` / `m012a-*` / `m012-*` / `i-f*` / `h4-membership-as-of`: named `#[test]` must exist, not be `#[ignore]`, and assert (`kinds.py:_prove_named_test:1229–1266`). Does not run the test; `observe_cargo` (`fsm.yaml:24–93`) is the run gate for the listed names only.
- `toolchain`, `version-freeze` (workspace/pyproject `0.0.0` only), `plane-mix-dicts`, `tenant-bin`, `unnamed-csr`, `agents-lean` (`file_max_lines` AGENTS.md ≤110).
- `artifact-budget` / `verification-independent`: real **when files exist**. `min_hits: 0` and `allow_empty: true` (`:1245–1293`) so a missing live PLAN/SUMMARY is not HIGH. Pause leftover `.planning/phases/45-doc-truth-round-2/` has VERIFICATION+CONTEXT, no PLAN/SUMMARY.

**Theatre (needle survives after the claim is gone):**
- `lifecycles`: keep `L_map=` keys, set `L_delivery=capability-green` — no HIGH (`:36–50`); only the collapsed strings `L_delivery=L_capability` / `governor green = product` are banned.
- `honeycomb-map` concat_absent (`:147–156`): must write the exact slogan `promote all honeycomb`. Legend step is `severity: low` (`:158–164`) — one of two LOW steps (other: `dogfood` “not an ADR”, `:178–184`).
- `dogfood` H0–H5 bold tokens; `meta-prompt` constitution laundry list; `changelog-planes` requires the words `product`/`process`/`Trajectory`; `id-map-cite` requires `v0.06`–`v0.09` (`:1300–1303`) not `v0.12` (row can be deleted while old needles remain). `h4-lease` is checkbox/H5 text, not ontology.
- `m010-semantic-open` extra `file_contains` `Define` / `terms.jsonl` (`:273–290`): rename the symbol, leave the token — pass. Same pattern on several m011 `file_contains` steps.

**Selftest does not prove each check, nor each step.** `_prove_check` (`selftest.py:130–157`): skip | declared mutations | else derive from `DERIVABLE_KINDS` (`:26–41`). One HIGH anywhere in the mutated copy counts as proven (`_run_mutations:176–178`). Not in DERIVABLE: `yaml_map_list`, `pointer_in_other_file`, `git_path_implies`, `markdown_heading_tag`, `glob_none`, `glob_paths_in_file`, `when_match_then_match`. Mixed checks “prove” by deleting `file_exists` or appending a needle, not by breaking the ledger/pointer rule. Explicit `selftest:`: mutations on `adr-status`, `idle-delivery-closed`, `state-readme`, `cite-lease`; **skip** `docs-coupling` (live git diff), `artifact-budget`, `verification-independent` (`checks.yaml:769,1242,1274`) — unproven by policy. `glob_absent` derive appends `rocksdb =` on a new line (`CHANGELOG` 2026-10-04) — does not prove table-form or same-line `# optional = true`.

## 2. Freeze robustness; tomllib kind vs “YAML rows”

**Not robust.** `glob_absent` + `unless_line_contains: "optional = true"` (`kinds.py:_kind_glob_absent:312–347`, `_line_has_needle_without_allow:302–309`). Needle is the substring `rocksdb =` (`checks.yaml:82`).

Undetected: `[dependencies.rocksdb]` table (no `rocksdb =`); `rocksdb="0.22"` (no spaces); `rocksdb = "0.22" # optional = true` (allow on same line); a second `optional = true` on the same physical line; workspace `[workspace.dependencies.rocksdb]`. Comment-only `# rocksdb =` is ignored (intended). Pytest `FreezeDepTests` (`test_kutha_gov.py:617–723`) covers inline `{ optional = true }` and missing-optional inline table; it does not cover the bypasses above. `freeze-cypher-deps` / `freeze-hnsw-dep` are whole-file substrings (`neo4j`/`hnswlib`) including comments (false HIGH) and miss a renamed crate / git dep (false LOW). GitNexus `impact` on `_kind_glob_absent`: 0 callers (dispatch via `RUNNERS` dict `kinds.py:2010–2032`), not unused.

**Fix:** new kind `cargo_dep_constraint` in `kinds.py` using stdlib `tomllib` (3.13): walk crate and workspace TOML; treat `[dependencies]`, `[dependencies.NAME]`, `[target.*.dependencies]`, `[workspace.dependencies]`; `rocksdb` allowed iff `optional is true`; workspace root forbids the key; cypher/hnsw remain “key absent”. YAML rows keep freeze ids; `unless_line_contains` unused for these. **Does not conflict** with “YAML rows, not new Python classes”: META already forbids `scripts/kutha_gov/checks/*.py` subclasses and allows a **kind** as last responsible moment (`.kutha/META.md:40–42`, `governor-intake.md:64–65`). No existing kind can parse TOML tables.

## 3. M002 durability vs governor; cheap machine checks

**Governor does not check M002 product-close claims.**
- No check id mentions `m002_*`. `honeycomb.yaml` ADR-010 evidence is `unknown_relation_rejected_…` (`honeycomb.yaml:107–108`), not SNAP/APPEND/COMMIT/SEG/ROCK oracles. `capability: none` skips nonempty-evidence, but listed names still go through `honeycomb-ledger` `rust_test_asserts` (`kinds.py:_names_from_yaml_rows:1065–1079`).
- ADR-010 Evidence bodies cite `m002_append01.rs` / `commit01` / `seg01` / `rock01` (ADR-010:21–41). No check that Evidence lines match test names or files.
- `i-f6-persist-open` / `m012a-s04-verify-persist` still name **legacy jsonl** oracles (`checks.yaml:469–478,1369–1377`), explicitly not segmented persist.
- `fsm.yaml` `observe_cargo.required` has no `m002_*`. Default `cargo test --workspace` does not enable `rocks-index`; `m002_rock01.rs:3` is `#![cfg(feature = "rocks-index")]`. `_prove_named_test` ignores `cfg`, so listing the fn would pass dictionary checks while default cargo never runs it.
- Bench gate (`gsd-auto-contract.md:38–40`) is prose. CHANGELOG Product 2026-10-04 cites `Bench vs 0245bbd` by convention. `I-F8-hot-reads` (includes “benchmark baseline”) is `disposition: deferred` (`invariants.yaml:185–189`).
- Process tag: `git tag` has `v0.12`; STATE/CHANGELOG still say “for the controller to create” (`.kutha/STATE.md:27`).

**Cheap conversions (prefer YAML/FSM rows):**
1. Append asserting `m002_*` fn names to ADR-010/012 `evidence:` → `honeycomb-ledger` already resolves them. Does not execute Rocks tests.
2. Add those names to `observe_cargo.required`; for ROCK-01 add `--features rocks-index` on `observe_cargo.args` (`fsm.yaml:24–31`) — existing FSM kind, not a class. Time cost: Rocks compile.
3. Flip `artifact-budget` `min_hits: 1` while a live `.planning/phases/*` exists (intake already says this, `governor-intake.md:98`); set SUMMARY `max_files_per_dir: 1` (contract is one SUMMARY; YAML allows 2, `checks.yaml:1256`).
4. `id-map-cite`: require `v0.12` (and `v0.11`) needles in `gsd-auto-contract.md`.
5. Bench: YAML `file_contains` on CHANGELOG `Bench vs` is theatre. Either a small kind `file_frontmatter_sha` on a dated bench note, or keep I-F8 deferred. Tag-exists needs a small `git_ref` kind (or stay controller honor).

## 4. Complexity budget — safe cuts

| Surface | Size |
|---------|------|
| Python kernel `scripts/kutha_gov/*.py` | ~4828 lines; `kinds.py` 2032 (~42%), `selftest.py` 574, `yaml_map_list` `:742–958` |
| Dictionaries | checks 1452, honeycomb 454, invariants 285, fsm 133, bridges 132; 64 checks = `defaults.budget` |
| Pytest | `test_kutha_gov.py` 1526 + fsm 407 + selftest 269 |
| Trees copied by selftest | `.compound-engineering` 4.9M, `.planning` 4.2M (`copy_harness_tree` skips `.git`/`target` only, `selftest.py:43–90`) |

`ci` serializes `run_checks` → `run_selftest` → `observe_cargo` (`fsm.yaml:120–121`). Selftest ~5 min is copytree + 64 prove cycles (honeycomb evidence glob `crates/**/*.rs`). Cargo `timeout_sec: 180` (`fsm.yaml:28`, `.env.example`). Duplicate `i-f*` vs `m012a-*` / `m012-*` re-assert the same oracles.

**Cut safely:** add `.compound-engineering` / archived `.planning/milestones` to `SKIP_DIR_NAMES`; do not run `run_selftest` inside `ci` when `checks.yaml`+`kinds.py` unchanged (keep `kutha-gov selftest` as its own job); drop slogan `file_contains` steps that cannot fail except by deleting the sentence; merge duplicate i-f/m012a rows. Do not drop ledger `yaml_map_list` or freeze (replace with tomllib). Do not drop `observe_cargo` required names.

## 5. STATE / planning / README / ROADMAP consistency

| Invariant | Machine? | Gap |
|-----------|----------|-----|
| Active Milestone/Slice on `.kutha/ROADMAP.md` | `trajectory` (`:18–34`) | `skip_values: ["None"]` — idle skips ROADMAP ticks. Unticking M002 S07 while `L_delivery=M002-closed` is convention. |
| Idle ⇒ `L_delivery=*-closed` | `idle-delivery-closed` (`:126–142`) | Does not read `.planning/STATE.md`. |
| README Phase = STATE Phase | `state-readme` (`:708–731`) | Milestone `None` skipped; README lease row “M002 closed” (`README.md:13`) is not `L_delivery=` so `cite-lease` is silent. |
| Docs `L_delivery=` token = STATE | `cite-lease` | `.planning/STATE.md` out of scope; GSD `milestone: v0.12` vs harness `Active Milestone: None` is two vocabularies, uncompared. |
| Honeycomb `freeze_as` in STATE | `honeycomb-ledger` (`:1064–1072`) | Real for frozen cells. |
| ROADMAP “honeycomb is a map” | LOW `honeycomb-map` | Slogan. |
| Id map not copied | `id-map-cite` | Links only; latest GSD version not required. |
| Process tag per milestone | none | `v0.12` tag exists; STATE prose stale. |
| `.kutha/ROADMAP.md` vs `.planning/ROADMAP.md` ticks | none | Dual lists; `id-map-cite` only forbids copying the table header. |

## Findings

- **HIGH** `freeze-rocks` line grep bypassable (`checks.yaml:74–92`, `kinds.py:302–347`). Mutation: `[dependencies.rocksdb]` / `rocksdb="0.22"` / same-line `# optional = true`.
- **HIGH** M002 oracles not in checks, honeycomb evidence, or `observe_cargo.required`; ROCK-01 tests are feature-gated (`m002_rock01.rs:3`, `fsm.yaml:24–93`). Delete `m002_seg01.rs` assertions, keep tokens in ADR Evidence — governor green.
- **HIGH** selftest proves a HIGH, not the claimed invariant (`selftest.py:26–41,164–178`). Ledger/`pointer`/`git_path_implies` steps ride on `file_exists` deletion.
- **MED** `artifact-budget` `min_hits: 0` + selftest skip while phase 45 dir is live (`checks.yaml:1242–1268`; intake `:98`). SUMMARY cap allows 2 files; contract is one.
- **MED** `id-map-cite` / `cite-lease` / `trajectory` leave GSD STATE, README lease prose, ROADMAP checkboxes, and tags uncompared (`cite-lease:1116–1117`, `trajectory:23`).
- **MED** ci wall time: selftest copies CE+planning; 64×`rust_test_asserts`; duplicate i-f/m012 rows (`selftest.py:74–90`).
- **LOW** only two `severity: low` steps (`honeycomb-map:162`, `dogfood:182`); last ci’s 1 LOW is one of these if a needle drifted. Theatre slogan checks (`lifecycles`, `meta-prompt`, `changelog-planes`, `id-map-cite` old versions).
- **LOW** `I-F7`/`I-F8`/`I-F13` deferred (`invariants.yaml:179–222`); bench gate stays honor-system until a kind or FSM observe.
