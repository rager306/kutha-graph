---
phase: 233-horn-blob
plan: 01
status: complete
completed: 2026-10-24
---
# 233-01: missing Horn blob fails closed

Miss branch only. No Op, wire, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --test b7_horn_blob -- --exact missing_rule_blob_fails_replay` **101**.

`missing_rule_blob_fails_replay` panicked at `expect_err("missing nonempty pin blob must fail closed")` with `Ok(GraphFold { … blobs: {} … })`. `replay_check` accepted a nonempty `inverse_knows` pin after the rule `StoreBlob` was dropped from the fold.

## GREEN

`eval.rs` `horn_behavior_reproducible`: `blobs.get` miss returns `false` (was `!rule_version.is_empty()`). RULE-04 comment already said missing blob fails closed; left unchanged. Empty pin still fails. Named body skip and unmatched Horn unchanged.

Oracle `missing_rule_blob_fails_replay`: one `knows` Assert fires `inverse_knows`; twin with the blob present is `replay_check` Ok; drop the pin's `StoreBlob` from the log and rebuild → `ExecutionReplayFailed` for that Behavior id.

`cargo test -p kutha-runtime --offline --test b7_horn_blob` **0**. Then crate `--offline --no-fail-fast` **0** after fixture restores. `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `eval.rs` (no `mod` children).

## Fixtures that relied on fail-open

Restored a named `derive_pq` `StoreBlob` (legacy + v1 pins, or `r2` in m011). Did not weaken `replay_check`.

- `m011_provenance`: pin → `r2` with no blob
- `m071_pin_wire_migrate`, `m072_pin_cite_migrate`, `m075_migrate_pins`, `m076_migrate_compat`, `m078_migrate_cli`, `m091_pin_cite_justifications`, `m134_pin_cite_delete_rerun`, `m135_migrate_pins_unlock_window`: appended Behavior/cite with nonempty pin and no body

## Isolation

Did not edit `fold.rs`, `fold_query.rs`, `leapfrog.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No bench A/B.
