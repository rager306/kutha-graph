---
phase: 266-replay-side-pack-lifecycle-revalidation
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
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/tests/m180_replay_meta.rs
  - crates/kutha-runtime/tests/m176_pack_lifecycle_schema.rs
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v170-v175.md
  - .kutha/STATE.md
---

# Phase 266 verification — replay-side pack lifecycle

Re-issued 2026-10-31 at HEAD `89bc325` by a fresh independent verifier that did not
write the previous report. The previous body is replaced, not appended to. Full
evidence: `.compound-engineering/artifacts/reviews/2026-10-31-v173-v174-reverification.md`.

## Corrections to the previous body

**"Both now have real fixtures" is false.** `m180_replay_meta.rs:164` and `:190` still
pass `source_event_ids: vec![]`. `enrichment.rs:108-112` returns
`EmptyField { field: "source_event_ids" }` before the pack-state check (`:113`) and the
cache-key check (`:155`), so `enrichment_for_unknown_pack_fails_replay` (`:149-178`) and
`duplicate_enrichment_cache_key_fails_replay` (`:181-227`) assert only
`EnrichmentRejected { .. }` — no reason — and only ever see the empty-source error. A
scratch probe that replays the same two logs prints
`replay: enrichment source_event_ids is empty` for both; with a real source event the
same logs print `replay: enrichment pack ghost is not installed` and
`replay: enrichment cache key same was already proposed`. The two tests were vacuous
when this report was first written and they are still vacuous. `git log
-S'source_event_ids: vec![]' -- crates/kutha-runtime/tests/m180_replay_meta.rs` names
only `572809d`; the fix commit `cebd27d` added two new tests and left these two
unchanged.

Required test change (not made here; this verifier may not edit tests): in both tests
log `source_event(0)` and pass `vec![source_event(0).id]`, then assert the reason as
`enrichment_wrong_version_and_retired_pack_fail_replay` already does (`:347-349`):
`reason.contains("not installed")` and `reason.contains("cache key")`.

**"Replay now rejects ... version mismatch" is code-true but test-unverified for
packs.** `pack_lifecycle.rs:144-163` refuses a wrong-version activate, a second
activate and a second retire. No replay test drives those transitions: the ten tests in
`m180_replay_meta.rs` and the unit test in `provenance.rs:436` cover
activate-before-install, duplicate install, retire-unknown, two manifest cases and four
enrichment cases. The admission-side equivalents are tested in
`m176_pack_lifecycle_schema.rs:47-73`. Required change: one replay test per pack
transition.

**"`open` routes every reconstruction branch through `replay_check`" is narrowed to what
was checked here**: `store.rs:800` (open) and `store.rs:1236` (rebuild) call it, and the
replay path reaches the shared lifecycle tracker at `provenance.rs:119`.

## What holds

- Six pack-lifecycle tests assert `PackLifecycleRejected`: activate-before-install
  (`:71`), duplicate install (`:87`), retire of an unknown pack (`:100`), missing
  manifest blob (`:116`, reason "replay manifest" at `:122`), wrong manifest identity
  (`:126`), and a valid store still opening (`:44`).
- `cargo test --test m180_replay_meta` at HEAD: exit 0, 10 passed, 0 failed. The ten
  names are the ten tests in the file, including the two vacuous ones above.
- The unit test `tail_proposal_after_checkpoint_must_cite_visible_source` exists
  (`provenance.rs:436`).
- The compacted-source exemption is positional, not "proposals a later checkpoint
  compacts away": `compacted = first_checkpoint.is_some_and(|cut| index < cut)`
  (`provenance.rs:168`). Every proposal before the first checkpoint is exempt; the tail
  is judged against the visible log. The previous phrasing overstated the predicate.

## Status: gaps_found

The phase's pack-lifecycle tests are real and pass, but the report repeated a fix that
did not land (two enrichment tests still assert the wrong reason) and it claims one
behavior, pack-level "version mismatch", that has no replay test. Closing action: apply
the two test changes above and add the pack-transition replay tests; then
`cargo test --test m180_replay_meta` with reason asserts is the re-check.

## Not re-run in this pass

`cargo test --workspace`, `uv run kutha-gov precommit` and `uv run kutha-gov ci --jobs 1`
were not re-run, so the previous "workspace 801 passed / 0 failed" and the governor exit
0 are inherited claims, not re-verified by this pass.
