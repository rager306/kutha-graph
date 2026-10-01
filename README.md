# Kutha

Research-stage **hybrid AI-native temporal graph engine** (Rust). An append-only event log is the source of truth; the working graph is a deterministic fold of that log; CSR / later HNSW / views are droppable leases. LLM proposes; dictionaries and the log own audited fact truth.

Primary wedge: legal / normative temporal agents — a norm **AS OF a date**, without treating the LLM as legal authority.

This is not a production graph database and not Graphiti / Samyama / Harvey.

## Status

| Plane | Where | Now |
|-------|--------|-----|
| Delivery lease | [`.kutha/STATE.md`](.kutha/STATE.md) | **M012**; harness phase **H5** |
| Architecture map | [`docs/ADR/README.md`](docs/ADR/README.md) | Spine + honeycomb **Proposed**, not Accepted |
| Capability | `crates/kutha-runtime` tests | FF5 green (`as_of(2015) ≠ as_of(2021)` on a statute-shaped fixture) |

Governor green ≠ ADR Accepted ≠ capability. Honeycomb cells are a map, not a backlog.

**Frozen until STATE names them:** RocksDB crate, Cypher/GPML parser, HNSW, ADR-050 six dictionaries, full legal/science packs.

## Quick start

Product (Rust workspace):

```text
cargo test --workspace
```

Harness (Python **3.13** via **uv**, not system `python3`):

```text
cp .env.example .env
uv run kutha-gov ci
uv run kutha-gov precommit
uv run kutha-gov map
uv run pytest
```

If this host sets `rustc-wrapper = "sccache"` and the wrapper cannot run, use `cargo --config 'build.rustc-wrapper=""' test --workspace` (same override the harness may need via `RUSTC_WRAPPER=`).

## Layout

| Path | Role |
|------|------|
| `README.md` | Human entry (status, commands, where to read) |
| `CLAUDE.md` | Shim → `AGENTS.md` |
| `AGENTS.md` | Agent operating contract (language, planes, freeze pointer) |
| `CHANGELOG.md` | Dated project history (product vs process) |
| `STRATEGY.md` | Wedge, metrics, non-goals |
| `Cargo.toml`, `pyproject.toml` | Rust workspace + uv harness (Python 3.13) |
| `.planning/` | GSD overlay: STATE, PROJECT, REQUIREMENTS, ROADMAP, `config.json`, `codebase/`, `intel/`, `INGEST-CONFLICTS.md` |
| `.kutha/` | Harness pyramid: STATE, ROADMAP, dictionaries, gitignored `events.jsonl` |
| `.cursor/` | Cursor rules, MCP, CBM adapter, GSD skills/core |
| `crates/kutha-common`, `crates/kutha-runtime` | Product: events, fold, quantum, CSR lease, allowlist, tenant ingest |
| `docs/ADR/` | Spine + honeycomb (Proposed) |
| `docs/process/` | Harness contract, governor intake, Codex subagents |
| `docs/process/governor-intake.md` | Control loop / bridge / map → governor dictionaries |
| `scripts/kutha_gov/` | Governor CLI interpreter |
| `.compound-engineering/artifacts/` | CE `docs_root` (applicability cards, research notes, handoffs archive) |

## Where to read next

1. [`.kutha/STATE.md`](.kutha/STATE.md) — what is allowed this week (harness lease)  
2. [`.planning/STATE.md`](.planning/STATE.md) — GSD phase focus (does not replace `.kutha/`)  
3. [`STRATEGY.md`](STRATEGY.md) — why this shape  
4. [`docs/ADR/README.md`](docs/ADR/README.md) — STCA → honeycomb  
5. [`docs/process/kutha-harness.md`](docs/process/kutha-harness.md) — parallel process plane  
6. [`docs/architecture/p0-spike-inventory.md`](docs/architecture/p0-spike-inventory.md) — P0 crate spike (in / not in; not Accepted product)  
7. [`.compound-engineering/artifacts/research/applicability/`](.compound-engineering/artifacts/research/applicability/) — 163 closed literature cards (`cards/` = SoT)

Agent instructions: [`AGENTS.md`](AGENTS.md). Claude Code also loads [`CLAUDE.md`](CLAUDE.md) (shim to the same file).

## License

Undecided (ADR-092). No SPDX identifier is Accepted. Do not treat this tree as Apache-2.0 or MIT by default.
