# SciRS2 (cool-japan/scirs)

User-supplied 2026-08-18. Pure-Rust SciPy-class scientific stack. Kutha-relevant crate is `scirs2-graph` (CSR, algorithms, hypergraphs, GNN facades).

| item | value |
|------|--------|
| Repo | https://github.com/cool-japan/scirs |
| Disk / CBM | cloned `/tmp/cool-japan/scirs` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README (v0.6.5); workspace `Cargo.toml`; `scirs2-graph/README.md`; `scirs2-graph/src/lib.rs`; `scirs2-graph/src/compressed.rs` (`CsrGraph`); `scirs2-graph/src/algorithms/hypergraph.rs` |
| Closed card | `scirs-graph-scientific` |
| Do not | vendor 3.1M SLoC / 29 crates as the Kutha engine; treat NetworkX-class algorithms as SoT; promote GNN/HGNN facades (`ruvector-gnn-facade` cousin) without a separate spike |

Kutha mapping: Query/Data **analytics lease** (CSR + shortest path / centrality / community / VF2) over the fold. Distinct from Samyama frozen CSR (hop engine) and Falkor GraphBLAS (query-as-matrix in a graph DB). Hypergraph *algorithms* here; n-ary *fact model* stays on `paper-hypergraph-higher-order`.
