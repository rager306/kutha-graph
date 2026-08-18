---
id: law-nexus-kb-ontology-catalog
source: law-nexus
axes: [Space, Time, Agent, Verify]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: "Legal normative AST: YAML meta-ontology + CTV component versions; clocks not collapsed"
status: closed
channels_failed: []
---

# Flexible meta-ontology as a fail-closed YAML catalog (not hardcoded Rust kinds)

CBM `law-nexus` (coverage: `crates/ln-kb-ontology/src/catalog.rs` no_recorded_issue, metadata_changed — source read). `OntologyCatalog` loads `prd/architecture/kb-ontology.yaml` via `include_str!`; comment: kinds, levels, and FSM transitions are **not hardcoded**; unknown tokens fail closed. Document groups = structural ladders (heuristic `system_observation`, never legal classification). ADR-0017 CTV: component (article/part/point) is the versioned unit; LRMoo Work/Expression; full event-sourced resolver still `[proposed]` / text path `[bounded]`. Distinct from `paper-ontology-temporal-versioning` (OWL TBox versions) and ADR-090 (pack *architecture*; this card is the neighbor *capability*).

## 1. Raw idea

Meta-ontology lives as **data**: YAML catalog of node/edge kinds, document groups, structural roles, non-claims, FSM transitions. Rust compiles the file into the binary but does not own the vocabulary. Decode/applicability refuse unknown tokens. CTV versions *component text* across amendments (44-ФЗ ~118 editions) so “article X on date Y” is not whole-act snapshot smoothing. Five clocks stay typed stamps (ADR-0009), not five algebraic axes.

## 2. STCA applicability

Space/Agent: this is ADR-050 dictionaries — a **flexible meta-ontology**. Changing a ladder is a catalog version event, not a core schema migration. Time: CTV is L_KB expression time; catalog `schema_version` is Control time — TR-02 no silent substitution. Verify: non-claims are first-class (tests forbid claiming CTV from AST counts). Kutha hosts this as the legal pack, does not fork YAML casually (ADR-090 D090-1). SPARQL/SHACL are not in this crate; validation is catalog + fail-closed Rust.

## 3. Quality / cost

Usefulness high: the only neighbor that already treats ontology-as-data with fail-closed unknown tokens. Optimality med: YAML+include_str is compile-time freeze of the catalog; runtime hot-reload is not this design. Cost: import catalog+CTV as pack DATA; core keeps generic bi-temporal primitives. Do not encode 44-ФЗ ladders in kutha-graph.

## 4. Demand

Russian legal agents cannot ship without component AS-OF. Engine demand: dict catalog port. Product demand: law-nexus compatibility tables.

## 5. Niche → effect

Legal normative AST: YAML meta-ontology + CTV component versions; clocks not collapsed.
