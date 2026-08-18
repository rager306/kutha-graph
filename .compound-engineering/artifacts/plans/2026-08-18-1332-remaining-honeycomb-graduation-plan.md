---
title: Remaining Honeycomb Graduation - Plan
type: research
date: 2026-08-18
artifact_contract: ce-unified-plan/v1
artifact_readiness: implementation-ready
product_contract_source: ce-plan-bootstrap
execution: knowledge-work
---

# Remaining Honeycomb Graduation - Plan

## Goal Capsule

- **Objective:** Open every remaining **planned** honeycomb cell as **Research** ADRs (Space 020–022, Composition 030–031, Agent 051–052, Verify 060–062, Query 070–071, Security 080–081, Packaging 091–092). Do not implement the engine. Do not mint Consensus Query 103+. Do not mark Accepted (no P0 spike). Do not rewrite ADR-000 D1–D10.
- **Product authority:** `STRATEGY.md`; ADR-000 D1–D10; opened cells 010–014, 040–043, 050, 090, 093; card SoT in `.compound-engineering/artifacts/research/applicability/`.
- **Open blockers:** None. User authorized “promote all” remaining planned titles.
- **Product Contract preservation:** new IDs for this artifact only.

## Product Contract

### Summary

P0 physics and dict-first control (010–014, 040–043, 050) are already Research cells. Empty bands are engineering (020/022), composition traits vs scheduling (030/031), security kernels (051/080/081), verify harness (060–062), query surface (070/071), optional enrichment (052), packaging (091/092). Literature bound is closed; this is design graduation.

### Key Decisions

- KD1. **Open all remaining planned titles in this unit.** 062 stays Research and **optional (default off)**. 092 is naming/license research, not a trademark filing.
- KD2. **Status = Research.** Not Proposed/Accepted (Accepted requires implementation).
- KD3. **Cards SoT for evidence; ADRs SoT for decisions.** No new matrix rows.
- KD4. **No engine, no Consensus 103+.**
- KD5. **Traps stay Alternatives.** Graphiti, Dify, Hindsight, CRDT-as-SoT, Geo-Raft-as-SoT, blockchain-as-SoT, RVF-as-hot-store, empty Cypher.
- KD6. **Noun fences:** 021 pack lifecycle ≠ 040 view contract; 030 envelope traits ≠ 031 competition under V ≠ admission (CASA); 051 capabilities ≠ 080 path-ABAC ≠ 081 WASM; 060 replay ≠ 061 fork; 070 GPML/Cypher ≠ 071 approximate retrieve; 091 RVF ≠ log.

### Requirements

- R1. Sixteen new files under `docs/ADR/` named in the citation map.
- R2. Honeycomb template; English bodies; Date 2026-08-18; Status Research.
- R3. Required card ids cited (092 cites STRATEGY + ADR-000 R10; no dedicated card).
- R4. Decisions `D0xx-n`; do not contradict D1–D10.
- R5. Repeat: log = SoT; leases droppable; LLM proposes, never writes truth.
- R6. `docs/ADR/README.md` moves all sixteen titles Planned → Opened.
- R7. Do not add `src/` Rust. Do not reopen 010–014/040–043/050/090/093 as rewrites (Related Decisions may now point at opened ids).

### Citation map (required cards)

| ADR | Required card ids |
|-----|-------------------|
| 020 Cargo & ports | `paper-named-graphs-rdf-dataset`, `paper-worlddb-worlds-edge-programs`, `paper-graph-tenant-isolation` |
| 021 Pack lifecycle | `paper-graph-pack-plugin-lifecycle`, `paper-online-pg-schema-evolution` |
| 022 Vertical slices | `paper-worlddb-worlds-edge-programs`, `paper-named-graphs-rdf-dataset` |
| 030 Cui traits | `paper-max-convolution-budgets`, `paper-sparql-multi-query-opt` |
| 031 Scheduling under V | `paper-query-admission-control`, `paper-max-convolution-budgets` |
| 051 Capabilities | `paper-object-capabilities`, `paper-pact-argument-provenance`, `paper-mas-isolation-lattice` |
| 052 Enrichment pack | `samyama-agentic-enrichment-gak`, `raven-in-db-ai-agents`, `paper-llm-compiler-not-executor` |
| 060 Strict replay | `paper-activegraph-log-is-sot`, `rocksdb-wal-recovery`, `paper-r3-record-replay-retroaction`, `paper-yankin-event-sourced-query` |
| 061 Fork-and-diff | `paper-graph-branch-fork`, `paper-graph-edit-distance` |
| 062 Regimes loop | `paper-cordon-semantic-tx`, `paper-kumiho-agm-revision` |
| 070 Cypher / AS OF | `paper-iso-gql-gpml`, `paper-az-projected-schema-cypher`, `paper-regular-path-queries`, `paper-tgql-intervals` |
| 071 Hybrid retrieve | `ruvector-hybrid-bm25-dense`, `paper-keyword-search-graphs`, `paper-mixed-vector-relational-access` |
| 080 ABAC / tenancy | `paper-xacml4g-path-abac`, `paper-temporal-grants-as-facts`, `oxify-zanzibar-rebac`, `paper-graph-tenant-isolation`, `paper-event-log-vacuum-legal-hold` |
| 081 Agent sandbox | `paper-wasm-udf-sandbox`, `paper-object-capabilities` |
| 091 RVF capsules | `ruvector-rvf-cow-seal`, `paper-constant-size-evidence` |
| 092 Naming & license | (none — STRATEGY + ADR-000 R10) |

Contrast-only: `dify-workflow-rag-orchestration`, `oxify-dag-llm-orchestration`, `hindsight-four-network-tempr`, `graphiti-bitemporal-fact-edges`, `paper-crdt-graph-eventual`, `paper-geo-raft-wan`, `paper-blockchain-graph-ads`, `ruvector-cypher-empty-success`.

### Assumptions

- A1. Keep honeycomb home in `docs/ADR/`.
- A2. 062 default-off; 092 does not pick a license or rename the product in this cell.
- A3. CMK/air-gap remains STRATEGY buyer constraint, not a graph noun.

### Sequencing

Space 020→022, Composition 030→031, Agent 051→052, Verify 060→062, Query 070→071, Security 080→081, Packaging 091→092, then index.

## Definition of Done

Sixteen Research files exist; index 1:1; required cites present; no `src/`; literature bound un-reopened.
