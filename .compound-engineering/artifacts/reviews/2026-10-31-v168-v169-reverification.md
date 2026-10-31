# Re-verification — v1.68 (M175) and v1.69 (M176) verification files

Scope: `.planning/milestones/v1.68-phases/{257,258}-*/{257,258}-VERIFICATION.md` and
`.planning/milestones/v1.69-phases/{259,260}-*/{259,260}-VERIFICATION.md`.
Input: `.compound-engineering/artifacts/research/2026-10-31-claims-audit-v160-v169.md`
(findings 1 and 2).
Verifier: fresh independent context, model `zai/glm-5.3`, at HEAD `a80e5fc` on
2026-10-31. This verifier did not write any of the four files before this pass.

Edits made: only the four VERIFICATION files above and this report. No commit, no
`graph index`, scratch in `tmp/v168-v169-verify/`.

**Verdict.** Both findings are confirmed. Finding 1 is a false API claim that survives
in both v1.68 files and in the code. Finding 2 is a real post-close rewrite of both
v1.69 files with the original PASS left in place; the counts are now pinned (762 at
close, 763 after `e9c5ff2`) and the missing re-verification is supplied by this pass.

## Finding 1 — `SecurityError::QueryRejected` is never constructed (CONFIRMED)

Commands and outputs:

- `rg -n 'QueryRejected' crates/` → two hits, both in
  `crates/kutha-runtime/src/security.rs`: line 62 (variant declaration) and line 74
  (the `Display` arm). No construction site anywhere in `crates/`.
- `security.rs:147-155`, `SecurityError::from_query_error`, maps every `QueryError`:

  | `QueryError` | mapped to |
  |---|---|
  | `UnknownRelation { name }` | `SecurityError::UnknownRelation { name }` |
  | `UnknownTerm { name }` | `SecurityError::PolicyDenied { reason }` |
  | `RelsShape { rels, k }` | `SecurityError::PolicyDenied { reason }` |

  So a query-engine rejection that happens *after* the policy allowed the path is
  reported as a policy denial. The claimed distinction ("keeping policy denials
  distinct") does not exist.
- No test drives the branch: `m175_security_integration.rs` has 4 tests and the only
  error assertion (`denied_gpml_returns_policy_error_not_empty_rows`) expects
  `PolicyDenied` for a denied relation. `m175_security_policy.rs` has 5 tests, none
  constructing `QueryRejected`.
- History: `924c592` is the only commit touching either v1.68 file, and it also adds
  `security.rs` and both test files. The "post-verification cleanup" paragraph was
  therefore written by the executor in the same commit as the report, not by the
  verifier.
- The documents were corrected at HEAD by `a80e5fc` (`CHANGELOG.md:181,188`,
  `.planning/MILESTONES.md:177` now say "reserved and never constructed"); the two
  verification files were not, until this pass.

Severity check: this is a taxonomy/API-honesty overclaim, not a capability claim. The
verified C4 behavior (closed-default denial before prepare, authorized execution equal
to direct execution) holds and its 9 targeted tests pass at HEAD.

## Finding 2 — v1.69 verification files were rewritten after the close (CONFIRMED)

`git log --format='%h %ad %s' --date=short -- <path>`:

```
259-VERIFICATION.md: d9f7ed3 2026-10-31 docs(gsd): record digest guard covers all pack ops
                     e9c5ff2 2026-10-31 test(runtime): make C1 checkpoint proof non-vacuous and guard digest
                     572fca4 2026-10-31 feat(runtime): add C1 pack lifecycle foundation and close v1.69
260-VERIFICATION.md: e9c5ff2 2026-10-31 test(runtime): make C1 checkpoint proof non-vacuous and guard digest
                     572fca4 2026-10-31 feat(runtime): add C1 pack lifecycle foundation and close v1.69
```

Close-time text (`git show 572fca4:<path>`) said "Schema 4/4; workspace 762 passed /
0 failed" (259) and "Runtime tests 5/5; workspace 762 passed / 0 failed" (260), with a
checkpoint test that persisted once and checkpointed immediately. At `572fca4` the
schema file has 4 `#[test]` items.

`e9c5ff2` added `pack_digest_separates_length_collisions` (schema test 5), made the
checkpoint runtime test non-vacuous (persist, six product Asserts, persist again,
checkpoint), and edited both verification bodies to 5/5 and 763 while leaving the
"Independent verifier PASSed" sentence untouched. `d9f7ed3` edited 259 again. No fresh
independent pass was recorded with those edits, which is what
`docs/process/gsd-auto-contract.md:224` requires for a `status: passed` verification.

Counts are real, and they belong to different trees:

| | close `572fca4` | post-fix `e9c5ff2` |
|---|---|---|
| schema tests | 4 | 5 (digest guard) |
| workspace | 762 / 0 | 763 / 0 |
| log | `tmp/m176-final-test.log` (762, no digest test) | `tmp/m176-final2-test.log` (763, digest test present) |
| checkpoint test | one persist, no sealed prefix (vacuous) | 8 events at a 2-event segment limit, Pack* sealed before checkpoint |

Both logs are gitignored scratch. Their tallies were re-counted here (762/0 over 176
`test result` lines; 763/0 over 176) and the digest test appears only in the second.
Neither log is durable evidence, so the four rewritten files cite them as context and
not under `read_only_inputs`.

`read_only_inputs` in all four files cite no `tmp/**` path, before or after this pass
(257/258 bodies mention `tmp/m175-final-test.log` once, now labelled as non-durable
inherited context).

Second, independent defect found while checking 260: its "replay does not revalidate
lifecycle ordering" paragraph was true at close but is false at HEAD. M180 (`572809d`)
added replay revalidation at `provenance.rs:118-134`, and the op-compat Pack* row now
says replay revalidates ordering and manifest identity. 260 now dates the limitation
and names the superseding commit.

## Targeted runs (this pass, at HEAD `a80e5fc`)

| command | result |
|---|---|
| `cargo test -p kutha-runtime --test m176_pack_lifecycle_schema --test m176_pack_lifecycle_runtime` | exit 0; schema 5 passed, runtime 5 passed, 0 failed |
| `cargo test -p kutha-runtime --test m175_security_policy --test m175_security_integration` | exit 0; policy 5 passed, integration 4 passed, 0 failed |
| `cargo test -p kutha-common` | exit 0 |
| `uv run kutha-gov precommit` | exit 0 (all checks OK, `high=0 low=0`) |

Logs: `tmp/v168-v169-verify/*.log`.

Not run: `cargo test --workspace`. HEAD is v1.79 with later milestones, so a HEAD
workspace count could not reproduce either 762 or 763; the workspace figures stay
inherited and are labelled as such in each file.

## Status decisions

| file | was | now | reason |
|---|---|---|---|
| `257-VERIFICATION.md` | passed | **gaps_found** | The report certified a `QueryRejected`/`PolicyDenied` distinction that the code does not implement and no test covers. C4 behavior itself is verified. Closing action: implement the mapping plus one `RelsShape` test, or delete the variant. |
| `258-VERIFICATION.md` | passed | **gaps_found** | Same false claim in its cleanup paragraph; same closing action. Integration behavior verified. |
| `259-VERIFICATION.md` | passed | **passed** | Post-close rewrite recorded; 762 = close count, 763 = post-fix count; this pass is the recorded re-verify; schema claims true at HEAD and tests pass. Caveat recorded: wrong-version *activate* is code-true but only exercised through *retire* in this phase. |
| `260-VERIFICATION.md` | passed | **passed** | Post-close rewrite recorded; checkpoint test is non-vacuous at HEAD; stale replay limitation dated and superseded by M180. Caveat recorded: first-install order across packs is not exercised by a multi-pack test. |

`independent: true` is kept in all four. `verifier_model` now records this pass's model
(`zai/glm-5.3`), not the original `zai/glm-5.3-flash` / `codebuddy/deepseek-v4.1-flash`.
The earlier verifier identity and its original counts are preserved in the body as
history, because the files no longer describe only one verifier's observations.

## Assumptions and limits

- "Fresh independent context" here means a different session and model instance from
  the executor and from the authors of the four files; no human or third-party model
  was involved.
- Test behavior was judged from the source and from running the four targeted binaries.
  No mutation test was performed, because this verifier may not edit `crates/`. The
  non-vacuity of the checkpoint test is read from the code instead: `split_events`
  (`segment.rs:924-948`) chunks at the 2-event test limit and `persist_log` (`:955-975`)
  seals every chunk except the last, so eight events give sealed segments and two events
  give one unsealed chunk.
- The pre-`e9c5ff2` checkpoint test is called "vacuous" on the same reading: one persist
  of two events produces a single unsealed chunk, so there is no sealed prefix to copy
  and the assertion passes regardless of the prefix-copy fix.
