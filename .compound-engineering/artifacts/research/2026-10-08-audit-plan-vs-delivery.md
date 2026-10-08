# Plan vs delivery (read-only, 2026-10-08)

Planning-plane audit. Lease SoT: `.kutha/STATE.md` (`L_delivery=M022-closed`, Active None, H5). CBM `kutha-graph` (11492 nodes; also duplicate `root-kutha-graph`); `check_index_coverage` metadata_match on cited crate/harness/matrix paths. Confirmed in graph: `GraphFold.as_of`, `Runtime.delivery_key_retry`, `segment::persist_log`, `store::replace_via_tmp`, unwired `SegmentIndex`, `Context.skip_emit`, `ff5_*`, `m021_*`, `m022_keyed_retry`. No cargo/uv/bench. Honeycomb stays Proposed. Dual CBM indexes are not SoT.

## 1. Milestone ledger (planned vs shipped)

| ID | GSD | Phases | Planned success | Shipped (code/docs) | Honest |
|----|-----|--------|-----------------|---------------------|--------|
| M001 | v0.01 overlay | 1–3 verify | Legal PIT S01–S03 | Pre-GSD; v0.01 FIT-01…05 over existing FF5 | Overlay ≠ delivery |
| H0–H5 | v0.03=H5 | 9–11 | Dogfood | Named; H5 lean+semantic gov | Closed process |
| M010 | — | — | Semantic open | `m010_semantic_open.rs` | Closed |
| M011 | v0.02 | 4–8 | S04–S08 semantic core | CORR/OUT/CSR/PROV/FIX tests | Closed; **no audit file** |
| M012a | v0.04 | 12–16 | F1–F3,F6–F8 | Log-native, stable refs | Closed; **no audit file** |
| M012 | v0.05 | 17–21 | F4/F5 dict facts | Allow/Rule/Adm/Act/DER | Closed; DER-03 residual spike |
| M013 | v0.06 | 22–30 | Baseline, recon, one write path | Facade, genesis, indexes | Closed; audit YAML still `gaps_found` |
| M014 | v0.07 | 31–33 | Typed clocks / `as_known_on` | Newtypes + `TtAnchor` | Closed |
| M015 | v0.08 | 34–35 | Known-on honesty | TT-only cut, anchor admit | Closed |
| M016 | v0.09 | 36–40 | T2–T6, F9, F16 | RuleAst, eval, FIXG-01 | Closed |
| M017 | v0.10 | 41–44 | Docs + replay + ingest | SNAP later M002; ingest repair | Closed |
| M018 | v0.11 | 45–48 | C01/Horn/oracles | Pins 2026-10-06; **no v0.11 audit** | Closed; PROJECT still “paused” |
| M002 | v0.12 | 49–55 | Durability protocol | SEG-01, SNAP-01, Rocks lease unwired | Closed; IO-01/VAC-01 intake |
| M019 | v0.13 | 56–62 | Coherence + matrix + DIFF | Lock, matrix, SEM oracles | Closed; MEM-01 partial |
| M020 | v0.14 | 63–67 | Audit correction | Persist tmp+rename; Horn/fold | Closed |
| M021 | v0.15 | 68–70 | Wedge pins + Retract-all | `m021_wedge_decisions.rs`, retract-all | Closed |
| M022 | v0.16 | 71–74 | Residual honesty | Keyed-retry live; skip_emit | Closed |
| M003 | — | — | Cypher AS OF skin | Absent | **Unleased** |
| M004 | — | — | Science fixture | Absent | **Unleased** |
| M005 | — | — | HNSW fence | Absent | **Unleased** |
| M006–M009 | — | — | Inflation guard | Unused ids | N/A |

v0.01 maps M001 in `gsd-auto-contract.md` but only verified fitness. Missing audits: v0.02–v0.05, v0.11. `.planning/MILESTONES.md` stops useful coverage after v0.13/v0.10 mix.

## 2. Consolidated residual register

Status: **C**=CLOSED with evidence, **O**=OPEN, **S**=STALE (docs still claim open/fixed wrongly), **D**=DUPLICATE of another R-id. **G**=owner-gated. **W**=material for legal wedge.

| ID | Item (first seen) | Now | G | W |
|----|-------------------|-----|---|---|
| R-01 | SNAP-01 sidecars/`File::create` (v0.10) | **C** M002 `replace_via_tmp` + fsync | | |
| R-02 | Fail-closed WAL / OPEN-01 (v0.10→M002) | **C** v0.12 | | |
| R-03 | Freeze-rocks line bypasses (v0.12) | **C** M019 `cargo_dep_constraint` | | |
| R-04 | C01 / FIXG-01 undecided (v0.10–v0.13) | **C** as *pin* M018/M021 pack-stamp + `fixg01_*`; pack crate **O** → R-20 | Y | Y |
| R-05 | HORN-KC / ASOF-ELIG / TR-02 (v0.13 owner list) | **C** matrix `killed`; fold unchanged | Y | Y |
| R-06 | Retract CI 1-of-3 (v0.14) | **C** M021 S02 `retract_correct_interval_*` | | |
| R-07 | Keyed retry after retract (audit P2) | **C** M022 `m022_keyed_retry.rs` | | |
| R-08 | pytest live `.kutha/` emit | **C** M022 `skip_emit` | | |
| R-09 | jobs>1 observe without overlap | **C** M022 overlap HIGH | | |
| R-10 | persist_log truncate-in-place (P1) | **C** M020 `replace_via_tmp` on segments | | |
| R-11 | CorrectInterval residual re-derive / live_triples / Retract `kutha:*` | **C** M020 S04 | | |
| R-12 | Rollover seal/manifest window | **O** deferred M022 | | |
| R-13 | GROUP-01 partial Typed/Dictionary group (v0.12/13) | **O** verdict (c); **D** handoff | | |
| R-14 | MEM-01 RSS bytes_per_fact (v0.12/13) | **O** characterization; not engine bound | | |
| R-15 | `open` O(N); no external chain | **O** | | |
| R-16 | Persister/`SegmentIndex` not wired | **O** `DUR-index-wiring` oracle [] | | |
| R-17 | Advisory `kutha.lock` / no lock-dir fsync | **O** | | |
| R-18 | IO-01 cloud writeback soak | **O** intake | | |
| R-19 | VAC-01 vacuum / litigation hold | **O** intake | Y | |
| R-20 | SPIKE-LAW-01 real-text corpus / ADR-090 pack | **O** thaw=`I-legal-pack-not-next` | Y | Y |
| R-21 | Five legal clocks F7 remainder | **O** TR-02 is mapping pin, not types | Y | Y |
| R-22 | Agent live∩eligible facade | **O** Phase 68 deferred | | Y |
| R-23 | Pi Durable / request_id | **O** out of wedge | Y | |
| R-24 | Cypher / HNSW / SIX-01 / ADR-080/093 / Consensus 103+ | **O** thaw map | Y | |
| R-25 | jsonl-as-export owner acknowledgement | **O** | Y | |
| R-26 | Dual-arg u64 tt/vt swap (v0.07) | **O** public API newtypes; Op storage u64 | | |
| R-27 | F8 no public fork-diff | **O** cosmetic | | |
| R-28 | DER-03 `inverse_knows` cascade | **C** documented residual spike + named test | | |
| R-29 | apply_dotenv mutates process env | **O** | | |
| R-30 | Gov: no worker timeout; cargo 180s vs rocks 420s; id-map substring; observe_cargo 101 flake | **O** | | |
| R-31 | `clippy -D warnings` 22 lints | **O** not CI gate | | |
| R-32 | Independent verify = honor-system | **O** (M020 codified re-verify-after-edit) | | |
| R-33 | WINDOWS #2–16 `open_count:11` | **S** many pre-fixed (PROC-02, lease grep, bench override) | | |
| R-34 | A-F09/11/12 “not in closed set” (semantic-gap) | **S**/untriaged; likely M002 overlap | | |
| R-35 | Whole-repo P3 (WAL tolerate-tail, receipts, head-dedup VT, digest_bytes, tenant skip, …) | **O** never leased | | |
| R-36 | Nyquist skip / SUMMARY frontmatter (v0.06) | **S** accepted debt | | |
| R-37 | Rocks `rocks_spike` still jsonl | **O** keep-as-spike | | |
| R-38 | ADR-092 license / SemVer 0.0.0 | **O** | Y | |
| R-39 | Honeycomb Proposed vs spike evidence | **C** as policy; not Accepted | | |
| R-40 | Bench host noise / M016 skip (historical) | **C** contract now records numbers | | |

Closed-in-later-milestone duplicates (do not re-lease): R-01–R-11, R-28, R-40. v0.13 “owner-gated still open” list is **S** vs M018/M021.

## 3. Contradictions / drift

| ID | Conflict |
|----|----------|
| G-D01 | `test-matrix.md` Counts: 76 rows / killed 66 (dated M021 S01). YAML: **77** `status:` / **67** killed (5 char, 4 spec-only, 1 gate). Extra killed: `SEM-retract-all-live`. `SEM-keyed-retry-live` is in YAML; MD recount stale. MD “gap owner = M019” while gap=0. |
| G-D02 | Living `.planning/REQUIREMENTS.md` is **only** six v0.16 ids. CONTEXT still names RETRACT-01, TR-02-01, C01-02, MATRIX-01, HYG/FC/PA/HF/PC, C01-01, HORN-KC-01, … Catalog collapse. |
| G-D03 | `PROJECT.md`: Current milestone **v0.14/M020**, “v0.11 paused”, Next “M018 S02 leased”, last_updated 2026-10-05. STATE/README/CHANGELOG: **M022 closed**. |
| G-D04 | `.kutha/ROADMAP.md` body: M021/M022 CLOSED; “Later milestones”: **M021 active**. Duplicate list numbers 14/15 (M022 then M003, then 14=M004). |
| G-D05 | Id map: v0.01 “closes M001”. ROADMAP M001 is pre-GSD S01–S03; v0.01 is FIT overlay. |
| G-D06 | `MILESTONES.md` has no v0.11/v0.14–v0.16 sections; RETROSPECTIVE stops at v0.02. |
| G-D07 | `legal-fixture-recon.md`: “C01 decision is **open** in ADR-013”. ADR-013/M021: pack stamp decided 2026-10-06/07. |
| G-D08 | STRATEGY `last_updated: 2026-10-04`; still “prove STCA as a graph database” / P0–P3 honeycomb investment vs one-slice delivery. |
| G-D09 | ADR README: 32 cells Proposed + P0 spike list — **aligned** with STATE `L_map=honeycomb-proposed`. Not a status lie. Spike inventory omits M020–M022 surfaces (inventory, not lease). |
| G-D10 | Dual CBM projects same root, different node counts (11492 vs 11159). |
| G-D11 | v0.13 audit still lists v0.11 reqs open / M018 paused. |
| G-D12 | README/STATE/CHANGELOG agree (M022-closed). Overlay files do not. |

## 4. Process-plane weight vs product (R5)

R5 (2026-10-01): ~40.8k `.planning` md + 12.3k CE + 6.0k harness py + 2.1k YAML vs **9.2k** product Rust incl. tests (~4.4:1 planning:product).

Now (line counts via path greps, not `wc`): product **src** ~8.3k (`kutha-runtime`+`kutha-common`); **tests+benches** ~17k (M002 `m002_seg01.rs` 1505 alone); harness `scripts/kutha_gov` ~5.5k; `.kutha` yaml+md ~2.8k; `docs/` ADR+process+arch ~3.5k; `.planning` **505 markdown files** (live overlay tiny; archives dominate); CE cards ~164×24 plus `papers.md` 1040. Product grew (mostly tests). Planning **file count** exploded. Living overlay (PROJECT/REQUIREMENTS/MILESTONES/WINDOWS) is stale — ceremony is **not** buying a current map. Governor fail-closed work (H5, matrix check, freeze-deps) is proportionate; GSD archive + literature store + WINDOWS ledger is not. Next lease should refresh the overlay or stop treating it as SoT (STATE already is).

## 5. REQUIREMENTS: no phase or no oracle

**Living file** (6 ids, none print a phase): DOC-AUDIT-01 (71, docs-only), KEY-RETRY-01 (72, cargo oracles), ADM-PIN-01 (72, characterization pin), SKIP-EMIT-01 / OVERLAP-01 (73, pytest), CLOSE-01 (74, process-only).

**Dropped from living REQUIREMENTS** (phase in CONTEXT only): RETRACT-01; TR-02-01, C01-02, MATRIX-01; HYG-01…04; FC-01…05; PA-01…03; HF-01…05; PC-01…04; DOC-06/07, REG-01, LANG-01; C01-01, HORN-KC-01, ASOF-ELIG-01, DET-02, HORN-CORR-01, CASC-01, ORACLE-01, GATE-01.

**Matrix `oracle: []`:** DUR-dead-code-fate, DUR-index-wiring, HAR-selftest-class, HAR-parallel-parity, BENCH-controller (GATE-01). Spec-only by design, not missing phases.

**No living REQ id** for R-12 rollover, R-20 SPIKE-LAW, R-21 F7, R-22 facade — they are deferred notes, not tracked requirements.
