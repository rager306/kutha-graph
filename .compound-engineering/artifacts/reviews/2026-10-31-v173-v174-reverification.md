# Re-verification — v1.73 (M180) and v1.74 (M181) after the claim audit

Fresh independent verifier, 2026-10-31. HEAD `89bc325` on `work/main-line`; working tree
clean before this pass. I did not write the reports, summaries, or tests under audit.

**Scope.** Confirm or refute the two HIGH findings of
`.compound-engineering/artifacts/research/2026-10-31-claims-audit-v170-v175.md` against the
code, then re-issue `266-VERIFICATION.md`, `267-VERIFICATION.md` and `268-VERIFICATION.md`.

**Verdict: both HIGH findings are confirmed.** Neither is a product defect; both are
verification claims that overstate what the tests prove. Finding 2 is narrower than the
audit states at HEAD, because commit `89bc325` already corrected the doc, `CHANGELOG.md`
and `268-01-SUMMARY.md`; the false claim survives in the test comment and
`.planning/MILESTONES.md`.

## Commands run and exit statuses

| command | exit | result |
|---|---|---|
| `cargo test --test m180_replay_meta` | 0 | 10 passed, 0 failed; the ten names are `valid_pack_with_manifest_replays`, `activate_before_install_fails_replay`, `duplicate_install_fails_replay`, `retire_unknown_pack_fails_replay`, `install_with_missing_manifest_blob_fails_replay`, `install_with_wrong_manifest_identity_fails_replay`, `enrichment_for_unknown_pack_fails_replay`, `duplicate_enrichment_cache_key_fails_replay`, `enrichment_missing_source_fails_replay`, `enrichment_wrong_version_and_retired_pack_fail_replay` |
| `cargo test --test m167_ecfr_pit` | 0 | 1 passed, 0 failed (`ecfr_pit_probe_reads_three_editions_from_real_text`) |
| `cargo run -q` in `tmp/v173-v174-verify/reason-probe` | 0 | replays the two enrichment logs and prints the actual replay error reason (output below) |
| `cargo test -q` in `tmp/v173-v174-verify/m167-count-removed` | 0 | 1 passed — count assert deleted, committed fixtures |
| `cargo test -q` in `tmp/v173-v174-verify/m167-edition-lost` | 101 | panic at the copy's `:147` (`assert_ne!`), lost 2017-03 edition |
| `cargo test -q` in `tmp/v173-v174-verify/m167-count-only` | 101 | panic at the count assert, `left: 1, right: 2`, same lost edition |

All scratch code lives in repo-local gitignored `tmp/v173-v174-verify/`; the repository
test files and `crates/` were not edited, and no commit, tag, or push was made. The two
required cargo runs used the existing `target/` (`CARGO_TARGET_DIR` unset); the scratch
crates used their own `tmp/v173-v174-verify/target` so they could not pollute it.

## Finding 1 (v1.73): the two enrichment tests are still vacuous — CONFIRMED

`crates/kutha-runtime/tests/m180_replay_meta.rs:164` and `:190` pass
`source_event_ids: vec![]`. `crates/kutha-runtime/src/enrichment.rs` orders the checks:

- `:96-107` empty/padded string fields,
- `:108-112` `EmptyField { field: "source_event_ids" }`,
- `:113-117` `UnknownPack`,
- `:130-135` `VersionMismatch`,
- `:136-142` `MissingSource`,
- `:143-152` blob presence and digest,
- `:155-159` `DuplicateProposal`.

Replay reaches this one function at `crates/kutha-runtime/src/provenance.rs:174`, so the
ordering applies to both gates. Both tests assert only
`Err(RuntimeError::EnrichmentRejected { .. })` (`:174-177`, `:223-226`) with no reason.

Measured, not inferred. The scratch probe rebuilds each test's event list and prints the
replay error:

```text
A  unknown_pack, empty source (as committed :149-178): EnrichmentRejected: replay: enrichment source_event_ids is empty
A' unknown_pack, real source (what a fixed test would use): EnrichmentRejected: replay: enrichment pack ghost is not installed
B  dup_cache_key, empty source (as committed :181-227): EnrichmentRejected: replay: enrichment source_event_ids is empty
B' dup_cache_key, real source (what a fixed test would use): EnrichmentRejected: replay: enrichment cache key same was already proposed
```

So both tests produce the empty-source error. `enrichment_for_unknown_pack_fails_replay`
never reaches `UnknownPack`, and `duplicate_enrichment_cache_key_fails_replay` never
reaches `DuplicateProposal`; the second duplicate proposal at `:220` is moot because the
first proposal at `:219` already fails.

**Mutations that would NOT be caught.** The two tests are insensitive to deleting the very
checks they are named for:

- delete `enrichment.rs:113-117` (the `UnknownPack` arm): the unknown-pack log still fails
  at `:108` with `EmptyField`, the test still matches `EnrichmentRejected { .. }` → green;
- delete `enrichment.rs:155-159` (the cache-key lookup): the duplicate-key log still fails
  at `:108` on its first proposal → green.

Any test that asserts only the error variant cannot distinguish these. The existing
`enrichment_missing_source_fails_replay` (`:229-277`) and
`enrichment_wrong_version_and_retired_pack_fail_replay` (`:280-350`) show the correct
shape: a real `source_event(0)` and a reason assertion (`:275`, `:348`).

**History check.** `git log -S'source_event_ids: vec![]' -- crates/kutha-runtime/tests/m180_replay_meta.rs`
names only `572809d`; the "fix" commit `cebd27d` added 148 lines to the file (two new
tests and the `provenance.rs:436` unit test) and left both originals untouched. The claim
"both now have real fixtures" was false when written and is false at HEAD.

**Coverage of the replay-side enrichment rejections at HEAD:**

| rejection | replay test |
|---|---|
| `MissingSource` | `m180:229-277`, reason "source event" |
| `RetiredPack` | `m180:336-349`, reason "retired" |
| `DigestMismatch` | `m183_digest_falsifiers.rs:132`, reason "do not match" |
| `VersionMismatch` | `m180:328-334`, variant only (real source) |
| `UnknownPack` | none — `m180:149-178` is the empty-source error |
| `DuplicateProposal` | none — `m180:181-227` is the empty-source error |
| `PackNotActive` | admission only (`m178_enrichment.rs:92-95`) |
| `MissingBlob` | none in either gate |

## Finding 2 (v1.74): the count assert cannot fail independently — CONFIRMED

`crates/kutha-runtime/tests/m167_ecfr_pit.rs:146` asserts `e0 == e1`, `:147` asserts
`e1 != e2`, and `:148-159` asserts the distinct-count is 2. The first two imply the count:
`{e0, e1, e2} = {e1, e2}` with `e1 != e2`, so the set has exactly two members. No triple
of texts passes both equality asserts with a count other than 2, so `:148` can never be
the first failing assert while `:146-147` are present.

Mutation evidence, on scratch copies of the test file with the committed fixtures:

1. count block deleted (`m167-count-removed`): exit 0, 1 passed. Removing the count assert
   is unobservable — the strongest form of "not an independent falsifier".
2. 304.1's 2017-03-14 fixture text replaced by its 2017-01-23 text, all asserts kept
   (`m167-edition-lost`): exit 101, panic at the copy's `:147`,
   `assertion left != right failed`. The lost edition is caught by the equality assert,
   before the count assert runs.
3. equality asserts deleted, count kept, same lost-edition fixture (`m167-count-only`):
   exit 101, `assertion left == right failed: section 0: fixture must keep exactly two
   distinct editions`, `left: 1, right: 2`. The count does catch a loss **on its own**.

So: a mutation that loses one edition does **not** pass the equality asserts, and the
count assert adds no detection while they are present. The audit's minimal fix is
validated: either keep the count and drop the equality asserts (variant 3 shows it then
carries both directions), or keep both and describe the count as a restatement.

**Where the false claim lives at HEAD.** `89bc325` ("correct v1.73-v1.75 claims after the
independent audit") already rewrote `docs/architecture/ecfr-edition-ambiguity.md:49-51`,
the v1.74 `CHANGELOG.md` entry, and `268-01-SUMMARY.md:12` to the correct wording, so the
audit's citations of those three are stale. Still false at HEAD:

- `crates/kutha-runtime/tests/m167_ecfr_pit.rs:141-145` — "This count assert is what
  catches a LOST edition", introduced by `a273326`;
- `.planning/MILESTONES.md:89` — "the count assert catches a lost edition, the equality
  asserts catch a gained one";
- `268-VERIFICATION.md:26` — replaced by this pass.

The v1.73 propagation the audit names is still in place: `CHANGELOG.md:108` ("both are now
real and prefix-scoped"), `.planning/MILESTONES.md:104-106`, `266-01-SUMMARY.md:11-13`
and `267-01-SUMMARY.md:11-13`. Those files are outside this verifier's edit set.

## Other corrections found while re-verifying

- **266 claims a pack-level "version mismatch" replay test.** The code rejects it
  (`pack_lifecycle.rs:144-163`, `activate_requires_same_installed_version`), but no replay
  test drives a wrong-version activate, a second activate, or a second retire; the
  admission-side equivalents are `m176_pack_lifecycle_schema.rs:47-73`.
- **"proposals a later checkpoint compacts away"** overstates the predicate. It is
  positional: `compacted = first_checkpoint.is_some_and(|cut| index < cut)`
  (`provenance.rs:168`) exempts every proposal before the first checkpoint, whether or not
  the rewrite dropped its source rows.
- **268's "raw XML for all eight part-304 sections" and "all nine texts reproduce
  byte-exactly"** cannot be re-checked: the fixture ships three sections × three dates
  (`m167:9-28`) and the raw XML lived in gitignored `tmp/ecfr-raw/`, which is gone. The
  citation-only conclusion is still supported by the effective-date record (FR 2017-00891,
  published 2017-01-23, effective 2017-03-14) and by the fixture pair being identical for
  editions 0/1 in all three committed sections.
- **The eight-section guard's scope.** The assertions fire when one of the nine committed
  texts changes. They cannot see a new edition published upstream; that needs a fixture
  refresh. This is inherent to a snapshot fixture, not a defect, and is now recorded in
  `268-VERIFICATION.md`.

## Re-issued verification files

| file | status | why |
|---|---|---|
| `266-VERIFICATION.md` | `gaps_found` | false "both now have real fixtures" sentence; pack-level version-mismatch and double-transition replay paths have no test |
| `267-VERIFICATION.md` | `gaps_found` | the two enrichment replay tests this phase is named for never reach `UnknownPack` / `DuplicateProposal` |
| `268-VERIFICATION.md` | `gaps_found` | citation-only conclusion holds and the lost/gained guard is real, but a claimed fix landed as a false comment and `MILESTONES.md:89` still states the inverse; both are one-line rewordings outside this verifier's edit set |

Each re-issued file keeps `independent: true` and the original frontmatter keys, names only
commands this pass ran with their exit statuses, states explicitly that
`cargo test --workspace`, `kutha-gov precommit` and `kutha-gov ci --jobs 1` were **not**
re-run here, and carries the corrections in place of the false sentences.

## Limits

- Fresh context, but the same model family as the original verifier
  (`cline/cline-pass/deepseek-v4.1-flash-1017` vs `codebuddy/deepseek-v4.1-flash`).
  Independence here means a fresh context that did not write the reports, not a different
  model.
- The workspace count (801) and the governor exit 0 were not re-run; this pass covers the
  audited claims and the two named test binaries.
- No `graph index`, no `kutha-gov` run, no network fetch. The `tmp/` probe crates are
  scratch evidence and are not part of the repository build.
