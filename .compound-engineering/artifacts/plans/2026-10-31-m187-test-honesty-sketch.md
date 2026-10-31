# M187 test-honesty milestone sketch (GSD-ready, planning only)

- **Status:** planning sketch, read-only. Not a lease, not a GSD artifact, not an ADR, not a capability claim.
- **Date:** 2026-10-31. Repo `/root/kutha-graph`, branch `work/main-line`, HEAD **`73e2c82`** (the task said `2680cd9`; the checkout moved — `2680cd9` is an ancestor and the built test binaries used below are newer than every `src/`/`tests/` file except `tests/fixtures/ecfr/README.md`).
- **Contract followed:** no tracked file edited, no commit, no `graph index`, no cargo build, no cargo run. Only this report and gitignored `tmp/task-l-m187/` were created. Binaries from `target/debug/deps/` (built 2026-10-31 14:14) were executed directly, which is how the measurements in §7 were taken without a build.
- **Lease:** none (`Active Milestone: None`, `.kutha/STATE.md:7-8`). Nothing here is authorized. M187 edits files under `crates/`, so it needs the lease in §2 before any worker starts.
- **Inputs:** `.compound-engineering/artifacts/research/2026-10-31-open-items-index.md` (defects 1–5), `.compound-engineering/artifacts/reviews/2026-10-31-v173-v174-reverification.md`, `.compound-engineering/artifacts/research/2026-10-31-test-health-audit.md`, `AGENTS.md`, `.kutha/STATE.md`, `.planning/ROADMAP.md` (last phase 274), `.planning/MILESTONES.md` (latest `Shipped:` v1.79), `.planning/REQUIREMENTS.md`, `.cursor/gsd-core/workflows/new-milestone.md`, `.cursor/gsd-core/workflows/plan-phase.md`, `.compound-engineering/artifacts/plans/2026-10-31-c2-vocabulary-milestone-sketch.md`.

## 1. Identity, and the v1.80 collision with C2

Two planning documents already claim the same ids. The open-items index (§ Decisions pending, item 1) assigns **M187** to test honesty; the C2 sketch (§1, §1.1, §7 O14) assigns **M187 / GSD v1.80 / phases 275–277** to C2 vocabulary. Both cannot stand.

**Recommendation: M187 test-honesty goes first and takes GSD v1.80; C2 becomes M188 / v1.81 / phases 277–279.**

| Item | Value | Basis |
|---|---|---|
| Harness milestone | **M187** (`S01`) | last closed is M186; the M-number/GSD-version offset is stable: M184↔v1.77, M185↔v1.78, M186↔v1.79 (`.planning/MILESTONES.md`) |
| GSD milestone | **v1.80**, name `Test honesty: replay oracles, scratch isolation, clock-free scaling` | `latest_completed_milestone` = v1.79, `phase_archive_path` = `.planning/milestones/v1.79-phases` |
| GSD phases | **275, 276** | last roadmap phase is 274 (`.planning/ROADMAP.md:13`); keep default numbering |
| Requirement ids | `TEST-275-01`, `TEST-275-02`, `TEST-276-01`, `TEST-276-02`, `TEST-276-03` | format precedent `PERF-274-01` (`.planning/REQUIREMENTS.md`, v1.79 block) |
| Thaw | **none** | every file in §3 is a `crates/*/tests/*.rs` file; no gate in the thaw map (`.kutha/STATE.md:34-44`) is touched, `ADR-050 six dictionaries` (`:39`) stays gated |

Why this order, not C2 first:

1. C2's Phase 276 gate runs `cargo test -p kutha-runtime --test m179_gate_gc` (C2 sketch, Phase 276 commands) — a file that defect 4 proves fails 5 of 6 times under concurrency (§7). Landing C2 first builds new pack tests on a scratch idiom already known to break.
2. C2's subtitle claim is pack install/activate/retire plus replay parity; defect 2 names exactly the replay paths (wrong-version activate, second activate, second retire) that have no oracle today (`pack_lifecycle.rs:144-163` has the rejections; no replay test drives them).
3. Test honesty is small, has no product change, and can close in one short pass; C2 is a three-phase product milestone.
4. A test-only milestone does not need a product capability to earn a GSD version: v1.79/M186 was a measurement milestone with no new capability.

**Numbering consequence if M187 goes first:** in `2026-10-31-c2-vocabulary-milestone-sketch.md`, §1 (Milestone/M187 → M188, v1.80 → v1.81, phases 275–277 → 277–279), §1.1 lease text (`M187`/`S01` → `M188`/`S01`), §7 O14, and every `DICT-275-01 … DICT-277-01` id become `DICT-277-01 … DICT-279-01`. Its Phase blocks keep their content and gate commands. **If the owner chooses C2 first instead**, test honesty becomes M188 / v1.81 / phases 278–280 and open-items-index item 1 must be rewritten — not recommended for the reasons above.

GSD artifacts the run will produce (schema from AGENTS.md § GSD artifact schema): `ROADMAP.md` gets `- [ ] **Phase 275: Replay and fixture falsifiers**` and `- [ ] **Phase 276: Scratch isolation and clock-free scaling**` under `## Phases` plus `### Phase 275: …` / `### Phase 276: …` under `## Phase Details`; `REQUIREMENTS.md` is rewritten milestone-scoped (v1.79's single requirement is archived at close); phase dirs `.planning/phases/275-replay-and-fixture-falsifiers/` and `…/276-scratch-isolation-and-clock-free-scaling/` hold `275-01-PLAN.md`, `275-02-PLAN.md`, `276-01-PLAN.md`, `276-02-PLAN.md` (frontmatter `wave: 1`, `depends_on: []`, `files_modified` = §3) and one `NNN-VERIFICATION.md` per phase with `status:` from `passed | gaps_found | human_needed`.

## 2. Exact `.kutha/STATE.md` lease text (parent controller pastes it; no GSD run does)

`Phase: H5` (`.kutha/STATE.md:9`) and `Release: 0.0.01` do not change.

```text
**Active Milestone:** M187
**Active Slice:** S01
**Phase:** H5
```

Lifecycle line inside the ```text block (today `L_delivery=M186-S01-closed`, `:16`) and its table row (`:23`):

```text
L_delivery=M187-S01
| L_delivery | M187 S01 leased (v1.80, test-only) | New capability / ADR Accepted / fixture refresh |
```

`## Next action` first line (`:26`):

```text
**Active Milestone:** M187 · **Active Slice:** S01 — test honesty only: the five open test defects (replay
vacuity, lifecycle replay gaps, the eCFR count restatement, fixed `tmp/` scratch collisions, wall-clock
scaling oracles) are closed inside `crates/kutha-runtime/tests/`. **Fence:** no file under
`crates/*/src/` is modified, no `Op`, no fixture `.txt`, no governor row, no `.kutha/dictionaries/` change.
If a fix needs `src/`, the slice STOPS and reports. Bench entry still recorded (§5). Thaws nothing; the thaw
map is unchanged and `ADR-050 six dictionaries` stays gated. Honeycomb stays Proposed; `I-legal-pack-not-next`
stays.
```

Thaw map: **no row added, no row edited.** Do not copy C2's `ADR-050` row into this lease — that row belongs to C2/M188 only.

## 3. Phases

Wave plan: all four plans are file-disjoint, so one wave, four workers with exclusive ownership; the parent owns integration, `graph index`, `.kutha/STATE.md`, `.planning/`, `CHANGELOG.md`.

### Phase 275 — Replay and fixture falsifiers (defects 1, 2, 3)

**275-01 · owned file: `crates/kutha-runtime/tests/m180_replay_meta.rs` · requirement `TEST-275-01`**

Goal: each replay falsifier drives the rejection it is named for and names that rejection.

| # | Site (HEAD) | Change | What the new assertion catches |
|---|---|---|---|
| 1 | `enrichment_for_unknown_pack_fails_replay`, `m180_replay_meta.rs:148-178` | build the log from `source_event(0)` (`:28-42`, the helper already exists) and pass `source_event_ids: vec![source.id]` at `:164`; assert `Err(EnrichmentRejected { reason }) if reason.contains("not installed")` (`enrichment.rs:44`) | today `enrichment.rs:108-112` rejects the empty source before `:113-117` can run, so the test passes for the wrong reason and is insensitive to deleting the `UnknownPack` arm (`enrichment.rs:113-117`). Measured reason at HEAD: `replay: enrichment source_event_ids is empty`; with a real source: `replay: enrichment pack ghost is not installed` (re-verification §Finding 1) |
| 2 | `duplicate_enrichment_cache_key_fails_replay`, `:180-227` | same real source, both proposals get `vec![source_id]` (`:190`); assert reason contains `already proposed` (`enrichment.rs:64`) | today both proposals die at `:108`; the duplicate lookup at `:155-159` and the post-success insert (`provenance.rs:181-183`) have no oracle. A mutation `if cache_keys.contains(..)` → `if false` must turn the test red |
| 3 | *(optional, same file)* `activate_before_install_fails_replay` `:72-85`, `retire_unknown_pack_fails_replay` `:101-114`, `install_with_missing_manifest_blob_fails_replay` `:117-124` | add reason asserts `activate_requires_same_installed_version` / `retire_requires_same_installed_version` (`pack_lifecycle.rs:158-161`) | these three assert the error variant only, so any earlier rejection satisfies them; cheap, adjacent, same file |

**275-02 · owned file: `crates/kutha-runtime/src` — none; `crates/kutha-runtime/tests/m180_replay_meta.rs` — add three tests (defect 2) and `crates/kutha-runtime/tests/m167_ecfr_pit.rs` (defect 3) · requirement `TEST-275-02`**

Defect 2 — three new `fn`s in `m180_replay_meta.rs`, each asserting `PackLifecycleRejected { reason }` with the reason text, against the same `install`/`at`/`source_event` helpers (`:14-42`):

| fn | log | expected reason |
|---|---|---|
| `activate_wrong_version_fails_replay` | source, manifest blob, `PackInstall{enrich,1.0.0}`, `PackActivate{enrich,2.0.0}` | `activate_requires_same_installed_version` (`pack_lifecycle.rs:159-161`) |
| `double_activate_fails_replay` | install 1.0.0, activate 1.0.0, activate 1.0.0 | `activate_requires_same_installed_version` (the `Installed`-only arm at `:149-151` makes the second activate illegal) |
| `double_retire_fails_replay` | install, activate, retire, retire | `retire_requires_same_installed_version` (`:148` allows only `Installed|Active`) |

Why it matters: the admission-side equivalents exist (`m176_pack_lifecycle_schema.rs:48` `lifecycle_ops_reject_invalid_ordering`, `:60-73`), but replay has no oracle for the three paths; the pack lifecycle is exactly the surface C2/M188 extends.

Defect 3 — `m167_ecfr_pit.rs`: **delete the distinct-count assert `:148-159`; keep `:146-147`**, and replace the comment `:141-145`. Reason, from measured mutations (re-verification §Finding 2): `:146-147` imply the count exactly (`e0==e1` and `e1≠e2` on three texts ⟹ two distinct), so the count can never fail while they pass (count deleted → still 1 passed, exit 0); a lost edition trips `:147` first (exit 101, `assertion left != right failed`). The count on its own also catches a loss (equal-assers deleted → exit 101, `left: 1, right: 2`), so keeping the equality pair and dropping the count loses **no** known detection and leaves the comment truthful. New comment: editions 0 and 1 are byte-identical because the 2017-01-23 amendment is citation-only; edition 2 is the only normative change; the count that used to sit here was implied by these two and was removed because it could not fail independently (evidence: v1.73–v1.75 re-verification, variant 1). No fixture `.txt` is touched (`fixtures/ecfr/*` stay byte-identical), and `AS_OF` (`:32-40`) plus the loop at `:129-140` remain the behavioural oracle.

Commands (each run from the crate dir; expected exit 0):

```text
cargo test -p kutha-runtime --test m180_replay_meta          # 13 fns after the change (10 + 3), all pass
cargo test -p kutha-runtime --test m167_ecfr_pit             # 1 fn
cargo test --workspace                                       # 0
uv run kutha-gov ci --jobs 1                                 # 0 with run_high=0
```

Verification mode: **independent verifier** (fresh context, `independent: true`), plus these manual mutations, pre/post recorded (there is no governor row, so `kutha-gov selftest` is not the mechanism):

| mutation | must turn red |
|---|---|
| `enrichment.rs:113-117` `UnknownPack` → `return Ok(())` | `enrichment_for_unknown_pack_fails_replay` |
| `enrichment.rs:155` `if cache_keys.contains(..)` → `if false` | `duplicate_enrichment_cache_key_fails_replay` |
| `pack_lifecycle.rs:149` allow `Active` for activate | `double_activate_fails_replay` |
| 304.1's `2017-03-14` fixture text replaced by its `2017-01-23` text (scratch copy only) | `m167` at `:147` |

### Phase 276 — Scratch isolation and clock-free scaling (defects 4, 5)

**276-01 · owned files: `m178_enrichment.rs`, `m179_gate_gc.rs`, `m185_enrichment_admission.rs` · requirement `TEST-276-01`**

Goal: no test persists into a CWD-relative fixed `tmp/` path.

| file | HEAD sites | change |
|---|---|---|
| `tests/m178_enrichment.rs` | `:199-206` (`ledger_survives_persist_open`), `:223-248` (`ledger_survives_checkpoint_sealed_prefix`) | add `mod common;` (none today; precedent `m167_ecfr_pit.rs:7`), `let dir = common::PersistScratch::new("m178-enrichment-persist" \| "…-checkpoint")`; delete the four `remove_dir_all` calls (`:200, :206, :224, :248`) |
| `tests/m179_gate_gc.rs` | `:102-115`, `:127-135` | same, labels `m179-gate-gc` / `m179-gate-gc-replay`; delete `:103, :115, :128, :135` |
| `tests/m185_enrichment_admission.rs` | `:242-244`, `:311` | same, one scratch `m185-checkpoint-source` reused for both persists and `checkpoint_sealed_prefix`; delete `:243, :311` |

`common::PersistScratch` already names dirs `{label}-{nanos}-{pid}` under `CARGO_MANIFEST_DIR/../..//tmp`, cleans on `Drop` (`common/mod.rs:13-33, 54-58`) and is used from 19 test files; `&PersistScratch` coerces to `&Path` for `persist`/`checkpoint_sealed_prefix`/`Runtime::open` (`m167_ecfr_pit.rs:205-208` is the working precedent). No production file changes; `common/mod.rs` is unchanged.

What the change catches / the falsifier: before the change, concurrent runs of one binary delete each other's store. Measured at HEAD (§7): 6 parallel copies of the single test → **5/6 failed** for each of the five tests, `StoreIo: No such file or directory` at `m178:201`, `m179:104`, and `store writer lock held (kutha.lock)` at `m185:244`. After the change the same probe must report **0/6**. Structural check: `rg -n 'PathBuf::from\("tmp/' crates/kutha-runtime/tests/` returns nothing.

**276-02 · owned files: `m182_meta_threats.rs`, `m183_digest_falsifiers.rs` · requirements `TEST-276-02` (oracles), `TEST-276-03` (close gates)**

Goal: no gate test asserts on wall-clock time; a test name states what it actually proves.

| test | HEAD | change | what it then catches / loses |
|---|---|---|---|
| `m182_meta_threats.rs:30-56` `replay_meta_is_not_quadratic` (4000 installs, `elapsed < 3 s`) | rename `replay_meta_uses_incremental_lifecycle_leases`; drop the clock; assert `rt.pack_lifecycle().len() == 4000` after `replay_check` and three source-shape anchors on `src/provenance.rs` (m184 pattern, `m184_admission_index.rs:247-267`): the file must **not** contain `PackLifecycleTracker::from_log` (today: absent) nor `derive_pack_lifecycle` (today: absent; the pre-v1.78 admission shape), and must **contain** `lifecycle.replay(&event.op)` (`provenance.rs:119`) | catches: a replay that rebuilds the lifecycle from the whole log per event, and a replay that drops the per-event ledger update (count). **Loses:** a semantically equivalent quadratic rewrite that keeps the incremental tracker is no longer detected — see §6 risk 1 |
| `m183_digest_falsifiers.rs:156-206` `manifest_replay_scales_linearly_with_packs` (400 vs 1600, ratio + 50 ms) | rename `manifest_replay_tracks_every_install_in_the_operator_index`; drop the clock; assert `rt.operator_count() == 1600` after replay (`pipeline.rs:596`), then a colliding install against `op0` is refused with `already declared` (m184 oracle shape, `m184_admission_index.rs:175-201`), plus the two absence anchors above | catches: a collision index that stops tracking installs, and a replay that drops retired-name release. **Loses:** the same quadratic caveat |

Both renames keep the old name and the T-269-01 / DIG-02 tags in the doc comment so the v1.75/v1.75 verification trail still resolves. The wall-clock numbers stay as a recorded measurement in the comment (m184's wording precedent, `m184_admission_index.rs:250-255`): measured today, `m182` whole binary 0.12 s serial and the 4000-op replay test 0.08 s wall; `m183` whole binary 0.47 s.

276-02 also carries the close-facing gates (no repository source):

```text
cargo test -p kutha-runtime --test m178_enrichment && --test m179_gate_gc && --test m185_enrichment_admission   # 0
cargo test -p kutha-runtime --test m182_meta_threats && --test m183_digest_falsifiers                          # 0
cargo test --workspace                       # 0
uv run kutha-gov ci --jobs 1                 # 0, run_high=0
uv run pytest                                # NOT required: no scripts/ change
```

Verification mode: independent verifier; falsifiers = the concurrency probe (6× parallel, expect 0/6 after), the `rg` structural check, and one mutation per renamed oracle (remove `operators.declare` from `provenance.rs:146` → the m183 replacement must fail; drop `known_ids.insert` at `provenance.rs:190` → the m180 reason tests must fail).

## 4. Test-only vs comments/fixtures; production behaviour

- **1, 2:** test-only. New event fixtures are built in-test from `source_event` (`m180_replay_meta.rs:28`), no committed fixture file.
- **3:** test body plus a comment in the same file. **No fixture `.txt` edit.** One out-of-`crates/` stale claim exists: `.planning/MILESTONES.md:89` ("equality asserts catch a lost or gained edition; the count restates them") — the last clause is right, the first is not measurable for a gained edition in a three-slot fixture. `.planning/` is parent-owned inside a GSD run; recommend leaving the v1.74 entry as shipped history and recording the correction in M187's close entry (AGENTS.md §9: dated history lives in `CHANGELOG.md`).
- **4:** test-only, plus three `mod common;` declarations.
- **5:** test-only, comment/name rewording. **Explicitly excluded:** the cleanest structural oracle for meta replay is an always-compiled examine counter of the `ingest_examine_count` family (`quantum.rs:508`), i.e. new `pub` API in `crates/kutha-runtime/src/`. That is a production change and must not ride inside a test-only lease — it is the named follow-up (§6).
- **Production behaviour: none of the five defects requires a `src/` edit.** Verified by reading the rejection order (`enrichment.rs:96-159`), the lifecycle rules (`pack_lifecycle.rs:116-168`) and the replay loop (`provenance.rs:100-197`); the tests are wrong about code that behaves correctly. If a worker's fix would need `src/`, the lease fence (§2) stops it and the finding goes back to the owner.

## 5. Bench gate

AGENTS.md:124 states: "**Bench gate:** every product milestone that edits `crates/` records pre/post `ingest_plain.n` and `as_of.n` in the close entry. A regression over 10% is a stop."

M187 edits `crates/kutha-runtime/tests/`, which is literally `crates/`, so the rule applies as written. Nothing in a test file is linked into `kutha-bench`, so the expected delta is zero and the stop line cannot be caused by this diff.

**Recommendation: record the entry** (do not rely on an exemption), with `KUTHA_BENCH_N=10000`, interleaved same-host samples, base = the commit the lease names, medians from sorted samples — and **3 samples per side instead of 6**, because the diff is test-only and the bench binary is unchanged. Record `ingest_plain.n`, `as_of.n`, plus `as_of.2n` and `ingest_keyed.n` as secondary (M186 showed those can move without being a gate). If a gate metric moves more than 10%, re-run before concluding (the M186 post-close audit found unsorted positional medians). An AGENTS.md wording change to exempt test-only diffs is an owner decision on a tracked doc and is **not** part of this lease.

## 6. Rollback, risks

**Rollback:** `git revert` of the milestone merge(s); no store, log, snapshot or migration is touched, because no `src/` file changes. A reverted tree simply restores the old oracles. Lease rollback is clearing `Active Milestone/Slice` back to `None` and `L_delivery` back to `M186-S01-closed`. There is no one-way door: no facts, no ids, no wire format.

| # | Risk | Mitigation |
|---|---|---|
| 1 | Defect 5's honest replacement cannot detect a *semantically equivalent* quadratic rewrite | state it in the close entry as a named residual; the follow-up is a meta examine counter (production change, own lease). Do not claim the new tests prove linearity |
| 2 | Source-text anchors (`m184` pattern) break on a legitimate refactor of `provenance.rs` | assert mostly on **absence** of the two quadratic shapes; the one presence anchor (`lifecycle.replay(&event.op)`) is the smallest possible |
| 3 | The m180 fix changes what two tests prove; if a hidden ordering assumption exists, they may expose a real product rejection bug | that would be a legitimate finding; the slice then stops and reports rather than editing `src/` (§2 fence) |
| 4 | Scope creep into the test-health audit's other items (`m019_dur_a` diagnostics: `assert_writer_lock_error` is at `m019_dur_a.rs:134` at this HEAD, the audit cited `:109` at an earlier commit; the stale process-global/thread-local comment at `m019_dur_a.rs:18`; `glob_none` scratch scope `.kutha/dictionaries/checks.yaml:226-229`) | out of scope: `glob_none` needs a governor-intake row, the others are unfixed-by-design until a failure is captured. Name them as follow-ups, do not edit |
| 5 | `mod common;` in three more test crates recompiles the shared module | already `#![allow(dead_code)]` (`common/mod.rs:2`); 19 files do it today |
| 6 | Adding three tests to `m180_replay_meta` changes the workspace test count cited in older entries | expected; record the new count in the close entry |

## 7. Evidence collected for this sketch (commands, no build)

| command | exit | result |
|---|---|---|
| `target/debug/deps/m182_meta_threats-84f39013d46db5c5 --test-threads=1` | 0 | 10 passed, 0 failed, finished 0.12 s |
| `…/m182_meta_threats replay_meta_is_not_quadratic --exact` | 0 | 1 passed, 0.08 s wall — 37× under the 3 s bound on an idle host |
| `…/m183_digest_falsifiers --test-threads=1` (whole) | 0 | 5 passed, 0.47 s |
| `…/m183_digest_falsifiers manifest_replay_scales_linearly_with_packs --exact` | 0 | 1 passed, 0.46 s wall |
| `…/m180_replay_meta --test-threads=1` | 0 | 10 passed, 0.02 s |
| 6× parallel `m178 ledger_survives_persist_open` | 5 of 6 non-zero | `m178_enrichment.rs:201: persist: StoreIo { No such file or directory }` |
| 6× parallel `m178 ledger_survives_checkpoint_sealed_prefix` | 5 of 6 non-zero | same class |
| 6× parallel `m179 toy_pack_installs_queries_retires_and_replays_without_core_edits` | 5 of 6 non-zero | `m179_gate_gc.rs:104` ENOENT |
| 6× parallel `m179 pack_retire_then_replay_is_parity_equal_to_reopen` | 5 of 6 non-zero | same class |
| 6× parallel `m185 checkpoint_tail_proposal_cannot_cite_a_dropped_source` | 5 of 6 non-zero | `m185_enrichment_admission.rs:244: store writer lock held (kutha.lock)` |
| serial re-run of each of the five | 0 | passes alone — so the failure is cross-run, not load |
| `find crates/kutha-runtime/{src,tests} crates/kutha-common/src -newer <m182 binary>` | 0 | only `tests/fixtures/ecfr/README.md`, so the binaries are current with sources |

Scratch `tmp/m178-enrichment-persist`, `tmp/m178-enrichment-checkpoint`, `tmp/m179-gate-gc`, `tmp/m179-gate-gc-replay`, `tmp/m185-checkpoint-source` left by the failing runs were removed afterwards; logs live in `tmp/task-l-m187/xrun/`.

## 8. What this sketch did not verify

The C2 sketch's phase content (only its id claims are used); whether `new-milestone` will accept a milestone whose requirements are all `TEST-*`; the exact workspace test count after the change; bench numbers (no bench run — the gate in §5 is the run that will produce them); whether `.planning/MILESTONES.md:89` should be corrected in place or only in the close entry (owner/GSD-writer decision).
