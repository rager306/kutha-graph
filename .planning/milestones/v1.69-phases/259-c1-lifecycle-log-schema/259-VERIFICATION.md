---
phase: 259-c1-lifecycle-log-schema
independent: true
status: passed
verifier_model: zai/glm-5.3
commands:
  - cmd: cargo test -p kutha-runtime --test m176_pack_lifecycle_schema --test m176_pack_lifecycle_runtime
    exit: 0
  - cmd: cargo test -p kutha-common
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: git log --format='%h %ad %s' --date=short -- .planning/milestones/v1.69-phases/259-c1-lifecycle-log-schema/259-VERIFICATION.md
    exit: 0
read_only_inputs:
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m176_pack_lifecycle_schema.rs
  - crates/kutha-runtime/tests/m176_pack_lifecycle_runtime.rs
  - docs/process/op-compat.md
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v160-v169.md
  - .planning/STATE.md
  - .kutha/STATE.md
---

# Phase 259 verification — C1 lifecycle log schema

Re-issued 2026-10-31 at HEAD `a80e5fc` by a fresh independent verifier that did not
write the earlier report. **This re-verification is the recorded re-verify** required by
`docs/process/gsd-auto-contract.md:224` after a `status: passed` verification is edited.
Full evidence: `.compound-engineering/artifacts/reviews/2026-10-31-v168-v169-reverification.md`.

## Record: this file was rewritten after the close

- Close commit `572fca4` (2026-10-31 00:37 +0700) carried "Schema 4/4; workspace 762
  passed / 0 failed".
- `e9c5ff2` (00:57) added the fifth schema test
  (`pack_digest_separates_length_collisions`), made the runtime checkpoint test
  non-vacuous, and rewrote this file to "Schema 5/5 (including a digest length-collision
  guard); workspace 763 passed / 0 failed" while leaving the "Independent verifier
  PASSed" sentence unchanged. `d9f7ed3` (01:12) edited this file again.
- `git log -- <path>` lists exactly those three commits, so the original PASS was
  attached to text describing a tree the verifier did not see, and no re-verification
  was recorded at that time.

**Counts.** 762 was the close count at `572fca4` (4 schema tests; the surviving
gitignored log `tmp/m176-final-test.log` tallies 762 passed / 0 failed and does not
contain the digest test). 763 is the post-fix count at `e9c5ff2` (5 schema tests;
`tmp/m176-final2-test.log` tallies 763 / 0 and does contain
`pack_digest_separates_length_collisions`). Both logs are scratch: context, not durable
evidence.

## What this pass verified at HEAD

- Targeted run exit 0: `m176_pack_lifecycle_schema` 5 passed,
  `m176_pack_lifecycle_runtime` 5 passed, 0 failed. `cargo test -p kutha-common` exit 0.
  `uv run kutha-gov precommit` exit 0.
- Pack events are meta. `Event::new` (`event.rs:305-340`) assigns `object_ids` only for
  `Assert`/`Behavior` and the correct-ops; PackInstall/PackActivate/PackRetire fall in the
  empty-vector arm (`event.rs:327-331`), and the fold treats them as non-facts.
  `lifecycle_ops_are_meta_and_do_not_create_graph_facts` asserts unchanged facts and
  fingerprint.
- Digests are length-prefixed per field for install, activate and retire
  (`event.rs:560-595`), and `pack_digest_separates_length_collisions` asserts that the
  `"ab"+"c"` and `"a"+"bc"` shapes differ for all three ops.
- Admission rejects duplicate install (`pack_id_already_known`), activate before
  install and retire with a wrong version (`activate_requires_same_installed_version`,
  `retire_requires_same_installed_version`), repeated activate/retire, empty and padded
  identifiers, and user-forged Pack* (`rt.emit` returns `MetaOpRejected`). The op-compat
  page carries the M176 Pack* row.
- The close-time findings remain represented at HEAD: F1 by the runtime checkpoint test
  (see `260-VERIFICATION.md`), F2 by the digest guard test, F6 by the op-compat row. The
  rejection reason for a wrong-version activate is `activate_requires_same_installed_version`.

## Caveat (not a gap in the phase outcome)

The sentence "admission rejects activate without install/version mismatch" is
code-true, but the schema test drives the wrong-version case through *retire*. No test
in this phase asserts the activate reason. Replay-side coverage arrives at M180.

## Status: passed

Every claim in this file is true at HEAD and the phase's tests pass. The post-close
rewrite is recorded above and this pass supplies the missing re-verification, so this
report no longer certifies text that no verifier saw.

## Not re-run in this pass

`cargo test --workspace`: the 763 figure is inherited from the post-fix log, not
re-measured. HEAD also carries later milestones, so a HEAD workspace count would not
reproduce it.
