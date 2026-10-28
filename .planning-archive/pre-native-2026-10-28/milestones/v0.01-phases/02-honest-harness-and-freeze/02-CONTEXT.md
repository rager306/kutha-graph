# Phase 2: Honest harness and freeze - Context

**Gathered:** 2026-09-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Prove process CI tells the truth about trajectory; keep product and harness on separate planes; keep frozen surfaces unstarted; keep honeycomb as a map. Phase 2 is **verification-first on the harness plane**, with a **Governor CLI control loop on every GSD execute cycle**. No freeze thaw, no M002, no legal pack, no Active Slice product work, no honeycomb promotion.

Carries forward Phase 1: product fitness remains `cargo test --workspace --offline`. Phase 2 **adds** `uv run kutha-gov ci` as the trajectory honesty gate (Phase 1 D-03 deferred this on purpose).

Requirements in scope: GOV-01, GOV-02, PLANE-01, PLANE-02, PLANE-03, FREEZE-01, MAP-01. GOV-03 stays Phase 3.

</domain>

<decisions>
## Implementation Decisions

### Governor cycle (user-locked 2026-09-29)

- **D-G1:** After every execute-wave and before phase-verify, run `uv run kutha-gov ci` as a required automated gate. Each wave-closing SUMMARY (and phase closeout VERIFICATION) includes a brief trajectory reading from `uv run kutha-gov explain trajectory` (see **D-10**). — **Reversibility:** reversible — process gate only
- **D-G2:** Any **HIGH** from `kutha-gov ci` **stops the wave** and blocks phase completion until cleared or explicitly leased outside this CONTEXT. **WARN** findings are recorded — never silenced or “green with caveat” (severity at closeout: **D-11**). — **Reversibility:** reversible — policy only
- **D-G3:** In-scope improvements when trajectory/semantics drift: harness YAML (checks, bridges, invariants, fsm, honeycomb index), `docs/process/*`, clarity edits to ADR wording / `.planning/ROADMAP.md` / GSD STATE captions. **Out of scope:** new product milestone, assumed M002, freeze thaw, legal pack, collapsing L_map / L_delivery / L_capability, treating governor green as ADR Accepted. — **Reversibility:** reversible — scope fence

### Trajectory SUMMARY template (Claude decided)

- **D-10:** Wave SUMMARY § Trajectory (English) must contain: (1) commands run — at least `uv run kutha-gov ci` and `uv run kutha-gov explain trajectory`; (2) `ci` outcome — exit code + HIGH count (and WARN count if non-zero); (3) **≤8 lines** excerpt from `explain trajectory` (or a one-paragraph paraphrase of the same facts); (4) one explicit sentence: green governor ≠ ADR Accepted ≠ L_capability. Do not paste full FSM dumps or entire events.jsonl. — **Reversibility:** reversible

### WARN at phase-verify (Claude decided)

- **D-11:** Phase-verify / phase complete may proceed when `ci` is **HIGH-free** even if WARNs remain, **iff** every WARN id is listed in `02-VERIFICATION.md` (or wave SUMMARY ledger) with check id + one-line note. Unlogged WARN = treat as process failure (fix ledger or escalate). HIGH always blocks. — **Reversibility:** reversible

### PLANE evidence (Claude decided)

- **D-12:** PLANE-01…03 are proven by **path/schema/source probes**, not new product features: (PLANE-01) product under `crates/kutha-*`, harness under `scripts/kutha_gov` + `.kutha/`, no Python inside `kutha-runtime`, no repo-root `ports/` / `adapters/` / `domain/`; (PLANE-02) product write surface remains typed `Op` in crates — no STCA-guide §5 JSON merge-patch / `object.created` as product write API; (PLANE-03) product `kutha-relations/v1` and harness `kutha-harness-relations/v1` remain distinct files/schemas. Evidence lives in VERIFICATION table; D-G3 clarity edits allowed if probes fail for doc drift. — **Reversibility:** reversible

### FREEZE evidence (Claude decided)

- **D-13:** FREEZE-01 proven by **absence + lease cite**: `.kutha/STATE.md` still documents freeze / Active Slice None / no M002 lease; workspace manifests do not add RocksDB/Cypher/HNSW (or equivalent) product deps; no legal-pack / ADR-050 six-dict delivery under crates. Do not “fix” freeze by implementing frozen surfaces. — **Reversibility:** reversible

### MAP / honeycomb (Claude decided)

- **D-14:** MAP-01 proven by: honeycomb ADRs 010–093 remain Proposed (not Accepted as shipped product); `.planning/ROADMAP.md` does not schedule one GSD phase per honeycomb cell; `uv run kutha-gov map` (or dictionary index read) still presents L_map as compact map. Clarity edits to captions OK under D-G3. — **Reversibility:** reversible

### Cargo beside ci (Claude decided)

- **D-15:** Do **not** require full `cargo test --workspace --offline` after every Phase 2 wave. Run it **once in the tracer / first wave** and again **before phase-verify** as product regression smoke (Phase 1 hard gate still valid). Intermediate waves rely on `kutha-gov ci` (which already observes cargo per fsm). If a wave modifies `crates/`, that wave must re-run cargo before closing. — **Reversibility:** reversible

### Claude's Discretion

User said “реши эти вопросы разумно” for gray areas 1–6 above → D-10…D-15. Planner may tighten probe command strings; must not weaken HIGH-stop or D-G3 fence.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 2 goal and success criteria
- `.planning/REQUIREMENTS.md` — GOV-01, GOV-02, PLANE-01…03, FREEZE-01, MAP-01
- `.planning/PROJECT.md` — core value + Phase 2 Governor cycle section
- `.planning/STATE.md` — GSD position (cite harness lease)
- `.kutha/STATE.md` — lifecycles, freeze, Active Slice None
- `.planning/phases/01-legal-pit-fitness/01-CONTEXT.md` — D-03 deferred `ci` to Phase 2
- `.planning/phases/01-legal-pit-fitness/01-VERIFICATION.md` — evidence-map pattern to extend

### Governor / harness
- `docs/process/kutha-harness.md`
- `docs/process/governor-intake.md`
- `.kutha/dictionaries/checks.yaml`, `invariants.yaml`, `bridges.yaml`, `fsm.yaml`, `honeycomb.yaml`
- `scripts/kutha_gov/` — CLI (`ci`, `explain`, `map`, `fold`)

### Architecture fences
- `docs/ADR/ADR-000-foundation-locks.md` — D1–D10
- `docs/ADR/ADR-002-stca-paradigm.md` — STCA
- `docs/ADR/ADR-010-event-log-runtime-quantum.md` — typed Op
- `.planning/INGEST-CONFLICTS.md` — typed Op vs stca-guide §5
- `AGENTS.md` — two planes; green ≠ Accepted

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Phase 1 VERIFICATION twelve-row map pattern → Phase 2 tables for GOV/PLANE/FREEZE/MAP probes
- `uv run kutha-gov ci` FSM already runs observe (cargo) + dictionary checks
- `.kutha/STATE.md` already names L_map / L_delivery / L_capability separately (GOV-02)

### Established Patterns
- New check = YAML row; bridges cite product without copying FSMs
- GSD docs English; chat Russian

### Integration Points
- Execute waves: close with `ci` + D-10 SUMMARY block; HIGH → stop (D-G2)
- Phase-verify: `ci` HIGH-free + WARN ledger (D-11) + cargo smoke (D-15) + probe tables (D-12…D-14)

</code_context>

<specifics>
## Specific Ideas

- Wave gate: `uv run kutha-gov ci`
- Trajectory: `uv run kutha-gov explain trajectory` (≤8 lines in SUMMARY)
- Map smoke: `uv run kutha-gov map`
- Product smoke: `cargo test --workspace --offline` (wave 1 + pre-verify)
- GOV-02 needle: three lifecycle keys still present and distinct in `.kutha/STATE.md`

</specifics>

<deferred>
## Deferred Ideas

- GOV-03 / Active Slice enforcement as delivery gate — Phase 3
- Making WARN fail-closed like HIGH — rejected for Phase 2 (D-11)
- Requiring cargo on every intermediate wave — rejected (D-15)
- M002 / Rocks / Cypher / HNSW / legal pack — freeze

</deferred>

---

*Phase: 2-Honest harness and freeze*
*Context gathered: 2026-09-29*
*User locked: D-G1, D-G2, D-G3*
*Claude decided (user: resolve reasonably): D-10…D-15*
