# RuVector dossier (Wave 1)

CBM: `root-vendor-source-ruvector` (~531k nodes). Not reindexed.

## Extracted

| card | confidence | note |
|------|------------|------|
| ruvector-hnsw | code | `ruvector-core` HnswIndex |
| ruvector-cypher-empty-success | code | sequential execute placeholder `Ok(Vec::new())` |
| ruvector-rvf-cow-seal | code | freeze examples; packaging not SoT |
| ruvector-hybrid-bm25-dense | observed | hybrid crate cited; SQL mocks not promoted |
| ruvector-gnn-facade | code | `graph_neural.rs` dummy `[0.7,0.2,0.1]`; primitives exist separately |
| ruvector-hnsw-delete-repair | code | `ruvector-hnsw-repair`: tombstone / batch / eager rewiring of HNSW neighbours after delete. Not PG-Constraint repair. Cousin: postgres `RemediationEngine::heal`. |

## Adapter mapping (2026-09-13)

Crate-level pins for crates **without** a dedicated card live in `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md` (`ruvector-temporal-tensor`, `ruvector-retrieval-receipt`, `ruvector-acorn`, `ruvector-temporal-coherence`). Do not extract new cards; vendor scout is already closed. `ruvector-proof-gate` is a cousin of `paper-query-admission-control`, not a new noun.

## Not extracted (keep as anti-patterns, not new cards)

npm FALLBACK stubs, GraphRAG fabricated paths — already covered by cypher empty-success + GNN facade. Do not treat utilization marketing as cards.
