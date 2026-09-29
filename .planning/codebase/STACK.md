---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
# Technology Stack

**Analysis Date:** 2026-09-29

## Languages

**Primary:**
- Rust 2021 edition (workspace `0.0.0`, `publish = false`) — product plane in `crates/kutha-common` and `crates/kutha-runtime`
- Host toolchain observed: `rustc` / `cargo` 1.98.1 — no `rust-toolchain.toml` pin in-repo; use a recent stable Cargo that supports workspace resolver `"2"`

**Secondary:**
- Python >=3.13 (pin file `.python-version` = `3.13`) — harness plane only (`scripts/kutha_gov`, package name `kutha-harness` in `pyproject.toml`)
- YAML — governor dictionaries under `.kutha/dictionaries/`, product relation allowlist at `crates/kutha-runtime/dictionaries/relations.yaml`
- TOML — `Cargo.toml`, `pyproject.toml`, workspace/crate manifests
- Markdown — process/product docs (`AGENTS.md`, `docs/ADR/`, `.kutha/STATE.md`); not runtime code
- Bash — thin wrappers (`scripts/kutha-gov`) and local pre-commit entries

Do **not** put Python inside `kutha-runtime`. Do **not** introduce TypeScript/Node as the graph core.

## Runtime

**Environment:**
- Product: native Rust library + binary `kutha-tenant` (`crates/kutha-runtime/src/bin/kutha-tenant.rs`) — in-process, no async runtime crate (no Tokio/async-std in workspace deps)
- Harness: CPython 3.13 via **uv** only — invoke with `uv run …` or `scripts/kutha-gov` (forces `--python 3.13`). System `python3` (e.g. 3.12) is not the supported harness interpreter

**Package Manager:**
- Rust: Cargo — lockfile `Cargo.lock` present
- Python: uv — lockfile `uv.lock` present; `[tool.uv] package = true`; setuptools build backend packages from `scripts/`

## Frameworks

**Core:**
- None (no web framework, ORM, or graph DB client). Product is a self-contained Rust event-log / fold / CSR lease spike
- Harness is a YAML-driven CLI interpreter (`kutha-gov`), not a web app

**Testing:**
- Rust: built-in `cargo test` (crate unit tests + `crates/kutha-runtime/tests/`)
- Python: pytest >=8 (`scripts/tests/`, configured in `pyproject.toml` `[tool.pytest.ini_options]`)

**Build/Dev:**
- Cargo workspace resolver `"2"` (`Cargo.toml`)
- uv for harness install/run
- Ruff (lint + format check), Astral `ty`, Meta `pyrefly` — via `uv run kutha-gov py` / pre-commit
- pre-commit (local hooks in `.pre-commit-config.yaml`) — `uvx pre-commit install --overwrite`

## Key Dependencies

**Critical (Rust workspace — `Cargo.toml` `[workspace.dependencies]`, locked in `Cargo.lock`):**
- `uuid` 1.24.1 — features `v7`, `std`, `serde` — event / harness IDs
- `serde` 1.0.229 + `serde_json` 1.0.151 — event encode/decode, snapshot/JSONL I/O
- `sha2` 0.10.9 — fingerprints / hashing in common + runtime
- `crc32fast` 1.5.0 — WAL / durability cousin integrity (`kutha-runtime` only)
- `kutha-common` (path) — lean `Event`, `Op`, intern map consumed by `kutha-runtime`

**Critical (Python — `pyproject.toml`):**
- `pyyaml` >=6 (locked 6.0.3) — load checks / FSM / honeycomb / process relations YAML

**Dev-only (Python `[dependency-groups].dev`):**
- `pytest` 9.1.1 — harness tests
- `ruff` 0.16.3 — lint/format (`line-length` 100, `target-version` py313)
- `ty` 0.0.72 — type check (Astral)
- `pyrefly` 1.2.0 — type check (Meta)

**Infrastructure:**
- Not applicable for product: no RocksDB, Redis, Postgres, HTTP client, or LLM SDK in crate deps (frozen until `.kutha/STATE.md` names them)

## Configuration

**Environment:**
- Copy `.env.example` → `.env` (gitignored). Loaders: `scripts/kutha_gov/config.py` (`load_dotenv`) and `crates/kutha-runtime/src/allow.rs` (`apply_dotenv`). CLI flags override env for harness budget / fail-on-warn
- Product env: `KUTHA_RELATIONS_PATH`, `KUTHA_MAX_CASCADE`, `KUTHA_HARNESS_LOG`, `KUTHA_TENANT_DIR`
- Harness env: `KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`, `KUTHA_GOV_CARGO_TIMEOUT_SEC`, `KUTHA_HARNESS_RELATIONS_PATH`, `KUTHA_TENANT_BIN`, `KUTHA_TENANT_TIMEOUT_SEC`
- Build-adjacent: `CARGO_TARGET_DIR`, optional empty `RUSTC_WRAPPER=` when sccache is broken (documented in `README.md`)

**Build:**
- `Cargo.toml` — workspace members `crates/kutha-common`, `crates/kutha-runtime`
- `crates/*/Cargo.toml` — per-crate deps
- `pyproject.toml` — harness package, tool configs (ruff/ty/pyrefly/pytest)
- `.compound-engineering/config.yaml` — CE `docs_root: .compound-engineering/artifacts` (process artifacts, not product SoT)
- `.cursor/mcp.json` — agent MCP: `codebase-memory-mcp` on PATH

## Platform Requirements

**Development:**
- Linux (documented host); Rust stable + Cargo; uv; Python 3.13 for harness
- Git (harness `gitdiff.py` / observe subprocesses)
- Optional: pre-commit via `uvx`

**Production:**
- Not applicable — research-stage library/CLI; no production deploy target, license undecided (ADR-092). Do not treat the tree as a shipped graph DB service

## Dual-plane rule (prescriptive)

| Plane | Stack | Root |
|-------|--------|------|
| Product | Rust crates only | `crates/kutha-*` |
| Harness | Python 3.13 + YAML dictionaries | `scripts/kutha_gov`, `.kutha/` |

Keep them orthogonal: governor green ≠ ADR Accepted ≠ product capability.

---

*Stack analysis: 2026-09-29*
