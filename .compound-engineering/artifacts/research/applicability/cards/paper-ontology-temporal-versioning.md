---
id: paper-ontology-temporal-versioning
source: paper
axes: [Time, Space, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Ontology schema and instance versions are first-class — not a silent latest OWL file

Papers: [τOWL temporal schema versioning](https://consensus.app/papers/details/080a93536ea950189ea37d2dfb1e6a77/?utm_source=cursor) (Zekri et al., 2016) and [τOWL framework](https://consensus.app/papers/details/3d580a4da08f584080bdf544386539ef/?utm_source=cursor) (DOI: 10.1007/s13740-016-0066-3) — conventional schema + logical/physical annotations; schema *and* instances versioned [2][5]; [instance-driven schema versioning](https://consensus.app/papers/details/3d50ea159bbc5a0b88ad240f4aa01483/?utm_source=cursor) (Brahmia et al., 2021) — non-conservative instance updates mint a new schema version so the pair stays consistent [1]; [KGCL](https://consensus.app/papers/details/dc032be0a36154b1a0a6a2297be2406b/?utm_source=cursor) (Hegde et al., 2024, DOI: 10.1093/database/baae133) high-level change language (“add synonym…”, “move X under Y”) [7]; [Valid Ontology](https://consensus.app/papers/details/e2c2613f2e30572f833ef171e901d1d0/?utm_source=cursor) (Grandi, 2009) one temporal XML document, snapshot OWL via temporal query — legal/e-Gov versions [8]; [Historical KG](https://consensus.app/papers/details/37b505a497da579394b7a6f1bdd7b5ed/?utm_source=cursor) (Cardoso et al., 2020) all versions in one graph [11]. Distinct from `paper-pgschema-types-keys` (static DDL) and `paper-pghive-schema-discovery` (mine types, no version algebra).

## 1. Raw idea

OWL 2 has no time. τOWL keeps a conventional (non-temporal) ontology plus annotations that say *what* may vary and *how* it is stored, so you do not fork the W3C language [5]. Schema versioning versions all three: conventional schema, logical annotations, physical annotations [2]. If instances stop conforming, Brahmia: mint a new *schema* version rather than reject the world or silently coerce [1]. KGCL is `diff`/`patch` for ontologies at curator granularity, not RDF triple dumps [7]. Legal multi-version OWL can live in one temporal XML and extract snapshots [8].

## 2. STCA applicability

Time/Space: **dictionaries are dated**. A meta-ontology (kinds, shapes, OWL TBox) is Control @T (ADR-050), not L_KB facts. Instance updates that break the TBox are TOKI-class write contracts: either reject, or log a schema-version event *and* the instance event. Do not treat “latest `.owl` in git” as SoT. Distinct from CTV (law-nexus): CTV versions *normative components*; τOWL versions *the schema of the schema*. Both are needed. KGCL patches are log events with CNL payloads.

## 3. Quality / cost

Usefulness high: every vertical pack will evolve types. Optimality med: τOWL is XML/OWL tooling, not Cypher/WCOJ. Cost: version dict manifests (semver + valid-time) like daily-archive ADR-044; compile *one* active schema into the query planner; keep history on the log.

## 4. Demand

Legal (Valid Ontology), geoscience charts, Gene Ontology — users ask “which ontology as of?”. Engine demand: schema_version on dict events.

## 5. Niche → effect

`no niche`
