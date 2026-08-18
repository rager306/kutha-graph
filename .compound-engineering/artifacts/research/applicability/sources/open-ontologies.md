# Open Ontologies (fabio-rovai/open-ontologies)

User-supplied 2026-08-18. Rust MCP server + Studio for AI-native ontology engineering. In-memory Oxigraph working copy.

| item | value |
|------|--------|
| Repo | https://github.com/fabio-rovai/open-ontologies |
| Disk / CBM | cloned `/tmp/user-url-scout/open-ontologies` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; `Cargo.toml` (v1.2.0, oxigraph 0.5); `src/lib.rs`; `src/graph.rs`; `src/reason.rs`; `src/shacl.rs`; `src/kgcl.rs`; `src/projection_check.rs` |
| Closed card | `open-ontologies-mcp-govern` |
| Do not | make Oxigraph/Rocks the Kutha SoT; treat LLM as executor; vendor PDDL/PyWhy/WASM plugin host without a spike |

Kutha mapping: Agent/Verify **generate–validate ontology pack**. OWL-RL + fragment SHACL + KGCL drift + projection-loss audit. Distinct from OBDA rewrite, τOWL version algebra, OxiXML parse-only SPARQL, OxiFY workflow DAG.
