# Phase 102 CONTEXT — m028×layouts + shrink corpus

**Source:** Matrix residuals after M041 (R2/R4). ADR-023 Clarification.  
**Lease:** M042 S01 / GSD v0.36.

## Goal

1. Trailing `emit_retract_dict` stays revoked after persist+open on every `PersistLayout` (P0 #1 property, not one layout).
2. `load_shrunk` / `save_shrunk_to` round-trip for `KUTHA_SAVE_SHRUNK` artifacts (Helix FileRegressionCorpus cousin). Reuse `sem::persist_dir`; do not panic Rollover without seam padding.

## Oracles

- `trailing_dict_retract_survives_all_persist_layouts`
- `trailing_dict_retract_solo_survives_all_persist_layouts`
- `shrink_corpus_roundtrip_load_matches_saved`
