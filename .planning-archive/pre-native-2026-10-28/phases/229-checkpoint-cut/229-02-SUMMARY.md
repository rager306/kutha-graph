---
phase: 229-checkpoint-cut
plan: 02
subsystem: runtime-store
tags: [checkpoint, cut, Horn, migrate, delivery-key]
requires: [{phase: 229-checkpoint-cut, provides: "fallible fork_at Result + b3 fork REDs GREEN"}]
provides: ["Horn cascade twin parity after cut", "migrate InvalidData on Checkpoint", "prefix keyed Assert retry after cut"]
affects: []
actuals: {tokens: 10012, tasks: 4, commits: 15}
plan_head_before: c30e5e932cc66df0fd1cc80445e86dc04bf2f30e
plan_head_after: 513f8bfd01176485ad3704b5f67302f481e672ff
tech-stack: {added: [], patterns: [compacted_assert_by_id reconstructed from restored facts]}
key-files: {created: [], modified: [crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/src/eval.rs, crates/kutha-runtime/src/hydrate.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/tests/b3_checkpoint_cut.rs, CHANGELOG.md, docs/ADR/ADR-012-snapshots-tiers-vacuum.md, docs/process/op-compat.md]}
key-decisions:
  - "Kill-fix r1: Snapshot.behaviors exact prefix Behavior records"
  - "Kill-fix r2: keep same-id multi-cause Behavior rows; rebuild indexes each minting row"
  - "Kill-fix r3: by_delivery_key + event_by_id from restored facts; no new Snapshot field"
requirements-completed: [D-03, D-04, D-05]
duration: 360min
completed: 2026-10-23
status: complete
---
# Phase 229 Plan 02: checkpoint cut
**Prefix keyed Asserts retry after a sealed cut; Horn cascade and migrate-refuse stay green.**
## Root cause (kill3 `r3_reregister_same_horn_after_checkpoint`)
Twin is persist+`open` (P). `emit_rule_ast` = kept `StoreBlob` + `kutha:rule` Assert (`delivery_key`=digest). Rewrite drops that Assert; restored facts still hold the pin. `delivery_key_retry` needs `by_delivery_key` then `event_by_id` + payload match. Both were log-only → re-register minted a second live pin → Horn fired twice → `live_count(A,relatedTo,C)=2` vs twin 1. Outcome **(ii)**: P already retries (`events_in_quantum=0`). Fingerprint unchanged.
## Fold-state audit (field | rebuilt identically? | fix)
| Field | Identical? | Fix |
| facts, next_seq, index_mode, tt_anchors | yes (JSON; tt_anchors also Checkpoint-blob union) | — |
| hot_examine, horn_examine | n/a (skip counters) | — |
| hot maps (vt_by_from, tt_by_ingest, claim_facts, fact_by_event, live_triples, allows_by_name, horns_by_premise, facts_by_relation, rules_by_pin, policies_by_pin, policy_by_ingest, horn_lease_ready) | yes (`rebuild_hot_maps` / bind) | — |
| dictionary_rels, rel_* | yes (`bind_reserved`) | — |
| blobs | yes (StoreBlob kept; `rebuild_blobs`) | — |
| compacted_behaviors | yes (`Snapshot.behaviors`) | r1–r2 |
| Runtime.log, outcomes | rewrite keeps SB/QO/JC | by design |
| dict, next_tt, max_cascade, knows, known_by | yes (snapshot) | — |
| derived_by_cause, live_behaviors, premise maps, compacted_behavior_by_id | yes (behaviors+log) | r2 |
| **by_delivery_key** | no (was log Asserts only) | **facts `or_insert` after log walk** |
| **by_event_id / event_by_id / log_event** | no (log only) | **`compacted_assert_by_id` from facts** |
Author RED (not kill-tests): horn re-register, product key, allow, policy — C `events_in_quantum=1` vs P=0.
## Fix
No new Snapshot field. Receipt checks and empty-fold placement unchanged. Re-checkpoint carries facts; next open rebuilds the same leases.
## GREEN
`b3_checkpoint_cut_kill3 --test-threads=1` 8/8 (608s). `b3_checkpoint_cut` 8/8. `b1_checkpoint_kill{,2,3}`, `b3_checkpoint_cut_kill{,2}`, `b1_checkpoint_receipt`, `m002_commit01`, `m110*` 0. `cargo test --workspace --offline --no-fail-fast` 0. `uv run kutha-gov precommit` 0. Bench skipped (hydrate/open, not ingest_plain/as_of).
## Deviations
Rule 2: reconstruct prefix Assert Events so payload match works, not only the key index. Kill-tests not edited. `.planning/STATE.md` / `229-VERIFICATION.md` not edited.
## Task commits
`e844b54` Horn indexes · `770150e` migrate · `97c984a` behaviors · `44962da` multi-path · `513f8bf` delivery-key retry · verifier `7f6d3d8`/`555c5a0`/`c8c5e96`
## Open questions
`kutha:admission` / `kutha:action` use the same fact walk (no dedicated author test). Dead keyed facts still `or_insert`; liveness in `delivery_key_retry` matches twin. CBM coverage `metadata_changed` (no parent reindex).
## Self-Check: PASSED
