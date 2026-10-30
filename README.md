# Kutha

[![License](https://img.shields.io/github/license/rager306/kutha-graph)](LICENSE)
[![Release](https://img.shields.io/github/v/release/rager306/kutha-graph?display_name=tag)](https://github.com/rager306/kutha-graph/releases/tag/0.0.01)
[![ci](https://img.shields.io/github/actions/workflow/status/rager306/kutha-graph/ci.yml?branch=main)](https://github.com/rager306/kutha-graph/actions/workflows/ci.yml)

Research-stage **hybrid AI-native temporal graph engine** (Rust). An append-only event log is the source of truth; the working graph is a deterministic fold of that log; CSR / later HNSW / views are droppable leases. LLM proposes; dictionaries and the log own audited fact truth.

Primary wedge: legal / normative temporal agents — a norm **AS OF a date** as three cuts (valid-time `as_of`, bitemporal `live_at`, knowledge-cut `as_known_on`), without treating the LLM as legal authority.

This is not a production graph database and not Graphiti / Samyama / Harvey.

## Status

| Plane | Where | Now |
|-------|--------|-----|
| Delivery lease | [`.kutha/STATE.md`](.kutha/STATE.md) | **M175-S01 active** (`L_delivery=M175-S01-open`; C4 path-policy foundation); harness phase **H5** |
| GSD planning | [`.planning/STATE.md`](.planning/STATE.md) | **v1.68 C4 path-policy foundation active** (2026-10-30) on gsd-core **1.19.0**; phases 257–258 |
| Architecture map | [`docs/ADR/README.md`](docs/ADR/README.md) | Spine + honeycomb **Proposed**, not Accepted |
| Capability | `crates/kutha-runtime` tests | FF5 green (`as_of(2015) ≠ as_of(2021)` on a statute-shaped fixture) |

Governor green ≠ ADR Accepted ≠ capability. Honeycomb cells are a map, not a backlog.

**Frozen items and their thaw gates:** see [`.kutha/STATE.md`](.kutha/STATE.md) Thaw map. Do not duplicate that table here.

## Repository

Public research tree under the [Apache License 2.0](LICENSE) (SPDX `Apache-2.0`). ADR-092 D092-2 records that choice. Security reports go through [`SECURITY.md`](SECURITY.md). How to change the tree: [`CONTRIBUTING.md`](CONTRIBUTING.md). What was turned on, and what was left off on purpose: [`docs/process/repository.md`](docs/process/repository.md).

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

This host’s Cargo uses **kache** as `rustc-wrapper` and **mold** as linker (`~/.cargo/config.toml`). Workspace `Cargo.toml` sets `profile.dev` to `line-tables-only` and disables dep DWARF/incremental so `target/` stays small across harness `tmp/` copies. If the wrapper cannot run, use `cargo --config 'build.rustc-wrapper=""' test --workspace` (same override via `RUSTC_WRAPPER=`).

## Layout

| Path | Role |
|------|------|
| `README.md` | Human entry (status, commands, where to read) |
| `CLAUDE.md` | Shim → `AGENTS.md` |
| `AGENTS.md` | Agent operating contract (language, planes, freeze pointer) |
| `CHANGELOG.md` | Dated project history (product vs process) |
| `STRATEGY.md` | Wedge, metrics, non-goals |
| `Cargo.toml`, `pyproject.toml` | Rust workspace + uv harness (Python 3.13) |
| `.planning/` | Active GSD overlay: STATE, PROJECT, REQUIREMENTS, ROADMAP, `config.json`, `codebase/`, `intel/` |
| `.planning-archive/` | Pre-reset GSD planning tree (2026-10-28); not active |
| `.kutha/` | Harness pyramid: STATE, ROADMAP, dictionaries, gitignored `events.jsonl` |
| `.cursor/` | Legacy Cursor path: installed `gsd-core`, workflow markdown, and old rules/agents; not an AllSpark runtime |
| `crates/kutha-common`, `crates/kutha-runtime` | Product: events, fold, fork diff, quantum, CSR lease, thin query IR / hop workload, allowlist, tenant ingest |
| `docs/ADR/` | Spine + honeycomb (Proposed) |
| `docs/process/` | Harness contract, governor intake, Codex subagents |
| `docs/process/governor-intake.md` | Control loop / bridge / map → governor dictionaries |
| `docs/process/gsd-auto-contract.md` | Id map, controller protocol, code-graph tool pair, verification and artifact gates for GSD auto runs |
| `docs/process/repository.md` | Public-repo baseline: community files, CI, GitHub settings, Apache-2.0 |
| `SECURITY.md`, `CONTRIBUTING.md` | Vulnerability reports and how a change is allowed to land |
| `scripts/kutha_gov/` | Governor CLI interpreter |
| `verus/`, `tools/verus/` | Verus proofs of pure kernels (evidence, not a gate) and the pinned release + checksum |
| `.compound-engineering/artifacts/` | CE `docs_root` (applicability cards, research notes, handoffs archive) |

## Where to read next

1. [`.kutha/STATE.md`](.kutha/STATE.md) — what is allowed this week (harness lease)  
2. [`.planning/STATE.md`](.planning/STATE.md) — GSD phase focus (does not replace `.kutha/`)  
3. [`STRATEGY.md`](STRATEGY.md) — why this shape  
4. [`docs/ADR/README.md`](docs/ADR/README.md) — STCA → honeycomb  
5. [`docs/process/kutha-harness.md`](docs/process/kutha-harness.md) — parallel process plane  
6. [`docs/architecture/p0-spike-inventory.md`](docs/architecture/p0-spike-inventory.md) — P0 crate spike (in / not in; not Accepted product)  
7. [`.compound-engineering/artifacts/research/applicability/`](.compound-engineering/artifacts/research/applicability/) — 164 files in `cards/` (163 original matrix rows + M002 intake `paper-kernel-io-dirty-throttle-cloud-oltp.md`, not a matrix row)

Agent instructions: [`AGENTS.md`](AGENTS.md). Claude Code also loads [`CLAUDE.md`](CLAUDE.md) (shim to the same file).

## What runs

These are spike capabilities in the crates. Honeycomb cells stay **Proposed**. None of this is a product release.

- Point-in-time reads: `as_of`, `live_at`, `as_known_on` (FF5: `as_of(2015) ≠ as_of(2021)` on a statute-shaped fixture).
- A sealed log prefix can be replaced by a checkpoint receipt. `fork_at` fails closed on a compacted cut. A checkpoint blob without `behaviors_exact: true` fails open. The next checkpoint copies Horn behaviors only from a blob that `Op::Checkpoint` cites. A compacted Horn whose `caused_by` does not support the head fails replay. A Behavior whose rule blob is missing fails replay (`ExecutionReplayFailed`). An ordinary `StoreBlob` whose bytes do not match its digest fails open (`UnknownRuleVersion`).
- `diff_folds` / `Runtime::diff` is a fact-set diff of two folds, including a fork against its parent. It is not graph-edit distance and not a merge. Scan on each side is the live picture; the diff also keeps dead rows.
- Two Horn paths that share one Behavior EventId both stay indexed, once per cause. Retract of that id clears every reverse pointer.

## Limitations

- Release id is **`0.0.01`** ([`VERSION`](VERSION)), format `N.N.NN`. Cargo and the harness say `0.0.1` because a leading zero is not a legal Cargo version. History is [`CHANGELOG.md`](CHANGELOG.md). This is not published to crates.io.
- Not a production graph database. Not Graphiti, Samyama, or Harvey.
- Cypher, HNSW, a legal pack, and ABAC stay frozen until their own leases.
- Pack and ontology licenses are still open. This tree is Apache-2.0 only.

## License

[Apache License 2.0](LICENSE) (SPDX `Apache-2.0`). ADR-092 records the choice and stays **Proposed**.
