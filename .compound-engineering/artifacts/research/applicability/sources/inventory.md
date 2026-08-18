# Wave-1 inventory and channel map

Date: 2026-08-17. CBM `list_projects` at inventory time.

Default confidence is the *starting* mode for scouts. A card may use a weaker tag; it must not use `code` unless a kernel tree was actually read.

| source | disk | CBM project | default confidence | notes |
|--------|------|-------------|--------------------|-------|
| ruvector | present: `vendor-source/ruvector` | `root-vendor-source-ruvector` | code | Do not reindex unless empty/corrupt (~531k nodes). |
| agentdb | present (not a kernel): `vendor-source/ruflo/plugins/ruflo-agentdb`; narrative `vendor-source/ruvector/examples/meta-cognition-spiking-neural-network/docs/AGENTDB-EXPLORATION.md` | unindexed | spec / observed | Scout plugin + docs; upgrade to `code` only for files actually read. |
| samyama | present: `/root/samyama-graph` and `vendor-source/samyama-graph` | `root-samyama-graph` (path `/root/samyama-graph`) | code | `vendor-source/samyama-graph` is not a separate CBM project. |
| rocksdb | present: `vendor-source/rocksdb` | `root-vendor-source-rocksdb` | code | Storage engine patterns, not a graph product. |
| helix | absent | unindexed | spec | ADR-000 cites indexes-as-access-paths. U5. |
| falkor | absent | unindexed | spec | ADR-000 cites GraphBLAS-like hot path. U5. |
| raven | absent | unindexed | spec / claim | Closed kernel. AE1. Never `code` unless a tree appears. |
| tarantool | absent | unindexed | spec | ADR-000 cites compute-near-data. U5. |
| papers | n/a | n/a | paper | After U3 Consensus→Jina. |

Not Wave 1 (do not scout now; queue if they appear as “new” mid-wave): CBM already has `graphiti`, `cozo`, `daily-archive`, `law-nexus`, `root-vendor-source-pogocache`. Those are adjacent, not starter-set promotion.

Later user-supplied: Dify (`https://github.com/langgenius/dify`) — not vendored, not CBM-indexed. Closed as `dify-workflow-rag-orchestration` (`confidence: code` for GitHub files actually read). See `sources/dify.md`.

Wave 17 neighbors (user-requested, not Wave-1 promotion):

| source | disk | CBM project | default confidence | notes |
|--------|------|-------------|--------------------|-------|
| law-nexus | present: `/root/law-nexus` | `law-nexus` | code for files read | YAML catalog + CTV. Coverage: catalog.rs metadata_changed. See `sources/law-nexus.md`. |
| daily-archive | present: `/root/daily-archive` | `daily-archive` | code for files read | Schema YAML + NodeSchemaDef. See `sources/daily-archive.md`. |
| reactivegraph | present: `/root/reactivegraph` | unindexed | code for files actually read | Do not index the whole tree. See `sources/reactivegraph.md`. |

Wave 19 user-supplied: Hindsight (`https://github.com/vectorize-io/hindsight`) — cloned to `/tmp/hindsight` for read, not vendored, not CBM-indexed. Closed as `hindsight-four-network-tempr`. See `sources/hindsight.md`.

User-supplied 2026-08-18 (cool-japan, not Consensus): OxiXML (`https://github.com/cool-japan/oxixml`), OxiFY (`https://github.com/cool-japan/oxify`), SciRS2 (`https://github.com/cool-japan/scirs`) — cloned to `/tmp/cool-japan/*` for read, not vendored, not CBM-indexed. Closed as `oxixml-xml-rdf-stack`, `oxify-dag-llm-orchestration`, `oxify-zanzibar-rebac`, `scirs-graph-scientific`. See `sources/oxixml.md`, `sources/oxify.md`, `sources/scirs.md`.

User-supplied 2026-08-18 (GitHub scout, not Consensus): ULTRA (`https://github.com/DeepGraphLearning/ULTRA`), CRP-SpMM (`https://github.com/scalable-matrix/CRP-SpMM`), TypeGraph (`https://github.com/nicia-ai/typegraph`), Open Ontologies (`https://github.com/fabio-rovai/open-ontologies`) — cloned to `/tmp/user-url-scout/*` for read, not vendored, not CBM-indexed. Closed as `ultra-kg-foundation-reasoner`, `crp-spmm-comm-reduced`, `typegraph-typed-sql-kg`, `open-ontologies-mcp-govern`. See `sources/ultra.md`, `sources/crp-spmm.md`, `sources/typegraph.md`, `sources/open-ontologies.md`.

User-supplied 2026-08-18 (GitHub scout, not Consensus): Engramx (`https://github.com/nickcirv/engram`) and Harvey LAB firm-knowledge (`https://github.com/harveyai/harvey-labs/tree/main/tasks/firm-knowledge`) — cloned to `/tmp/user-url-scout/{engram,harvey-labs}` for read, not vendored, not CBM-indexed. Closed as `engramx-code-context-spine`, `harvey-lab-firm-knowledge`. See `sources/engramx.md`, `sources/harvey.md`. Do not confuse Engramx with Wang 2026 Engram. Do not vendor harvey-labs.

## Channel failures

None at inventory time.
