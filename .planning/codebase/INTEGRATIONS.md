---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
# External Integrations

**Analysis Date:** 2026-09-29

## APIs & External Services

**Product runtime:**
- None — no HTTP clients, cloud SDKs, LLM APIs, or third-party graph services in `crates/kutha-common` / `crates/kutha-runtime` dependencies
- Do not wire Graphiti, Dify, Hindsight, OpenAI, Anthropic, or similar as source of truth (product formula: event log = SoT; LLM may propose elsewhere, not as audited fact authority)

**Harness / agent tooling (local process, not product SoT):**
- `codebase-memory-mcp` — Cursor MCP stdio server configured in `.cursor/mcp.json` (`command`: `codebase-memory-mcp` on PATH). Structural code-graph evidence for agents; not a honeycomb ADR and not product storage
- Compound Engineering config (`.compound-engineering/config.yaml`) — local docs/artifacts root only; no remote CE SaaS call from product crates

**Subprocess boundaries (in-repo tools):**
- Harness `observe` runs `cargo test` / related cargo commands (`scripts/kutha_gov/observe.py`)
- Harness tenant phase runs binary `kutha-tenant` (`scripts/kutha_gov/tenant.py` → `crates/kutha-runtime/src/bin/kutha-tenant.rs`)
- `scripts/kutha_gov/gitdiff.py` shells out to `git` for worktree diffs

## Data Storage

**Databases:**
- Not detected — no Postgres, SQLite client crate, RocksDB, or other DB. RocksDB remains frozen until `.kutha/STATE.md` names M002
- Product persistence is **local filesystem** via `crates/kutha-runtime/src/store.rs`:
  - `events.wal` — WAL durability cousin (CRC), not Rocks-as-SoT
  - `events.jsonl` — encoded event log
  - `snapshot.json` — fold snapshot lease
  - `terms.jsonl` — derived intern picture (durable meanings prefer `Op::Define`)
- Default tenant directory: `.kutha/tenant` (override `KUTHA_TENANT_DIR`)

**File Storage:**
- Local filesystem only
- Harness process log: `.kutha/events.jsonl` (often gitignored; H0 time axis) — path override `KUTHA_HARNESS_LOG`
- Dictionaries (committed YAML): `.kutha/dictionaries/*.yaml`, `crates/kutha-runtime/dictionaries/relations.yaml`
- CE durable artifacts: `.compound-engineering/artifacts/` (plans, research cards, handoffs) — not product SoT

**Caching:**
- None (no Redis/memcached). In-memory `Runtime` / CSR lease / intern map only for the session or loaded store dir

## Authentication & Identity

**Auth Provider:**
- Not applicable — no user auth, OAuth, ABAC, or API keys in product/harness runtime
- Env vars are process configuration only (budget, paths, timeouts), not identity credentials
- Secrets: do not commit `.env`; use `.env.example` as the documented template. Never put product secrets in harness config loaders (`scripts/kutha_gov/config.py` states no product secrets)

## Monitoring & Observability

**Error Tracking:**
- None (no Sentry/Datadog/OpenTelemetry crates or SaaS)

**Logs:**
- Product: `eprintln!` / `println!` on `kutha-tenant` CLI; library errors via `RuntimeError` / `std::io::Error`
- Harness: stdout findings / check results; append to `.kutha/events.jsonl` during `kutha-gov ci` quantum (emit/observe/tenant/fold phases)
- No centralized log shipping

## CI/CD & Deployment

**Hosting:**
- Not applicable — no production host/container deploy defined for Kutha product

**CI Pipeline:**
- No `.github/workflows/` in this repository
- Local gate: `.pre-commit-config.yaml`
  - `uv run kutha-gov precommit` (dictionary checks only — no cargo quantum, no JSONL write)
  - `uv run ruff check` / `ruff format --check` on `scripts/`
- Full harness quantum: `uv run kutha-gov ci` (manual / agent; FSM in `.kutha/dictionaries/fsm.yaml`)
- Product tests: `cargo test --workspace`

## Environment Configuration

**Required / documented env vars:**

| Variable | Plane | Role |
|----------|-------|------|
| `KUTHA_GOV_BUDGET` | Harness | Cui-lite budget (CLI `--budget` wins) — `scripts/kutha_gov/config.py` |
| `KUTHA_GOV_FAIL_ON_WARN` | Harness | Treat warnings as failures — `scripts/kutha_gov/config.py` |
| `KUTHA_GOV_CARGO_TIMEOUT_SEC` | Harness | Cargo observe timeout — `scripts/kutha_gov/observe.py` |
| `KUTHA_HARNESS_RELATIONS_PATH` | Harness | Process relation allowlist path — `scripts/kutha_gov/process_allow.py` |
| `KUTHA_TENANT_BIN` | Harness | Override path to `kutha-tenant` binary — `scripts/kutha_gov/tenant.py` |
| `KUTHA_TENANT_TIMEOUT_SEC` | Harness | Tenant subprocess timeout — `scripts/kutha_gov/tenant.py` |
| `KUTHA_HARNESS_LOG` | Both | Harness JSONL path (default `.kutha/events.jsonl`) — `kutha-tenant` + tenant.py |
| `KUTHA_TENANT_DIR` | Both | Persist dir (default `.kutha/tenant`) — `kutha-tenant` + tenant.py |
| `KUTHA_RELATIONS_PATH` | Product | FF6 relation allowlist YAML — `crates/kutha-runtime/src/allow.rs` |
| `KUTHA_MAX_CASCADE` | Product | Cascade limit — `crates/kutha-runtime/src/quantum.rs` |
| `CARGO_TARGET_DIR` | Build | Optional cargo target dir for tenant binary discovery |
| `RUSTC_WRAPPER` | Build | May need clearing if broken sccache (`README.md`) |

**Secrets location:**
- `.env` (gitignored) — local machine only; never commit
- `.env.example` present as template (do not invent cloud API keys for this stack)

## Webhooks & Callbacks

**Incoming:**
- None — no HTTP server or webhook endpoints in product/harness

**Outgoing:**
- None — no webhook clients or callback URLs in product/harness

## Integration anti-patterns (prescriptive)

### Do not add a production DB client “for convenience”

**What happens:** Adding RocksDB, Postgres, or a hosted graph API while freeze is on  
**Why it's wrong:** Violates delivery lease and D1/D2 (event log = SoT; projections are leases)  
**Do this instead:** Keep filesystem WAL/JSONL/`store::persist` until `.kutha/STATE.md` explicitly opens M002 Rocks

### Do not treat MCP / CE / governor as product integrations

**What happens:** Modeling Cursor MCP or `kutha-gov` as customer-facing integrations  
**Why it's wrong:** They are harness/agent planes; collapsing them into product SoT breaks lifecycle orthogonality  
**Do this instead:** Document MCP under agent tooling; keep product integrations empty until a real external membrane ships under an Accepted honeycomb cell

---

*Integration audit: 2026-09-29*
