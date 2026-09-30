---
phase: 11-semantic-governor
verified: 2026-09-30T11:20:41Z
status: passed
score: 8/8 must-haves verified
covered_files:
  - .env.example
  - .kutha/META.md
  - .kutha/ROADMAP.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - .kutha/dictionaries/invariants.yaml
  - .planning/phases/11-semantic-governor/11-01-PLAN.md
  - .planning/phases/11-semantic-governor/11-01-SUMMARY.md
  - .planning/phases/11-semantic-governor/11-02-PLAN.md
  - .planning/phases/11-semantic-governor/11-02-SUMMARY.md
  - .planning/phases/11-semantic-governor/11-03-PLAN.md
  - .planning/phases/11-semantic-governor/11-03-SUMMARY.md
  - .planning/phases/11-semantic-governor/11-04-PLAN.md
  - .planning/phases/11-semantic-governor/11-04-SUMMARY.md
  - AGENTS.md
  - CHANGELOG.md
  - docs/process/governor-intake.md
  - docs/process/kutha-harness.md
  - scripts/kutha_gov/__main__.py
  - scripts/kutha_gov/fsm.py
  - scripts/kutha_gov/kinds.py
  - scripts/kutha_gov/rust_source.py
  - scripts/kutha_gov/selftest.py
  - scripts/tests/test_fsm.py
  - scripts/tests/test_kutha_gov.py
  - scripts/tests/test_selftest.py
covered_digest: "v2:sha256:cb716a1aff82e09c52ae13fb1f73b51f2af627ed2faef5362f77f945559804a2"
behavior_unverified: 0
overrides_applied: 0
---

# Phase 11: Semantic governor Verification Report

**Phase Goal:** The governor checks semantic truth — vacuity, test assertions, lease citations, evidence links, ADR references, and context size — and records product gaps F1–F8 as deferred invariants so Phases 9–10 cannot regress silently
**Verified:** 2026-09-30T11:20:41Z
**Status:** passed
**Re-verification:** No — initial verification

Independent evidence only. SUMMARY.md claims were treated as hypotheses. Kill-tests ran in `git worktree add /tmp/kutha-verify HEAD` (`300f297`); the worktree was removed afterwards. The checkout working tree was not used as a mutation target.

## Goal Achievement

### Observable Truths

Roadmap success criteria 1–8 (SEM-01…08). PLAN frontmatter truths restated these and were folded in.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `uv run kutha-gov selftest` mutates a temporary copy of each mutable YAML check and observes a HIGH; checks that cannot be mutated are listed with a reason (SEM-01) | ✓ VERIFIED | Live `selftest`: 34 `OK` + `SKIP docs-coupling reason=depends on the live git diff`; exit 0. `selftest.py` copies once (`TemporaryDirectory` + `copy_harness_tree`), never writes `root`. `git_path_implies` is the only skip. Unproven/vacuous fail closed (`test_empty_file_contains_needles_is_vacuous`, `test_synthetic_unproven_pointer_only`). |
| 2 | Kind `rust_test_asserts` proves named Rust tests are non-vacuous; former `fn …` needle checks use it (SEM-02) | ✓ VERIFIED | `kinds.py` `_kind_rust_test_asserts` + `rust_source.py` brace match after noise strip. Converted rows: `h4-membership-as-of`, `m010-semantic-open`, `m011-*`, `observe-required-fn`, `honeycomb-ledger`. Leftover `"pub fn csr_lease("` is `file_absent` (source needle), not a test-name check. Pytest red paths: missing / ignored / no-assert / comment-string. Kill-tests D1/D2 HIGH on `m011-e2e`. |
| 3 | `cite_equals` fails when a doc lease citation disagrees with `.kutha/STATE.md` (SEM-03) | ✓ VERIFIED | `cite-lease` four `cite_equals` steps (`L_delivery`, `L_map`, `L_capability`, Active Milestone). Cite globs exclude `CHANGELOG.md` and `.planning/`. Kill-test A: append `L_delivery=M011-S03-done` to `README.md` → HIGH `cite-mismatch` (`M011-S03-done` vs `M011-S08-done`). |
| 4 | `honeycomb.yaml` `evidence` and `capability: named` resolve or CI fails (SEM-04) | ✓ VERIFIED | Two `capability: named` cells (ADR-011, ADR-013). `honeycomb-ledger` has `nonempty_list: evidence` plus `rust_test_asserts` `names_from_yaml` + `require_evidence_when`. Kill-test E: ADR-011 `evidence: []` → 2 HIGH. Kill-test F: rename `ff6_unknown_relation_does_not_append` → HIGH unresolved evidence. |
| 5 | Every `ADR-NNN` and `D1`–`D10` reference in the D-X1 scope resolves; dangling fails (SEM-05) | ✓ VERIFIED | `refs-resolve` scans `AGENTS.md`, `README.md`, `.kutha/**/*.md`, dictionaries YAML, `docs/process/*.md`; `allow_dangling: [ADR-100]`. Kill-test B: `See ADR-999.` on `AGENTS.md` → HIGH `refs-adr`. Pytest covers range endpoints, `D050-2` non-locks, ADR-100 allow. |
| 6 | `AGENTS.md` has an enforced size budget and forbids lease tokens (SEM-06) | ✓ VERIFIED | `agents-lean`: `file_max_lines` max 110 + `file_absent` of `L_delivery=`, `L_map=`, `L_capability=`, `M011 is active`. Live `wc -l` = 101. No lease-assignment tokens in `AGENTS.md`. Kill-test C: 121 lines → HIGH `max-lines`. |
| 7 | Product gaps F1–F8 appear as `disposition: deferred` invariants with `until`; recorded, not enforced (SEM-07) | ✓ VERIFIED | Eight rows `I-F1-outcomes` … `I-F8-hot-reads` in `invariants.yaml`: each `disposition: deferred`, each has `until`, source `docs/architecture/semantic-gap-review.md`. No `check:` field (not enforced). No `docs/ADR/ADR-` substring in `invariants.yaml`. |
| 8 | Each new kind has a red-path pytest; META and governor-intake document intake; Process CHANGELOG records the change; `ci` stays 0 HIGH (SEM-08) | ✓ VERIFIED | Pytest 98 passed. META rows for `rust_test_asserts`, `cite_equals`, `refs_resolve`, `file_max_lines`, FSM `run_selftest`. Intake section “What a green check proves”. CHANGELOG 2026-09-30 Process/Trajectory entries (H5, AGENTS diet, semantic-gap review, semantic governor). `rg` of `Narrative remains Phase 11 SEM-08` in `CHANGELOG.md` = 0. Live `ci`: 0 HIGH, 35 checks, `h5_selftest=35/35`, harness line `(H5 dogfood)`, no `cui: truncated`. |

**Score:** 8/8 truths verified (0 present, behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `scripts/kutha_gov/selftest.py` | Copy-mutate-restore vacuity runner | ✓ VERIFIED | 501 lines; `run_selftest` copies once; restore in `finally`. |
| `scripts/kutha_gov/rust_source.py` | Brace-matched `#[test]` body parse | ✓ VERIFIED | Strips comments/strings/chars; `assert_hits` / `parse_fns`. |
| `scripts/kutha_gov/kinds.py` | Four new kinds in `ALLOWED_KINDS` + `RUNNERS` | ✓ VERIFIED | `rust_test_asserts`, `cite_equals`, `refs_resolve`, `file_max_lines`. |
| `scripts/kutha_gov/fsm.py` | `run_selftest` in `ALLOWED_STATE_KINDS` | ✓ VERIFIED | Records `h5_selftest=proved/total`; HIGH on non-SUCCESS rows. |
| `scripts/kutha_gov/__main__.py` | CLI `selftest`; rung H5 from evidence | ✓ VERIFIED | `h5_selftest` overrides H4 cargo heuristic. |
| `.kutha/dictionaries/fsm.yaml` | `run_selftest` after `run_checks`; budget 48 | ✓ VERIFIED | Transition `run_checks → run_selftest → observe_cargo`. `.env.example` `KUTHA_GOV_BUDGET=48`. 35 checks < 48. |
| `.kutha/dictionaries/checks.yaml` | New kinds + `selftest:` skip/mutations | ✓ VERIFIED | `docs-coupling` skip reason present; 4 declared `selftest.mutations` blocks; rest derived. |
| `.kutha/dictionaries/invariants.yaml` | I-F1…I-F8 + I-cite/refs/agents | ✓ VERIFIED | Deferred F-rows; new check dispositions wired. |
| `.kutha/dictionaries/honeycomb.yaml` | Named evidence lists | ✓ VERIFIED | ADR-011/013 `capability: named` with non-empty evidence. |
| `.kutha/META.md` | Kind + FSM allowlist rows | ✓ VERIFIED | All four kinds + `run_selftest`. |
| `docs/process/governor-intake.md` | “What a green check proves” | ✓ VERIFIED | Presence ≠ meaning; skip is not a silent pass. |
| `docs/process/kutha-harness.md` | H5 marked done | ✓ VERIFIED | Ladder row **Done**; “H5 is delivered”. |
| `.kutha/ROADMAP.md` | H5 checkbox checked | ✓ VERIFIED | `- [x] **H5**`. `h4-lease` requires that needle. |
| `AGENTS.md` | selftest command ≤110 | ✓ VERIFIED | Line 56; 101 lines. |
| `CHANGELOG.md` | Real Process/Trajectory | ✓ VERIFIED | Four dated 2026-09-30 Process entries with Trajectory. |
| `scripts/tests/test_selftest.py` | VACUOUS/UNPROVEN/live OK | ✓ VERIFIED | Empty-needles VACUOUS; live-repo green; copy excludes `.git`/`target`. |
| `scripts/tests/test_kutha_gov.py` | Red paths for new kinds | ✓ VERIFIED | Missing/ignored/no-assert; cite mismatch; ADR-999; over-max lines. |

**Artifacts:** 17/17 verified

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `kutha-gov selftest` | tempfile copy of `ctx.root` | mutations only under copy | ✓ WIRED | `run_selftest` uses `TemporaryDirectory`; `test_selftest_freeze_ok_leaves_tracked_harness_bytes`. |
| `fsm.yaml` `run_selftest` | `cmd_ci` harness line | `h5_selftest` | ✓ WIRED | Live `ci` printed `evidence: h5_selftest=35/35` and `(H5 dogfood)`. |
| `checks.yaml` `selftest` | derived mutations | fail-closed UNPROVEN | ✓ WIRED | `_prove_check` UNPROVEN when neither derivable nor declared. |
| `rust_test_asserts` names | `crates/**/*.rs` fn items | attribute block + body asserts | ✓ WIRED | Kill-tests D1 (`#[ignore]`) and D2 (body `let _x = 1`) HIGH. |
| `honeycomb.yaml` evidence | `names_from_yaml` | empty named evidence HIGH | ✓ WIRED | Kill-tests E and F. |
| `.kutha/STATE.md` captures | `cite-lease` cite globs | mismatch HIGH | ✓ WIRED | Kill-test A. |
| ADR-NNN / D-lock tokens | `docs/ADR/ADR-*-*.md` + honeycomb locks | `allow_dangling` ADR-100 | ✓ WIRED | Kill-test B. |
| `AGENTS.md` line count | `file_max_lines` 110 | `agents-lean` | ✓ WIRED | Kill-test C. |
| ROADMAP H5 checkbox | `h4-lease` needle | id kept; description renamed | ✓ WIRED | Needles `- [x] **H4**` and `- [x] **H5**`. |
| F1–F8 review note | deferred invariants | source without ADR-file substring | ✓ WIRED | `source: docs/architecture/semantic-gap-review.md`. |

**Wiring:** 10/10 verified

### Data-Flow Trace (Level 4)

No UI. Governor findings flow from files on `ctx.root` through kind runners to `CheckResult.findings`. Live `ci`/`precommit` printed real HIGH=0 rows (not a static stub). Kill-test mutations changed those files and produced HIGH on the named check.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `cite-lease` | capture groups | `.kutha/STATE.md` + cited docs | Yes — mismatch used live STATE `M011-S08-done` | ✓ FLOWING |
| `honeycomb-ledger` | evidence names | `honeycomb.yaml` + `crates/**/*.rs` | Yes — rename/blank produced named HIGH | ✓ FLOWING |
| `run_selftest` | `h5_selftest` | `run_selftest()` row statuses | Yes — `35/35` on live tree | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Live selftest | `uv run kutha-gov selftest` | 34 OK + 1 SKIP; exit 0 | ✓ PASS |
| Dictionary precommit | `uv run kutha-gov precommit` | 35 OK; high=0 | ✓ PASS |
| Harness pytest | `uv run pytest -q scripts/tests` | 98 passed | ✓ PASS |
| Python tooling | `uv run kutha-gov py` | `PYTHON_TOOLING_GREEN` | ✓ PASS |
| Full CI quantum | `uv run kutha-gov ci` | 0 HIGH, 35 checks, H5, no `cui: truncated` | ✓ PASS |

### Kill-tests (isolated worktree `/tmp/kutha-verify` @ `300f297`)

Each mutation was applied, one named check was run via `uv run --project /root/kutha-graph kutha-gov --root /tmp/kutha-verify --check <id> …`, then `git checkout -- .` restored the worktree. Working tree was not mutated.

| ID | Mutation | Named check | Result | Status |
| -- | -------- | ------------ | ------ | ------ |
| A | Append `L_delivery=M011-S03-done` to `README.md` | `cite-lease` / `cite_equals` | HIGH 1: `README.md cites 'M011-S03-done' but .kutha/STATE.md is 'M011-S08-done'` | ✓ HIGH |
| B | Append `See ADR-999.` to `AGENTS.md` | `refs-resolve` | HIGH 1: `AGENTS.md cites ADR-999 with no matching ADR file` | ✓ HIGH |
| C | Pad `AGENTS.md` to 121 lines | `agents-lean` | HIGH 1: `AGENTS.md has 121 lines (wc -l), max 110` | ✓ HIGH |
| D1 | `#[ignore]` above `e2e_fixture_supports_and_conflict_at_named_cuts` | `m011-e2e` / `rust_test_asserts` | HIGH 1 at `m011_e2e_fixture.rs:174` | ✓ HIGH |
| D2 | Replace that test body with `let _x = 1;` (no `assert…!`) | `m011-e2e` / `rust_test_asserts` | HIGH 1 at `m011_e2e_fixture.rs:173` (step message overlays “missing”; finding is on the still-present `#[test]` fn) | ✓ HIGH |
| E | Blank ADR-011 `evidence: []` (`capability: named`) | `honeycomb-ledger` | HIGH 2: `capability=named cells missing evidence tests: ADR-011` and `evidence names missing … ADR-011` | ✓ HIGH |
| F | Rename `fn ff6_unknown_relation_does_not_append` in `ff6_allowlist.rs` | `honeycomb-ledger` | HIGH 1: `evidence names missing as asserting #[test]: ff6_unknown_relation_does_not_append` | ✓ HIGH |
| G | Add check `empty-needles` with `file_contains` / `needles: []` | `selftest --check empty-needles` | `VACUOUS empty-needles mutations=0 reason=derivable steps produced no mutation`; exit 1 | ✓ VACUOUS |

### Probe Execution

| Probe | Command | Result | Status |
| ----- | ------- | ------ | ------ |
| — | — | Phase does not declare `scripts/*/tests/probe-*.sh` | SKIP |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ----------- | ----------- | ------ | -------- |
| SEM-01 | 11-01 | Vacuity selftest | ✓ SATISFIED | Live selftest + pytest VACUOUS/UNPROVEN + FSM `run_selftest` |
| SEM-02 | 11-02 | Non-vacuous named Rust tests | ✓ SATISFIED | Kind + converted checks + D1/D2 |
| SEM-03 | 11-03 | Lease citations equal STATE | ✓ SATISFIED | `cite-lease` + kill-test A |
| SEM-04 | 11-02 | Honeycomb evidence / named capability | ✓ SATISFIED | `honeycomb-ledger` + E/F |
| SEM-05 | 11-03 | ADR/lock refs resolve | ✓ SATISFIED | `refs-resolve` + kill-test B |
| SEM-06 | 11-03 | AGENTS budget + no lease tokens | ✓ SATISFIED | 101 lines; kill-test C |
| SEM-07 | 11-04 | Deferred I-F1…I-F8 | ✓ SATISFIED | Eight `deferred` + `until` rows |
| SEM-08 | 11-01…11-04 | Tests, META, intake, CHANGELOG, ci 0 HIGH | ✓ SATISFIED | pytest/py/ci + docs |

No orphaned Phase 11 requirements. REQUIREMENTS.md maps SEM-01…08 only to this phase.

### Freeze / drift guards (requirement 3)

| Check | Result |
| ----- | ------ |
| `git diff --exit-code 76da78d..HEAD -- crates` | exit 0 (no crate edits) |
| `git diff --exit-code 76da78d..HEAD -- .cursor/rules` | exit 0 |
| `git diff --numstat 76da78d..HEAD -- docs/ADR` | 13 files, 58 insertions, **0 deletions** (Phase 10 dated Proposed notes + README pointer) |
| `git diff 76da78d..HEAD -- .kutha/STATE.md` | Only Phase 9 H5 lease: `Phase: H4` → `H5`; Next-action paragraph retargeted to harness/docs rung. Lifecycles unchanged (`L_delivery=M011-S08-done`). |
| H5 `- [x]` in `.kutha/ROADMAP.md` | present |
| CHANGELOG Phase 11 SEM-08 pointer stub | `rg` count 0 in `CHANGELOG.md` |
| SemVer / tag / `gh release` in Phase 11 CHANGELOG entries | absent. Older entries still say versions stay `0.0.0` and CHANGELOG is not GitHub Releases (standing policy, not a bump). |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | No `TBD` / `FIXME` / `XXX` in phase Python | — | — |

**Anti-patterns:** 0 blockers

Phase test files contain `#[ignore]` only inside synthetic fixtures that assert the ignore detector (`test_rust_test_asserts_high_when_ignored`). No skipped requirement-linked tests.

### Test Quality Audit

| Test File | Linked Req | Active | Skipped | Circular | Assertion Level | Verdict |
|-----------|-----------|--------|---------|----------|-----------------|---------|
| `scripts/tests/test_selftest.py` | SEM-01 | yes | 0 | no | Behavioral (VACUOUS/UNPROVEN/live OK) | OK |
| `scripts/tests/test_kutha_gov.py` | SEM-02…06 | yes | 0 | no | Value + behavioral red paths | OK |
| `scripts/tests/test_fsm.py` | SEM-01/08 | yes | 0 | no | FSM budget / kinds | OK |

**Disabled tests on requirements:** 0
**Circular patterns detected:** 0
**Insufficient assertions:** 0 (independent kill-tests also exercise the live YAML rows)

### Decision Coverage

CONTEXT `<decisions>` D-V1…D-V3, D-T1/D-T2, D-E1, D-C1, D-X1, D-B1, D-D1, D-Q1…D-Q4, D-G1…D-G3 are present in shipped kinds, dictionaries, docs, and tests. One documented alternative: D-T1 allowed assert-strip **or** `#[ignore]` insertion; implementation uses `#[ignore]` only for derived selftest (see observations). Non-blocking.

### Human Verification

N/A — Infrastructure/foundation phase with no user-facing elements.
All acceptance criteria are verifiable programmatically (CLI, pytest, isolated kill-tests).

### Residual observations (non-blocking)

These do not fail a requirement. They are honesty notes about what the proofs do **not** show.

1. **Mutation selftest proves a check can fail once, not every step.** `_run_mutations` requires ≥1 HIGH per mutation unit. A multi-step check can stay `OK` after proving a single derived step. `rust_test_asserts` derived mutation inserts `#[ignore]` on the **first** named test then `break`s — sibling names in the same step are not mutation-proven.
2. **Derived `#[ignore]` does not prove the assertion detector.** `selftest.py` never strips `assert…!(`. The no-assert path is proven by pytest (`test_rust_test_asserts_high_when_no_assert_in_body`) and by kill-test D2 on the live `m011-e2e` row, not by `kutha-gov selftest`.
3. **`docs-coupling` is skipped** (`depends on the live git diff`). Vacuity of `git_path_implies` is not mutation-proven. Declared skip with reason; SEM-01 allows this.
4. **CLI category/message overlay.** `m011-e2e` sets `category: m011-s08` and message `… missing {missing}`, so ignored vs vacuous vs missing print the same sentence. Kill-test D2 still HIGH’d on the existing `#[test]` line after the body became `let _x = 1`.
5. **I-F1…I-F8 are recorded, not enforced.** A product crate can still implement the gaps; the governor will not HIGH. That is SEM-07 / Out of Scope, not a miss.
6. **`capability: none` cells may list evidence** (e.g. ADR-012, ADR-014). SEM-04 only fail-closes **named** capability with empty/unresolved evidence. Extra evidence names still go through `names_from_yaml` and must exist.

### Gaps Summary

**No gaps found.** Phase goal achieved. Ready to proceed.

---

## Verification Metadata

**Verification approach:** Goal-backward from ROADMAP success criteria 1–8 + SEM-01…08
**Must-haves source:** `.planning/ROADMAP.md` Phase 11 (PLAN frontmatter merged, no scope reduction)
**Automated checks:** live selftest/ci/precommit/pytest/py + 8 isolated kill-tests
**Human checks required:** 0
**Code-graph claims:** none

---

_Verified: 2026-09-30T11:20:41Z_
_Verifier: Claude (gsd-verifier)_
