# Phase 17 Context: Allowlist as log facts

**Milestone:** v0.05 / harness M012 S01
**Requirements:** ALL-01, ALL-02, ALL-03
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F5 ambient allowlist)

## Locked decisions

- **D-01:** Relation allowlist entries are appendable versioned facts on the event log (ALL-01).
- **D-02:** `Runtime::admit` consults the fold cut for allowed relations, not tip YAML/env alone (ALL-02).
- **D-03:** Unknown relation still fails closed; FF6 stays green (ALL-03).
- **D-04:** Do not implement ADR-050 six dictionaries. Do not regress M012a LOG/REF/ING/DUR/TIME/HOT oracles.
- **D-05:** Honeycomb stays Proposed. No Rocks/Cypher/HNSW/legal pack.

## Spike anchors

- `crates/kutha-runtime/src/allow.rs` — ENV_PATH, relations_path, in-memory allowed set
- `crates/kutha-runtime/src/quantum.rs` — `Runtime::admit`
- `crates/kutha-runtime/tests/ff6_allowlist.rs`

## Non-goals

- Rule registry / rule_version hash (Phase 18)
- Admission meta-facts / action record (Phases 19–20)
- Multi-hop derivation (Phase 21)
- Full six dictionaries

## Harness

- Active Milestone M012, Active Slice **S01**, Phase H5, `L_delivery=M012-leased`
- Do not clear S01 until Phase 17 verification passes
