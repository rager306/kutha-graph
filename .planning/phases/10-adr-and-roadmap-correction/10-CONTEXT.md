# Phase 10: ADR and roadmap correction - Context

**Gathered:** 2026-09-30
**Status:** Ready for planning

<domain>
## Phase Boundary

Trace the verified findings F1–F8 (see `.planning/REQUIREMENTS.md`) to ADR-000/001/002 and the honeycomb cells, give each cell a verdict, write dated **Proposed** amendments where a verdict is `amend`, and correct the **proposed** order of "Later milestones" in `.kutha/ROADMAP.md`. Nothing is leased, nothing is promoted to Accepted, no lock is rewritten.

**In scope:** ADR-01, ADR-02, ADR-03, ADR-04.

**Out of this phase:** any `crates/` edit; new governor kinds or checks (Phase 11); leasing M012a/M012/M002; ADR-050 six dictionaries as implementation; editing ADR-000 D1–D10 wording; new honeycomb cells (ADR-100+); Consensus/literature waves.

</domain>

<decisions>
## Implementation Decisions

### Review artifact (ADR-01)

- **D-R1:** One durable doc `docs/architecture/semantic-gap-review.md`, same register as `docs/architecture/semantic-contract-validation.md` ("Proposed review, not an ADR, not delivery authorization"). No lease values inside (Phase 9 D-A2 spirit; cite `.kutha/STATE.md` instead). Contents: (1) findings F1–F8 each with **evidence** (file + symbol, or the 2026-09-30 measurement) and a one-line "why it matters for the lock it touches"; (2) a **cell verdict table** — every affected cell with verdict `no change` | `amend` | `open question` and evidence; (3) lock assessment for D1–D10 (which hold, which get a question, none rewritten); (4) a candid note that Phase 8's D-F2 choice (justifications as a durable sidecar mirroring D-O1) is superseded in intent by F1 — the sidecar was a P0 shortcut, the target is log-native records; (5) measured baselines (2026-09-30, `/tmp` bench, N≈121k log events for ≈40k asserts): read scans linear (`claim_supported_at` 15→32→65 µs and `as_of` 28→68→144 µs per query for N×1/2/4), ingest ≈O(1)/emit, late `Retract` ≈134 µs, `persist` 19.5 s and `open` 8.2 s at that size, tampered snapshot accepted by `open` (live 40199 vs truth 40200) and caught only by `replay_check`, `Define` ids differ across persists. State plainly that persist/open were measured at one N only (no growth-shape claim) and that "no non-transitive/no action" claims come from source reading, not a failing test. — **Reversibility:** reversible

- **D-R2 (hypothesis mapping to verify against ADR text, not to copy):**
  - F1 (log-native records, no authoritative sidecars) → ADR-010 (D010-1, D010-4), ADR-014 (D014-1, Clarification 2026-09-13 outcomes), ADR-060 (verification obligations / fingerprint scope) — likely `amend` ADR-014 and ADR-060, `open question` ADR-010 (record kinds)
  - F2 (stable references, not fold seq) → ADR-011 (identity/support clarification), ADR-013 (Retract target), ADR-061 (references must survive forks) — likely `amend` ADR-011, `open question` ADR-013/061
  - F3 (proposition identity, generic conflict, idempotent ingest) → ADR-013 (supports, conflict view), ADR-011 — likely `amend` ADR-013
  - F4 (n-ary derivation, transitive eligibility, rule registry) → ADR-011 D011-3, ADR-060, ADR-050 — likely `open question` ADR-011/050, `amend` ADR-060 only if it states one-hop semantics
  - F5 (admission enforced, policy version in log, action entity) → ADR-050 (D050-2), ADR-051, ADR-081 (frozen) — likely `amend` ADR-050 (admission status as bi-temporal facts; policy version pinned), `open question` ADR-051/081 for the action entity
  - F6 (durability: verify-on-open, atomic append-only log, stable Define ids) → ADR-012, ADR-010 D010-4, ADR-060 D060-2 — likely `amend` ADR-012, `no change`/note ADR-060
  - F7 (declared time scale, TT↔wall mapping) → ADR-013 D013-1, ADR-000 D4 — `open question` only; D4 wording untouched
  - F8 (indexes, Materializer on the path, benchmarks, unexercised STCA axes, ADR-061 diff half) → ADR-040, ADR-041, ADR-061, ADR-002 — `open question` ADR-040/041/061; ADR-002 `no change` with an evidence note in the review doc
  The executor verifies each mapping against the current ADR text and changes a verdict when the text disagrees; every changed verdict is stated in the review doc. — **Reversibility:** reversible

### Amendments (ADR-02)

- **D-M1:** Follow the existing convention: a dated `### Clarification (2026-09-30; Proposed)` (or `### Open question (2026-09-30)`) subsection beside the existing `Clarification (2026-09-13)` sections, ≤ 15 lines each, citing `docs/architecture/semantic-gap-review.md`. Status line stays `Proposed`; the `## Date` line is not changed; no Decision (D0xx-n) text is rewritten or deleted — additions only. ADR-000 gets at most a pointer sentence under Open Research Questions if the review raises a lock question (D4 time scale); D1–D10 text is untouched. — **Reversibility:** reversible

- **D-M2:** Amendments describe the **target contract** in the ADR's own vocabulary (e.g. "authoritative records are log records; sidecars are leases", "references are stable ids, not fold sequence numbers", "admission status is a bi-temporal fact; the policy version is pinned in the log", "snapshot identity = (log offset, fingerprint), verified on open"). They must not name crates, functions, or milestone ids as commitments; sequencing lives only in `.kutha/ROADMAP.md` (non-authoritative). — **Reversibility:** reversible

### Proposed roadmap order (ADR-03)

- **D-O1:** Edit only "Later milestones" (and the H-rung text it needs) in `.kutha/ROADMAP.md`. New proposed order (still non-authoritative; ids only for the nearest steps, inflation guard preserved): (1) **M011 tail** stays as the done record; (2) **M012a** — single-log SoT and stable references (outcomes/justifications/resume as log records; `EventId`/proposition ids; idempotent ingest; `open`→verify; atomic persist; stable `Define` ids; declared time scale; fold-internal indexes); (3) **M012** — dictionaries as facts, naming the subset it thaws: versioned relation-allowlist entries, rule registry (rule_version = definition hash), admission status meta-facts, policy version pinned in the log; (4) **M002** — reframed: **log durability protocol first** (segmented append-only log, hash chain, atomic manifest, lease identity verified on open; replay parity across stores), Rocks only behind it for indexes; (5) benchmark baseline moved up (right after M012a indexes); (6) thin real-text legal golden fixture (test-only, under a lease) before Cypher; (7) **M003** Cypher skin; then the unchanged tail (science fixture, pack lifecycle after two verticals, HNSW fence, security, packaging). No new milestone becomes Active. The `trajectory` check's inflation guard (`\bM\d{3}\b` max 12) must still pass — `M012a` should be written so the regex count stays within the limit. — **Reversibility:** reversible

- **D-O2:** `docs/process/kutha-harness.md` and `STRATEGY.md` are checked for any sentence that contradicts the new order (e.g. "M002 = Rocks adapter"); fix only contradictions, no rewrites. — **Reversibility:** reversible

### Map consistency (ADR-04)

- **D-C1:** `docs/ADR/README.md` gains a one-line pointer to the review doc under a suitable section (not a new index row for an ADR). `honeycomb.yaml`: rows change only if an amendment changes a stage or edge (expected: none; if `evidence`/`depends_on` gains an item, keep vocab closed). `map` stays `Proposed` everywhere; `uv run kutha-gov map` and `precommit --check honeycomb-ledger` stay green. — **Reversibility:** reversible

### Governor and process inheritance

- **D-G1:** Each wave runs `uv run kutha-gov ci` (0 HIGH) + `explain trajectory`; SUMMARY carries an ≤8-line Trajectory excerpt. `docs-coupling` requires a `CHANGELOG.md` diff with `docs/architecture/**`, `.kutha/**`, `docs/process/**` changes — add a **Process pointer** entry (narrative is Phase 11 SEM-08) as in Phase 9. — **Reversibility:** reversible
- **D-G2:** Freeze, Proposed honeycomb, three lifecycles, and Active Slice None are not touched. `.kutha/STATE.md` is not edited in this phase. — **Reversibility:** reversible

### Claude's Discretion

Review-doc structure beyond D-R1, amendment wording, and exact ROADMAP phrasing — as long as verdicts are evidence-backed, additions-only in ADRs, Status Proposed, and freeze holds. Docs-only phase: the evidence in D-R1 is already gathered (CBM `get_file_outline`/`trace_path`/`search_code` plus source reads and a bench, 2026-09-30); executors verify symbols by reading source and must not claim graph verification unless they run CBM tools themselves.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md` (Phase 10), `.planning/REQUIREMENTS.md` (ADR-01…04, findings table F1–F8)
- `docs/ADR/ADR-000…002`, `docs/ADR/ADR-010…014`, `ADR-040/041`, `ADR-050/051`, `ADR-060/061`, `ADR-081`, `docs/ADR/README.md`
- `docs/architecture/semantic-contract-validation.md` (register and format model), `docs/architecture/stca-guide.md`
- `.kutha/ROADMAP.md`, `.kutha/dictionaries/honeycomb.yaml`, `.kutha/dictionaries/checks.yaml` (`trajectory` inflation guard, `honeycomb-ledger`, `docs-coupling`)
- `docs/process/kutha-harness.md`, `STRATEGY.md`
- Source evidence (read-only): `crates/kutha-common/src/event.rs`, `crates/kutha-runtime/src/{fold,quantum,store,wal,allow,tenant,materializer,csr}.rs`, `crates/kutha-runtime/tests/m011_*.rs`
- `.planning/milestones/v0.02-phases/08-end-to-end-candidate-fixture/08-CONTEXT.md` (D-F2 sidecar decision to be acknowledged)

</canonical_refs>
