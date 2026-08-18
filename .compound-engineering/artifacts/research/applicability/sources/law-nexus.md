# law-nexus dossier (Wave 17 neighbor scout)

CBM: `law-nexus` (also `root-law-nexus`). Disk: `/root/law-nexus`. Kutha pack architecture already in ADR-090; this dossier is capability evidence, not a new honeycomb ADR.

| item | value |
|------|--------|
| Default confidence | `code` only for files actually read |
| Read | `crates/ln-kb-ontology/src/catalog.rs`; `prd/architecture/kb-ontology.yaml` (embedded via `include_str!`); ADR-0017 CTV |
| Coverage | `catalog.rs` / `kb-ontology.yaml`: `no_recorded_issue`, `metadata_changed` — claims qualified from source read |
| Closed card | `law-nexus-kb-ontology-catalog` |
| Do not | fork YAML ladders into kutha-graph; treat five clocks as algebraic axes; claim CTV from AST counts; run OWL/SPARQL/SHACL as legal SoT |

Kutha mapping: dictionaries as fail-closed YAML catalog; CTV versions *components* (article/part/point), not whole-act snapshots. Host as STCA legal pack (ADR-090 D090-1).
