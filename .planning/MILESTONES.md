# Project Milestones: Kutha

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
