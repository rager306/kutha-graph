---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
# Coding Conventions

**Analysis Date:** 2026-09-29

## Naming Patterns

**Files:**
- Rust modules: one concern per file under `crates/*/src/` — snake_case (`quantum.rs`, `leapfrog.rs`, `materializer.rs`)
- Rust integration tests: snake_case under `crates/kutha-runtime/tests/` with milestone/feature prefix (`m011_claim_supports.rs`, `ff6_allowlist.rs`, `h4_process_allows.rs`)
- Python packages: snake_case under `scripts/kutha_gov/` (`dictionary.py`, `process_allow.py`)
- Python tests: `test_*.py` under `scripts/tests/` (`test_kutha_gov.py`, `test_fsm.py`)
- Governor dictionaries: kebab-case ids in YAML (`m011-claim-supports`, `observe-required-fn`) in `.kutha/dictionaries/`

**Functions:**
- Rust: `snake_case` — `emit`, `replay_check`, `leapfrog_intersect`, `load_allowed_names`
- Python: `snake_case` — `run_step`, `get_checks`, `cmd_precommit`, `interpret_cargo_output`
- Private Python helpers: leading underscore — `_str`, `_severity`, `_detect_root` in `scripts/kutha_gov/kinds.py` / `__main__.py`

**Variables:**
- Rust locals: `snake_case` (`fact_seq`, `valid_from`, `max_cascade`)
- Graph/relation **string** vocabulary: camelCase domain names as data — `"knows"`, `"knownBy"`, `"inForceAs"`, `"relatedTo"` (not Rust identifiers)
- Env vars: `SCREAMING_SNAKE` — `KUTHA_RELATIONS_PATH`, `KUTHA_GOV_BUDGET`, `KUTHA_TENANT_BIN`

**Types:**
- Rust structs/enums: `PascalCase` — `Runtime`, `RuntimeError`, `GraphFold`, `QuantumReceipt`, `Op`
- Rust type aliases: `PascalCase` — `EventId`, `TermId`, `ValidTime`, `TransactionTime` in `crates/kutha-common/src/event.rs`
- Python: `PascalCase` classes — `Finding`, `CheckResult`, `Context`, `DictCheck`, `HarnessTests`
- Python enums: `StrEnum` with uppercase members — `Severity.HIGH`, `Severity.LOW` in `scripts/kutha_gov/protocol.py`

**Language (all artifacts):**
- Source, identifiers, comments, docstrings, commit messages, CI logs: **English only**
- Do not mix Russian into code or docs; chat with humans is Russian, documents stay English (`AGENTS.md`)

## Code Style

**Formatting (Python):**
- Tool: **ruff** via `uv run` — config in `pyproject.toml` `[tool.ruff]`
- Line length: **100**
- Target: **py313**
- `src = ["scripts"]`
- Format check: `uv run ruff format --check scripts/` (also via `uv run kutha-gov py` and pre-commit on staged `scripts/` files)

**Linting (Python):**
- Ruff lint select: `E`, `F`, `I`, `UP`, `B`, `SIM`, `C4`, `RUF`; ignore `E501` (`pyproject.toml` `[tool.ruff.lint]`)
- Type checkers (dogfood, not optional style): **ty** (`--error-on-warning`) and **pyrefly** on `scripts/kutha_gov` + `scripts/tests` via `cmd_py` in `scripts/kutha_gov/__main__.py`
- Run full Python gate: `uv run kutha-gov py`

**Formatting / linting (Rust):**
- No checked-in `rustfmt.toml` or `clippy.toml` — use `cargo fmt` / workspace edition **2021** defaults (`Cargo.toml` `[workspace.package]`)
- Edition and version come from workspace: `version = "0.0.0"`, `publish = false`
- Prefer idiomatic Rust without adding clippy allow-lists unless a slice lease requires it

**Pre-commit (`.pre-commit-config.yaml`):**
1. `uv run kutha-gov precommit` — dictionary checks only (no cargo quantum, no JSONL)
2. `uv run ruff check` on staged Python under `^scripts/`
3. `uv run ruff format --check` on staged Python under `^scripts/`
- Install: `uvx pre-commit install --overwrite`
- Do **not** put full `kutha-gov ci` in the commit hook

## Import Organization

**Rust order (typical module):**
1. `crate::…` / `kutha_common::…` / `kutha_runtime::…` deps
2. External crates (`serde`, `sha2`, `uuid`, …)
3. `std::…`
4. Re-exports only from crate roots — `crates/kutha-runtime/src/lib.rs` and `crates/kutha-common/src/lib.rs`

**Python order:**
1. `from __future__ import annotations` (required in harness modules)
2. stdlib (`argparse`, `pathlib`, `unittest`, …)
3. third-party (`yaml`)
4. `kutha_gov.*` package imports
5. In tests that mutate `sys.path`, use `# noqa: E402` on imports after path insert (`scripts/tests/test_kutha_gov.py`)

**Path aliases:**
- Not applicable — no `paths` / TypeScript aliases
- Product: path deps `kutha-common = { path = "crates/kutha-common" }` in workspace
- Harness package root: `scripts/` via setuptools `package-dir` in `pyproject.toml`

## Error Handling

**Patterns (Rust product):**
- Return `Result<T, E>` with local enums implementing `std::fmt::Display` + `std::error::Error` — not `thiserror` / `anyhow` crates
- Primary runtime errors: `RuntimeError` in `crates/kutha-runtime/src/quantum.rs` (`ReplayDivergence`, `UnknownFact`, `UnknownRelation`, `UnknownClaim`, `BrokenLineage`)
- Tenant ingest: `TenantError` in `crates/kutha-runtime/src/tenant.rs`
- Fail-closed admit path: unknown relation / unknown claim → `Err`; log must not grow (assert in tests with `rt.log().len()`)
- Prefer `matches!(err, RuntimeError::…)` over string compares

**Patterns (Python harness):**
- Checks append `Finding` with `Severity.HIGH` or `Severity.LOW` — HIGH fails CI; LOW advisory unless `--fail-on-warn` / `KUTHA_GOV_FAIL_ON_WARN`
- Unknown kind / unknown FSM kind / load failure → HIGH fail-closed (`scripts/kutha_gov/kinds.py`, `dictionary.py`)
- Check runner wraps exceptions as HIGH `category="check-error"` in `_run_one` (`scripts/kutha_gov/__main__.py`)
- Do not raise for expected policy failures; emit findings

**YAML governor (policy, not exceptions):**
- New **check** = append a row to `.kutha/dictionaries/checks.yaml` (and ledger row in `invariants.yaml` or `bridges.yaml` when required)
- New **kind** = rare change to `ALLOWED_KINDS` / runners in `scripts/kutha_gov/kinds.py` plus `.kutha/META.md`
- Unknown `kind` in a step → HIGH `"unknown-kind"` — never silent skip

## Logging

**Framework:** Not a logging crate — CLI prints and check findings

**Patterns:**
- Governor findings: `Finding.format()` → `[HIGH|low] loc (check/category) message` (`scripts/kutha_gov/protocol.py`)
- `kutha-gov py` prints per-tool `OK`/`FAIL` and `PYTHON_TOOLING_GREEN` / `PYTHON_TOOLING_FAIL`
- Cargo observe treats test output as **evidence**, not SoT (`scripts/kutha_gov/observe.py` module docstring)

## Comments

**When to Comment:**
- Module-level `//!` / `"""…"""` stating plane (product vs harness), ADR id, and fail-closed intent
- Doc comments on public ops and ADR-tied fields (`claim` on `Op::Assert` in `crates/kutha-common/src/event.rs`)
- Inline comments for non-obvious freeze / lifecycle rules — keep short

**Rustdoc / Python docstrings:**
- English; cite ADR numbers when encoding locks (e.g. ADR-011, ADR-010)
- Integration tests: file-level `//!` describing the fitness case (`crates/kutha-runtime/tests/m011_claim_supports.rs`)

## Function Design

**Size:** Prefer focused modules; avoid new god-classes. Large files exist (`kinds.py`, `quantum.rs`) — extend by YAML steps or small helpers, not new parallel interpreters

**Parameters:**
- Rust: pass `Op` / ids by value or reference consistently with neighbors; `emit(&mut self, op: Op) -> Result<QuantumOutcome, RuntimeError>`
- Python steps: `Mapping[str, object]` with typed extractors `_str`, `_str_list`, `_path_list`, `_int`

**Return Values:**
- Rust: `Result` for fallible admit/emit/replay; plain values for pure helpers (`leapfrog_intersect`)
- Python checks: always `CheckResult` with findings list; `passed` iff no HIGH

## Module Design

**Exports (Rust):**
- Keep modules `mod foo;` private in `lib.rs`; `pub use` only the public surface (`Runtime`, `RuntimeError`, `GraphFold`, …)
- Use `pub(crate)` for shared internals (`cascade_limit`, allow helpers)
- Do **not** put Python inside `kutha-runtime`; do **not** add repo-root `ports/` / `adapters/` / `domain/`

**Exports (Python):**
- Package entry: `kutha-gov = "kutha_gov.__main__:main"` (`pyproject.toml`)
- `scripts/kutha_gov/checks/` must stay empty of check classes — only `__init__.py` re-exports `get_checks` from `dictionary.py` (enforced by `test_checks_come_from_dictionary_not_python_classes`)

**Barrel Files:**
- Rust crate `lib.rs` is the barrel
- Python: no deep `__all__` barrels; import concrete modules

## Two-Plane Rule (mandatory)

| Plane | Location | Owns |
|-------|----------|------|
| Product | `crates/kutha-*` | Event log = SoT, fold, quantum, receipts |
| Harness | `scripts/kutha_gov`, `.kutha/` | Trajectory, freeze, dictionary checks, FSM quantum |

- Do not collapse governor green with ADR Accepted or product capability
- Product relation schema `kutha-relations/v1` ≠ harness process relations (`plane-mix` checks)
- Intern map (ADR-011) ≠ agent dictionaries (ADR-050) — do not merge vocabularies

## Adding Code Checklist

1. **Product spike:** edit `crates/kutha-common` or `crates/kutha-runtime`; add unit `#[cfg(test)]` and/or `tests/*.rs`; keep freeze deps out of `Cargo.toml`
2. **Governor check:** append YAML in `.kutha/dictionaries/` — do **not** add a Python `Check` subclass
3. **FSM phase:** append `.kutha/dictionaries/fsm.yaml` — do **not** hardcode phases in Python
4. **After Python edits:** `uv run kutha-gov py` and `uv run pytest`
5. **After Rust edits:** `cargo test --workspace`; if the test is a fitness gate, add its function name to `observe_cargo.required` in `fsm.yaml` and a bridge/check as needed

---

*Convention analysis: 2026-09-29*
