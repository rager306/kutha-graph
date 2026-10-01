# Phase 18 Context: Rule registry

**Milestone:** v0.05 / harness M012 S02
**Requirements:** RULE-01, RULE-02, RULE-03
**Depends on:** Phase 17 (allowlist as log facts)
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F4 free-string rule_version)

## Locked decisions

- **D-01:** A rule registry stores definitions; `rule_version` on Behavior equals the definition hash (RULE-01).
- **D-02:** Unknown or mismatched `rule_version` fails closed on emit/eligibility (RULE-02).
- **D-03:** Free-string `rule_version` is rejected unless it matches a registry hash on the leased path (RULE-03).
- **D-04:** Do not implement ADR-050 six dictionaries. Do not regress Phase 17 ALL or M012a oracles.
- **D-05:** Honeycomb stays Proposed. No Rocks/Cypher/HNSW/legal pack.

## Spike anchors

- `crates/kutha-common/src/event.rs` — `Op::Behavior { rule_version: String, .. }`
- `crates/kutha-runtime/src/quantum.rs` — `derivation_eligible_at`, `follow_ons`, emit path
- Phase 17 `Op::AllowRelation` pattern for log-native dictionary facts

## Non-goals

- Admission meta-facts / action record (Phases 19–20)
- Multi-hop derivation (Phase 21)
- Full provenance polynomials / MATCH compiler
- Six dictionaries

## Harness

- Active Milestone M012, Active Slice **S02**, Phase H5, `L_delivery=M012-S01-done`
- Do not clear S02 until Phase 18 verification passes
