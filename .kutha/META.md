# Harness meta-prompt (constitution)

Version: 1  
Not an ADR. Not product SoT. LLM does not execute these checks.

This is the **process analog of ADR-050**: a versioned constitution plus dictionaries. The governor CLI is an interpreter. Adding a check is a dictionary edit. Adding a *kind* is a rare kernel change (allowlist below + `scripts/kutha_gov/kinds.py` + a test).

## Fail-closed cycle

```text
ci → load META.md → load dictionaries/checks.yaml → load dictionaries/fsm.yaml
  → walk FSM (unknown kind/transition → fail)
  → for each check, for each step: kind must be in the allowlist
  → observe_cargo (cargo test is evidence, not product SoT)
  → emit log → emit_tenant (harness JSONL → Kutha log, AS OF process)
  → fold → terminal ok|fail
```

Unknown `kind` → HIGH. Unknown FSM kind → HIGH (`unknown-fsm-kind`). Missing dictionary → HIGH. Python `Check` subclasses are not the intake path.

## How to add a check

1. Append an entry to `.kutha/dictionaries/checks.yaml` using an **allowed kind**.
2. Run `uv run kutha-gov ci` and `uv run kutha-gov explain <id>`.
3. Do not add `scripts/kutha_gov/checks/*.py`.

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

## Allowed FSM kinds

| Kind | Meaning |
|------|---------|
| `noop` | no work; emit `ok` |
| `require_file` | path must exist |
| `run_checks` | interpret the checks dictionary (budget from env / FSM defaults) |
| `observe_cargo` | run `cargo test` as evidence (not SoT); required test names must appear `... ok` |
| `emit_log` | append `harness.run` triples |
| `emit_tenant` | ingest process JSONL through `kutha-tenant` (`runStatus` / `observed`); AS OF must match last status |
| `fold_log` | fold the process JSONL |
| `decide` | `ok` iff HIGH == 0 (LOW fails only with fail-on-warn) |
| `terminal` | accepting/rejecting sink |

Severity on a check step: `high` (default) or `low`.

Settings: copy `.env.example` to `.env`. CLI `--budget` / `--fail-on-warn` override env `KUTHA_GOV_BUDGET` / `KUTHA_GOV_FAIL_ON_WARN`, which override `defaults.budget` in `fsm.yaml`. Cargo observe timeout: `KUTHA_GOV_CARGO_TIMEOUT_SEC`, then `timeout_sec` on the FSM state. Tenant ingest: `KUTHA_HARNESS_LOG`, `KUTHA_TENANT_DIR`, `KUTHA_TENANT_TIMEOUT_SEC`.
