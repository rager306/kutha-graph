---
phase: 231-hop-workload
plan: 01
status: complete
completed: 2026-10-23
---
# 231-01: query IR Scan + Hops

RED then GREEN. No Op/wire/fingerprint. `eval.rs` / `fold_query.rs` / `leapfrog.rs` / `csr.rs` / `quantum.rs` / `fold.rs` bodies untouched. CBM `query_ir.rs` `not_tracked` (new); hot paths `metadata_match`. No `index_repository`.

## RED (`19fb9e2`)
Stub `execute` → `Ok(Triples([]))`; stub `try_hops_named` → dummy Hops. `cargo test -p kutha-runtime --offline --test b5_hop_workload -- --exact b5_hop_scan_equals_as_of_ff5` **101**. All six named tests failed: scan empty vs `as_of` bag `[(15,9,16),(18,9,19)]`; hops `expected Nodes, got Triples([])`; fail-closed `unwrap_err` on stub `Ok`. Shuffle nonempty assertion would also kill an empty==empty pass.

## GREEN
`9b9f460` Scan: `live_at`, `sort_unstable`, `limit==0` unlimited. `try_hops_named` is `dictionary().id` only (UnknownTerm start, UnknownRelation rel, `rels/k mismatch`). Out-of-range interned TermId → UnknownTerm. Scan + fail-closed **0**.
Hops: `k==0` → `Nodes({start})`; else `BTreeSet` frontier; unlabeled `csr_lease_at.neighbors`; typed `typed_csr_lease_at.edges_out` filtered (len 1 repeats, len==k per hop); sorted unique ends + limit. Nested-join helper stays in `b5_hop_workload.rs`. `cargo test -p kutha-runtime --offline --test b5_hop_workload` **0** (6/6).

## Gaps
Typed CSR keeps support multiplicity; hop answers collapse to a node set (diamond + duplicate supports). Scan is a sorted bag (`as_of` multiplicity kept). VT window-end is exclusive (`vt >= valid_to`). TT retract uses `invalidated_at <= tt`. Unlabeled 1-hop drops labels. Per-hop `rels` length is not a named oracle (shape is fail-closed only).

## Commits
1. `19fb9e2` test RED stub + six oracles
2. `9b9f460` feat Scan + named lookup
3. this: feat hop walk + SUMMARY + CHANGELOG (docs-coupling)
