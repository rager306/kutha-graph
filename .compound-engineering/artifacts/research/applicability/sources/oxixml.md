# OxiXML (cool-japan/oxixml)

User-supplied 2026-08-18. Pure-Rust XML + RDF foundation (Xerces/Xalan/FOP/Saxon/Oxigraph *capabilities*, not those runtimes).

| item | value |
|------|--------|
| Repo | https://github.com/cool-japan/oxixml |
| Disk / CBM | cloned `/tmp/cool-japan/oxixml` for this scout; not vendored; not indexed |
| Default confidence | `code` only for files actually read |
| Read | README; `Cargo.toml`; `sparql/oxixml-sparql-syntax/src/lib.rs`; `rdf/oxixml-model/src/lib.rs`; `security/oxixml-dsig/src/lib.rs`; `validation/oxixml-schematron/src/lib.rs` |
| Closed card | `oxixml-xml-rdf-stack` |
| Do not | treat OxiXML as Kutha SoT; claim SPARQL *execution* (README: engines stay in `oxirs-arq` / `oxirs-shacl`); vendor 46 crates into the kernel |

Kutha mapping: ingest/validate/export pack (XML trees, XSD/Schematron, RDF `Dataset`/`GraphName`, SPARQL algebra parse, XMLDSig). Named-graph *noun* stays on `paper-named-graphs-rdf-dataset`. SHACL compile stays on `paper-shacl-sparql-compile`.
