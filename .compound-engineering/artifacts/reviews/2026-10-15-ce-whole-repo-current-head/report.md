## Code Review Results

**Scope:** standalone branch diff `chore/m013-roadmap-reset` vs merge-base `1d85af8` (main); 990 tracked files, 76218 changed lines, 15228 exec non-test lines; reviewed head `5dd5338`; working tree is the reviewed head except the corrected historical review artifact and one untracked CE handoff (both excluded).
**Intent:** Close milestones M023-M090 (STATE: L_delivery=M090-closed) and fix prior CE-review findings (2026-10-07/10-09/10-11) on persist/open recovery, Horn semantics, and governor silent-pass oracles, with killed matrix oracles per docs/process/ce-review-regression-map.md. Must not regress durability/semantic oracles, matrix honesty, or lease/honeycomb discipline; honeycomb stays Proposed.
**Mode:** markdown report-only

**Reviewers:** CorrectnessHorn, PersistReliability, GovernorAdversarial, PerfContractStructure, TestingOracles, StandardsSecurityAgent
- CorrectnessHorn -- Horn-clause firing/retraction semantics changed across M050-M056
- PersistReliability -- store migrate/open/heal paths touched by M067/M072/M076/M078
- GovernorAdversarial -- governor dictionary loader is a silent-pass boundary
- PerfContractStructure -- bench RSS accounting and retract/admission scaling shapes
- TestingOracles -- matrix oracle honesty (killed-mutation backing) per regression map
- StandardsSecurityAgent -- lease/honeycomb discipline, allow-path handling

**Preliminary fast pass:** no urgent P0/P1 candidates found; only guarded test expect/panic in new migrate tests. Nothing withdrawn at fast pass.

### Triage Groups

| Group | Findings | Context | Preferred Resolution | Why |
|-------|----------|---------|----------------------|-----|
| Operator migrate path bricks and under-reports | #1, #9, #13 | store::migrate_pin_cite / kutha-migrate operator surface (M067/M076/M078). Apply-queue. | Fix #1 first (pre-RULE-01 stores with cites cannot reopen); then #13 (surface dropped-tail counts) and #9 (CLI contract). | One operator run hits all three: a bricked reopen (#1) with no dropped-tail accounting (#13) behind a CLI that masks misuse (#9). |
| Runtime scaling and measurement cliffs | #2, #3 | Bench RSS accounting and retract-time live-behavior scan. Apply-queue. | Fix #3 first (O(N*D) retract scan affects production); then #2 (bench denominator) so later scaling claims are trustworthy. | Both decide whether scaling numbers can be trusted: #3 is the cost, #2 is the ruler. |
| Governor dictionary silent-pass triple | #6, #7, #8 | scripts/kutha_gov/dictionary.py loader (checks.yaml select checks). Apply-queue. | Fix in file order #6 (steps filter), #7 (scalar rows), #8 (duplicate ids) -- one fail-closed loader change covers all three shapes; add the three load oracles together. | Same silent-pass class as prior #23 in three guises; a single LoadError-pattern fix plus oracles closes the family. |

### P1 -- High

| # | File | Issue | Reviewer | Confidence |
|---|------|-------|----------|------------|
| 1 | `crates/kutha-runtime/src/store.rs:413` | migrate_pin_cite leaves stale justifications sidecar; reopen fails | PersistReliability | 75 |

- **#1** -- migrate_pin_cite remaps JustificationCite rule_version pins in the log but deletes only snapshot.json, leaving justifications.jsonl (a lease of log cite rows) with legacy pins. On next open, finish_open runs sidecar_or_stale_prefix (store.rs:1091-1097,1103-1113) which fails InvalidData because the stale sidecar is no longer a prefix of log-derived rows. Any pre-RULE-01 store that persisted while containing cites -- the exact migration population -- is bricked until the operator hand-deletes justifications.jsonl. Existing oracles (m072, m076) persist before appending cites, so the sidecar is empty and the prefix check is vacuous; the brick path has no oracle. Fix: in migrate_pin_cite, remove justifications.jsonl alongside snapshot.json (regenerable lease; next persist rewrites it), or rewrite it from the remapped events; audit migrate_op_wire/migrate_pin_wire the same way. Validation: confirmed (incidence unmeasured -- defect the code permits, not shown to occur).

### P2 -- Moderate

| # | File | Issue | Reviewer | Confidence |
|---|------|-------|----------|------------|
| 2 | `crates/kutha-runtime/benches/baseline.rs:143` | Bench RSS delta covers unrelated runtimes | PerfContractStructure | 75 |
| 3 | `crates/kutha-runtime/src/eval.rs:205` | Retract path scans all live behaviors per trigger | PerfContractStructure | 75 |
| 4 | `crates/kutha-runtime/src/eval.rs:350` | Horn discovery misses late-starting premise overlap | CorrectnessHorn | 75 |
| 5 | `docs/architecture/test-matrix.yaml:1769` | SEM-m054-id-kinds-split oracle field is self-certifying | TestingOracles | 75 |
| 6 | `scripts/kutha_gov/dictionary.py:29` | Scalar or missing steps yield vacuous OK check | GovernorAdversarial | 75 |
| 7 | `scripts/kutha_gov/dictionary.py:90` | Scalar rows in checks.yaml silently skipped | GovernorAdversarial | 75 |
| 8 | `scripts/kutha_gov/dictionary.py:97` | Duplicate check ids silently overwrite | GovernorAdversarial | 75 |

- **#2** -- rss_before is captured before all 6 benchmark runtimes (plain+keyed at 1x/2x/4x), while rss_after is divided by the dedicated sample live count only (n). Delta therefore covers ~15n facts of allocator high-water that is never returned, inflating bytes_per_fact ~15x and making mode comparisons meaningless; #15 fixed only the after side but left the before side misplaced. Fix: capture rss_before immediately before the dedicated sample alloc (after the factor loops), ideally with allocator trim, or measure the sample in a child process / via allocator stats; document denominator as sample live only. Validation: confirmed (exact factor unmeasured, allocator-dependent).
- **#3** -- After the indexed direct-children phase, every retract/correct still clones and walks the full live_behaviors vector with horn_body_matches_except per entry. N retracts over D live derived is O(N*D); retract_examine bumps per live behavior so instrumented tests will show linear examine counts per retract. Fix: index the second phase by (subject, relation) or premise event via horns_by_premise/facts_by_relation so only behaviors whose body could mention the dead premise are re-checked; or record reverse premise edges at fire time. Validation: confirmed (scale unmeasured).
- **#4** -- Multi-premise Horn discovery is point-gated on the trigger valid_from, so an overlapping premise that starts later is never found and the conjoined head is silently missed, order-dependently. Concrete: Assert AB [2015,2020) first; Assert BC trigger [2010,None) second -- at the BC trigger, search_at filters other premises via live_facts_for_relation(rel, MAX, vt=2010) (eval.rs:619), AB is not live at vt=2010, so no AC head is emitted even though [2015,2020) is non-empty. Reverse insertion order derives [2015,2020). Same logical state yields different derived sets; as_of misses the triple in one order. Fix: make premise discovery overlap-aware (collect candidate premise facts whose VT overlaps the trigger window and conjunct each candidate window) or retry search at each distinct candidate valid_from within the trigger window. Validation: confirmed. Design call: overlap-aware discovery vs retry-at-candidate-starts -- decide once, it also bounds residual single-combination-DFS risk.
- **#5** -- The oracle field names the test-matrix governor meta-check, which validates matrix-level consistency (schema, named oracles exist, owners parse) -- none of which observes kinds.py family placement. Re-inlining the planning/cargo/matrix families into kinds.py keeps every test-matrix assertion green; the only real tripwire (scripts/tests/test_m054_id_structure.py) is named solely inside kill_mutation text, so the row's killed status is self-certifying rather than oracle-backed. Fix (human decision): change oracle to name the pytest oracle (or enroll a dictionary file_contains check on test_m054_id_structure.py needles the way parallel-check-error-parity enrolls its pytest), so the meta-check actually requires the oracle to exist. Report-only: needs a human decision on matrix oracle naming; not validated (unselected finding, retained as reported).
- **#6** -- A check row with `steps: [just-a-string]`, `steps: not-a-list`, or a missing `steps:` key passes the steps filter (dictionary.py:29) holding zero steps; run() iterates nothing and returns a finding-free result that prints `OK <id>` / `"passed": true` while zero oracles ran. The --check subset surfaces it without any hint the check is vacuous. Backstop only: full selftest classifies it UNPROVEN (selftest.py:323-324) unless the row also carries a selftest.skip reason. Fix: validate at load -- row steps must be a non-empty list (else LoadError alongside survivors, per #20 pattern); add oracle: stepless/scalar-steps row -> HIGH load on get_checks. Validation: confirmed (vacuous-OK incidence in real rows unmeasured).
- **#7** -- Appending `- just-a-string` under `checks:` evades every surface: the loader `continue`s silently (no LoadErrorCheck, no HIGH -- no dictionary self-structure oracle exists for select checks), `list` hides the row, full `ci` stays green while the malformed row never ran. Same silent-filter class as prior #23, fixed for the test matrix but never reached the governor dictionary. Fix: collect non-mapping rows into a LoadError alongside survivors (fail closed, keep valid checks per #20 precedent), plus a test_selftest.py-style oracle. Validation: confirmed.
- **#8** -- Duplicating an existing check id (e.g. second `id: docs-entry` with weaker steps via bad merge) permanently retires the original check's oracles with zero signal: last row silently overwrites the first in the loader dict, everything prints OK/green, no oracle fires (no `unique: id` on select checks; the invariants-ledger step only cross-refs ids, never uniqueness). Fix: track seen ids in get_checks; on duplicate, keep-first plus LoadErrorCheck naming the dup id; add oracle (duplicate-id row -> HIGH load) and consider a dictionary self-check `unique: id` on select checks. Validation: confirmed (duplicate-id incidence unmeasured).

### P3 -- Low

| # | File | Issue | Reviewer | Confidence |
|---|------|-------|----------|------------|
| 9 | `crates/kutha-runtime/src/bin/kutha-migrate.rs:13` | kutha-migrate --help exits 2 and extra args ignored | PersistReliability | 100 |
| 13 | `crates/kutha-runtime/src/store.rs:491` | Op-wire and pin rewrites drop torn tails without count | PersistReliability | 75 |

- **#9** -- --help prints usage to stderr and exits 2 instead of 0, breaking standard CLI contracts and scripts; extra positional args are silently ignored instead of rejected; a nonexistent path is rejected with exit 2 while the library migrate_compat would create_dir_all it. Fix: return exit 0 after --help (stdout), reject argc>2 with usage+exit 2, document that the target dir must exist (or create it to match library semantics). Validation: confirmed.
- **#13** -- migrate_op_wire/migrate_pin_wire/migrate_pin_cite silently drop torn jsonl tails and truncated/CRC-bad WAL/segment tails with no dropped count in their reports, unlike migrate() which reports dropped_uncommitted. An operator cannot tell a rewrite lost tail records; matrix honesty gap. Fix: count dropped tail records/lines per source in the op-wire/pin rewrites and surface them in OpWireMigrateReport/PinWireMigrateReport (or fail-closed on torn tails). Validation: confirmed (torn-tail incidence unmeasured).

### Actionable Findings

| # | File | Issue | Route | Notes |
|---|------|-------|-------|-------|
| 1 | `crates/kutha-runtime/src/store.rs:413` | Stale justifications sidecar bricks reopen after migrate_pin_cite | `manual -> downstream-resolver` | `suggested_fix` present -- remove or rewrite justifications.jsonl in migrate_pin_cite; add non-empty-sidecar migrate+open oracle |
| 2 | `crates/kutha-runtime/benches/baseline.rs:143` | Bench RSS delta covers unrelated runtimes | `manual -> downstream-resolver` | `suggested_fix` present -- move rss_before past factor loops; document denominator |
| 3 | `crates/kutha-runtime/src/eval.rs:205` | Retract scans all live behaviors per trigger | `manual -> downstream-resolver` | `suggested_fix` present -- index second phase by (subject, relation) or premise event |
| 4 | `crates/kutha-runtime/src/eval.rs:350` | Horn discovery misses late-starting premise overlap | `manual -> downstream-resolver` | `suggested_fix` present -- overlap-aware discovery; design call, decide window semantics once |
| 6 | `scripts/kutha_gov/dictionary.py:29` | Scalar/missing steps yield vacuous OK | `manual -> downstream-resolver` | `suggested_fix` present -- fail-closed load validation + HIGH oracle |
| 7 | `scripts/kutha_gov/dictionary.py:90` | Scalar rows silently skipped | `manual -> downstream-resolver` | `suggested_fix` present -- LoadError alongside survivors + oracle |
| 8 | `scripts/kutha_gov/dictionary.py:97` | Duplicate ids silently overwrite | `manual -> downstream-resolver` | `suggested_fix` present -- keep-first + LoadErrorCheck + oracle |
| 9 | `crates/kutha-runtime/src/bin/kutha-migrate.rs:13` | CLI contract: --help exit, extra args, missing dir | `gated_auto -> downstream-resolver` | `suggested_fix` present -- exit 0 help, reject argc>2, document dir semantics |
| 13 | `crates/kutha-runtime/src/store.rs:491` | Dropped torn tails uncounted in migrate reports | `manual -> downstream-resolver` | `suggested_fix` present -- per-source dropped counts in reports |

### Prior Findings Re-verification (2026-10-11 retained findings vs current HEAD 5dd5338)

Prior #1 stays withdrawn (intended Op wire break per docs/process/op-compat.md) -- listed here as withdrawn, never as a defect.

| Prior # | Status on 5dd5338 | HEAD evidence | Residual |
|---------|-------------------|---------------|----------|
| #3 | closed | eval.rs:224 horn_body_matches_except guard; oracles m054_horn_soundness.rs:127, :201 | none |
| #4 | closed | provenance.rs:167 Correct/CorrectInterval arm; oracle m054:51; hardened by quantum.rs:702 head-match | none |
| #5 | closed | fold_query.rs:318 uncovered_vt_windows; oracle m054:304 residual [2005,2010) | none |
| #6 | closed | persist_append.rs:298 reconcile_uncommitted_tip; oracle m055:84 exact EventId sequence | none |
| #7 | closed | store.rs:999 lock-before-heal; oracle m052_lock_before_heal.rs:142 | none |
| #8 | closed | pipeline.rs:309 rollback_user_cascade; oracle m052_cascade_atomic.rs:33 later-hop snapshot equality | none |
| #9 | closed | segment.rs:826 active_scan_needs_heal; oracle m054:127 torn empty active | none |
| #10 | closed | segment.rs:142 generation-unique names; oracles m054:159, :190 | none |
| #11 | closed | store.rs:1082 replay_check choke point; oracles m052_open_replay.rs:139,152,170 | none |
| #12 | closed | sem/generator mod present (tests/common/sem/mod.rs:5) | none |
| #14 | closed | checks.yaml:1329 when_glob_exists marker; oracle test_kutha_gov.py:1911 | none |
| #17 | closed | eval.rs:619 live_facts_for_relation hot path | none |
| #19 | closed | persist_append.rs:737 graph_suffix_after_durable | none |
| #20 | closed | shrink.rs:247 sha256 digest | none |
| #22 | closed | kind_matrix.py:126 schema gate; oracles schema HIGH | none |
| #23 | closed | kind_matrix.py:146 non_maps HIGH; oracle mixed scalar/mapping rows | none |
| #24 | closed | event.rs:517 id_bytes variant bits | none |
| #25 | closed | baseline.rs:199 as_of cross-check | none |
| #26 | closed | admission.rs:260 has_live_keyed_support | none |
| #27 | closed | fold_query.rs:511 rules_by_pin index | none |
| #28 | closed | quantum.rs:704 head-match on object | none |
| #29 | closed | segment.rs:883 short-orphan header recreate; oracle m056:91 | none |
| #30 | closed | store.rs:594-603 verify-fail manifest remove + sync_dir; oracles m056:159, m002_commit01 | none |
| #34 | closed | parallel.py:143 submit-failure harvest; oracle test_jobs.py:366 | none |
| #35 | closed | 58-VERIFICATION.md:8 kutha-gov py exit 0 | none |
| #36 | closed | statute fixture punkt_15_1.txt:1 | none |
| W1 | closed | docs/process/gsd-auto-contract.md:124 canonical CBM name | none |
| O1 | closed | AGENTS.md:67 --check/--jobs/--budget flags documented | none |
| #2 | partial | eval.rs:359 pin+premise conjunction oracle m054:256 covers basic conjunction only | residual = new finding #4 (late-starting premise overlap) |
| #15 | partial | baseline.rs:326 sample kept alive across after-RSS read | residual = new finding #2 (rss_before still precedes factor loops) |
| #16 | partial | eval.rs:164 indexed derived_by_cause first phase | residual = new finding #3 (second-phase full live scan) |
| #21 | partial | kinds.py 2897 -> 2292 lines (kind_cargo/kind_matrix/kind_planning split) | residual = RUNNERS dispatch still inline; further family splits pending |
| #1 | withdrawn | intended Op wire break, docs/process/op-compat.md | never re-raised |

### Learnings & Past Solutions

- [Known Pattern] honored m010-s03 (live-intern-Define): Runtime::intern appends Op::Define to the live log as a fold no-op (crates/kutha-runtime/src/quantum.rs:383-391) with graph_len/log_offset excluding Define (quantum.rs:449-451) and from_snapshot stripping Define (hydrate.rs:74-76) -- matches pattern, no contradiction. (StandardsSecurityAgent residual note, verified 2026-10-15 round)
- [Known Pattern] honored m010-s02 (durable Define prefix): encoded_log synthesizes the dictionary Define prefix then graph events excluding live Defines (store.rs:1127-1138); terms.jsonl remains a derived picture. (StandardsSecurityAgent residual note, verified 2026-10-15 round)
- [Known Pattern] honored m011-s01 (portable claim_id, independent supports): admit_claim fail-closes unknown claims via fold.claim_known (admission.rs:178-182); Fact.claim_id lineage retained through Correct (fold.rs:783-791). (StandardsSecurityAgent residual note, verified 2026-10-15 round)
- [Known Pattern] honored m011-s02 (unknown-claim fail-closed + replay caused_by): replay_check walks the log in order and rejects Behavior whose caused_by is not in the preceding id set (provenance.rs:30-36), after fingerprint match (provenance.rs:22-24). (StandardsSecurityAgent residual note, verified 2026-10-15 round)
- [Known Pattern] honored membership-snapshot-editions: each membership edition is one snapshot assert with deterministic sorted-CSV object via encode_membership_object (scripts/kutha_gov/time_log.py:29-31) chained by emit_chained (tenant.rs:102,108-141); tip YAML stays the admit lease. (StandardsSecurityAgent residual note, verified 2026-10-15 round)
- [Known Pattern] pre-existing hardening gaps deliberately not raised as findings (unchanged in 1d85af8..HEAD, local-trust threat model): tenant ingest reads the whole harness log via read_to_string with no cap (tenant.rs:67,124-138); KUTHA_RELATIONS_PATH used verbatim with no containment check (allow.rs:45-50, fail-closed to empty set on error); allow::apply_dotenv promotes arbitrary keys from a discovered .env (allow.rs:9-41). Revisit only if the harness log or env becomes remotely influenced. (StandardsSecurityAgent residual note, verified 2026-10-15 round)

### Coverage

- Scope: standalone branch diff chore/m013-roadmap-reset vs merge-base 1d85af8 (main); 990 tracked files, 76218 changed lines, 15228 exec non-test lines; reviewed head 5dd5338; working tree is the reviewed head except the corrected historical review artifact (.compound-engineering/artifacts/reviews/2026-10-11-ce-whole-repo/report.md, excluded from findings) and one untracked CE handoff (excluded).
- Untracked files excluded unless staged (1 path: .compound-engineering/artifacts/handoffs/m089-closed-gsd-auto-check-debt.md).
- No plan discovered (branch keywords ambiguous: many m013/roadmap plans); requirements completeness skipped, noted.
- Indexes refreshed by parent at reviewed HEAD: CBM project kutha-graph reindexed mode=full (14382 nodes, 34604 edges, 2026-10-15); GitNexus kutha-graph reindexed via gitnexus analyze --index-only at 5dd5338 (4949 nodes). Reviewers instructed: CBM list_projects first, check_index_coverage on cited paths, rg arbiter for Rust Self:: edges (both graphs miss them).
- Parent verification runs (not reviewer claims): cargo test 11 CE regression suites (m050,m052 x3,m053,m054 x2,m055,m056 x3) = 33 passed; uv run pytest test_test_matrix+test_multi_check+test_agent_surface = 35 passed. Full workspace/governor ci not run (report-only).
- Standards: AGENTS.md root governs all changed files; no CODING_STANDARDS.md exists.
- Cross-model peer: adversarial lens: in-process fallback (cross-model peer not run: host serving family unattestable in this harness; prior codex route failed pre-egress with requested model not advertised by installed ACP agent; no other eligible attested-different target)
- Cross-model peer outcome: in-process-fallback (no eligible attested-different target; prior codex route failed pre-egress). Peer coverage copied verbatim above. No cross-model corroboration for any finding.
- Validator: one batch of 9 selected findings (#1, #2, #3, #4, #6, #7, #8, #9, #13 -- every P0/P1 and every actionable finding; skip count 0: no finding has cross-model corroboration). Outcome: 9 confirmed, 0 rejected, 0 unresolved, 0 malformed. Every confirmed reason notes unmeasured incidence; each finding carries validation_status confirmed with that reason. Finding #5 (report-only, owner human) was not selected and is retained as reported. Protected-subject classification (report leaf, independent): null for all 9 (no alleged failure falls in memory-safety, concurrency, data-loss, authorization-authentication, injection, public-contract, secrets-exposure, or cryptography -- #1 is a reopen-availability brick with no destructive loss beyond the intended snapshot removal; #13's torn-tail drop follows the pre-existing read-path policy and the allegation is missing accounting); validator agreed null on all 9 -- no reclassification.
- Mechanics: raw compact returns used non-canonical shapes (id/location/file:range, no title/evidence/flags) so the first helper pass reported 26 malformed; findings were normalized to the valid schema preserving every reviewer-supplied severity and confidence (no raises), rerun helper reports 20 findings, 0 malformed, 0 pre-existing, 6 suppressed at anchor 50, first_evidence_backfilled 0 (all quotes reviewer-supplied and verified verbatim against HEAD).
- Semantic reconciliation: no merges; nearby-line findings describe distinct failure modes and fix paths (H1 trigger discovery vs retract scan; GOV scalar-row vs scalar-steps vs duplicate-id; bench RSS vs RSS text-pin). No settlement conflicts: no plan or KTDs discovered (plan.source none) and no settled_conflict flags; settlement suppression not evaluated. No direct-dependency reclassification: pre-existing partition empty, all cited files are in-diff.
- Soft-bucket demotion (mode-aware count 16): 10 mechanical primaries demoted (7 to testing_gaps: #10,#14,#15,#16,#17,#19,#20; 3 to residual_risks: #11 nondet ids, #12 policy scan, #18 empty---check boundary) plus 6 anchor-50 suppressed routed (3 testing_gaps, 3 residual_risks). Retained single-reviewer P2/P3 primaries all quote an explicit violated contract or prove a current defect: #5 matrix killed-status contract (report-only, owner human), #9 CLI contract, #13 migrate dropped_uncommitted precedent; #2/#3/#4/#6/#7/#8 prove current bench, scaling, semantic, and silent-pass defects.
- Anchor-50 findings routed to soft buckets per rule (none P0): intern dup-Define, pipeline unindex scan, Correct-of-Behavior leak to residual_risks; cascade count-only snapshot, keyed-retry lower bound, jobs wall-clock delay to testing_gaps.
- Prior finding #1 (Op decode, intended documented break per docs/process/op-compat.md) not present in returns and never re-raised; 2026-10-11 historical report (head dd38c95) superseded by this review at 5dd5338.
- Parent verification carried (orchestrator source-verified, not reviewer claims): P1 migrate_pin_cite removes only snapshot.json while finish_open rejects non-prefix sidecars; H1 point-gated search_prems/live_facts_for_relation; governor dictionary scalar-row continue, duplicate-id overwrite, steps filter.
- StandardsSecurityAgent: 0 findings; prior_status all closed; its 6 residual NOTEs treated as Known Pattern confirmations (learnings), not risks.
- Validator skip count 0: no finding has cross-model corroboration (peer is in-process fallback sharing the session model family basis); every P0/P1 and every actionable finding is selected.
- Indexes: CBM kutha-graph full reindex at HEAD (14382 nodes, 34604 edges, 2026-10-15); GitNexus kutha-graph reindexed at 5dd5338 (4949 nodes); source reads remain the arbiter.
- Stage 5c: skipped -- mode.apply_local=false (report-only; no apply, no push, no PR). No Applied section.
- Residual risks:
  - Single-combination DFS: search_at returns the first matching premise combination only, so even with overlap-aware discovery a trigger overlapping two disjoint residuals of the same premise relation derives only one head window; full coverage needs all-combination enumeration.
  - Rule-pin gating is also point-based (live_horns filters pin live at trigger valid_from), so a pin whose window starts after the trigger start is skipped even though the three-way conjunction is non-empty; typically masked because pins are open-ended.
  - migrate() verify-failure rollback removes manifest.json + sync_dir (store.rs:594-603) but leaves the just-written generation-unique seg files as orphans; cleanup happens only on the next persist_log/migrate via remove_obsolete_segments, so an abandoned failed migrate leaks disk and leaves confusing seg files next to a legacy store.
  - migrate_compat/migrate_pins drop and reacquire kutha.lock between chained steps (each step locks independently); a concurrent second operator CLI interleaves but converges via idempotency markers -- reviewed, no defect, but the window is untested.
  - reconcile_uncommitted_tip truncates with set_len + sync_data (persist_append.rs:312-340); on platforms where fdatasync does not persist size changes, a power loss could resurrect the suffix -- next open heals via tail checks, so fail-closed, but prefer sync_all after truncation.
  - Pin-cite orphan policy (marker stamped, unmappable cites left) means evidence added after migration never applies on rerun; documented idempotency, operator must delete pin-cite.v1 -- confirm this is stated in operator docs, not just code comments.
  - kinds.py:791 -- generic yaml_map_list silently drops non-dict rows (`maps = [row for row in rows if isinstance(row, dict)]`), so scalar rows in invariants/bridges/honeycomb.yaml are invisible even to their require_fields/unique oracles. Same #23 class, pre-existing, out of the M057-M090 diff; loader-level fail-closed (GOV-NEW-01 pattern) would cover all dictionaries at once.
  - Generic yaml_map_list has no min-rows concept: an empty select list passes all require/unique/cross-ref steps vacuously (only checks.yaml-itself has an empty guard via LoadError, and test-matrix has the non-empty-rows guard at kind_matrix.py:138). Pre-existing; no live instance observed.
  - selftest parallel path (_run_selftest_parallel, selftest.py:189-230) has no submit-failure serial backstop unlike run_checks_parallel: a pool submit failure propagates as an exception. Loud/fail-closed (no silent pass), but un-oracled and inconsistent with the #34 at-most-once hardening.
  - VERIFIED-CLEAN (explicit split check, no action): RUNNERS vs ALLOWED_KINDS aligned 26/26 with identical names after the kind_cargo/kind_matrix/kind_planning splits (3 kinds added in-range for M058/M060/M061: disk_free, budget_covers_checks, env_int_at_least). DERIVABLE_KINDS covers 24/26; git_path_implies is covered by the docs-coupling explicit skip (checks.yaml:838-839) and yaml_needles_in_glob has zero dictionary users so its runner path is live-unexercised -- first future use ships without a live oracle until selftest proves it.
  - kinds.py still 2292 lines with RUNNERS dispatch inline (#21 partial): file_max_lines/glob_max_lines governance pressure remains; further family splits (cite/refs/rust-asserts) still pending.
  - Lazy cold path: live_facts_for_relation falls back to full live_facts_at scan when hot_maps not warm (fold_query.rs:26-31); Horn firing before ensure_fold_indexes stays O(all facts) per premise.
  - Retract cascade retains O(live_derived) per-trigger cost even after indexing (P2/P3); large derived graphs need a benchmark guard before close.
  - RSS allocator high-water retention means even a correctly placed before/after pair overstates steady-state bytes_per_fact on glibc/jemalloc without trim.
  - Text-coupled governor oracles (parallel-check-error-parity file_contains needles, kinds-split text needles, agent-surface doc needles) can rot independently of behavior: a correct refactor that rewords parallel.py or the docs flips them red, while a behavioral regression preserving the needles stays green.
  - Multi-check CLI oracles double as live-tree health checks; a red live tree masks M077/M079/M085/M086 selection regressions and vice versa.
  - Op-migrate facade rows (SEM-m075/m076/m078) are covered by exact remap-count tests today, but no test asserts migrate_compat ordering (op-wire before pins) beyond marker presence -- a swapped sequence with both markers stamped would pass.
  - Demoted from primary (P3 crates/kutha-common/src/event.rs:283 -- Synthesized migration Define ids randomize per retry. Use Event::stable_define for synthesized Defines so Define ids are content-stable across retry runs.)
  - Demoted from primary (P3 crates/kutha-runtime/src/fold_query.rs:553 -- Policy pin election scans all buckets per admission. Maintain a max-ingested live-policy pointer (or per-pin liveness + ordered set) updated on push/invalidate instead of scanning all buckets per admission.)
  - Demoted from primary (P3 scripts/kutha_gov/__main__.py:33 -- Empty --check flag runs full suite instead of failing closed. Change the _select_checks guard to `if check_id is None:` so "" flows into _parse_check_ids and hits the empty-token exit-2 path; add oracle `--check ""` -> exit 2 for precommit/json/selftest.)
  - Anchor-50 reliability note (P2 crates/kutha-common/src/intern.rs:45 -- Mixed-source duplicate Define can skew TermIds. Enforce emit-once per name in rewrite_op_wire_values (track emitted names; drop redundant modern Defines so output Define order equals dict.intern order) and make TermDictionary::from_strings keep-first.)
  - Anchor-50 reliability note (P2 crates/kutha-runtime/src/pipeline.rs:57 -- unindex_behavior scans every cause bucket. Store reverse pointer (cause per behavior id) and remove from the single bucket in O(1) instead of scanning all values; retain empty-bucket cleanup.)
  - Anchor-50 reliability note (P3 crates/kutha-runtime/src/pipeline.rs:85 -- Correct of Behavior facts leaks index entries. Unindex (or lazily filter) the target when Correct/CorrectInterval invalidates a Behavior fact, mirroring the Retract arm.)
- Testing gaps:
  - No oracle for late-starting non-trigger premise overlap (AB starts after BC trigger start) asserting order-independent derivation.
  - No oracle for rule-pin VT truncation (narrow pin window intersecting wide premise windows).
  - No oracle for multi-residual premise combinations (one premise with two disjoint live windows overlapping one trigger).
  - No oracle persists a store containing legacy-pin JustificationCite rows (non-empty justifications.jsonl) and then runs migrate_pin_cite/migrate_compat + open -- the P1 brick path; m072/m076 always persist before appending cites, leaving the sidecar empty.
  - No oracle for legacy-before-modern mixed source with shared interned terms asserting single-Define output and post-open TermId stability (P2).
  - No CLI tests for --help exit code (expect 0), extra-arg rejection, or empty-arg handling (m078 covers only success/idempotency/missing-dir).
  - No assertion on dropped-tail counts in op-wire/pin migrate reports (reports lack the field entirely, unlike MigrateReport.dropped_uncommitted).
  - M079/M085/M086 append oracles assert inclusion only (assertIn both ids + exit 0) never exclusivity: a regression that drops the filter and runs the full suite still passes them whenever the live tree is green (e.g. _join_check_flags returning None). Missing assertions: json payload check-count == 2, or assertNotIn a third check id for precommit/selftest.
  - No oracle for the `--check ""` empty-string shape (GOV-NEW-04): M077 covers the trailing-comma shape (`docs-entry,` -> exit 2) but not the single-empty-flag shape that fails open to the full suite.
  - No loader fail-closed oracles for scalar rows, duplicate ids, or scalar/missing steps in checks.yaml (GOV-NEW-01/02/03): the CE #20 nameless-row oracle exists (test_selftest.py:497-501), but its sibling shapes are uncovered and no dictionary self-structure check (require_fields/unique on select checks) exists, unlike the matrix schema/row oracles from #22/#23.
  - CBM index on kutha-graph is fresh at HEAD (2026-10-15, full mode, all 12 cited paths metadata_match, no_recorded_issue); structural claims above were additionally grounded in direct source reads, which remain the arbiter for dispatch-table edges.
  - No bench/test asserting bytes_per_fact denominator isolation (delta attributable to sample only) or RSS high-water handling.
  - No retract scaling oracle: N derived + N retracts asserting sublinear retract_examine growth; current second-phase scan would fail it.
  - No Correct-of-Behavior index-hygiene test (live_behaviors/derived_by_cause lengths after Correct targeting a Behavior).
  - No many-policy admission scaling test (live_policy_pin_at bump counts vs policy count).
  - No reopen test pinning kinds.py line-count ceiling after the partial split (2897 -> 2292) to prevent regrowth.
  - No ingest-bound oracle (oversize harness log / oversize membership object) -- acceptable while the harness log stays a local operator artifact; add only with a remotely-influenced input.
  - No negative test asserting KUTHA_RELATIONS_PATH pointing outside the repo fails closed to deny-all rather than falling back to the default dictionary -- load_allowed_names error arm (allow.rs:56-59) covers it by construction but has no dedicated case.
  - try_persist_with_layout Rollover accept branch uncovered: m053 oracle exercises only the empty-runtime reject path (sealed=0 -> false), so an always-false mutant stays green; no test builds a sealed>=2 store and asserts true. crates/kutha-runtime/tests/m053_ce_review_regressions.rs:109-124
  - m055 exact-sequence oracle is graph-ids only (Define EventIds excluded by construction): a retry path that duplicates Define frames while keeping graph ids unique is invisible to both failpoint tests. crates/kutha-runtime/tests/m055_append_tip_rollback.rs:51-59
  - Manifest-removal sync durability unproven behaviorally: no oracle models a crash between remove_file(manifest) and return in the migrate verify-fail unflip path, so the sync_dir ordering pin has no crash-shaped execution behind it. crates/kutha-runtime/tests/m056_persist_heal.rs:159-211
  - Scaling oracles trust self-reported bump counters with no enforcement that every scan path bumps: a future (or unnoticed current) O(N) loop that omits bump() is bounded-green by construction. Nothing today fails if a new full-scan path forgets its counter (cf. eval.rs:414-415 fallback loop vs 388-389 indexed loop). crates/kutha-runtime/src/eval.rs:414-415
  - Anchor-50 oracle weakness (P3 crates/kutha-runtime/tests/m052_cascade_atomic.rs:26 -- Cascade atomicity snapshot is count-only. Compare fold fingerprint plus exact log EventId sequence before/after instead of the count triple.)
  - Anchor-50 oracle weakness (P3 crates/kutha-runtime/tests/m056_delivery_key_same_relation.rs:45 -- Keyed retry oracle uses weak lower bound. Tighten to assert_eq!(first_b.events_in_quantum, 1) to match the first_a pin.)
  - Anchor-50 oracle weakness (P3 scripts/tests/test_jobs.py:357 -- Jobs ordering oracle depends on wall-clock delay. Replace the wall-clock delay with a deterministic scheduler hook (barrier/event) or mark the timing margin explicitly with a retry-tolerant comparison.)
  - Demoted from primary (P3 crates/kutha-runtime/src/store.rs:1239 -- Test-only pub surface ships in every build. Gate the seam surface behind cfg(any(test, feature = "test-seams")) so default builds carry no oracle instrumentation; keep stop_after plumbing where it is already free (None fast path).)
  - Demoted from primary (P3 crates/kutha-runtime/tests/m053_ce_review_regressions.rs:74 -- fork_at outcome rebuild asserted by count only. Compare sorted rebuilt event ids against the log's QuantumOutcome event ids instead of comparing lengths.)
  - Demoted from primary (P3 crates/kutha-runtime/tests/m054_scaling_bench.rs:58 -- RSS bench-alive oracle pins source text, not behavior. Keep as tripwire but add a behavioral anchor (e.g. run the bench helper in-process and assert the sample runtime is alive at sample time), or document the row as text-pinned.)
  - Demoted from primary (P3 crates/kutha-runtime/tests/m054_scaling_bench.rs:72 -- as_of bench validation oracle is text-only. Pair with the existing as_of_matches_emitted_relatedto_triples behavioral test explicitly in the matrix oracle list (it already exists in the same file) so the row is not text-only.)
  - Demoted from primary (P3 crates/kutha-runtime/tests/m056_persist_heal.rs:161 -- Manifest-remove sync ordering asserted by text search. Add an observable seam (e.g. a sync counter like persist_log_sync_count asserted > 0 on the verify-fail path) and keep the text pin only as a secondary tripwire.)
  - Demoted from primary (P3 scripts/tests/test_kutha_gov.py:2186 -- Disk-free oracle coupled to live free space. Mock shutil.disk_usage in this test or derive the threshold from measured free space minus a margin.)
  - Demoted from primary (P3 scripts/tests/test_multi_check.py:49 -- Multi-check oracles run against live tree. Run the --check selection assertions against a minimal scratch tree (as test_test_matrix.py does) and keep at most one live-tree smoke test.)

---

> **Verdict:** Ready with fixes
>
> **Reasoning:** One P1 (#1, confirmed): migrate_pin_cite bricks exactly the pre-RULE-01 stores that persisted with cites -- the migration population -- until an operator hand-deletes justifications.jsonl. Six P2s: order-dependent Horn miss (#4, silent wrong answers), O(N*D) retract scan (#3), untrustworthy bench ruler (#2), and a governor silent-pass triple (#6/#7/#8) where malformed checks print OK while running nothing. Two P3s (#9 CLI contract, #13 dropped-tail accounting). All 9 validated findings confirmed; incidence unmeasured on each, so severity stands on code-permitted defects. All 2026-10-11 retained priors verify closed except the four known partials whose residuals are this round's #2/#3/#4 and the kinds.py remainder.
>
> **Fix order:** #1 (P1 brick) -> governor triple #6/#7/#8 (one fail-closed loader change + 3 oracles) -> #4 (overlap-aware discovery; decide window semantics once) -> #3 (index retract second phase) -> #2 (bench denominator) -> #13 + #9 -> human decision on #5 (matrix oracle naming)

### Actionable Findings Summary

- #1 [P1] crates/kutha-runtime/src/store.rs:413 -- migrate_pin_cite leaves stale justifications sidecar; reopen fails. manual, suggested_fix present, confidence 75.
- #2 [P2] crates/kutha-runtime/benches/baseline.rs:143 -- bench RSS delta covers unrelated runtimes. manual, suggested_fix present, confidence 75.
- #3 [P2] crates/kutha-runtime/src/eval.rs:205 -- retract path scans all live behaviors per trigger. manual, suggested_fix present, confidence 75.
- #4 [P2] crates/kutha-runtime/src/eval.rs:350 -- Horn discovery misses late-starting premise overlap. manual, suggested_fix present, confidence 75.
- #6 [P2] scripts/kutha_gov/dictionary.py:29 -- scalar or missing steps yield vacuous OK check. manual, suggested_fix present, confidence 75.
- #7 [P2] scripts/kutha_gov/dictionary.py:90 -- scalar rows in checks.yaml silently skipped. manual, suggested_fix present, confidence 75.
- #8 [P2] scripts/kutha_gov/dictionary.py:97 -- duplicate check ids silently overwrite. manual, suggested_fix present, confidence 75.
- #9 [P3] crates/kutha-runtime/src/bin/kutha-migrate.rs:13 -- kutha-migrate --help exits 2 and extra args ignored. gated_auto, suggested_fix present, confidence 100.
- #13 [P3] crates/kutha-runtime/src/store.rs:491 -- op-wire and pin rewrites drop torn tails without count. manual, suggested_fix present, confidence 75.
