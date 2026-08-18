---
id: paper-allen-interval-constraints
source: paper
axes: [Time, Query, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/clinical: during/overlaps/meets on valid-time intervals — qualitative constraints, not Timeline Index storage and not TVG journeys"
status: closed
channels_failed: []
---

# Allen relations are constraints on dated facts — not a third clock

Papers: Allen’s interval algebra (13 primitive relations: *before, meets, overlaps, during, …*) is the qualitative calculus; full IA satisfiability is NP-complete; path-consistency on tractable subclasses [6][20]; Point Algebra vs Interval Algebra; MinCons (solve with ≤ k timeline points) is NP-complete in general, polynomial on convex relations [4]; metric + qualitative networks compose with limited loss [8]; TICSP unifies qualitative IA with quantitative durations in one four-ary constraint net [7]; phenotyping: graphical Allen bars over EHR intervals compile to i2b2 [5]; IA^fuz adds fuzzy/uncertain qualitative constraints [12]. Distinct from `paper-temporal-interval-index` (storage access path for `[start,end]`), `paper-tvg-journeys-restless` (time-respecting *walks*), `paper-temporal-motifs` (timed-automata patterns; queued as general TCN cousin), `paper-tgql-intervals` (QL syntax). Spatial RCC stays a GeoSPARQL cousin, not this card.

## 1. Raw idea

Two valid-time intervals on the fold can be constrained *qualitatively*: statute A **during** case B, filing **meets** hearing, embargo **overlaps** publication [6][20]. That is a constraint network over intervals, not a B-tree on timestamps and not “is there a journey.” Path-consistency infers tighter relations [3][20].

## 2. STCA applicability

Time: Allen is a *predicate on existing valid-time*, not a third axis (decision-time stays queued). Query: compile `DURING`/`OVERLAPS` into interval tests (index card) plus optional path-consistency on a constraint lease. Verify: an inconsistent network is a why-not on the dated facts. Data: no new SoT. Agent: LLM may emit Allen atoms; the solver is deterministic. Metric bounds (durations, clock times) attach as numbers on the same intervals [8].

## 3. Quality / cost

Usefulness high: legal/clinical language is qualitative. Optimality med: IA is NP-complete; ORD-Horn/convex fragments are the P0 pole [4][6]. Cost: P0 = pairwise Allen tests using the interval index; honeycomb = path-consistency on a small TQCN lease. Do not run full IA on the whole fold. Fuzzy IA stays optional.

## 4. Demand

“Every filing *during* the investigation that *overlaps* the freeze” is Allen, not Cypher `*` and not a timeline scan alone. Engine demand: qualitative interval predicates with a tractable fragment as default.

## 5. Niche → effect

Legal/clinical: during/overlaps/meets on valid-time intervals — qualitative constraints, not Timeline Index storage and not TVG journeys
