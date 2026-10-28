# Phase 231 — B3-lite hop workload (M157 S01)

Source: 2026-10-23 scout (CBM `kutha-graph` indexed 2026-10-23T12:38:37Z + `rg` arbiter), lease `46b7114`, research §§9–11. Discuss skipped; decisions locked.

## Goal

A handwritten typed query IR over existing read APIs (`live_at`, `typed_csr_lease_at` / `csr_lease_at`), equal to `as_of` on FF5/statute and to a nested join of `live_at` triples on a chain+diamond, plus a self-relative 2-hop bench. No parser, planner, Op, wire, fingerprint, or thaw. Honeycomb stays Proposed.

## Locked decisions

1. New `crates/kutha-runtime/src/query_ir.rs`. `pub type Workload = QueryIr`. `QueryIr` interned plain data (`Clone/Debug/PartialEq/Eq`; **no serde** — FoldDiff pattern, not on-log): `Scan { tt: TransactionTime, vt: ValidTime, limit }` (`limit==0` unlimited; `tt==MAX` equals `as_of(vt)`); `Hops { start: TermId, rels: Vec<TermId>, k: u32, tt, vt, limit }` with `rels` empty = unlabeled any-rel, len 1 = same rel every hop, len==k = per-hop, else error. `k==0` → `Nodes({start})`. Named ctor `try_hops_named(&Runtime, …)` uses `dictionary().id()`, never `intern()` (`intern` is `&mut` and appends Define).
2. `pub fn execute(rt: &Runtime, q: &QueryIr) -> Result<QueryRows, QueryError>` only — not `&GraphFold` (no intern map; leases/`dict.len()` live on Runtime; keeps `fold_query.rs` untouched). Scan: `live_at`, sort, limit (sorted **bag**; as_of multiplicity kept). Hops: `BTreeSet` frontier; unlabeled = `csr_lease_at.neighbors`; typed = `typed_csr_lease_at.edges_out` filtered by hop rel, objects set-deduped. Answer = sorted unique **end nodes** (not paths). Diamond and support multiplicity collapse. `QueryError::{UnknownRelation{name}, UnknownTerm{name}}`; relation Display copies RuntimeError (`unknown relation {name} (not in allowlist)`); miss is intern-map, not admit. Out-of-range TermId → UnknownTerm. Re-export from `lib.rs`. Do not edit bodies of `eval.rs`, `fold_query.rs`, `leapfrog.rs`, `csr.rs`, `quantum.rs`.
3. Oracles in `tests/b5_hop_workload.rs` (in-memory `Runtime::default()` only; no PersistScratch; do not migrate old tests). Nested-join helper is test-local. (a) Scan vs sorted `as_of` on FF5 + statute names. (b) k-hop vs nested join over `live_at` on chain+branch+diamond. (c) unlabeled 1-hop == untyped `neighbors` (labels dropped). (d) PIT: edition window-end or retract; 2-hop from A differs at two cuts (VT and TT). (e) xorshift shuffle; answers equal and nonempty.
4. Do **not** edit `benches/baseline.rs` (29 expected keys). New `[[bench]] name="hop_k" harness=false` (`benches/hop_k.rs`); duplicate parse/put (no shared module that touches baseline). Metrics `hop2.{n,2n,4n}` (2-hop, fan-out 4, 3 layers) and `csr_lease.{n,2n,4n}` as ratio vs a **new** calibration loop **and** 2n/n. Default `KUTHA_HOP_N=256`. Band in `docs/architecture/benchmark-baseline.md` (self-relative; no absolute-ns budget). 10% gate: 9 alternating A/B pairs vs BASE `46b7114`, `taskset -c 0`, direct `--bench` binaries; one `CARGO_TARGET_DIR` per arm; BASE via `git worktree add` then `remove`. hop2 has no BASE twin.
5. Cargo: never `cargo build --workspace`; iterate `cargo test -p kutha-runtime --test b5_hop_workload --offline`; workspace tests + `uv run kutha-gov ci --jobs 1` only at close. Extra target dir only BASE A/B, deleted after. `cargo clean` + empty `tmp/` at end. No leftover worktrees. Do not touch `/root/prime-agent`.
6. Docs at close: ADR-041 Evidence (Kutha hop number; leapfrog still off query path; Proposed); ADR-040 Evidence (hops consume existing CSR lease); benchmark-baseline hop section; SEM-m157-* rows; CHANGELOG Product with PRE/POST ingest/as_of and hop band; README layout word; stca-guide one sentence. Deferred: competitor baseline (owner+docker+20 GB); planner B5; parser B6; Verus V2.

## RED

`rg 'QueryIr|QueryIR|k_hop|query_ir' crates` = 0; CBM `search_graph`/`search_code` = 0. `benches/baseline.rs` has no csr/hop/leapfrog metric. `leapfrog_intersect` CBM inbound 3 (all `leapfrog.rs` tests); `rg` only `lib.rs` re-export + `leapfrog.rs`. Stub `execute` → `Ok(QueryRows::Triples(vec![]))`. Named tests (fixed): `b5_hop_scan_equals_as_of_ff5`, `b5_hop_nested_join_oracle`, `b5_hop_unlabeled_equals_untyped_neighbors`, `b5_hop_cuts_pit`, `b5_hop_fail_closed_unknown_relation`, `b5_hop_shuffle_deterministic`. Shuffle must assert nonempty (else stub empty==empty passes).

## Known traps

- `intern()` mutates the log; query lookup is `dictionary().id()`. Typed CSR keeps multiplicity; hop answers are a node set. `as_of` is unsorted; Scan sorts. `baseline.rs` has **no** calibration loop (absolute ns); hop_k adds one. CBM misses many `Self::` edges; `rg` arbiter. Fingerprint hashes facts (`fold_query.rs:221`); do not edit. Cited paths `metadata_match`.

## Gates

- Bench: A/B as above; >10% `ingest_plain.n` or `as_of.n` is a stop. Cite numbers in CHANGELOG.
- Close: `cargo test --workspace --offline --no-fail-fast` exit 0 and `uv run kutha-gov ci --jobs 1` `run_high=0` in `231-VERIFICATION.md` (independent fresh verifier writes kill-tests; ≤60 lines).
- Artifact budget: ≤2 PLAN ≤150, 1 SUMMARY ≤60, 1 VERIFICATION ≤60, 1 CONTEXT ≤80.
- Stop: new Op/wire/fingerprint; hot-path body edits; ADR-000 D1–D10 conflict; bench >10%.
