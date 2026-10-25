---
phase: 234-blob-digest
independent: true
status: pass-with-findings
verifier_model: Cursor Grok 4.6
commands:
  - cmd: cargo test -p kutha-runtime --offline --test b8_blob_digest_kill
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-common/src/rule.rs
---

# Phase 234 — independent verification

**Verdict: PASS-WITH-FINDINGS.** After the Behavior walk, `replay_check`/`store::open` return `UnknownRuleVersion { pin: digest }` when `!blob_digest_matches`. Matching sha256 / policy / rule-body hashes open. Apply stays void. No new Op. CBM `kutha-graph`: `provenance.rs` `metadata_changed`; source read. No `index_repository`.

## Claims

| id | claim | result |
|----|--------|--------|
| a | same-length tamper → UnknownRuleVersion | PASS `kill_same_length_tamper_fails_open_matching_twin_ok` |
| b | matching twin opens | PASS same test |
| c | truncated length (CRC rewritten) fails | PASS `kill_truncated_length_fails_open_crc_rewritten` |
| d | missing Horn + leading bad blob → ExecutionReplayFailed | PASS `kill_missing_horn_wins_over_leading_bad_ordinary_blob` |
| e | policy / rule-body hash matching opens | PASS `kill_policy_and_rule_body_hash_matching_blobs_open` |
| f | digest = sha256(other bytes) fails | PASS `kill_digest_sha256_of_other_content_fails` |
| g | apply infallible; no new Op | PASS `from_dict_and_events` Ok; 3b25f36 does not touch `Op` |

## Fixture diffs

OK, not HIGH. Unmatched legacy-keyed Named blobs were dropped so post-migrate `open` survives digest check; Behavior/cite/CLI remapped_lines oracles remain. m071 pre-migrate `open` is jsonl parse (allowed). b7 Named skip remaps digest+pin to Named hash so skip stays Ok.

## Findings

1. **LOW** Migrate fixtures no longer plant a legacy-digest StoreBlob of Named bytes. `migrate_pin_wire` still rewrites Behavior pins, not StoreBlob keys.
2. **LOW** b7 `kill_present_non_horn_body_skips_ok` now matches Named hash; Horn pin on Named bytes would be UnknownRuleVersion.

## Commands

- `cargo test -p kutha-runtime --offline --test b8_blob_digest_kill` **0** (5/5)
- `cargo test --workspace --offline --no-fail-fast` **0**
- `uv run kutha-gov ci --jobs 1` **0**, `run_high=0`

PersistScratch RAII; `tmp/` empty. No production edit.
