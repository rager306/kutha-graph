# reactivegraph dossier (Wave 17 neighbor scout)

Disk: `/root/reactivegraph`. **Not CBM-indexed** — do not index the whole tree unless a later spike needs one file.

| item | value |
|------|--------|
| Default confidence | `code` only for files actually read |
| Read | `crates/rg-domain/src/knowledge.rs`; `doc/SCHEMA.md`; `crates/rg-composition/src/cypher_guard.rs`; ADR-0020 (superseded) / ADR-0030 |
| Closed card | `reactivegraph-ontology-version-facts` |
| Do not | take embedded redb/ruvector as Kutha SoT (ADR-0030 anti-pattern vs D1); confuse Cypher shape-guard with SHACL/SPARQL |

Kutha mapping: `OntologyVersion` registry + fact Active→Superseded/Invalidated + snapshot AS-OF + read-only Cypher gate. Borrow lifecycle and query fence; reject sole-substrate lock.
