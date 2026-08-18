# OxiFY (cool-japan/oxify)

User-supplied 2026-08-18. Rust DAG LLM workflow platform + Zanzibar-style ReBAC crate.

| item | value |
|------|--------|
| Repo | https://github.com/cool-japan/oxify |
| Disk / CBM | cloned `/tmp/cool-japan/oxify` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; workspace `Cargo.toml`; `crates/oxify-engine/README.md`; `crates/oxify-engine/src/lib.rs` (Kahn topo-sort, `execute`); `crates/oxify-authz/src/lib.rs` (`RelationTuple`); `crates/oxify-authz/src/types.rs` (`TimeWindow`); `crates/oxify-authz/src/leopard.rs`; `crates/oxify-authz/src/engine.rs` (SQLite `AuthzEngine`); `crates/oxify-authz/src/quantum.rs` (Kyber **placeholder**) |
| Closed cards | `oxify-dag-llm-orchestration`; `oxify-zanzibar-rebac` |
| Do not | make CeleRS/Qdrant/pgvector the Kutha log; confuse engine “quantum.rs” with Kutha quantum receipts; treat `oxify-authz::quantum` as shipped PQC (`pqcrypto_kyber` commented placeholder) |

Kutha mapping: Agent pack (DAG compile, cousin of Dify/AZ plan-DAG) and Security pack (ReBAC tuples + Leopard reachability index). WASM/Rhai code nodes are cousins of `paper-wasm-udf-sandbox`. AReBAC-from-papers stays closed as this *code* ReBAC surface.
