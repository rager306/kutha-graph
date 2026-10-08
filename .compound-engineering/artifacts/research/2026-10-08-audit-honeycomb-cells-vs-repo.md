# Honeycomb cells 020–093 vs repo (read-only audit)

Date: 2026-10-08. Auditor: CBM `kutha-graph` generation `2026-10-08T07:14:05Z` (11492 nodes). `list_projects` first; `search_graph`/`trace_path`/`get_code_snippet`/`query_graph`/`search_code`; `check_index_coverage` on cited paths = `no_recorded_issue` / `metadata_match` (best-effort, not completeness). Literals/absences: `rg`. No cargo/uv/bench/gitnexus analyze; no git writes; ADRs stay Proposed.

STATE: `L_map=honeycomb-proposed`, `L_delivery=M022-closed`, `L_capability=ff5-green`. Thaw: Cypher=M003, HNSW=M005, six dicts=SIX-01, 080/081=security after M003, full 090=`I-legal-pack-not-next`, 093=M004, Consensus 103+=owner. YAML: all `map: Proposed`, audited `capability: none`.

Legend: Code Y/partial/N. YAML = `delivery`/`capability`. Honest = yaml vs code (not vs ADR title).

## 1. Per-cell

| Cell | Code | YAML | Honest? | Gap |
|------|------|------|---------|-----|
| 020 | Y: 2-crate workspace; no repo `ports/` | spike/none | Y (P0) | No `app-shell`/`modules/<slice>`; isolation is crate split not Port gates |
| 021 | N (`rg` PackInstall=0) | map-only/none | Y | No pack install/activate/retire |
| 022 | N slice crates | map-only/none | Y | No hexagon-inside-slice; 090/093 cannot copy a skeleton |
| 023 | Y: `test-matrix.yaml` 76 rows; gov `test-matrix`; `kutha_gov/honeycomb.py` | map-only/none | Y | Matrix map ≠ coverage. ADR-023 body still lists TR-02 open (stale vs killed `SEM-tr02-legal`) |
| 030 | N Cui `h_i(v)` | map-only/none | Y | Cascade `max_cascade` is 014, not max-convolution |
| 031 | N query admit/queue | map-only/none | Y | No leftover-V scheduler |
| 040 | partial: `Materializer` build/unload; `csr_lease_at` mounts+unloads | spike/none | Y | No `apply(delta)`/`snapshot`/`rollback`; typed CSR is parallel, not IVM |
| 041 | partial: `CsrLease`/`TypedCsrLease`; `leapfrog_intersect` | spike/none | Y | Graph-checked: leapfrog callers = 3 unit tests only. Horn=`eval.rs` scan/`horns_by_premise`. No GraphBLAS |
| 042 | N (freeze tests in gov only) | frozen/none | Y | HNSW not leased |
| 043 | N planner | map-only/none | Y | No cost model / lease catalog |
| 050 | partial: 5 `kutha:` rels; YAML genesis seed | **frozen**/none | Y | Not six kinds. `allow.rs` YAML+`.env` `set_var` still seeds **new** logs; admit uses fold `kutha:allows` |
| 051 | partial: `check_admission` + action bind | map-only/none | Y | P0 allow-list/policy blob (`deny` utf8), not OCap grants |
| 052 | N | map-only/none | Y | No enrichment pack / CA model cache |
| 060 | partial: `replay_check` | spike/none | Y | Fingerprint + `check_tt_anchors` + `caused_by` + Horn `horn_behavior_reproducible`. No `Unavailable`. `provenance_fingerprint` not in `replay_check` |
| 061 | partial: `fork_at` prefix clone | spike/none | Y | `rg` fork_diff/GED=0. No named refs/merge |
| 062 | N (default off) | map-only/none | Y | No regimes pack; effect identity unimplemented |
| 070 | N Cypher; Y fold cuts | **frozen**/none | Y | Query = Rust `as_of`/`live_at`/`as_known_on`/`csr_lease_at`. No parser. See (g) |
| 071 | N | map-only/none | Y | No BM25/ANN/BANKS |
| 080 | N ABAC; `tenant.rs` = H2 JSONL ingest | **frozen**/none | Y | Name clash only: harness tenant ≠ 080 slice |
| 081 | N | frozen/none | Y | No WASM |
| 090 | partial: FF5+facade+TR-02+C01 pins | **frozen**/none | Y | Mapping pin, not ontology/CTV/AST/practice |
| 091 | N | map-only/none | Y | No RVF |
| 092 | Y name Kutha; N SPDX | map-only/none | Y | README: license undecided |
| 093 | N | frozen/none | Y | Science pack frozen |

Cell tensions (not yaml lies): D041-2 “MATCH uses LFTJ” vs 070 “P0 is sorted-row intersect, not MATCH”. ADR-090 F13 “TR-02 enforced by types” vs `Op.valid_from: u64` and `as_of(impl Into<ValidTime>)` accepting raw years. D10 wedge vs `I-legal-pack-not-next` (priority ≠ thaw). D3 vs YAML genesis: fold is SoT after first Allow; YAML is still bootstrap root of trust for empty logs.

## 2. Wedge readiness (ADR-090)

**Have (physics, not pack):** `ff5_legal_pit` `as_of(2015)≠as_of(2021)` on `inForceAs`; `facade_statute` + `examples/statute_as_of` public API; `m021` TR-02 three moments (force=VT, publication=`TtAnchor`/`as_known_on`, decision=`as_of`); `m016_golden_fixture` C01 sunset inexpressible as Horn Const; C01 `appliesTo` pack Assert; `conflict_report_at`; `fork_at`; `replay_check`; allowlist includes `inForceAs`. Matrix `SEM-tr02-legal` killed.

**Still missing for a minimal credible legal-agent demo (not full ontology):** (1) answer envelope TR-06 (`Unknown`/`Conflict`/`Abstain`/`MissingAnchor`) — `as_of` returns `Vec<(TermId,TermId,TermId)>` only (`rg` Abstain=0 in crates); (2) consumer that does not treat support-live `as_of` as “in force” when Horn derived is ineligible (`asof_elig01`); (3) clock-safe write/query so a wall year cannot be passed as VT; (4) user-facing query besides Rust fns — only `kutha-tenant` (harness) and the example; (5) CC/CTV/`edition_ast_at`, AmendmentEvent, practice overlay, applicability (S3–S6); (6) TR-07/08/09 agent+hybrid retrieve; (7) typed non-success on missing `TtAnchor` is `Option` not `MissingAnchor`; (8) corpus fixtures as test data without claiming a pack. Do **not** thaw full 090 to ship (1)–(4)+(8).

## 3. Logical/semantic gaps

| ID | Sev | Evidence | Scenario / proposed oracle |
|----|-----|----------|----------------------------|
| G-B01 | high | graph+source | `as_of` lists ineligible derived after last support retract (`asof_elig01`). Demo that prints `as_of` as “norm in force” is false. Oracle: legal envelope lists eligible/conflict/abstain; must not invert killed pin |
| G-B02 | high | read-in-source | `as_of(2015u64)` vs `as_known_on(2015)`: compiler cannot stop publication year as VT (`Op` clocks still `u64`). Oracle: compile_fail or runtime reject mixing wall into `as_of` |
| G-B03 | high | rg+source | No query API besides Rust methods / example / harness tenant. Oracle: HTTP/Cypher/GPML file exists — today false |
| G-B04 | high | rg | TR-06 outcomes absent. Oracle: query type includes `Abstain`/`Conflict`/`Unknown` |
| G-B05 | med | graph-checked | `leapfrog_intersect` inbound = unit tests only. Oracle: Horn/MATCH path calls it |
| G-B06 | med | rg | No `fork_diff`. Oracle: public diff of two `fork_at`/`as_of` cuts is nonempty edit script |
| G-B07 | med | read-in-source | Strict replay ≠ D060-1: Horn-only execution replay; missing rule blob can skip (`!rule_version.is_empty()`); no `Unavailable`; provenance hash unused by `replay_check`. Oracle: missing CA input → typed Unavailable, not silent Ok |
| G-B08 | med | read-in-source | YAML genesis+`.env` `set_var` (`allow.rs`). Oracle: empty dir without YAML cannot admit `inForceAs` until log `Allow`; changing YAML after genesis does not admit (already fold — pin `admit_consults_fold`) |
| G-B09 | med | read-in-source | Untyped CSR drops labels/multiplicity (ADR-040). Oracle: two rels same endpoints, untyped neighbor set collapses; typed lease does not |
| G-B10 | med | read-in-source | HORN-KC: Horn fires at TT=`MAX`. Historical knowledge-cut mint is not ingest law (`known04`). Oracle: legal derivation “as known on D” ≠ ingest-at-head |
| G-B11 | low | read-in-source | Policy = blob `deny`; not 050 Policy dictionary / 051 grant events |
| G-B12 | low | read-in-source | Stale docs: ADR-023 “TR-02 still open”; `semantic-contract-validation.md` “execution replay still absent”. Oracle: those sentences gone or dated superseded |
| G-B13 | low | rg | 021 missing vs 090 pack shape — cannot install legal pack even if thawed |

Not-verified-by-failing-test: G-B02/G-B04/G-B06/G-B07 (Unavailable) — no kill-test asserts the desired fail-closed product.

## 4. Candidate next milestones (value-to-wedge, thaw-safe)

Next lease is a controller decision (STATE). Ranked:

1. **Legal answer envelope + demo CLI (not Cypher).** Ship a `norm_view(vt, wall)` that returns triples **plus** eligibility, `conflict_report_at`, and TR-06 statuses; thin bin wrapping `Runtime` (extend example, not 070). **Non-goals:** M003 Cypher, 090 ontology, HNSW, ABAC, six dicts, Cui, RVF.
2. **Clock-safe facade (TR-02 at call sites).** Typed `Op` VT/wall; no raw `u64` `as_of` meaning publication. **Non-goals:** fifth core axis, law-nexus five peer clocks, legal corpus pack.
3. **Fork-diff of live triples (061 restriction).** Set-diff/`as_of` edit script between two prefix forks — “what changed in the statute,” not GED. **Non-goals:** named git-like refs, CRDT merge, 091 capsules.

Do not lease: Cypher, HNSW, SIX-01, 080/081, full 090, 093, Consensus 103+, GraphBLAS, WASM. Fixture-only FZ-44 quotes remain test data (`m016` pattern), not a pack thaw.
