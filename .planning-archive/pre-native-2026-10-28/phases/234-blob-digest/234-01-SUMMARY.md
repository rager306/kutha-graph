---
phase: 234-blob-digest
plan: 01
status: complete
completed: 2026-10-25
---
# 234-01: reject a tampered ordinary StoreBlob

Digest check after the Behavior walk. No Op, apply fallibility, fingerprint, or version bump.

## RED

`cargo test -p kutha-runtime --offline --test b8_blob_digest -- --exact tampered_storeblob_same_length_fails_open` **101**.

`tampered_storeblob_same_length_fails_open` panicked at `tampered same-length StoreBlob must fail open (got Ok)`. Persist of a matching ordinary blob, one-byte same-length rewrite via `write_test_segmented` (CRC valid), `store::open` returned Ok.

## GREEN

`provenance.rs` `replay_check`: after the Behavior walk, `Op::StoreBlob` with `!blob_digest_matches` returns `UnknownRuleVersion { pin: digest.clone() }`. Admit already used `blob_digest_matches`. `GraphFold::apply` stays infallible.

Oracle: twin with original bytes opens; tampered same-length body is `InvalidData` / `unknown rule version {digest}`. Missing Horn blob stays `ExecutionReplayFailed` (`missing_horn_blob_stays_execution_replay_failed`).

`cargo test -p kutha-runtime --offline --test b8_blob_digest` **0**. Then crate `--offline --no-fail-fast` **0**. `uv run kutha-gov precommit` **0**. rustfmt `--edition 2024` on `provenance.rs` (no `mod` children).

## Fixtures that opened a digest-mismatch log

Did not special-case legacy pins in production. Dropped the unmatched legacy-keyed `StoreBlob` from migrate helpers (keep matching v1 Named body). Pin/cite assertions kept.

- `m011_provenance`: pin `r2` → `sha256_hex` of the Named body (still moves provenance, `replay_check` Ok)
- `m071_pin_wire_migrate`: pre-migrate pin asserted via jsonl parse, not `store::open`
- `m072_pin_cite_migrate`, `m075_migrate_pins`, `m076_migrate_compat`, `m078_migrate_cli`, `m091_pin_cite_justifications`, `m134_pin_cite_delete_rerun`, `m135_migrate_pins_unlock_window`: matching v1 blob only
- `b7_horn_blob_kill` `kill_present_non_horn_body_skips_ok`: Named skip retargets pin+digest to `ast.hash()` (Horn pin on Named bytes is `UnknownRuleVersion`)

## Isolation

Did not edit `fold.rs`, `fold_query.rs`, `VERSION`, Cargo/pyproject versions. One `target/`. No worktree. No bench A/B.
