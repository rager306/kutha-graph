---
phase: 260-c1-lifecycle-runtime-lease
independent: true
status: passed
verifier_model: zai/glm-5.3
commands:
  - cmd: cargo test -p kutha-runtime --test m176_pack_lifecycle_schema --test m176_pack_lifecycle_runtime
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: git log --format='%h %ad %s' --date=short -- .planning/milestones/v1.69-phases/260-c1-lifecycle-runtime-lease/260-VERIFICATION.md
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/segment.rs
  - crates/kutha-runtime/tests/m176_pack_lifecycle_runtime.rs
  - docs/process/op-compat.md
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v160-v169.md
  - .planning/STATE.md
  - .kutha/STATE.md
---

# Phase 260 verification — C1 lifecycle runtime lease

Re-issued 2026-10-31 at HEAD `a80e5fc` by a fresh independent verifier that did not
write the earlier report. **This re-verification is the recorded re-verify** required by
`docs/process/gsd-auto-contract.md:224`. Full evidence:
`.compound-engineering/artifacts/reviews/2026-10-31-v168-v169-reverification.md`.

## Record: this file was rewritten after the close

- Close commit `572fca4` (2026-10-31 00:37 +0700) carried "Runtime tests 5/5; workspace
  762 passed / 0 failed" and said F1 "was added before close".
- `e9c5ff2` (00:57) rewrote the body to describe the checkpoint test as "first persists
  lifecycle events, appends six product Asserts, persists again so Pack* sits in the
  sealed prefix, then checkpoints" and raised the count to 763, keeping the same
  "Independent verifier PASSed" sentence. `d9f7ed3` did not touch this file.
- `git log -- <path>` lists `572fca4` and `e9c5ff2` only. The original PASS therefore
  described a tree the verifier did not see, with no re-verification recorded.

**Counts.** 762 was the close count at `572fca4` (schema 4/4; gitignored
`tmp/m176-final-test.log` tallies 762 / 0 and has no digest test). 763 is the post-fix
count at `e9c5ff2` (schema 5/5; `tmp/m176-final2-test.log` tallies 763 / 0 with
`pack_digest_separates_length_collisions`). Both logs are scratch: context, not durable
evidence.

## What this pass verified at HEAD

- Targeted run exit 0: `m176_pack_lifecycle_runtime` 5 passed and
  `m176_pack_lifecycle_schema` 5 passed, 0 failed; precommit exit 0.
- Lifecycle state is a read model over the log: `derive_pack_lifecycle`
  (`pack_lifecycle.rs:37-84`) walks admitted events, records installs in a first-seen
  order vector, and ignores every other op; one shared `PackLifecycleTracker` serves
  admission and replay.
- Graph neutrality: `lifecycle_state_is_replay_derived_and_graph_neutral` and
  `lifecycle_query_ignores_product_graph_events` show product Asserts neither enter the
  lifecycle view nor move the fold fingerprint.
- Persist/open, fork prefix cuts and `checkpoint_sealed_prefix` all preserve the derived
  state (`lifecycle_state_survives_persist_open`,
  `lifecycle_fork_cut_preserves_prefix_state`,
  `lifecycle_state_survives_checkpoint_sealed_prefix`).
- The checkpoint test is non-vacuous at HEAD. `split_events` (`segment.rs:924-948`)
  chunks the log at the 2-event test limit and `persist_log` (`:955-975`) seals every
  chunk except the last. Eight persisted events therefore give three sealed segments
  plus one active, with install/activate in the first sealed one, so the assertion after
  `checkpoint_sealed_prefix` depends on the prefix copy. The close-time version had two
  events, i.e. one chunk that was also the last, so nothing was sealed and the
  assertion held with or without prefix copying; `e9c5ff2` is what made the test real.

## Correction: the replay limitation is superseded

The previous body said "Replay-side ordering is not revalidated (admission is the gate);
a hand-edited log can open with orphan transitions." That was true at `572fca4`. M180
(`572809d`) added replay revalidation: `provenance.rs:118-134` feeds every
PackInstall/PackActivate/PackRetire through the shared tracker on `replay_check`, and
the Pack* row of `docs/process/op-compat.md` now states that a hand-edited log with
orphan transitions, a duplicate install, or a missing/mismatched manifest fails
`replay_check` and `open`. The limitation is dated, not current.

## Caveat (not a gap in the phase outcome)

"in first-install order" is code-true (`pack_lifecycle.rs:38,62,82`), but no test in
this phase installs two packs, so ordering across packs is unexercised by phase 260.

## Status: passed

Every claim in this file is true at HEAD after the two corrections above, and the
phase's runtime tests pass. The post-close rewrite is recorded, this pass supplies the
missing re-verification, and the one stale limitation is now dated and superseded.

## Not re-run in this pass

`cargo test --workspace`: the 763 figure is inherited from the post-fix log, not
re-measured; HEAD carries later milestones, so a HEAD workspace count would not
reproduce it.
