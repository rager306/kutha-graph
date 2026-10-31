# AGENTS.md — Kutha / kutha-graph

Operating contract for **AllSpark** agents (prime-agent runtime). Chat with the human is **Russian**; this file, docs, code, commits, and logs stay **English**.

## Runtime: AllSpark, not Cursor

- Cursor is **not used**. `.cursor/` remains only as the installed location of `gsd-core` and legacy skill markdown. `.cursor/rules/*.mdc`, `.cursor/hooks`, `.cursor/agents`, and Cursor `Task` syntax are **not loaded** by AllSpark. Do not rely on `/gsd-*`, `/ce-*`, or other Cursor slash commands.
- This file is the only always-on instruction. Rules that matter must live here or in a linked document under `docs/process/`; a rule only in `.cursor/rules/` is invisible.
- Work through `python_repl`; use native tools `edit`, `graph`, `lsp`, and `jev`. Use `bash()` for shell commands, never `subprocess`/`os.system`. Python state persists across cells and compactions; keep large data on disk.
- Skills: prime-agent skills live in `/root/.local/libexec/skills/<name>/SKILL.md`; project skills in `.cursor/skills/<name>/SKILL.md`. Read the `SKILL.md`; do not assume its slash command is registered.
- MCP: `await mcp.list_tools(server)` then `await mcp.call_tool(server, tool, args)`. Connections include `codebase-memory-mcp`, `gitnexus`, `exa`, `tavily`, `jina`, `zai-web-search`, `keenable`, `arsumbris`, `google-dev-knowledge`.
- Subagents: `handle = await rlm.spawn("task", name="gsd-verifier")`; results arrive only by `agent_message`. Child → parent: `await agent_message.send("...", receiver_role="parent")`. Parent → child: `receiver_role="child", receiver_name="..."`. Recover handles with `await rlm.list_subagents()`; snapshot with `await rlm.collect([handle])`.
- Children do **not** inherit this file. Paste the relevant contract (lease, thaw map, owned files, verification rule) into every child prompt. Give each child disjoint file ownership. The parent owns integration, shared files, graph indexing, and the final Russian reply.
- AllSpark-owned state under `~/.allspark-home/` (sessions, leases, `graph.db`) is not repository content; do not edit or commit it.
- Durable artifacts go to `.compound-engineering/artifacts/`; one-shot scratch goes to repo-local gitignored `tmp/`. Never use system `/tmp`, `$TMPDIR`, or `.tmp` for surviving output. GSD experiments use `tmp/planning-probe/`, never the live `.planning/`.
- Custom model providers: `/root/.allspark-home/.prime/agent/models.json` (`providers.<name>` with `api: openai-completions`, `baseUrl`, `apiKey` — mandatory even when the key also sits in `auth.json` via `/login`, or the loader drops the provider with a warning — and `models[]` with `contextWindow`/`maxTokens`/`cost` from models.dev). Verify with `allspark model list <name>` from a clean shell; a preset `PRIME_AGENT_CODING_AGENT_DIR` makes the CLI resolve a doubled `.allspark-home` path. Configured: `cline` (api.cline.bot; `cline-pass/deepseek-v4.1-flash-1017`, `cline-pass/qwen3.7-max`) and `open-design` (amr-link.open-design.ai/v1; `deepseek-v4.1-flash`).

## Rust review: BeautifulRust (advisory)

BeautifulRust is installed in AllSpark at commit `f863cd3` (MIT, fork `rager306/BeautifulRust`); use `ALLSPARK_HOME=/root/.allspark-home`, because the nested home has an empty `auth.json` and returns provider HTTP 400. `allspark/check.py` reports `0 errors, 0 warnings`.

- Use `rust-best-practices` and `rust-review` as **advisory** guidance for Rust changes and targeted audits. `rust-port` is deferred: Kutha is Rust-native, and Python stays on the harness plane.
- BeautifulRust is not a source of truth, not an ADR source, not a governor check, and not a CI gate. ADR-000 D1–D10, `docs/ADR/`, and the current lease win. Treat Kutha as an internal library (`publish = false`, research stage): simplicity rules, not public-API ceremony.
- Do not paste the full `lints.toml` or enable `missing_docs`, `as_conversions`, `pedantic`, or `max-fn-params-bools = 0` globally. Do not add a `cargo fmt --check` gate; HEAD is not rustfmt-clean. Do not use `--all-features` for CI clippy: it compiles `librocksdb-sys`, while CI runs with `KUTHA_GOV_SKIP_ROCKS=1`.
- `rust_review` is a map, not a verdict: its 48 `unwrap` runtime count includes inline tests; the production count is 4. See `docs/process/beautifulrust.md` for measured baselines, the narrow lint subset, and the review procedure. A new CI phase still needs the governor-intake row.

## Two planes and the GSD overlay (do not collapse)

| Plane | Owns | Must not own |
|-------|------|----------------|
| **Product** (`crates/kutha-*`) | Temporal graph truth: event log = SoT | Roadmap ceremony |
| **Harness** (`scripts/kutha_gov`, `.kutha/`) | Trajectory honesty, freeze, lease, lifecycle non-collapse | Architecture decisions, product readiness |
| **GSD core** (`.planning/`, `.cursor/gsd-core`) | Phase planning, execution artifacts, progress | Architecture truth, harness lease, ADR status |

Governor green ≠ ADR Accepted ≠ capability. Honeycomb **Proposed** ≠ delivery backlog. `.planning/STATE.md` is GSD progress; `.kutha/STATE.md` is the harness lease; `docs/ADR/` is architecture. GSD tools must **never** edit `.kutha/STATE.md`; the parent controller sets and clears the lease. Do **not** use GSD Path (`@opengsd/gsd-path`, `.project/`, `/gsd-path`) in this repo.

## Where to start

Read `.kutha/STATE.md` first; do not start work it does not name. Then `.planning/STATE.md` (GSD focus, does not thaw or lease), `docs/ADR/README.md`, and `docs/architecture/p0-spike-inventory.md`. Repo layout: `README.md` § Layout. Dated history: `CHANGELOG.md` and `.compound-engineering/artifacts/handoffs/`.

## Product snapshot

- **Product:** Kutha — hybrid AI-native temporal graph engine (research stage, not a production database).
- **Idea stack:** **STCA** (ADR-002) → vision (ADR-001) → locks D1–D10 (ADR-000) → honeycomb cells (ADR-010–093, all **Proposed**).
- **Formula:** event log = SoT; graph = deterministic fold; CSR/HNSW/views = droppable leases; LLM proposes; dictionaries + log own audited truth.
- **Wedge:** legal / normative temporal agents (norm **AS OF** a date). Science next; finance/clinical wait for a pack.

## gsd-core: planning engine

Installed **v1.19.0** at `.cursor/gsd-core` (legacy Cursor path, profile `full`). AllSpark invokes the CLI directly; there is no native AllSpark runtime integration. Upgrades are owner-approved and follow `.cursor/gsd-core/workflows/update.md`.

```text
GSD="node .cursor/gsd-core/bin/gsd-tools.cjs"
$GSD runtime-identity --raw
$GSD validate health --cwd /root/kutha-graph
$GSD validate consistency --cwd /root/kutha-graph
$GSD roadmap validate --cwd /root/kutha-graph
$GSD smart-entry --json --cwd /root/kutha-graph
$GSD progress --json --cwd /root/kutha-graph
$GSD state --raw --cwd /root/kutha-graph
$GSD state-snapshot --cwd /root/kutha-graph
$GSD find-phase <N> --cwd /root/kutha-graph
$GSD phase list-plans <N> --cwd /root/kutha-graph
$GSD audit-open --cwd /root/kutha-graph
$GSD init new-milestone --cwd /root/kutha-graph   # context only; the workflow writes
```

- Read the workflow before acting: `.cursor/gsd-core/workflows/<name>.md`; skill wrappers are `.cursor/skills/gsd-*/SKILL.md`. `new-milestone`, `plan-phase`, `execute-phase`, `verify-work`, `complete-milestone`, `audit-milestone`, `progress`, `health`, `update` are files, not slash commands.
- GSD `gsd-*` agents in `.cursor/agents/` are prompt files, not registered AllSpark agents. When a workflow names one, read its `.md` and pass it to `rlm.spawn` with a descriptive name.
- `.planning/config.json` still records `runtime: cursor`; that is installer metadata, not an active integration. Do not invoke Cursor hooks. Do not add a second GSD copy at another path.
- `--project-dir <root>` skips ancestor discovery; the given root must contain `.planning/`. Use it for probe copies.
- `state.json` is a derived snapshot; `STATE.md`, `ROADMAP.md`, and `MILESTONES.md` are the canonical GSD artifacts. Do not hand-author `state.json`.

### GSD artifact schema

- `.planning/ROADMAP.md` needs a `## Phases` list of `- [ ] **Phase N: Name** - ...` entries and `## Phase Details` with `### Phase N: Name`. A roadmap with only custom `## Phases (vX)` headings is **not recognized**.
- `NN-MM-PLAN.md` frontmatter needs `wave: N` (newer GSD). `NN-MM-SUMMARY.md` must pair with `NN-MM-PLAN.md`; a phase-level `NN-SUMMARY.md` is an orphan.
- `NN-VERIFICATION.md` `status:` is closed: `passed | gaps_found | human_needed`. Anything else blocks `progress`, `smart-entry`, and `audit-open`.
- `.planning/STATE.md` frontmatter follows `gsd_state_version: '1.0'` with `status`, `progress`, and `percent`; body carries current position, decisions, blockers, session continuity.
- `.planning/MILESTONES.md` is reverse chronological; `Shipped:` entries drive `latest_completed_milestone` and the archive path. `Active:` entries do not.
- `.planning/REQUIREMENTS.md` is milestone-scoped and archived at close. Do not let it grow into a second roadmap.

### GSD health after reset (2026-10-28)

The pre-native overlay was archived and `.planning` rebuilt on **1.19.0**. Milestone **v1.63 as_of regression** closed on 2026-10-29; no milestone is active. Current probes: `validate health` **healthy**, `validate consistency` clean, `audit-open` clear, `progress` 0 phases, `state-snapshot` `completed`. `init new-milestone` sees `latest_completed_milestone=v1.63` and `phase_archive_path=.planning/milestones/v1.63-phases`. `roadmap validate` warns V004 until the next phase exists; that is the only expected warning.

Pre-reset history: `.planning-archive/pre-native-2026-10-28/`, git branch `archive/pre-gsd-native-2026-10-28`, and local backup `tmp/backups/pre-gsd-1.19-2026-10-28.tar.gz`. Do not restore the old tree into `.planning/`; it has non-canonical verification statuses and an unrecognized ROADMAP.

Start the next milestone with the GSD `new-milestone` workflow (`.cursor/gsd-core/workflows/new-milestone.md`) only after `.kutha/STATE.md` names the lease. `new-milestone` archives and rewrites `.planning`; check `latest_completed_milestone` and `phase_archive_path` first. Never let GSD touch `.kutha/STATE.md`, `docs/ADR/`, or `crates/`.

## Commands

Product:

```text
cargo test --workspace
$CARGO_TARGET_DIR/debug/kutha-migrate <store-dir>   # after observe build; Op-wire then pins
```

Harness (Python **3.13** via **uv** only — not system `python3`):

```text
uv run kutha-gov ci          # FSM quantum: relations → checks → observe → emit → tenant → fold
uv run kutha-gov selftest    # mutate a copy; unproven fails
uv run kutha-gov precommit   # dictionary checks only (no cargo, no JSONL)
uv run kutha-gov json        # check results as JSON (optional --check ID)
uv run kutha-gov fsm         # print the process machine
uv run kutha-gov map         # compact L_map index
uv run kutha-gov py          # ruff + ty + pyrefly
uv run kutha-gov fold | list | explain …
uv run pytest                # harness unit suite; NOT part of kutha-gov ci
```

Flags (ci / precommit / selftest / json where noted): `--jobs N` (parallel checks; `1` = serial oracle), `--skip-cargo` (ci only; not closeout evidence), `--check ID[,ID…]` (json|precommit|selftest only — not `ci`; repeatable; each value may be comma-separated), `--budget V` (Cui truncate). Env: `KUTHA_GOV_JOBS`, `KUTHA_GOV_BUDGET`. Prefer `run_high=` / exit status over `ci | tail` (pipe hides exit).

Pin: `.python-version`. New governor check or CI phase: YAML row — `.kutha/META.md` and `docs/process/governor-intake.md` (not a new Python class). Hook: `uvx pre-commit install --overwrite` (precommit, not full `ci`).

## Verification honesty

- A product close needs `cargo test --workspace` exit 0 and `uv run kutha-gov ci --jobs 1` with `run_high=0`; `precommit` alone is not a close gate.
- `uv run pytest` is a separate harness gate. It is **not** inside `kutha-gov ci`; run it explicitly when changing `scripts/`.
- GitHub Actions is not evidence until it actually runs: check `gh run list -w ci.yml`. The local gate and the remote gate are different claims.
- Independent verification means a fresh verifier context, not the executor. `NN-VERIFICATION.md` needs `independent: true`, commands with exit statuses, and `status` from the closed set. Re-verify after any edit to the report or its cited tests.
- Bench gate: every product milestone that edits `crates/` records pre/post `ingest_plain.n` and `as_of.n` in the close entry. A regression over 10% is a stop.

## Working conventions

1. Honor locked ADR-000 **D1–D10**. Do not revive: pure Samyama product, pure ActiveGraph without hot projections, hard FSM as sole agent control, TypeScript as graph core, RVF as primary storage, Graphiti/Dify/Hindsight as SoT.
2. STCA first. New product detail → honeycomb ADR-010+ (`docs/ADR/README.md`), never silent rewrites of 000/001/002. **Accepted** only when that cell is in the running engine.
3. Honeycomb is a **map**. One steel thread at a time. “Promote all” is forbidden. GSD tracks phases, not cell enumeration.
4. Prefer falsifiable spikes over generic “build a graph DB” advice.
5. Core stays self-contained Rust (no mandatory external graph DB / Graphiti runtime / LLM for temporal truth).
6. Harness is a **parallel STCA plane** (`docs/process/kutha-harness.md`). H0 = files + JSONL + dictionaries + FSM; H2 = typed triples on the Kutha log via `kutha-tenant`. Do **not** copy `stca-guide.md` §5 JSON merge-patch as the product write surface. Control loop → check: `docs/process/governor-intake.md`.
7. Three harness lifecycles stay orthogonal: **L_map** · **L_delivery** · **L_capability**. Bridges may cite; they may not copy state machines.
8. Intern map (ADR-011) ≠ agent dictionaries (ADR-050). Do not collapse them.
9. Humans start at `README.md`. Agents follow this file, then `.kutha/STATE.md` + `.planning/STATE.md`. Dated history: `CHANGELOG.md`; the local `kutha-changelog` skill is `.cursor/skills/kutha-changelog/SKILL.md`. Commits are English; chat verbs are Russian. Release ids are `N.N.NN` in `VERSION` (now `0.0.01`); Cargo and pyproject use `0.0.1`. `publish` stays false. Tag only the id STATE names.
10. **Verus** is a bounded evidence tool for pure kernels only (VT interval algebra, later `leapfrog_intersect`; never I/O, serde, hashes, or whole-fold replay). Use it only inside a product slice, proofs outside `crates/`, no `assume`/`admit`, no `vstd` in production crates. Verus pass ≠ governor green ≠ ADR Accepted ≠ capability. Plan and costs: `.compound-engineering/artifacts/research/2026-10-22-long-horizon-direction-roadmap.md` § Verus.

## Code graph and search

The native `graph` tool is the primary structural plane: `search`, `trace`, `architecture`, `coverage`, `changes`, `index`. Use `lsp` for definitions, references, implementations, hover, renames, and diagnostics. Do not use `Grep`/`Read` to answer call-chain questions that `graph` or `lsp` can answer.

**codebase-memory-mcp (CBM)** is secondary evidence: `await mcp.call_tool("codebase-memory-mcp", ...)`; list projects first, canonical name `kutha-graph`. Forbidden: `delete_project`, `manage_adr` (`docs/ADR/` owns ADRs). Only the parent/integrator runs `index_repository`. GitNexus is secondary and only for M013 refactor phases per `docs/process/gsd-auto-contract.md`. `rg` is the arbiter for literal/text questions; `ast-grep` is for AST questions. Graph coverage ≠ governor green ≠ review approval.

## Layout rules

- Compound Engineering `docs_root` is `.compound-engineering/artifacts` (`.compound-engineering/config.yaml`). Plans, research, ideation, reviews, and handoffs go there, never under `docs/`.
- Do **not** add repo-root `ports/` / `adapters/` / `domain/` (ADR-022). Do **not** put Python inside `kutha-runtime`.
- One writer: only the integrator edits shared lockfiles, `.kutha/events.jsonl`, tenant data, graph indexes, and `.planning/` during a GSD run.
