# Synthesized constraints (SPEC)

## STCA paradigm (manifest-spec restatement)
- source: docs/architecture/stca-guide.md
- type: protocol
- content: Spatiotemporal Compositional Architecture unites (1) Space — vertical slices and Ports; (2) Time — append-only event log → deterministic fold; (3) Composition — Cui generalized items and max-convolution. Kutha formula in this document: STCA core + pluggable hot materializations (CSR/HNSW/…) + dict-first agents + native bi-temporal semantics (Graphiti as reference, without Graphiti runtime). Status: working manifest-spec. Normative support named: ADR-002; vision ADR-001; foundation D1–D10 ADR-000; detail honeycomb ADR-010+. Classification confidence: medium. Classification locked: false.

## STCA Algorithm 1 — deterministic fold
- source: docs/architecture/stca-guide.md
- type: protocol
- content: Input G0 = empty, Lt = [e1…et]; output Gt. G ← G0; for each ek in Lt apply object.created / relation.created / object.patched (JSON merge-patch) and related event types; return G. Graph Gt = foldl(apply, G0, Lt). State is not rewritten in place (not State-on-Write). This operator vocabulary competes with ADR-010 D010-3 typed assert/retract/correct; ADR wins (see INGEST-CONFLICTS.md INFO).

## STCA Algorithm 2 — Relation Behaviors
- source: docs/architecture/stca-guide.md
- type: protocol
- content: Input event e, graph G, registry of edge behaviors. Output list of generated events. For each behavior with matching trigger and edge type: if the event is incident to the edge endpoints, execute(rel, e, G). Behaviors and Relation Behaviors propose Patches; the runtime validates and writes to the log.

## STCA Algorithm 3 — Cui max-convolution
- source: docs/architecture/stca-guide.md
- type: protocol
- content: Input budget V, plugins M1..Mn with h_i(v). Output vector H where H[V] is the optimal composition. H_next[W] = max_{0≤v≤W}{ h_i(v) + H[W-v] }. In Kutha: cascade budgets, agent limits, competing materializer plugins — without a hard workflow engine.

## STCA Cargo workspace target shape
- source: docs/architecture/stca-guide.md
- type: schema
- content: Target workspace tree named in this SPEC: app-shell/ (sole composition root / only binary), common/ (Event, Patch, UuidV7, …), modules/<domain>-module/ with domain/ (no sqlx/ORM), internal/ (pub(crate)), adapters/ (outgoing ports), lib.rs (public Port traits). For Kutha-engine the same document names common + runtime crate + materializer packs + dictionary/agent packs; hot path without mandatory network adapters.

## STCA isolation laws (AI-compiler gates)
- source: docs/architecture/stca-guide.md
- type: protocol
- content: (1) Domain Ignorance — domain/ without infrastructure. (2) Private-by-Default — only Ports leave the module. (3) Single Composition Root — only app-shell knows concrete adapters.

## STCA verification regimes
- source: docs/architecture/stca-guide.md
- type: nfr
- content: Strict Replay — replay historical log; external I/O via content-addressed cache SHA256(prompt…); divergence → ReplayDivergenceError. Fork-and-Diff — fork_at(event_id) → change behavior → stabilize → structural graph diff. Regimes Gated Loop (optional) — Diagnose → Route on Action Seam → Static / Sandbox / In-Sample / Held-Out gates; held-out failure → discard. Identifiers: UUID v7. Section 5 names a typed ActiveRuntime skeleton (Event, Patch, MaterializedGraph::apply_event, BehaviorFn, RelationBehaviorFn, emit-cycle to idle) as pedagogical; full listing deferred to Studio artifact / ADR-000 prototype appendix.

## STCA agent change protocol
- source: docs/architecture/stca-guide.md
- type: protocol
- content: (1) Change only at seams: Behavior / Relation Behavior / Port. (2) Rigid event payload schema in common/. (3) Domain — pure unit tests without network. (4) Adapters — replay fixtures with CA-cache.

## Semantic contract — three consumers of the same history
- source: docs/architecture/semantic-contract-validation.md
- type: protocol
- content: Proposed validation design, not implemented capability or delivery authorization. Knowledge base must preserve claim/version identity, source supports, VT × TT, admission policy; must not infer that an observed statement is automatically an admitted fact. Agent trace must preserve runs/attempts, observed inputs, typed dependencies, actions and outcomes; must not infer that log order or a read edge proves causality, entailment, or authority. Agent context must preserve evidence references, cut, policy, dependency revisions, completeness; must not infer that a summary or cached answer is fresher or more authoritative than its evidence. Claim support, computational dependency, and permission remain distinct. A dependency DAG is sufficient as an initial representation for acyclic computations; recursive rule provenance needs its own bounded representation. Do not require a full provenance-polynomial engine just to retain explicit references. Normative homes remain the linked honeycomb ADRs.

## Semantic contract — ownership and current limits
- source: docs/architecture/semantic-contract-validation.md
- type: nfr
- content: ADR-011/012 — recover term meanings from retained authoritative history without leases; store.rs + live Runtime::intern append Op::Define; open recovers without snapshot.json (M010 S01–S03); bootstrap knows/knownBy stay silent until persist synthesizes them. ADR-011/013 — independent supports survive withdrawal of another; disagreement is not implicit supersession; fold.rs claim_id + m011_claim_supports.rs. ADR-011/060 — thin P→Q: derivation_eligible_at; named test derived_q_loses_eligibility_when_last_premise_support_withdrawn (not full provenance polynomials). ADR-013/070 — explicit TT/VT and whole-version versus interval correction; current Correct replaces a whole version; FF5 compares two VT cuts. ADR-010/014 — saturation, budget stop, and unknown recovery are distinguishable; a budget-stopped emit can retain events; its receipt is not persisted by the file store. ADR-040/060 — exact lease identity and separate state/provenance/execution checks; CSR is untyped neighbor-set adjacency; replay checks fold fingerprints and Behavior caused_by (M011 S02); execution replay still absent. ADR-051/052/080 — admission, current authorization, and source-revision eligibility are Proposed, not implemented ABAC or agent memory. ADR-061/062 — prefix fork is not merge; local replay is not remote exactly-once execution. ADR-030 — separable scalar allocation only where its assumptions hold. Nothing here starts M002, a legal/science pack, a query parser, or new dictionaries.

## Semantic contract — candidate acceptance fixture (oracles)
- source: docs/architecture/semantic-contract-validation.md
- type: nfr
- content: Synthetic rule and two evidence sources, not a legal corpus. At TT t1, admitted source versions a and b independently support proposition P over VT [2010,infinity). Pinned deterministic rule r derives Q from P; no other rule or source supports Q in this fixture. At t2, source a withdraws support for P over [2015,2020) and proposes replacement P' that does not entail P; b unchanged. At t3, b withdraws support for P on that interval. Conflict variant: P' = not-P. Expected observations (proposed oracles, not satisfied merely because FF5 or governor checks pass): (1) At (t1,2017) retain two supports; at (t2,2017) b still supports P and Q; correcting a does not erase b; if a supports the opposite, preserve both sides and report conflict to admission policy. (2) At (t3,2017) last positive support gone: Q loses eligibility through this derivation; withdrawal alone does not prove the opposite; earlier TT cuts unchanged. (3) Under an explicit interval-patch contract, VT 2012 and 2021 retain residual versions of a; current whole-version Correct API must not be used as if it supplied those residuals automatically. (4) Summary/action justification records exact source revisions and rule version; changed dependencies require current reevaluation; stale cached output cannot renew its own admission. (5) At each named cut, incremental maintenance and a clean reconstruction agree for this exact query contract; discarded CSR/snapshots do not change those answers once semantic recovery is implemented; approximate retrieval has a separate quality contract (ADR-040). M011 S03 ships a thinned executable slice of observations 1–2 via derivation_eligible_at; the fuller narrative remains a future fixture specification.

## Semantic contract — adversarial probes
- source: docs/architecture/semantic-contract-validation.md
- type: nfr
- content: Required observations for named probes (not additional subsystems): remove all leases / retain authoritative history → same terms/claims/outcomes or explicit failure; budgets 0/1/2 and crash after committed prefix → distinguishable zero/partial/full progress; change only a causal reference or rule version → provenance/execution verification detects it; duplicate delivery / two branches same local sequence → no duplicated support, cross-branch refs cannot target the wrong claim; two sources plus combining task / swap arrivals → same dependency set under stable identities; positive rule cycle / fresh-entity generator → finite saturation terminates, unbounded generation rejected/bounded; same endpoints different relations/supports → typed queries do not lose labels or multiplicity through neighbor-set CSR; A->B [1,2) B->C [3,4) → snapshot/co-temporal path absent, journey may exist under explicit waiting; two n-ary interactions with equal participants → incidence round-trip preserves identity/roles/positions/multiplicity; present access revoked / old data requested → AS OF does not bypass current authorization; remote success then crash before local outcome → reconcile via stable effect identity or keep unknown; one CSR build shared by two queries → charge shared work once, independent peak-RAM and deadline constraints still hold.

## Semantic contract — remaining encoding choices
- source: docs/architecture/semantic-contract-validation.md
- type: protocol
- content: Future implementation must still select logged term-definition format, stable claim/support references, quantum outcome/continuation records, partial-correction API, and admitted rule subset — for one authorized fixture, not by building every honeycomb cell. First technical prerequisite named: semantic recovery without snapshots (M010 done). Claim/support identity S01–S03 are in. Next dependency work still needs an explicit Active Slice lease. Not authorization to start M002. General multi-resource optimization, four-valued reasoning, full recursive provenance, and hypergraph-native storage are not prerequisites for the initial fixture.
