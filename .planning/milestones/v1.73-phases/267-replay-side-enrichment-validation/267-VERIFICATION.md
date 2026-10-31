---
phase: 267-replay-side-enrichment-validation
independent: true
status: gaps_found
verifier_model: cline/cline-pass/deepseek-v4.1-flash-1017
commands:
  - cmd: cargo test --test m180_replay_meta
    exit: 0
  - cmd: cargo run -q (scratch probe tmp/v173-v174-verify/reason-probe; replays the two enrichment logs and prints the replay error reason)
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/enrichment.rs
  - crates/kutha-runtime/tests/m180_replay_meta.rs
  - crates/kutha-runtime/tests/m178_enrichment.rs
  - crates/kutha-runtime/tests/m183_digest_falsifiers.rs
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v170-v175.md
  - .kutha/STATE.md
---

# Phase 267 verification — replay-side enrichment validation

Re-issued 2026-10-31 at HEAD `89bc325` by a fresh independent verifier that did not
write the previous report. The previous body is replaced, not appended to. Full
evidence: `.compound-engineering/artifacts/reviews/2026-10-31-v173-v174-reverification.md`.

## Corrections to the previous body

**"Both now have real fixtures" is false for the two enrichment tests.** The report's own
finding was that `enrichment_for_unknown_pack_fails_replay` and
`duplicate_enrichment_cache_key_fails_replay` were vacuous because an empty
`source_event_ids` tripped an earlier check. Both tests still pass
`source_event_ids: vec![]` at `m180_replay_meta.rs:164` and `:190`, and
`enrichment.rs:108-112` still returns `EmptyField { field: "source_event_ids" }` before
the pack-state check (`:113`) and the cache-key check (`:155`). Both tests assert only
`EnrichmentRejected { .. }`, so they pass on the empty-source error.

Measured, not inferred. A scratch probe replays exactly the two logs the tests build:

- committed log of `enrichment_for_unknown_pack_fails_replay` →
  `EnrichmentRejected: replay: enrichment source_event_ids is empty`
- same log with `vec![source_event(0).id]` and the source event logged →
  `EnrichmentRejected: replay: enrichment pack ghost is not installed`
- committed log of `duplicate_enrichment_cache_key_fails_replay` →
  `EnrichmentRejected: replay: enrichment source_event_ids is empty`
- same log with a real source →
  `EnrichmentRejected: replay: enrichment cache key same was already proposed`

So both tests assert an error for the wrong reason. `git log
-S'source_event_ids: vec![]' -- crates/kutha-runtime/tests/m180_replay_meta.rs` names
only `572809d`; the fix commit `cebd27d` added two new tests and left these two
unchanged.

Required test change (not made here; this verifier may not edit tests): in both tests
log `source_event(0)` and pass `vec![source_event(0).id]`, then assert the reason as
`enrichment_wrong_version_and_retired_pack_fail_replay` already does (`:347-349`):
`reason.contains("not installed")` and `reason.contains("cache key")`. Without the
reason assert, deleting the `UnknownPack` arm (`enrichment.rs:113-117`) or the
cache-key lookup (`:155-159`) would leave both tests green.

**Which replay-side enrichment rejections are actually tested at HEAD.** Reason-asserting
replay tests exist for `MissingSource` (`m180:229-277`, reason "source event"),
`RetiredPack` (`m180:336-349`, reason "retired") and `DigestMismatch`
(`m183_digest_falsifiers.rs:132`, reason "do not match"). `VersionMismatch` is asserted
by variant only (`m180:328-334`). `UnknownPack` and `DuplicateProposal` have no valid
replay test (this finding). `PackNotActive` is tested only at admission
(`m178_enrichment.rs:92-95`) and `MissingBlob` has no test in either gate. The previous
sentence "rejects inconsistent enrichment proposals" is true of the code
(`enrichment.rs:108-159`, reached from `provenance.rs:174`) but not of the test set.

**The prefix-scoping half of the sentence is true.** The exemption is positional:
`compacted = first_checkpoint.is_some_and(|cut| index < cut)` (`provenance.rs:168`), and
`provenance.rs:436` unit-tests a post-checkpoint tail proposal. "Both now have real
fixtures" was wrong only about the two enrichment tests.

## What holds

- `cargo test --test m180_replay_meta` at HEAD: exit 0, 10 passed, 0 failed.
- Replay reaches the shared `check_proposal` (`provenance.rs:174`) with the pack state,
  the source predicate, the payload bytes and the running cache-key set, so admission and
  replay apply the same rules; `cache_keys.insert` happens only after a proposal passes
  (`:181-186`).
- `enrichment_missing_source_fails_replay` (`:229-277`) uses a real source event and
  asserts reason "source event"; `enrichment_wrong_version_and_retired_pack_fail_replay`
  (`:280-350`) uses a real source and asserts "retired".
- The empty-pin and payload-digest gates exist in code (`enrichment.rs:96-107`,
  `:143-152`) and are asserted at admission (`m178:156-174`) and, for digest mismatch, at
  replay (`m183:132`).

## Status: gaps_found

Two rejection paths this phase is named for — unknown pack and duplicate cache key — have
no test that reaches them at HEAD, and the report claimed they did. The code implements
both correctly (verified by the probe with a real source), so the gap is test evidence,
not behavior. Closing action: the two test changes above; `cargo test --test
m180_replay_meta` then fails if either check is removed.

## Not re-run in this pass

`cargo test --workspace`, `uv run kutha-gov precommit` and `uv run kutha-gov ci --jobs 1`
were not re-run, so the previous "workspace 801 passed / 0 failed" and the governor exit
0 are inherited claims, not re-verified by this pass.
