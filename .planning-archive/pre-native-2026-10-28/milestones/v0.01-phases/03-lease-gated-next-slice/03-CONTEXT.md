# Phase 3: Lease-gated next slice - Context

**Gathered:** 2026-09-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Prove that **while Active Slice is None**, no new product-slice implementation starts; that a legal pack is not substituted for a lease; and that the next product milestone is only whatever `.kutha/STATE.md` will name — not assumed M002 and not “implement honeycomb.”

**Today’s lease (cite only):** Active Milestone M011; Active Slice **None**; `L_delivery=M011-S03-done`; freeze until explicit M002. Phase 3 under this lease is **verification-first / negative proof** — not delivery of a new crate slice.

When STATE later names an Active Slice, that work is **out of this phase’s default plans** (see **D-L4**): stop and replan or insert a leased phase — do not silently expand Phase 3 into product delivery mid-flight.

Requirements in scope: GOV-03, NEXT-01, NEXT-02.

</domain>

<decisions>
## Implementation Decisions

### Phase posture while Active Slice is None (Claude decided)

- **D-L1:** Complete Phase 3 as **verification-only** under Active Slice None: prove GOV-03 / NEXT-01 / NEXT-02 with evidence probes + governor cycle; **do not** leave the phase open waiting for a future lease. A future named slice is a **new** discuss/plan (or inserted phase), not an infinite hold on Phase 3. — **Reversibility:** reversible

### What counts as a GOV-03 violation (Claude decided)

- **D-L2:** “New product-crate slice work” means **new leased product capability** in `crates/kutha-*` beyond the already-landed M011 S03 surface (new slice modules/APIs/features named as Active Slice delivery). **Not** a violation: GSD/docs, harness YAML under D-G3, FIT/GOV regression evidence, clarifying comments, or re-running existing tests. Evidence: `.kutha/STATE.md` still shows `Active Slice: None` (or equivalent bold cite); VERIFICATION records no new slice-delivery commits/paths for a named Active Slice; no legal-pack corpus start. — **Reversibility:** reversible

### Governor cycle inheritance (Claude decided)

- **D-L3:** Inherit Phase 2 **D-G1…D-G3** and companions **D-10 / D-11 / D-15** for every Phase 3 execute wave (ci + explain trajectory in SUMMARY; HIGH stops; WARN ledgered; cargo at tracer + pre-verify unless `crates/` touched). Do not weaken to precommit-only. — **Reversibility:** reversible

### Mid-phase lease appearance (Claude decided)

- **D-L4:** If during execute `.kutha/STATE.md` Active Slice changes from None to a named value: **hard stop** the current verification plan; record in STATE/VERIFICATION; **do not** implement that slice under Phase 3 plans. Human chooses `/gsd-discuss-phase` / plan for the leased slice or an inserted phase. — **Reversibility:** reversible — process gate

### NEXT-02 while M011 still open (Claude decided)

- **D-L5:** While M011 is not closed in `.kutha/STATE.md`, NEXT-02 is proven **negatively**: captions/ROADMAP/STATE do not treat M002 or “implement honeycomb” as the implied next product milestone; no M002/Rocks/Cypher/HNSW scaffolding started; prose cites “next milestone = whatever STATE names after M011 close.” Do not require M011 to be closed to finish Phase 3. — **Reversibility:** reversible

### Legal pack / honeycomb substitute fence (Claude decided)

- **D-L6:** NEXT-01 / legal-pack fence uses **path + lease cite** (same spirit as Phase 2 FREEZE/MAP): Active Slice None; no legal corpus/pack delivery tree started as substitute; honeycomb ADRs remain Proposed/map; ROADMAP still not phase-per-cell. Optional D-G3 clarity edits only. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно» for gray areas 1–6 → D-L1…D-L6. Planner may tighten probe command strings; must not start a leased slice, thaw freeze, or assume M002 under this CONTEXT.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 3 goal and success criteria
- `.planning/REQUIREMENTS.md` — GOV-03, NEXT-01, NEXT-02
- `.planning/PROJECT.md` — core value; Phase 2 governor cycle
- `.planning/STATE.md` — GSD position
- `.kutha/STATE.md` — Active Slice None; M011; freeze; lifecycles
- `.planning/phases/02-honest-harness-and-freeze/02-CONTEXT.md` — D-G1…G3, D-10/11/15
- `.planning/phases/01-legal-pit-fitness/01-CONTEXT.md` — verification-first pattern

### Harness / fences
- `docs/process/kutha-harness.md`
- `AGENTS.md` — Active Slice lease; freeze; two planes
- `.kutha/dictionaries/honeycomb.yaml` — L_map index

### Architecture
- `docs/ADR/ADR-000-foundation-locks.md`
- `.planning/INGEST-CONFLICTS.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- Phase 1–2 VERIFICATION probe tables + SUMMARY § Trajectory
- Phase 2 `kutha-gov ci` / `explain trajectory` / cargo smoke pattern
- `.kutha/STATE.md` already names Active Slice None and freeze

### Established Patterns
- Negative proof phases close when evidence is green — they do not wait for delivery events
- Do not overwrite harness STATE from GSD plans

### Integration Points
- Wave close: D-G1 ci + D-10 SUMMARY
- Phase-verify: HIGH-free + GOV-03/NEXT probe tables + D-L4 lease-change check
- REQUIREMENTS batch GOV-03 / NEXT-01 / NEXT-02 only after VERIFICATION passed

</code_context>

<specifics>
## Specific Ideas

- Lease needle: `Active Slice` / `**Active Slice:**` None in `.kutha/STATE.md`
- Wave gate: `uv run kutha-gov ci` + `explain trajectory`
- Product smoke: `cargo test --workspace --offline` at tracer + pre-verify (D-15)
- Anti-assumption: no new Rocks/Cypher/HNSW deps; ROADMAP Phase 3 text remains lease-gated

</specifics>

<deferred>
## Deferred Ideas

- Implementing a named Active Slice — new phase after STATE lease
- Closing M011 in harness STATE — harness process, not GSD Phase 3 delivery
- Assumed M002 start — freeze until explicit lease
- Legal corpus / ADR-090 pack — out of scope

</deferred>

---

*Phase: 3-Lease-gated next slice*
*Context gathered: 2026-09-29*
*Claude decided (user: resolve reasonably): D-L1…D-L6; inherits D-G1…G3*
