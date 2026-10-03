---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: "5/5"
exit: pass
commands:
  - cmd: "git diff 4d669ca..HEAD -- Cargo.toml Cargo.lock crates/*/Cargo.toml"
    exit: 0
  - cmd: "isolated CARGO_TARGET_DIR tmp/wt-target/{v08,prefix,head}; KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline"
    exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test m017_verify44_killtests -- --test-threads=1 --nocapture (HEAD and 4d669ca)"
    exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test m017_ingest_scaling -- --test-threads=1 (HEAD)"
    exit: 0
  - cmd: "same --test m017_ingest_scaling at 1866721"
    exit: 101
  - cmd: "cargo test --offline -p kutha-runtime --test ff5_legal_pit -- --test-threads=1"
    exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - docs/architecture/benchmark-baseline.md
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/snapshot.rs
  - crates/kutha-runtime/benches/baseline.rs
  - crates/kutha-runtime/tests/m016_rule_eval.rs
  - crates/kutha-runtime/tests/m012_multi_hop.rs
  - crates/kutha-common/src/rule.rs
---

Independent verifier. Shared `CARGO_TARGET_DIR` reused the v0.08 bench binary; numbers below are isolated-target reruns (`tmp/wt-target/{v08,prefix,head}`).

| tree | ingest_plain.n | as_of.n |
|------|---------------:|--------:|
| v0.08 (`2382432`) | 98571210 | 291484 |
| `4d669ca` | 314629975 | 300479 |
| HEAD | 104834889 | 280212 |

**PERF-01:** M016 `cfe4a1f` (Horn eval replaces inverse cascade) is the scan culprit. `4d669ca` ingest ~3.2× v0.08. Scaling oracle RED at `1866721` (exit 101), GREEN at HEAD (exit 0).

**PERF-02:** HEAD ingest +6.3% vs same-host v0.08 (cap +10% = 108428331). Manifests/lock empty vs `4d669ca`. `horns_by_premise` is `#[serde(skip)]`; snapshot JSON has no key. Mutates only `push_fact` / `rebuild_horn_lease` (incl. `bind_reserved` + `rebuild_hot_maps`) / `invalidate_fact_at`.

Kill-tests (a–g) pass on HEAD and `4d669ca`; V44 dumps identical (`late_rule_retro_fire=false`). FF5 pass. Semantics unchanged.
