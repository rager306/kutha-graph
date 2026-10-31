# Project Milestones: Kutha

## v1.79 Meta lease overhead A/B (Shipped: 2026-10-31)

**Delivered:** M186. Performance honesty: the per-append meta-lease cost is a measured number with an open residual, not an assumption.

**Phases completed:** 274 (1 plan)

**Key accomplishments:**
- Two interleaved same-host A/Bs (6 samples each), base `4db8770` for v1.78-only and `05dfea0` for v1.77+v1.78; base commit ids recorded because the worktrees are removed.
- Against `4db8770`: `ingest_plain.n` +2.7%, `as_of.n` +5.7%; one 253 µs base outlier named, not dropped.
- Against `05dfea0` (corrected 2026-10-31 from sorted samples): `ingest_plain.n` **−4.2%** (faster), `as_of.n` +1.7%, **`as_of.2n` +19.0%** (reproduced at +16.3% in the other A/B), `ingest_keyed.n` +12.1%.
- Decision: keep the leases for the write path; the gate metrics (`ingest_plain.n`, `as_of.n`) stay inside 10%, and the secondary `as_of.2n` / `ingest_keyed.n` moves are recorded as unexplained, with no structural mechanism (read path unchanged).
- **Verifier caught a false scope claim** (the first A/B measured v1.78 only, not v1.77+v1.78) and a deleted baseline worktree; both fixed by the second run and recorded commit ids.

---

## v1.78 Enrichment admission leases (Shipped: 2026-10-31)

**Delivered:** M184→M185 sequence's last step: enrichment proposal admission uses runtime leases instead of replaying the log per write. No new Op, no feature.

**Phases completed:** 273 (1 plan)

**Key accomplishments:**
- Proposal admission and the emit pre-check use `PackLifecycleTracker`, a visible-log id set, the blob map, and a borrowed cache-key set — no `derive_pack_lifecycle`, no full event-id scan.
- `check_proposal` is read-only; the caller inserts the key after commit, so a rejected proposal cannot poison the lease and admission does not clone the key set.
- Leases are rebuilt on hydrate/open/fork and updated on every append, including `intern`'s `Op::Define`.
- **Two verifier rounds:** the first blocked for a never-updated key lease, a per-write clone, and a source predicate broader than replay's; the second found `intern` bypassing the append choke point and a test that never created a checkpoint. All fixed.
- Recorded limit: replay exempts pre-checkpoint proposals from source existence, because the rewrite may drop the cited rows.
- Tests: enrichment admission 6/6, workspace 832 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.77 Admission operator index (Shipped: 2026-10-31)

**Delivered:** M184. Closes the admission-side complexity gap left by v1.76: the runtime holds droppable operator and lifecycle leases, so pack admission no longer rebuilds either from the log per write.

**Phases completed:** 272 (1 plan)

**Key accomplishments:**
- `Runtime` holds `OperatorIndex` and `PackLifecycleTracker`, rebuilt from the log on hydrate/open/fork and updated on commit.
- Measured per-install cost is flat (400 → 3200 installs: 255 µs → 287 µs); before, 213 µs → 2.33 ms.
- **Blocker caught by the verifier:** the first rebuild applied retire after the install loop, so a legal install-after-retire was rebuilt as a collision and an accepted store could fail to reopen. Fixed inline in log order, guarded on live/open/fork/checkpoint.
- A constructor that skips `replay_check` now marks leases untrusted and admission fails closed (`meta_leases_untrusted`).
- `operator_count()` (live names) vs `pack_operators()` (audit rows incl. retired) documented and pinned.
- Tests: admission index 10/10, workspace 826 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.76 Digest falsifiers and linear index (Shipped: 2026-10-31)

**Delivered:** M183. Closes the two gaps v1.75 recorded instead of overclaiming: the raw-SHA-256 meta digest checks now have direct falsifiers, and operator collision detection is indexed.

**Phases completed:** 270–271 (2 plans)

**Key accomplishments:**
- Policy-hash-addressed blobs pass the generic `StoreBlob` gate and are still rejected as manifests and as enrichment payloads; deleting the raw check fails both tests (mutation-checked).
- `OperatorIndex` (operator→pack) plus `validate_install_indexed`: one lookup per operator instead of a scan over every declared set. Re-declare and retire release names correctly.
- Removed the unused `validate_install_against` path so there is one collision rule.
- Honest limits recorded: admission itself is still O(P) per install; the scaling test is tolerant rather than high-power.
- Tests: digest falsifiers 5/5, workspace 816 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.75 Meta-surface threat audit (Shipped: 2026-10-31)

**Delivered:** M182. A real security-audit slice: threat register for the C1/C1b/C5 meta surfaces and the replay reader gate; at close 6 threats had a direct adversarial test and 2 were transitively covered (direct falsifiers added in M183). No feature, no thaw.

**Phases completed:** 269 (1 plan)

**Key accomplishments:**
- Register: 10 threats, 4 trust boundaries; at close 6 direct, 2 transitive (direct falsifiers in M183), 2 accepted with re-open triggers.
- **Defect found and fixed:** meta replay was O(n²) (2000 pack installs ≈ 2.18 s on open); single-pass tracker plus incremental manifest checks bring it to ~13 ms.
- Admission and replay share `PackLifecycleTracker` and `check_proposal`; the verifier caught that the first unification dropped retired-pack operator release, and the fix plus a regression test are in.
- Two threats are recorded as transitively covered by the generic `StoreBlob` digest gate rather than claimed as direct tests.
- Tests: threats 10/10, workspace 811 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.74 eCFR edition ambiguity explained (Shipped: 2026-10-31)

**Delivered:** M181. The M167 fixture's edition residual is explained from the source and guarded by an assertion, not papered over with a fabricated edition.

**Phases completed:** 268 (1 plan)

**Key accomplishments:**
- Raw XML for all eight part-304 sections shows the 2017-01-23 edition differs only by an `<XREF>` "amendment published" note; extraction correctly drops it.
- FR document 2017-00891 was published 2017-01-23 and effective 2017-03-14, which explains why the middle eCFR row has no text change.
- The PIT probe asserts exactly two distinct extracted texts per section: the count assert catches a lost edition, the equality asserts catch a gained one.
- No fixture `.txt` was edited; all nine committed texts reproduce byte-exactly from live eCFR. No legal pack, no corpus claim.
- Tests: eCFR PIT 1/1, workspace 801 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.73 Replay-side meta revalidation (Shipped: 2026-10-31)

**Delivered:** M180. Reader-gate hardening: replay and store open revalidate C1 lifecycle, C1b manifest identity, and C5 enrichment proposals instead of deriving a silently wrong lease. No new Op and no feature.

**Phases completed:** 266–267 (2 plans)

**Key accomplishments:**
- `Runtime::check_meta_replay` validates each meta op against the exact log prefix before it, reusing writer-side rules.
- Rejects activate-before-install, duplicate install, retire-unknown, version mismatch, and missing/mismatched manifests; rejects unknown/retired/inactive pack, wrong version, empty pins, missing source, missing/mismatched payload, and duplicate cache key for enrichment.
- Source existence is required for every proposal after the first checkpoint; only proposals a later checkpoint compacts away are exempt.
- `open` routes every reconstruction branch through `replay_check`, so a bad store cannot be opened, forked from disk, or verified post-migrate.
- Tests: replay meta 10/10 plus a tail-rule unit test; workspace 801 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.72 Gate GC toy-pack probe (Shipped: 2026-10-31)

**Delivered:** M179. Gate GC answered with a read-only probe: a toy non-legal pack installs, product facts answer GPML `AS OF` at two cuts, the pack retires, and a reopened store matches. Zero production edits.

**Phases completed:** 265 (1 plan)

**Key accomplishments:**
- Toy pack installs with a manifest and activates; manifest blob and operator registry are genuinely loaded.
- `AS OF 2015` returns `50-kmh`, `AS OF 2020` returns `30-kmh` (half-open boundary), with the pack installed and after retire.
- Retire changes lifecycle/registry only; facts and fingerprint unchanged; reopen parity is non-vacuous.
- No `crates/*/src`, Cargo, build.rs, or dictionary change in this slice.
- Scope honesty: the query path does not consult pack state, so the probe evidences pack neutrality, not query sufficiency.
- Tests: gate 2/2, workspace 790 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.71 C5 enrichment foundation (Shipped: 2026-10-31)

**Delivered:** M178. Bounded ADR-052 foundation: typed content-addressed enrichment proposals and a replay-derived ledger. Derived data only; never a kernel fact.

**Phases completed:** 263–264 (2 plans)

**Key accomplishments:**
- `Op::EnrichmentProposal` carries pack id/version, source event ids, model/prompt/cache pins, and a raw-SHA-256 `StoreBlob` digest; meta-only with empty `object_ids`.
- Admission is fail-closed: active pack only, exact version, non-empty pins, existing sources, blob present and digest-matching, unique cache key; validation runs before the payload append so no orphan blob remains.
- `Runtime::enrichment_ledger()` is replay-derived and lifecycle-aware; survives persist/open, fork cuts, and checkpoint_sealed_prefix; no model call and no derived lease.
- Tests: enrichment 10/10, workspace 788 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0. No LLM, HNSW, trust-path write, or ADR Accepted.

---

## v1.70 C1b pack manifest and operator registry (Shipped: 2026-10-31)

**Delivered:** M177. Bounded C1 extension from ADR-021: a content-addressed pack manifest and a replay-derived operator registry. No operator execution, code loading, dictionary facets, lease rebuild, WASM, marketplace, schema SMO, or ADR Accepted.

**Phases completed:** 261–262 (2 plans)

**Key accomplishments:**
- `PackManifest` (`kutha-pack-manifest/v1`) as canonical JSON addressed by raw SHA-256, pinned through optional `PackInstall.manifest_digest` → `StoreBlob`; legacy manifest-less installs still parse and admit.
- Admission requires the blob, exact identity, schema v1, unique valid read-only operator names, and no collision with an installed/active pack; retired packs release names; padded identifiers and padded operator names fail closed.
- `Runtime::pack_operators()` is replay-derived, lifecycle-aware, and executes nothing; survives persist/open, fork prefix cuts, and checkpoint_sealed_prefix.
- Tests: manifest 10/10, registry 5/5, workspace 778 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.

---

## v1.69 C1 pack lifecycle foundation (Shipped: 2026-10-31)

**Delivered:** M176. Bounded C1 foundation from ADR-021: typed D021 lifecycle log events plus a replay-derived droppable lifecycle lease.

**Phases completed:** 259–260 (2 plans)

**Key accomplishments:**
- `Op::PackInstall` / `PackActivate` / `PackRetire`: typed meta events, deterministic length-prefixed digests, empty object_ids, no graph facts.
- Admission replays lifecycle state and rejects duplicate install, invalid/mismatched activate, invalid/mismatched retire, empty identifiers, and user-forged Pack*; only dictionary-mode `emit_pack_*` works.
- `Runtime::pack_lifecycle()` is a replay-derived installed/active/retired lease; survives persist/open, fork prefix cuts, and checkpoint_sealed_prefix; graph fingerprint unchanged.
- Tests: schema 4/4, runtime 5/5, workspace 762 passed / 0 failed. No artifact loading, operator registration, dictionary facets, lease rebuild, WASM, marketplace, schema SMO, or ADR Accepted.

---

## v1.68 C4 path-policy foundation (Shipped: 2026-10-30)

**Delivered:** M175. Bounded C4 security foundation permitted by M003a + Gate GB: closed-default path-policy compiler over existing typed QueryIr, deny-before-prepare. No grants-as-facts, tenant slices, vacuum authority, capabilities, WASM, full ABAC, or ADR Accepted.

**Phases completed:** 257–258 (2 plans)

**Key accomplishments:**
- `crates/kutha-runtime/src/security.rs`: closed-default `PathPolicy`; scan/unlabeled/zero-hop deny unless explicitly allowed; labeled hops require every relation to resolve and be policy-allowed.
- `compile_path_policy` decides before `prepare`; denied queries never materialize leases or rows.
- `execute_authorized` preserves authorized GPML/typed results; denied queries return typed `SecurityError::PolicyDenied`; post-policy query failures use distinct `SecurityError::QueryRejected`.
- Tests: policy 5/5, integration 4/4, workspace 753 passed / 0 failed. Security ADRs stay Proposed.

---

## v1.67 Gate GB fixture parity (Shipped: 2026-10-30)

**Delivered:** M174. Gate GB closed for supported M003a scope: GPML `AS OF` parity across FF5, B3 layered workload, and M167 eCFR sections/cuts. Security remains separately owner-leased; no C4/C6 implementation.

**Phases completed:** 255–256 (2 plans)

**Key accomplishments:**
- FF5 vt 2015/2020: GPML == typed IR == `as_of` facade.
- B3: every layer-0 start x 1/2 hops; GPML == typed IR; execution non-empty.
- M167 eCFR: 3 sections x 7 cuts GPML-vs-facade parity; unknown relation/term fail closed.
- Security eligibility boundary recorded; ADR-080/081 / C4/C6 require a separate owner-leased milestone; M003b stays gated on C2.

---

## v1.66 M003a GPML AS OF parser (Shipped: 2026-10-30)

**Delivered:** M173 / M003a. Read-only GPML/Cypher subset parser into typed QueryIr with intern/allowlist binder; differential proof on B3/FF5; fail-closed vocabulary.

**Phases completed:** 253–254 (2 plans)

**Key accomplishments:**
- `crates/kutha-runtime/src/gpml.rs`: read-only subset parser + `VocabBinder` / `RuntimeBinder`; zero intern calls; FF6 allowlist checked before dictionary id.
- Focused tests 7/7; differential tests 5/5 including B3 render/parse parity and FF5 `AS OF` equals typed facade; workspace 741 passed / 0 failed.
- Mixed repeated/per-hop chain fails closed at prepare with `RelsShape`; no writes, aggregation, path functions, planner, HNSW, or legal pack.

---

## v1.65 B3 hot-path benchmark (Shipped: 2026-10-30)

**Delivered:** M172. Fixed the layered `relatedTo` workload, recorded the Kutha baseline/floor, and compared FalkorDB/Samyama.

**Phases completed:** 251–252 (2 plans)

**Key accomplishments:**
- Pinned Kutha baseline: `hop2.walk.n` 170,568 ns; 2n 353,765; 4n 714,556; ratio floors 2.28/4.61/2.22.
- FalkorDB median 4,779 µs; Samyama 14,511 µs; Kutha walk-only ≈29.5×/≈89.6× faster, prepared ≈9.3×/≈28.3×.
- Decision: skip B4 (HNSW) and B5 (planner); B6 (Cypher AS OF) still requires M003. No thaw.

---

## v1.64 ingest regression (Shipped: 2026-10-30)

**Delivered:** M171. The first index commit `199807d` is named; the per-assert map write cost is accepted as the O(1) lookup tradeoff.

**Phases completed:** 249–250 (2 plans)

**Key accomplishments:**
- Five interleaved N=10000 samples: parent `c26bf35` 93.5 ms -> first bad `199807d` 130.4 ms (+39.6%); old `cdf1942` 96.3 ms -> HEAD 114.1 ms (+18.5%).
- Cause: `Runtime::append_indexed` (`by_event_id`, `by_delivery_key`) and GraphFold `fact_by_event` / `live_triples`; per-assert write cost for O(1) lookup.
- `027a2ce` capacity reserve lowers HEAD but does not clear D-06; `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded opt-out.
- No product change, no Op/ADR/CI change; phase 250 accepts the tradeoff.

---

## v1.63 as_of regression (Shipped: 2026-10-29)

**Delivered:** M169. The 3.32× as_of gap is traced to `cec5b64` and reduced to 1.35×.

**Phases completed:** 247–248 (2 plans)

**Key accomplishments:**
- First bad commit `cec5b64` added a per-fact HashSet `is_dictionary_rel` probe to `LiveFactFilter::Product`.
- Dense `dictionary_rel_flags: Vec<bool>` replaces the HashSet; semantics unchanged.
- N=10000 `as_of.n` 303 µs → 123 µs (old 91 µs); residual 1.35× is the accepted product-filter cost.
- `kutha-gov ci` green; cargo observe budget raised to 600 s for cold builds.

---

## v1.62 Performance truth (Shipped: 2026-10-29)

**Delivered:** M168. Interleaved old-vs-HEAD bench and `bytes_per_fact` honesty.

**Phases completed:** 245–246 (2 plans)

**Key accomplishments:**
- Old `cdf1942` reproduces the 2026-10-01 `as_of` (84–99 µs) and `bytes_per_fact` (13.7 KB).
- HEAD medians: ingest +12.6%, as_of **3.32×**, open −3.0%, persist −67.0%, replay +1.1%.
- The as_of gap is a real regression; next step is a bisect, not another benchmark reshuffle.
- `bytes_per_fact` is `n/a` on a zero RSS delta.

---

## v1.61 Legal PIT probe (Shipped: 2026-10-29)

**Delivered:** M167. A public-domain eCFR fixture (3 sections × 3 editions) and a probe proving `as_of`, TT-only `as_known_on`, `norm_view`, and checkpoint/replay parity on real text.

**Phases completed:** 243–244 (2 plans)

**Key accomplishments:**
- 1 CFR 304.1/304.2/304.10 at 2015-12-18, 2017-01-23, 2017-03-14; fetch tool and licence note.
- 7 day-scale cuts; 2017-03-13 vs 2017-03-14 is the discriminating boundary.
- `cargo test --workspace` 730 passed; `kutha-gov ci --jobs 1` `run_high=0`.
- No new Op, no core schema change, no thaw.

---

## v1.60 Operational honesty (Shipped: 2026-10-28)

**Delivered:** M166. A cargo timeout or missing binary no longer retries; GitHub CI installs Rust 1.99.0 and runs pytest plus the governor gate; a fresh same-host benchmark baseline is recorded.

**Phases completed:** 240–242 (3 plans)

**Key accomplishments:**
- `run_cargo_observation` breaks on 124/127; new retries-env regression.
- CI action parse fixed; pytest and governor gates wired.
- Baseline: `ingest_plain.n` 115,733,914 ns; `as_of.n` 334,032 ns; +21.3% / 4.0× vs 2026-10-01; `bytes_per_fact` 0 B is a follow-up RED.

**What's next:** no milestone is leased.

---

## v1.59 Compacted Horn cause (Shipped: 2026-10-27)

**Delivered:** Checkpoint and replay lineage hardening (M154–M165): checkpoint receipt, checkpoint-is-a-cut, fork diff, hop workload, missing Horn blob, StoreBlob digest, shared-head causes, one head per cause, vintage and cited lineage, compacted Horn cause.

**Phases completed:** 228–239 across M154–M165 (12 plans total)

**Git range:** `v1.48..30deaa9`

**What's next:** GSD native reset on 2026-10-28; next milestone unleased.

---

## Pre-native history

GSD milestones v0.01–v1.58 and their phase artifacts are preserved in `.planning-archive/pre-native-2026-10-28/`, on the git branch `archive/pre-gsd-native-2026-10-28`, and in `tmp/backups/pre-gsd-1.19-2026-10-28.tar.gz`. They are not active GSD artifacts after the reset.
