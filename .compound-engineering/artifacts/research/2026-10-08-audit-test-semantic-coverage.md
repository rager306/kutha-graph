# Audit: do tests verify meaning? (2026-10-08)

Read-only. CBM `kutha-graph` gen `2026-10-08T07:14:05Z`. Cited paths: `check_index_coverage` `no_recorded_issue` / `metadata_match` (best-effort). `rg` for literals. No cargo/uv. Matrix YAML has **77** rows (`test-matrix.md` still says 76). Not a coverage percentage.

## Behavior → oracle map

| Public behavior | Named oracles (independent expected unless noted) | Class |
|---|---|---|
| `Runtime::emit` Assert | `ff5_as_of_*`, `ff6_unknown_relation_*`, `facade_statute`, fixtures | semantic |
| `emit` Retract | `retract_correct_interval_event_id_clears_all_residuals`, `retract_keeps_loser` | semantic |
| `emit` Correct / CorrectInterval | `interval_patch_leaves_vt_2012_and_2021_residuals`, `whole_version_correct_*`, `horn_correct_*` | semantic |
| Facade `assert`/`retract`/`correct`/`derive` | only `statute_as_of_via_facade` (latest-live-by-subject+vf) | thin semantic |
| `as_of` / `live_at` | FF5, `true_on_d_differs_from_known_on_d`, `asof_elig01_*` | semantic |
| `as_known_on` / `known_at` | `true_on_*`, `known04_*`, `tr02_*`, `as_known_on_miss_without_anchor` | semantic |
| `claim_supported_at` | `as_of_and_claim_supported_at_skip_non_overlapping_facts`, claim-retract in `m011_claim_supports` | semantic (thin) |
| Horn / cascade / `rule_follow_ons` | `horn_correct_*`, `casc01_*`, `det02_*`, `cascade_order_*`; `cascade_idles_*` / `known04_*` also characterization | mixed |
| Admit / justification | `ff6_*`, `unknown_claim_*`, `casc01_*`, `m012_*` admission/action, `adm_pin01_*` | semantic |
| Persist / open / segments / WAL / snapshot | `define_ids_stable_*`, `persist_roundtrip_*`, DUR-* crash pins | mixed (bytes/fsync vs meaning) |
| Tenant bridge | `h2_*`, `h4_*` as_of / unmapped relation | semantic |
| CSR lease | FF5 CSR neighbors, `typed_csr_*`, `csr_fold_diff_*` | semantic + differential |
| `ingest` | no `Runtime::ingest`; tenant `ingest_harness_jsonl_str` via H2/H4 | — |

## Classification stats (honest counts, not %)

**Matrix statuses (YAML, 77 rows):** killed 67 · characterization 5 · spec-only 4 · gate 1. Layers: many DUR `crash`/`kill`, SEM `decision`/`property`, HAR `governor`.

**Sampled asserting fns read in source (~40):** ~22 semantic oracles (hand-built expected triples / fail-closed) · ~8 differential (Eager=Lazy, replay=live, persist=memory, naive=fold) · 5 labeled characterization · 3 tautological/mirror (below) · harness: real fail-closed plus live-tree / stubbed-CI smoke.

Differential is not independent of `GraphFold::apply` unless a second model exists. `m019_sem_model::Naive` is the only list-scan sibling; its `Retract` still first-live (`find_live`), while `GraphFold::apply` now invalidates every live residual (`fold.rs` Retract). Mix + naive can agree on a first-live Retract of CI residuals if that combo is rare (N_OPS=8).

## Suspect kill claims (honor-system / overstated)

Spot-checked 18 `killed` rows vs oracle body + `kill_mutation` text.

| Row | Verdict |
|---|---|
| `SEM-horn-kc` | **Honor-system.** Kill is “invert `known04`”. Product mutation `search_prems` at cut TT would fail the mint-at-head assert; “as_known_on never re-derives” is not independently asserted (early cut hides premises too). |
| `SEM-c01-decision` | **Partial.** `RuleTerm::constant("01.01.2028").is_none()` would die. “Fifth axis” unused would not. |
| `SEM-tr02-legal` | **Partial.** Collapsing `as_of` into publication would fail. Adding a fifth unused clock would not. |
| `SEM-seeded-det` | **Honor-system.** Kill is “compare EventIds or drop the seed loop” (test edit), not a product mutant. |
| `DUR-wrong-role-active` | **Partial.** Test only sets `manifest.active` = sealed name → `InvalidData`. Stacked “skip sequence + snapshot-ahead + …” is not what the test injects. |
| `DUR-sealed-trailer-lie` | **Partial.** Same: real trailer-bytes lie; kill lists neighboring skips. |
| `SEM-bitemporal-model` | **Partial.** `Fact::is_live_at` skip-`valid_from` / object-swap / Retract-first replay are plausible. Retract-all-residuals is **not** killed here (naive matches old first-live). |
| `SEM-cascade-order` | **Plausible** if Lazy reversed `rule_follow_ons` (order + names compared). Not an independent expected hop sequence. |
| `HAR-freeze-*` / `HAR-observe-fn` | **Governor needles**, not product meaning. |
| Others checked **plausible:** `SEM-ff5`, `SEM-ff6`, `SEM-correct-interval`, `SEM-true-on-vs-known-on`, `SEM-retract-all-live`, `SEM-keyed-retry-live`, `SEM-horn-correct`/`ORACLE-if16`, `SEM-asof-elig`, `SEM-casc-admit`, `SEM-define-stable`, `DUR-sidecar-atomic`, `DUR-dir-fsync` (test `persist_dir_fsync_count` delta). |

## Untested material `pub` / `pub(crate)` (zero test CALLS, inbound `include_tests=true`)

Graph-checked. Production-only callers do not count as oracles.

| Symbol | File:line | Why material |
|---|---|---|
| `GraphFold::claim_known` | `fold_query.rs:181` | Admit `claim: Some`; true if **any** fact (incl. invalidated) has that claim. No test caller. |
| `GraphFold::has_live_triple_except` | `fold_query.rs:203` | Same-batch Horn re-fire after scheduled Retract. Only `head_event` / `horn_fires_except`. |
| `GraphFold::fact_index_by_event_id` | `fold_query.rs:159` | **First live only.** `Correct`/`CorrectInterval`/`rule_follow_ons` still use it. |
| `leftover_halves` | `fold.rs:67` | CI prefix/suffix math; no direct test (happy-path residuals only). |
| `Runtime::admit_claim` | `admission.rs:169` | No test caller (emit path only). |
| `Runtime::retract` / `correct` / `derive` facades | `quantum.rs:467,492,527` | One inbound test: `statute_as_of_via_facade`. “Latest live subject+vf” unpinned. |
| `horn_fires` / `retract_derived` / `rule_follow_ons` | `eval.rs:30,151,186` | No test callers; covered only via `emit` outcomes. |

`Runtime::emit` has hundreds of test callers. `Runtime::admit` has none (pipeline only).

## Property / differential generators

`m017_determinism::ingest_mix` (N=24) and `common/sem::ingest_mix_n` (N=8 / persist N=4): LCG Assert / Retract-FIFO / CI 2015–2020 / Correct / TtAnchor. **Does not reach:** Retract-before-any-Assert (empty pool forces Assert); unknown relation; keyed retry; Correct of a **derived** `Behavior` id; same-TT query of an uncommitted cascade; empty log (`saw_assert` required). **Partial:** overlapping VT (CI vs `valid_from=2010+i%5`, not two overlapping Asserts on one (s,r)); rollover/migrate/export via `persist_schedule` (yes); Horn only `WithHorn` + genesis `inverse_knows`. Seeded tests compare stamps/triples, not user EventIds.

## Harness Python

Fail-closed **yes** for process claims: unknown kind/FSM/transition, unknown process relation, selftest unproven/vacuous, `test_matrix` HIGH on missing id / `killed`+`none`. **Not** product STCA. Smoke/tautology: `test_ci_green_on_this_tree` (stub cargo/tenant), `test_no_high_on_live_tree`, `test_discovers_core_checks`.

## Tautological / mirror tests

- `model_as_of_alias_is_live_at_max` — `Naive::as_of` **is** `live_at(MAX, vt)`.
- `known_at_equals_is_known_at_filter` — `known_at` **is** that filter (`fold_query.rs:89-98`).
- `assert_seed_budget` — env floor, not domain.
- Naive `Retract` mirrors pre-M021 first-live (risk of agreeing with a buggy fold).

## Missing oracles (priority)

**G-C01 HIGH** Correct/CorrectInterval of a CI event_id that already minted 3 residuals. Input: Assert 2010–∞, CI 2015–2020 (3 live rows, one id), then Correct that CI id. Expected: all residuals handled or fail-closed, not first-live only. Why: `fact_index_by_event_id` first-live (`fold_query.rs:159`). Evidence: graph + source.

**G-C02 HIGH** Retract/Correct of a Horn **derived** `Behavior` id, then `as_of` + `as_known_on` mid-anchor. Input: `knows` Assert → inverse; Correct the inverse event; query VT and TT. Expected: named residual/hide, no silent stale inverse. Mix never targets derived ids. Evidence: `rg` + `eval.rs` `retract_derived` keys on `caused_by`.

**G-C03 HIGH** `as_known_on` lists a not-yet-in-force fact (TT-only). Input: TtAnchor 2017; Assert `valid_from=2020`; `as_known_on(2017)` vs `as_of(2017)`. Expected: known contains, as_of empty (`known_at` ignores VT). TR-02 trap. Evidence: source `fold_query.rs:86-110`.

**G-C04 HIGH** User Assert EventIds identical after persist/open (not just Define / derived SHA). Input: one Assert; persist; open; compare log ids. Stamps omit EventId. Evidence: `rg` + `SEM-define-stable` / `det02`.

**G-C05 MED** Facade `retract(subject, vf)` with two live product facts, same subject+vf, different relations. Expected: documented “latest” only, other stays live. Evidence: `quantum.rs:467` + inbound tests.

**G-C06 MED** Keyed retry after Retract of a **CI** id (shared `delivery_key` on residuals). Expected: remint, not short-circuit on invalidated key. `m022` only retracts a plain Assert. Evidence: `apply` copies `delivery_key` onto residuals.

**G-C07 MED** Same-TT cascade vs `as_known_on` / fork mid-quantum. Input: `knows` Assert (inverse same ingested_at); persist omitted QO (`DUR-committed-prefix` exists); query publication of derived-only. Expected: atomic hide or named prefix. Evidence: graph `commit_user_cascade`; no combo oracle.

**G-C08 MED** Two overlapping Asserts same (s,r), different objects; `conflict_report_at` + `as_of` both windows. Mix uses one object then Correct. Evidence: read `sem.rs` generator.

**G-C09 MED** Clock-type negatives: `as_of(WallTime)` / `as_known_on(ValidTime)` must not compile. UI only `takes_valid_time(TT|Wall)`. Tests pass raw `u64` (`From<u64>`). Evidence: `clock_mix.rs`, `event.rs` From impls.

**G-C10 MED** ADR-090 TR-06 typed non-success (`Conflict`/`Abstain`/`MissingAnchor`) as product outcomes; TR-01 five-clock stamps unused. Only `UnknownRelation` / `IntervalPatchRejected`. Evidence: ADR-090 + `rg`.

**G-C11 MED** Empty-runtime `as_of` empty + persist/open identity (format `persist_empty_dir_*` exists; no live-set oracle). Evidence: mix requires Assert.

**G-C12 LOW** `claim_known` after last support Retract still admits `claim: Some(old)` (invalidated rows). Expected: decide + pin. Evidence: `claim_known` any-fact; `admit_claim` no tests.

**Crash / interleave gaps (classes, not %):** no dual-fault snapshot+WAL; no injected skip of real `sync_all` (counter only); no persist **during** `emit` cascade; no two-Runtime interleave of overlapping quanta (two-writer lock only); TR-08/09 retrieve/agent traps absent (pack frozen).
