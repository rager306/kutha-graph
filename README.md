# Kutha

Research-stage **hybrid AI-native temporal graph engine** (Rust). An append-only event log is the source of truth; the working graph is a deterministic fold of that log; CSR / later HNSW / views are droppable leases. LLM proposes; dictionaries and the log own audited fact truth.

Primary wedge: legal / normative temporal agents — a norm **AS OF a date**, without treating the LLM as legal authority.

This is not a production graph database and not Graphiti / Samyama / Harvey.

## Status

| Plane | Where | Now |
|-------|--------|-----|
| Delivery lease | [`.kutha/STATE.md`](.kutha/STATE.md) | **M011** active (S03 thin P→Q done); harness phase **H4** |
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
| `crates/kutha-common`, `crates/kutha-runtime` | Product: events, fold, quantum, CSR lease, allowlist, tenant ingest |
| `.kutha/` | Process harness (STATE lease, dictionaries, JSONL) |
| `.planning/` | GSD overlay (STATE/ROADMAP/REQUIREMENTS, codebase map, intel) |
| `docs/ADR/` | Architecture decisions |
| `STRATEGY.md` | Wedge, metrics, non-goals |
| `AGENTS.md` | Agent operating contract (GSD-first, language, freeze) |
| `CHANGELOG.md` | Dated project history (product vs process) |
| `docs/process/governor-intake.md` | Control loop / bridge / map → governor dictionaries |
| `.compound-engineering/artifacts/` | Legacy research archive (applicability cards, old CE handoffs) |

## Where to read next

1. [`.kutha/STATE.md`](.kutha/STATE.md) — what is allowed this week (harness lease)  
2. [`.planning/STATE.md`](.planning/STATE.md) — GSD phase focus (does not replace `.kutha/`)  
3. [`STRATEGY.md`](STRATEGY.md) — why this shape  
4. [`docs/ADR/README.md`](docs/ADR/README.md) — STCA → honeycomb  
5. [`docs/process/kutha-harness.md`](docs/process/kutha-harness.md) — parallel process plane  
6. [`.compound-engineering/artifacts/research/applicability/`](.compound-engineering/artifacts/research/applicability/) — 163 closed literature cards (`cards/` = SoT)

Agent instructions: [`AGENTS.md`](AGENTS.md). Claude Code also loads [`CLAUDE.md`](CLAUDE.md) (shim to the same file).

## License

Undecided (ADR-092). No SPDX identifier is Accepted. Do not treat this tree as Apache-2.0 or MIT by default.
