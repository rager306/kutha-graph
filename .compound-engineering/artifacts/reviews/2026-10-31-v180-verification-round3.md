# v1.80 verification, round 3 — phases 275 and 276

Independent verifier round 3, fresh context, no part in the implementation.
HEAD `bf96497` on `work/main-line`. Scope: re-verify TEST-276-02 (the scaling
oracles must measure the product replay gate), carry the 275 verdict, and
rewrite both VERIFICATION files inside the 60-line artifact budget.

## Verdict

| Phase | Status | Basis |
|-------|--------|-------|
| 275 | `passed` | tests read and green this round; mutant exits inherited from round 2 |
| 276 | `passed` | round-2 gap on TEST-276-02 closed and falsified here |

## Evidence run this round

| Command | Exit | Result |
|---------|------|--------|
| `cargo test -p kutha-runtime --test m182_meta_threats --test m183_digest_falsifiers` | 0 | 10 passed, 5 passed |
| 11-binary v1.80 targeted run | 0 | 74 passed, 0 failed |
| m182 `replay_meta_is_not_quadratic`, mutant | 101 | count 8002000 vs 4000 |
| m183 `manifest_replay_scales_linearly_with_packs`, same mutant | 101 | count 160400 vs 400 |
| m175 `post_policy_rels_shape_failure_is_query_rejected`, mutant | 101 | got `PolicyDenied`, want `QueryRejected` |
| restores rerun | 0 | 10 + 5 green; m175 7/7 green; sha256 equal |
| `uv run kutha-gov precommit` | 0 | `artifact-budget`, `verification-independent`, `cite-lease` all high=0 |

`cargo test --workspace` was NOT run by this verifier; the parent owns it. The
round-2 record is a green rerun after a first-run flake in `m019_dur_a`
(`kutha.lock` writer lock), documented in
`.compound-engineering/artifacts/research/2026-10-31-test-health-audit.md`.

## TEST-276-02 — the oracle now sees the product path

The fix is the right seam. `PackLifecycleTracker::replay` increments a
thread-local `REPLAY_CALLS`, so a tracker rebuilt from a prefix cannot reset the
count; `Runtime::meta_replay_probe` resets, calls the very `check_meta_replay`
that `replay_check` calls (same function, not a copy), and returns
`(replays, declarations)`. m182 asserts `replays == 4000`, m183 asserts
400/1600 replays and declarations plus `4x packs == 4x declarations`. Neither
test references `Instant`, `elapsed` or `Duration`: the wall-clock pair from
round 2 is gone, so the oracles are structural.

Mutant (the documented pre-fix shape): in `provenance.rs`, replace
`lifecycle.replay(&event.op)?;` with
`lifecycle = crate::PackLifecycleTracker::from_log(&events[..=index])?;`.
`replay_meta_is_not_quadratic` fails at its own assert with
**left 8002000 / right 4000** after ~30 s (sum over 4000 prefixes), exit 101.
m183 fails at **160400 vs 400**: each rebuild replays every event in the prefix
including the `StoreBlob` no-ops, because `replay` increments before `check`
discards non-lifecycle ops. That 2x-per-install factor is expected arithmetic,
not a leak. Both oracles therefore catch the quadratic rebuild, which is exactly
what round 2 proved they could not.

## Verification-honesty trap found (new this round)

The first "restore then green" rerun still failed with the mutant numbers:
`shutil.copy2` restores the backup's original mtime, so cargo's fingerprint saw
an unchanged source and reused the mutant build. Bumping the mtime (`os.utime`)
fixed it. Any mutant/restore protocol that copies a file back must touch it, or
the green run is a lie. Recorded in the harness memory for future verifier
rounds.

## TEST-276-01 and TEST-276-03

TEST-276-01 re-checked first hand: no `"tmp/` or `format!("tmp` literal anywhere
under `crates/`; the 38 `.join("tmp")` sites carry a pid/nanos/seq suffix or
only canonicalize a root; `common::PersistScratch::new` uses
`{label}-{nanos}-{pid}`; 13 `scratch()` call sites across m176/m177/m178/m179/
m184/m185. Governor `fixed-tmp-persist-dir` passed, high=0. The concurrent-pair
collision falsifier is the round-2 record, inherited.

TEST-276-03 re-checked in source and by mutant: `security.rs::from_query_error`
maps `UnknownTerm` and `RelsShape` to `QueryRejected` and keeps
`UnknownRelation`. Reverting the `RelsShape` arm to `PolicyDenied` fails the
named test (exit 101). Restore sha256-equal, 7/7 green.

## Artifact and gate state

Both VERIFICATION files were over budget (132 and 176 lines) and listed plan
paths in `read_only_inputs`; both checks were the only two HIGH findings in
`precommit` before the rewrite. After the rewrite both are within 60 lines,
carry `independent: true`, `verifier_model`, `commands` with exits, and
`read_only_inputs` of tracked code and docs only; `precommit` exits 0 with
`artifact-budget`, `verification-independent` and `cite-lease` clean. Round 2's
`cite-lease`/`state-readme` failures are gone: README now matches the lease.

## Residual risks

- The harness gate for a product close is `kutha-gov ci --jobs 1` plus
  `cargo test --workspace`; neither was run here (parent owns both).
- `m019_dur_a` still flakes on the writer lock; a close run can land red.
- The 60-line budget forces the mutant detail into this review file; the
  VERIFICATION files cite it rather than restate it.
