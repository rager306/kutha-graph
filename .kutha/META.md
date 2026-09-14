# Harness meta-prompt (constitution)

Version: 1  
Not an ADR. Not product SoT. LLM does not execute these checks.

This is the **process analog of ADR-050**: a versioned constitution plus dictionaries. The governor CLI is an interpreter. Adding a check is a dictionary edit. Adding a *kind* is a rare kernel change (allowlist below + `scripts/kutha_gov/kinds.py` + a test).

## Fail-closed cycle

```text
ci → load META.md → load dictionaries/checks.yaml → load dictionaries/fsm.yaml
  → load dictionaries/relations.yaml
  → load dictionaries/invariants.yaml
  → load dictionaries/bridges.yaml
  → walk FSM (unknown kind/transition → fail)
  → for each check, for each step: kind must be in the allowlist
  → observe_cargo (cargo test is evidence, not product SoT)
  → emit log (unknown process relation → no JSONL append)
  → emit_tenant (harness JSONL → Kutha log, AS OF process)
  → fold → terminal ok|fail
```

Unknown `kind` → HIGH. Unknown FSM kind → HIGH (`unknown-fsm-kind`). Missing dictionary → HIGH. Python `Check` subclasses are not the intake path. `git_path_implies` is coupling, not a release bumper; it does not tag, publish, or rewrite changelog sections. `yaml_map_list` is ledger interpretation, not product ADR-050 dictionaries. `invariants.yaml` is the control loop; `bridges.yaml` cites product freeze/tests; honeycomb cells stay in `docs/ADR/`.

## How to add a check

A control-loop “must” is not a governor requirement until it has a row in `.kutha/dictionaries/invariants.yaml`. A fence that cites product is a row in `.kutha/dictionaries/bridges.yaml`. Kutha requirements stay in ADRs / STATE / crates tests. Protocol: `docs/process/governor-intake.md`.

1. Choose the surface. Do not put honeycomb cells in either ledger.
2. Control loop: append an **invariants** row (`id`, `claim`, `source`, `disposition`). Silence is not a disposition. Dispositions: `deferred` | `check` | `kind`.
3. Bridge: append a **bridges** row (`id`, `claim`, `cites`, `check`). No `disposition`.
4. Append `.kutha/dictionaries/checks.yaml` using an **allowed kind**. The check id lives in exactly one ledger (`invariants-ledger` / `bridges-ledger`).
5. Run `uv run kutha-gov precommit --check invariants-ledger` and `uv run kutha-gov explain <id>`. Full `ci` still owns cargo quantum.
6. Do not add `scripts/kutha_gov/checks/*.py`.

## How to add a kind (last responsible moment)

Only when no existing kind can express the rule. Then: name it here, implement it in `kinds.py` (check kinds) or `fsm.py` (FSM kinds), add a test, ship a YAML row that uses it.

## How to extend the FSM

Append a state and a transition in `.kutha/dictionaries/fsm.yaml` using an **allowed FSM kind**. Do not hardcode a new `ci` phase in Python.

## Allowed kinds

| Kind | Meaning |
|------|---------|
| `file_exists` | path must be a file |
| `file_equals` | file strip-equals a string |
| `file_contains` | file contains all/any needles |
| `file_absent` | file contains none of the needles |
| `concat_absent` | concatenated files contain none of the needles |
| `concat_contains_any` | concatenated files contain at least one needle |
| `glob_absent` | no glob match contains any needle |
| `glob_none` | glob must match no files (build artifacts) |
| `markdown_heading_tag` | files matching glob have `## Status` + allowed tag |
| `pointer_in_other_file` | exactly one regex capture in A must appear in B |
| `yaml_needles_in_glob` | every string at a YAML path must appear (with prefix) in a glob of files |
| `git_path_implies` | if the git diff matches `when_any`, it must also match `then_any` (empty/no-git skips) |
| `yaml_map_list` | YAML list of maps: required fields, unique ids, closed vocab, cross-file refs (`other` may be a list), or `absent_other` partition |

## Allowed FSM kinds

| Kind | Meaning |
|------|---------|
| `noop` | no work; emit `ok` |
| `require_file` | path must exist |
| `run_checks` | interpret the checks dictionary (budget from env / FSM defaults) |
| `observe_cargo` | run `cargo test` as evidence (not SoT); required test names must appear `... ok`; optional `build:` compiles `kutha-tenant` once |
| `emit_log` | append `harness.run` triples (fail-closed on `kutha-harness-relations/v1`) |
| `emit_tenant` | ingest process JSONL through the **built** `kutha-tenant` binary (`runStatus` / `observed`); AS OF last *emitted* cut must match last status |
| `fold_log` | fold the process JSONL |
| `decide` | `ok` iff HIGH == 0 (LOW fails only with fail-on-warn) |
| `terminal` | accepting/rejecting sink |

Severity on a check step: `high` (default) or `low`.

Settings: copy `.env.example` to `.env`. CLI `--budget` / `--fail-on-warn` override env `KUTHA_GOV_BUDGET` / `KUTHA_GOV_FAIL_ON_WARN`, which override `defaults.budget` in `fsm.yaml`. Cargo observe timeout: `KUTHA_GOV_CARGO_TIMEOUT_SEC`, then `timeout_sec` on the FSM state. Tenant ingest: `KUTHA_HARNESS_LOG`, `KUTHA_TENANT_DIR`, `KUTHA_TENANT_TIMEOUT_SEC`, `KUTHA_TENANT_BIN` (unset = `$CARGO_TARGET_DIR/debug/kutha-tenant`). Process allowlist path: `KUTHA_HARNESS_RELATIONS_PATH`, then `.kutha/dictionaries/relations.yaml`. Product and process relation dictionaries must not mix schemas (`plane-mix`).
