---
phase: 268-ecfr-edition-ambiguity-explained
independent: true
status: gaps_found
verifier_model: cline/cline-pass/deepseek-v4.1-flash-1017
commands:
  - cmd: cargo test --test m167_ecfr_pit
    exit: 0
  - cmd: cargo test -q (scratch copy tmp/v173-v174-verify/m167-count-removed; count assert deleted, committed fixtures)
    exit: 0
  - cmd: cargo test -q (scratch copy tmp/v173-v174-verify/m167-edition-lost; 304.1 2017-03-14 text replaced by the 2017-01-23 text)
    exit: 101
  - cmd: cargo test -q (scratch copy tmp/v173-v174-verify/m167-count-only; equality asserts deleted, count kept, same lost-edition fixture)
    exit: 101
read_only_inputs:
  - crates/kutha-runtime/tests/m167_ecfr_pit.rs
  - crates/kutha-runtime/tests/fixtures/ecfr/README.md
  - docs/architecture/ecfr-edition-ambiguity.md
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v170-v175.md
  - .kutha/STATE.md
---

# Phase 268 verification — eCFR ambiguity

Re-issued 2026-10-31 at HEAD `89bc325` by a fresh independent verifier that did not
write the previous report. The previous body is replaced, not appended to. Full
evidence: `.compound-engineering/artifacts/reviews/2026-10-31-v173-v174-reverification.md`.

## Corrections to the previous body

**"the count assert uniquely catches a lost edition" is false.**
`m167_ecfr_pit.rs:146` asserts `e0 == e1` and `:147` asserts `e1 != e2`, both before the
count assert at `:148-159`. Those two imply the set `{e0, e1, e2}` has exactly two
members, so `:148` cannot fail while they pass. The count assert is a restatement, not an
independent falsifier.

Measured on scratch copies of the test file (`tmp/v173-v174-verify/`; the repository
test file was not touched):

- count block deleted, committed fixtures: exit 0, 1 passed. Removing the count assert is
  unobservable.
- 304.1's 2017-03-14 text replaced by its 2017-01-23 text (a lost edition): exit 101,
  panic at `ecfr_mut.rs:147`, `assertion left != right failed`. The equality assert
  catches the loss first; the count assert is never reached.
- equality asserts deleted, count kept, same lost-edition fixture: exit 101, panic at the
  count assert, `left: 1, right: 2`. The count does catch a loss on its own.

So a lost edition does not pass the equality asserts, and a gained one does not either:
`assert_eq!(e0, e1)` fails as soon as the middle edition gains a distinct text. The
equality asserts carry both directions; the count restates them. Keep the count as the
only guard left if the equality asserts are removed, but do not call it the unique
falsifier.

At HEAD the architecture doc (`docs/architecture/ecfr-edition-ambiguity.md:49-51`),
`CHANGELOG.md` (v1.74 entry) and `268-01-SUMMARY.md:12` already state the corrected
version, because commit `89bc325` rewrote them. Two artifacts still carry the old claim
and are outside this verifier's edit set: the test comment at `m167_ecfr_pit.rs:141-145`
("This count assert is what catches a LOST edition", written by `a273326`) and
`.planning/MILESTONES.md:89` ("the count assert catches a lost edition, the equality
asserts catch a gained one"). Required change: reword both.

**The previous sentence "the comment names which assert catches which direction" is
false.** The comment names them, but the wrong way round (`a273326` introduced the
inversion). That is the same defect as above.

**"Checked against raw XML for all eight part-304 sections ... all nine committed texts
reproduce byte-exactly from live eCFR" cannot be re-checked in the repository.** The
fixture ships three sections × three dates (nine files, `m167:9-28`) and the probe
asserts those three. The raw XML lived in gitignored `tmp/ecfr-raw/`, which no longer
exists. What this pass can confirm is internal consistency: for each of the three
sections, editions 0 and 1 are byte-identical and edition 2 differs, which is what the
equality asserts check. The eight-section check and byte-exact reproduction are
source-time statements, not repository evidence; they are removed from this verification.

## What holds

- `cargo test --test m167_ecfr_pit` at HEAD: exit 0, 1 passed
  (`ecfr_pit_probe_reads_three_editions_from_real_text`).
- The citation-only explanation is supported inside the repository: the doc records FR
  document 2017-00891, published 2017-01-23 and effective 2017-03-14, and the committed
  fixture pair is identical for editions 0/1 and different for edition 2 in all three
  sections. No fixture `.txt` was edited to create a discriminator.
- The test's own PIT assertions (AS OF cuts, `as_known_on`, `norm_view`,
  persist/checkpoint/reopen) run and pass in the same 1/1 run.

## Limitation worth recording

The guard covers the three committed dates only. It fires when one of the nine committed
texts changes, not when eCFR later publishes a new edition: that needs a fixture refresh
and no assert can see it. The fixture is a snapshot, so "guarded" means "no silent edit
to the snapshot", not "no new edition upstream".

## Status: gaps_found

The citation-only conclusion is right, the fixtures are honest, and the lost/gained guard
is real. The gap is that a claimed verifier fix landed as a false statement in the test
comment, `MILESTONES.md:89` still states the inverse, and this file claimed the count
assert was the unique falsifier. Closing action: the two one-line rewordings above; no
test behavior change is needed.

## Not re-run in this pass

`cargo test --workspace`, `uv run kutha-gov precommit` and `uv run kutha-gov ci --jobs 1`
were not re-run, so the previous "workspace 801 passed / 0 failed" and the governor exit
0 are inherited claims, not re-verified by this pass.
