---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
<!-- refreshed: 2026-09-29 -->

# Codebase Concerns

**Analysis Date:** 2026-09-29

> Distinguishes **intentional research-stage gaps** (freeze / spike scope in `.kutha/STATE.md`) from **actionable debt**. Do not plan M002 Rocks, Cypher, HNSW, ADR-050 six dictionaries, legal/science packs, or ADR-080/081 until STATE leases them. Governor green ≠ ADR Accepted ≠ capability — collapsing those is itself a process failure mode.

## Tech Debt

**God-module Runtime + co-located tests (`quantum.rs`):**
- Issue: Product control plane (emit, cascade, admit, fork, CSR lease, replay, derivation eligibility) plus a large `#[cfg(test)]` suite live in one ~670-line file.
- Files: `crates/kutha-runtime/src/quantum.rs`, `crates/kutha-runtime/src/lib.rs`
- Impact: High blast radius on every M011-class change; hard to review; CBM coverage on this path shows `metadata_changed` (index lag risk for graph-assisted work).
- Fix approach: Split tests into `crates/kutha-runtime/tests/` (pattern already used for FF5/M010/M011); extract admit/cascade/derivation into focused modules without expanding scope past the Active Slice lease.

**Harness interpreter concentration (`kinds.py` + fat YAML):**
- Issue: Check-kind interpreter is ~917 lines; governor dictionary is ~809 lines. New process fences tend to grow YAML + `kinds.py` together.
- Files: `scripts/kutha_gov/kinds.py`, `.kutha/dictionaries/checks.yaml`, `scripts/kutha_gov/fsm.py`, `scripts/kutha_gov/__main__.py`
- Impact: Process-plane regressions are expensive; agents edit the wrong plane when product ADRs and harness checks drift.
- Fix approach: Keep intake via YAML rows (`docs/process/governor-intake.md`); resist new Python `Check` subclasses; split kind handlers only when a kind family is stable and leased.

**Dual relation allowlists (product vs harness):**
- Issue: Two fail-closed lists with overlapping but different names — product `knows`/`knownBy`/`inForceAs`/… vs harness `status`/`cargo`/`allows`/….
- Files: `crates/kutha-runtime/dictionaries/relations.yaml`, `.kutha/dictionaries/relations.yaml`, `crates/kutha-runtime/src/allow.rs`, `scripts/kutha_gov/process_allow.py`
- Impact: Wrong `KUTHA_RELATIONS_PATH` / `KUTHA_HARNESS_RELATIONS_PATH` silently fails closed or admits the wrong vocabulary; easy to “fix” H2/H4 by pointing the product runtime at harness YAML.
- Fix approach: Document path ownership in one place; never merge lists until ADR-050 is leased; keep env defaults as in `.env.example`.

**Custom `.env` loader inside the product crate:**
- Issue: Hand-rolled dotenv walk + `std::env::set_var` runs from allowlist/cascade paths.
- Files: `crates/kutha-runtime/src/allow.rs` (`apply_dotenv`), `crates/kutha-runtime/src/quantum.rs` (`cascade_limit`)
- Impact: Process env leaks into product binaries; surprising cascade budgets in CI; not a full dotenv feature set (quoting/export edge cases).
- Fix approach: Prefer explicit env / CLI for product; confine dotenv to harness (`kutha-gov`) if possible under a later lease.

**WAL cousin is rewrite-on-persist, not append-only:**
- Issue: `append_events` opens with `truncate(true)` and rewrites the whole file each `persist`.
- Files: `crates/kutha-runtime/src/wal.rs`, `crates/kutha-runtime/src/store.rs`
- Impact: Name suggests Rocks WAL semantics; durability story is “full snapshot cousin,” not crash-safe append. Large logs rewrite cost grows with log length.
- Fix approach: Treat as intentional until M002; when leasing durability work, either rename or implement true append + checkpoint — do not pretend Rocks equivalence.

**Dual durable encodings (WAL preferred over JSONL):**
- Issue: `open` prefers `events.wal` when present; also writes `events.jsonl`, `snapshot.json`, `terms.jsonl`.
- Files: `crates/kutha-runtime/src/store.rs`
- Impact: Divergent files after manual edits → silent preference for WAL; operators may edit JSONL and see no effect.
- Fix approach: Document recovery precedence; add a check or tool that fingerprints WAL vs JSONL when both exist (process or product spike under lease).

**Hand-rolled YAML subset for relations:**
- Issue: `parse_relations` is a line scanner, not a YAML library.
- Files: `crates/kutha-runtime/src/allow.rs`
- Impact: Nested YAML, anchors, or non-list shapes fail closed (safe) but confuse authors; drift from harness PyYAML parsing.
- Fix approach: Keep fail-closed; if schema grows past FF6 stub, lease a real parser or share a validated intermediate format — without importing ADR-050.

## Known Bugs

**Cascade budget abort commits a partial quantum:**
- Symptoms: `QuantumOutcome.receipt.aborted_on_budget == true` while earlier events in the same `emit` are already folded and appended; pending follow-ons (e.g. inverse `knownBy`) may be missing.
- Files: `crates/kutha-runtime/src/quantum.rs` (`Runtime::emit`, `follow_ons`), `crates/kutha-runtime/src/receipt.rs`
- Trigger: Assert storm or low `KUTHA_MAX_CASCADE` / `Runtime::new(max_cascade)` so `used >= max_cascade` mid-cascade.
- Workaround: Keep cascade budget high for demos; call `replay_check` after emit; treat aborted receipts as failed quanta in callers (callers today often ignore the flag outside tests).
- Note: Covered by `budget_aborts_storm` in `quantum.rs` tests — behavior is known, not accidental, but unsafe to promote as production quantum semantics.

**CSR adjacency ignores relation identity:**
- Symptoms: `CsrLease::from_fold` pushes all live objects for a subject into one neighbor row regardless of relation.
- Files: `crates/kutha-runtime/src/csr.rs`, `crates/kutha-runtime/src/leapfrog.rs`
- Trigger: Multi-relation graphs (legal `inForceAs` + demo `knows` on same subjects).
- Workaround: Build CSR only on homogeneous fixtures; do not use CSR as a relation-typed index.
- Note: Acceptable P0 lease picture; wrong for Cypher-style typed MATCH (frozen until explicit lease).

## Security Considerations

**No auth / ABAC / multi-tenant isolation in product:**
- Risk: Any process that can call `Runtime::emit` or write tenant ingest can mutate the log.
- Files: `crates/kutha-runtime/src/quantum.rs`, `crates/kutha-runtime/src/tenant.rs`, `crates/kutha-runtime/src/bin/kutha-tenant.rs`
- Current mitigation: Research-stage local spike; FF6 allowlist fail-closed for unknown relation names; freeze blocks ADR-080/081 until M002-class lease (`.kutha/STATE.md`).
- Recommendations: Do not invent ABAC in a side branch; keep security honeycomb Proposed. Treat `kutha-tenant` as dogfood, not a public API.

**Allowlist parse failure → empty set (fail-closed):**
- Risk: Missing/`InvalidData` relations file admits nothing — safe but looks like a “broken product” outage.
- Files: `crates/kutha-runtime/src/allow.rs` (`load_allowed_names`)
- Current mitigation: Empty set rejects all named relations; tests in `crates/kutha-runtime/tests/ff6_allowlist.rs`.
- Recommendations: Surface load errors in higher-level tooling (tenant/CI observe) rather than silent empty set in operator UX.

**Harness subprocess / exception swallowing:**
- Risk: Broad `except Exception` in FSM/CLI paths can turn infra failures into LOW/observe noise.
- Files: `scripts/kutha_gov/fsm.py`, `scripts/kutha_gov/__main__.py`, `scripts/kutha_gov/observe.py`
- Current mitigation: HIGH severity still fails CI for dictionary checks; cargo observe has timeouts via `KUTHA_GOV_CARGO_TIMEOUT_SEC`.
- Recommendations: Keep unknown check kinds HIGH (`.kutha/META.md`); avoid widening bare `except` blocks.

**Secrets:**
- Risk: `.env` may hold local paths/budgets; must never be committed or quoted in docs.
- Files: `.env` (gitignored if present), `.env.example` (safe template only)
- Current mitigation: Example file documents keys without secrets; custom loader skips already-set vars.
- Recommendations: Never read `.env` contents into planning docs or commits.

## Performance Bottlenecks

**Linear fold scans for live views and supports:**
- Problem: `live_at`, `live_supports`, claim support counts, and CSR build walk all facts.
- Files: `crates/kutha-runtime/src/fold.rs`, `crates/kutha-runtime/src/csr.rs`, `crates/kutha-runtime/src/quantum.rs` (`derivation_eligible_at` also scans the log)
- Cause: In-memory `Vec<Fact>` with no secondary index; intentional droppable picture.
- Improvement path: Keep O(n) until a leased materializer/index; do not add HNSW or Rocks “for speed” under freeze. Measure with existing P0 timing print in `quantum.rs` tests before optimizing.

**Full rewrite persist path:**
- Problem: Every `persist` rewrites WAL + JSONL + terms + snapshot.
- Files: `crates/kutha-runtime/src/store.rs`, `crates/kutha-runtime/src/wal.rs`
- Cause: Spike durability model.
- Improvement path: Defer to M002 durability lease; until then avoid persist in hot loops.

**Leapfrog is intersection-only:**
- Problem: `leapfrog_intersect` on sorted rows — not variable-ordered full LFTJ / MATCH.
- Files: `crates/kutha-runtime/src/leapfrog.rs`
- Cause: P0 spike bound (AGENTS.md / literature note).
- Improvement path: Extend only under an explicit query/data lease; Cypher parser stays frozen.

## Fragile Areas

**Lifecycle non-collapse (process plane):**
- Files: `.kutha/STATE.md`, `.kutha/dictionaries/checks.yaml` (`lifecycles`, `freeze`, `lease-closed`), `.kutha/dictionaries/invariants.yaml`, `.kutha/dictionaries/bridges.yaml`, `docs/process/kutha-harness.md`
- Why fragile: Agents equate governor green, honeycomb Proposed, M011 delivery, and FF5 capability. STATE is a **lease**, not SoT; SoT for process time is gitignored `.kutha/events.jsonl`.
- Safe modification: Update `L_map` / `L_delivery` / `L_capability` rows together with ROADMAP/AGENTS only when the milestone lease changes; never “promote all” honeycomb; run `uv run kutha-gov precommit` / `ci`.
- Test coverage: Harness tests in `scripts/tests/test_kutha_gov.py` (freeze message, when/then lease-closed). Product capability remains `crates/kutha-runtime/tests/ff5_legal_pit.rs`.

**Freeze fence until M002:**
- Files: `.kutha/STATE.md` (Freeze section), `.kutha/dictionaries/checks.yaml` (`freeze`, honeycomb `freeze_as`), `.kutha/ROADMAP.md`, `AGENTS.md`
- Why fragile: Adding `rocksdb`, Cypher, HNSW, or ADR-050 six dictionaries trips HIGH freeze checks and collapses research honesty.
- Safe modification: Do not start M002 / legal pack / ADR-100 without Active Milestone + Active Slice lease text in STATE.
- Test coverage: Freeze needles + `freeze_as` ↔ STATE coupling in governor dictionaries.

**Semantic open / Define / snapshot offset coupling:**
- Files: `crates/kutha-runtime/src/store.rs`, `crates/kutha-runtime/src/quantum.rs` (`graph_len`, `from_snapshot`, `intern`), `crates/kutha-runtime/tests/m010_semantic_open.rs`
- Why fragile: Define ops are SoT for terms but stripped from graph stream / snapshot offset; persist synthesizes Define prefix. Wrong filter → intern meaning loss or offset skew.
- Safe modification: Follow M010 solutions under `.compound-engineering/artifacts/solutions/architecture-patterns/`; always run m010 + replay tests after store/quantum edits.
- Test coverage: Strong for open-without-snapshot paths; weak for adversarial dual-file divergence.

**Hardcoded `knows` → `knownBy` demo Behavior:**
- Files: `crates/kutha-runtime/src/quantum.rs` (`follow_ons`)
- Why fragile: Cascade physics is not dictionary-driven; legal relations do not get inverses; Behavior name `"inverse_knows"` is demo-specific.
- Safe modification: Do not generalize to “all inverses” without an ADR-012/Behavior lease; keep demo cascade isolated from FF5 statute fixtures.
- Test coverage: Cascade idle / budget tests in `quantum.rs`; FF5 uses `inForceAs`, not inverse cascade.

**Claim / lineage / derivation (M011):**
- Files: `crates/kutha-runtime/src/fold.rs`, `crates/kutha-runtime/src/quantum.rs` (`admit_claim`, `replay_check`, `derivation_eligible_at`), `crates/kutha-runtime/tests/m011_claim_supports.rs`
- Why fragile: Thin P→Q oracle is not full provenance polynomials; `caused_by` lineage ≠ support multiplicity semantics.
- Safe modification: Further M011 slices need explicit Active Slice in STATE (currently **None** after S03-done).
- Test coverage: Good for S01–S03 scenarios; no property tests for adversarial UUID / ordering storms.

**H2/H4 tenant mapping:**
- Files: `crates/kutha-runtime/src/tenant.rs`, `crates/kutha-runtime/tests/h2_harness_tenant.rs`, `crates/kutha-runtime/tests/h4_process_allows.rs`, `scripts/kutha_gov/tenant.py`
- Why fragile: Process JSONL → product Op is a dogfood bridge; unmapped relations must not append; same-second cuts use emitted `valid_from`.
- Safe modification: Keep harness kinds out of product ADRs; change mapping tables with both planes’ tests green.

## Scaling Limits

**In-memory EventLog + Fact vector:**
- Current capacity: Suitable for fixture-scale logs (statute / harness JSONL), not multi-million event corpora.
- Limit: RAM + O(n) live scans + full persist rewrite; no compaction of retracted facts.
- Scaling path: M002 Rocks + leased materializers (CSR already droppable); vacuum/compaction only under an explicit data-plane lease. Freeze forbids premature Rocks.

**Harness CI quantum budget:**
- Current capacity: `KUTHA_GOV_BUDGET` / FSM phases in `.kutha/dictionaries/fsm.yaml`; cargo timeout defaults ~180s.
- Limit: Full `kutha-gov ci` runs cargo + tenant + checks; grows with dictionary and workspace tests.
- Scaling path: Keep precommit as dictionary-only (`.pre-commit-config.yaml`); do not move product proofs solely into Python.

**Honeycomb / literature surface:**
- Current capacity: ADR-010–093 Proposed; 163 closed applicability cards.
- Limit: Treating honeycomb or cards as a delivery backlog creates sprawl without capability.
- Scaling path: One steel thread at a time (H4 overlay dogfood in; not M002). Literature bound closed — do not mint aggregator waves.

## Dependencies at Risk

**No RocksDB / graph-DB crates (intentional):**
- Risk: Temptation to vendor Rocks/Neo4j/Graphiti for “real storage.”
- Impact: Violates freeze and STRATEGY non-goals (external graph as SoT).
- Migration plan: Only under explicit M002 lease in `.kutha/STATE.md`; Ports own the contract.

**Minimal Rust deps today:**
- Risk: `sha2`, `uuid`, `serde`, `crc32fast` only at workspace level — good for spike purity; gaps appear when networking/ABAC arrives.
- Impact: Low for current stage.
- Migration plan: Add crates per leased slice; keep `publish = false`, version `0.0.0`.

**Python 3.13 + uv-only harness:**
- Risk: System `python3` misuse; Astral `ty` / Meta `pyrefly` version churn in `pyproject.toml` dev group.
- Impact: Harness CI fails on wrong interpreter.
- Migration plan: Always `uv run …`; pin via `.python-version` and `uv.lock`.

**Tooling index lag (CBM):**
- Risk: `check_index_coverage` reported `metadata_changed` for `quantum.rs`, `.kutha/STATE.md`, `.kutha/dictionaries/checks.yaml` (index generation 2026-09-16).
- Impact: Graph-first search may miss recent edits; false confidence in absence claims.
- Migration plan: Integrator-only `index_repository` when freshness is stale or user asks; do not treat graph coverage as governor green.

## Missing Critical Features

> These are **intentional gaps** under freeze / research stage — list so planners do not invent them as silent debt fixes.

**Durable Rocks-backed SoT:**
- Problem: File WAL cousin + in-memory runtime only (`crates/kutha-runtime/src/lib.rs`, `store.rs`).
- Blocks: Production durability, shared multi-process tenants.
- Lease: M002 in STATE — not started.

**Query language (Cypher/GPML) and HNSW/hybrid:**
- Problem: No parser; leapfrog intersect only; no vector index.
- Blocks: Profile B/C multi-hop product claims; hybrid GTM metrics in `STRATEGY.md`.
- Lease: Frozen explicitly in `.kutha/STATE.md`.

**ADR-050 six agent dictionaries / full agent runtime:**
- Problem: FF6 single relation allowlist stub only (`allow.rs`).
- Blocks: Dict-first agent governance product story.
- Lease: Frozen; do not grow FF6 into six kinds quietly.

**Legal / science vertical packs:**
- Problem: FF5 is a statute-shaped **fixture**, not a legal corpus pack; no science pack.
- Blocks: Wedge GTM “legal temporal agents” as shippable product.
- Lease: STATE — do not start a legal pack; M004 science is roadmap-future only.

**Enterprise security plane (ABAC, sandbox, CMK):**
- Problem: Sketched in STRATEGY buyer constraints; ADR-080/081 frozen.
- Blocks: Enterprise ship narrative.

**Full provenance / consensus query:**
- Problem: M011 thin P→Q only; Consensus Query 103+ frozen.
- Blocks: Rich audit polynomials and distributed query claims.

## Test Coverage Gaps

**Cascade abort caller contracts:**
- What's not tested: Production-facing handling that aborted quanta must not be treated as success (binaries/tenant ignore receipt flags).
- Files: `crates/kutha-runtime/src/bin/kutha-tenant.rs`, `crates/kutha-runtime/src/tenant.rs`, `crates/kutha-runtime/src/quantum.rs`
- Risk: Partial inverse edges persist unnoticed in dogfood paths.
- Priority: Medium (spike-acceptable; High before any external API).

**WAL vs JSONL divergence:**
- What's not tested: Conflicting content between `events.wal` and `events.jsonl`.
- Files: `crates/kutha-runtime/src/store.rs`, `crates/kutha-runtime/src/wal.rs`
- Risk: Silent wrong recovery.
- Priority: Medium.

**Multi-relation CSR correctness:**
- What's not tested: Mixed relations in one CSR lease.
- Files: `crates/kutha-runtime/src/csr.rs`, `crates/kutha-runtime/tests/ff5_legal_pit.rs` (homogeneous enough)
- Risk: False confidence in leapfrog on typed graphs.
- Priority: Low until typed query lease.

**No criterion/bench harness in workspace:**
- What's not tested: Continuous latency/regression vs Samyama-class targets in `STRATEGY.md`.
- Files: workspace `Cargo.toml` (no bench targets detected)
- Risk: Performance claims stay anecdotal (P0 print in tests only).
- Priority: Low until hot-path milestone.

**Property / fuzz tests absent:**
- What's not tested: Random op sequences for fold↔replay equivalence beyond hand-written cases.
- Files: `crates/kutha-runtime/src/fold.rs`, `crates/kutha-runtime/src/quantum.rs`
- Risk: Edge cases in Correct/Retract/Define ordering.
- Priority: Medium for next runtime lease after M011.

**Harness `pass` on bad timeout env:**
- What's not tested: Invalid `KUTHA_GOV_CARGO_TIMEOUT_SEC` falls through silently (`observe.py`).
- Files: `scripts/kutha_gov/observe.py`, `scripts/kutha_gov/tenant.py`
- Risk: Misconfigured CI uses defaults without signal.
- Priority: Low.

## Process / Plane Collapse Risks (cite freeze)

| Risk | Why it matters | Guard |
|------|----------------|-------|
| Treat honeycomb Proposed as backlog | Sprawl without capability | `honeycomb.yaml` + map checks; H4 “promote all” forbidden |
| Treat `kutha-gov ci` green as FF5/product ready | False ship signal | Orthogonal `L_capability=ff5-green` vs governor |
| Start Rocks/Cypher/HNSW/legal pack without lease | Freeze break | `.kutha/STATE.md` Freeze; check `freeze` |
| Collapse intern map (ADR-011) with agent dictionaries (ADR-050) | Wrong abstraction | AGENTS.md working convention #8 |
| Write CE durable artifacts under `/tmp` | Lost handoffs | `AGENTS.md` docs_root rules; use `.compound-engineering/artifacts/` |
| Further M011 work without Active Slice | Unscoped product edits | STATE: Active Slice **None**; S03 done |

## Intentional Non-Debt (do not “fix” without lease)

- In-memory P0 runtime and WAL cousin — spike, not unfinished Rocks (`lib.rs`, `store.rs`, `wal.rs`).
- FF6 single allowlist — not a broken ADR-050 (`allow.rs`).
- Demo `knows`/`knownBy` cascade — not a missing general Behavior engine (`quantum.rs`).
- Leapfrog intersect-only — not a missing Cypher parser (`leapfrog.rs`).
- All honeycomb ADR-010–093 **Proposed** — map honesty, not acceptance debt (`docs/ADR/`, `.kutha/dictionaries/honeycomb.yaml`).
- Version `0.0.0` / no GitHub Releases — policy, not release engineering failure (`Cargo.toml`, `pyproject.toml`).

---

*Concerns audit: 2026-09-29*
