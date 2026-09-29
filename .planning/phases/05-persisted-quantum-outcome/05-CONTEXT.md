# Phase 5: Persisted quantum outcome - Context

**Gathered:** 2026-09-29
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver M011 **S05** (leased Active Slice): **persist quantum outcome records** so budgets **0 / 1 / 2** are distinguishable as zero / partial / full progress (OUT-01), and a crash after a committed event prefix leaves **no terminal-success** on reopen — resume is an **explicit** record, never inferred (OUT-02). Named cargo tests + governor registration (GATE-01).

**In scope:** OUT-01, OUT-02, GATE-01 (pattern from Phase 4). ADR-014 clarification on partial progress and recoverable outcomes.

**Out of this phase:** S06 typed CSR, S07 provenance-only checks, S08 end-to-end fixture, full cryptographic receipt layout (PQ/TEE), ADR-050 meta_prompt_version binding, ADR-062 exact-once remote effects, Rocks/Cypher/HNSW, legal pack. Do not treat honeycomb as Accepted.

</domain>

<decisions>
## Implementation Decisions

### Persisted outcome form (Claude decided)

- **D-O1:** Persist quantum outcomes as an **authoritative durable file** beside the log (working name `quantum_outcomes.jsonl` under the store directory), written by `store::persist` and always loaded by `store::open`. This file is **not** a droppable lease (unlike `snapshot.json` / CSR). Discarding snapshot must not erase outcome history. Do **not** put outcomes only in `QuantumOutcome` RAM or only in snapshot. Do **not** invent a product graph fact for receipts. A new fold-no-op `Op` is allowed only if research proves the sidecar cannot survive the same open-without-snapshot path as Define — default is **sidecar**. — **Reversibility:** costly — once directories on disk carry the file, open/persist contracts depend on it

### Progress encoding for budgets 0/1/2 (Claude decided)

- **D-O2:** Persist a disposition that maps to OUT-01 as:
  - **Zero:** budget abort with **no** events committed in that quantum (`events_in_quantum == 0`, e.g. `max_cascade = 0`)
  - **Partial:** budget abort with a **non-empty** committed prefix (`aborted_on_budget && events_in_quantum > 0`, e.g. `max_cascade = 1` on inverse-`knows` storm)
  - **Full:** quantum reached idle without budget abort (`!aborted_on_budget`)
  Call `Ok(QuantumOutcome)` remains **not** completion proof (ADR-014). Planner may name the enum (`Zero`/`Partial`/`Full` or `BudgetStopped`/`Completed`) as long as the three fixtures are distinguishable after persist→open. — **Reversibility:** reversible — encoding only

### Crash after prefix and resume (Claude decided)

- **D-O3:** Crash-after-prefix fixture (OUT-02): after a quantum commits a prefix to the log and persist writes events **without** a terminal **Full** (or Completed) outcome for that quantum, `open` must report **incomplete/unknown** for that quantum — **never** inferred success. Resume requires an **explicit** continuation/resume outcome record (new row in the outcomes file, or a named API that appends one) that cites the original quantum identity (receipt digest and/or first event id). `open` must **not** auto-resume. — **Reversibility:** reversible — test and API surface

### Fixture width (Claude decided)

- **D-O4:** Ship **two named oracles** only: (1) budgets 0/1/2 → Zero/Partial/Full after persist→open; (2) crash-after-prefix → no terminal success + explicit resume record. Do **not** implement full ADR-062 remote exactly-once, duplicate-delivery across forks, or cryptographic authenticator fields in this phase. Thin anti-duplicate for the same quantum id on a second resume attempt may be a single fail-closed assert if cheap; otherwise defer. — **Reversibility:** reversible — test scope

### Governor and process inheritance (Claude decided)

- **D-O5:** Inherit Phase 2 **D-G1…D-G3** and **D-10 / D-11 / D-15**, and Phase 4 wave-close pattern: every execute wave runs `uv run kutha-gov ci` (HIGH stops) + `explain trajectory` + D-10 Trajectory in SUMMARY; cargo on crate-touching waves. — **Reversibility:** reversible

- **D-O6:** Named cargo tests registered in governor (FSM observe + check/bridge needles), GATE-01, `ci` HIGH 0. Active Slice remains **S05** for the phase (GATE-02). Freeze / Proposed honeycomb (GATE-03). — **Reversibility:** reversible

- **D-O7:** Do not change in-memory `emit` cascade semantics except as needed to **record** outcomes for persist; whole-version Correct and CorrectInterval stay untouched. — **Reversibility:** reversible

### Claude's Discretion

User said «реши эти вопросы разумно и при необходимости доисследуй» for gray areas 1–4 → D-O1…D-O4 (plus D-O5…D-O7). Planner/researcher may refine file schema fields (quantum id, committed event ids, disposition, resume_of) and exact budget fixture ops; must not thaw freeze, expand into S06–S08, or treat call `Ok` as completion.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### GSD overlay
- `.planning/ROADMAP.md` — Phase 5 goal, success criteria (S05 leased)
- `.planning/REQUIREMENTS.md` — OUT-01, OUT-02, GATE-01…03
- `.planning/PROJECT.md` — v0.02 Semantic core close
- `.planning/STATE.md` — GSD position
- `.kutha/STATE.md` — Active Slice **S05**; `L_delivery=M011-S04-done`; freeze
- `.kutha/ROADMAP.md` — M011 S05 “After this” line
- `.planning/phases/04-partial-correction-with-residual-intervals/04-CONTEXT.md` — D-C5…C6 governor pattern
- `.planning/milestones/v0.01-phases/02-honest-harness-and-freeze/02-CONTEXT.md` — D-G1…G3, D-10/11/15

### Product contract
- `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md` — clarification: partial progress; persist does not yet write receipt; Ok ≠ completion
- `docs/ADR/ADR-010-event-log-runtime-quantum.md` — quantum = emit→idle; outcomes must survive dropping leases
- `docs/architecture/semantic-contract-validation.md` — probe: budgets 0/1/2; crash after committed prefix
- `docs/ADR/ADR-000-foundation-locks.md` — D1–D10

### Implementation anchors
- `crates/kutha-runtime/src/quantum.rs` — `QuantumOutcome`, `emit`, `max_cascade`, `budget_aborts_storm`
- `crates/kutha-runtime/src/receipt.rs` — `QuantumReceipt { event_ids, digest, aborted_on_budget }`
- `crates/kutha-runtime/src/store.rs` — `persist` / `open` (events, WAL, terms, snapshot; no outcomes today)
- `AGENTS.md` — planes, lease, freeze

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `QuantumReceipt::from_events` — digest over event digests + aborted flag
- `Runtime::new(max_cascade)` / env `KUTHA_MAX_CASCADE` — budget knob for 0/1/2 fixtures
- `budget_aborts_storm` — already shows partial (`max_cascade=1`, aborted, 1 event)
- `store::persist` / `open` — directory contract to extend
- M011 / Phase 4 GATE-01 YAML pattern (fsm required + file_contains + bridge)

### Established Patterns
- Event log = SoT for graph facts; snapshot/CSR = droppable leases
- `Ok(QuantumOutcome)` with `aborted_on_budget` already separates call success from idle completion in RAM
- Fail-closed: missing authoritative recovery inputs fail explicitly (M010 open path)

### Integration Points
- Extend `store::persist` to write outcomes; `store::open` to load them
- Optional Runtime buffer of last/pending outcomes between emit and persist
- New tests under `crates/kutha-runtime/tests/` (e.g. `m011_quantum_outcome.rs`)
- Governor dictionary rows for named test fns

### Research notes (this discuss)
- ADR-014 lines 52–58 name the gap: persist does not write the receipt; budget 0/1/2 and crash-boundary fixtures required before stronger claims
- ADR-010: quantum outcomes must survive dropping leases → outcomes must not live only in snapshot
- Inverse-`knows` cascade is the cheapest budget storm fixture (2 events when unlimited; 1 when budget 1)

</code_context>

<specifics>
## Specific Ideas

- Budget ladder: `Runtime::new(0)`, `Runtime::new(1)`, `Runtime::default()` / `new(2+)` on the same Assert→inverse knows pattern as `budget_aborts_storm` / `cascade_idles_without_inverse_loop`
- Crash fixture: persist events (and maybe outcomes omitted for the open quantum) then open and assert no terminal Full; then write explicit resume record and assert it is visible

</specifics>

<deferred>
## Deferred Ideas

- Full constant-size cryptographic receipt / Merkle authenticator (ADR-014 open questions)
- ADR-050 meta_prompt_version on receipt tuples
- ADR-062 remote exactly-once / cross-branch duplicate delivery
- Typed CSR (S06), provenance-only check (S07), end-to-end fixture (S08)
- Rocks / Cypher / HNSW / legal pack

</deferred>

---

*Phase: 5-Persisted quantum outcome*
*Context gathered: 2026-09-29*
