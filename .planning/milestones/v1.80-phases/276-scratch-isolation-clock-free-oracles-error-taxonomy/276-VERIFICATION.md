---
phase: 276-scratch-isolation-clock-free-oracles-error-taxonomy
independent: true
status: passed
verified_at: 2026-10-31
verifier_model: cline/cline-pass/deepseek-v4.1-flash-1017
commands:
  - cmd: "cargo test -p kutha-runtime --test m182_meta_threats --test m183_digest_falsifiers (this round; 10 + 5 passed)"
    exit: 0
  - cmd: "the same two files with provenance.rs mutated: per-prefix tracker rebuild = O(n^2) (run once per file)"
    exit: 101
  - cmd: "cargo test -p kutha-runtime --test m175_security_policy post_policy_rels_shape_failure_is_query_rejected -- --exact (mutant: RelsShape back to PolicyDenied)"
    exit: 101
  - cmd: "mutants reverted sha256-identical, then the two test files and the 11-binary targeted run (74 passed)"
    exit: 0
  - cmd: "uv run kutha-gov precommit"
    exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/tests/m182_meta_threats.rs
  - crates/kutha-runtime/tests/m183_digest_falsifiers.rs
  - crates/kutha-runtime/tests/m175_security_policy.rs
  - crates/kutha-runtime/tests/common/mod.rs
  - crates/kutha-runtime/src/pack_lifecycle.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/security.rs
---
# Phase 276 verification — scratch isolation, clock-free oracles, taxonomy
Round 3, fresh context, no part in the implementation. **Verdict: `passed`** on
TEST-276-01, -02, -03. The round-2 gap on TEST-276-02 is closed: the oracles
count the product replay gate, so an excluded O(n^2) regression would be red.

**TEST-276-02.** `replay` bumps a thread-local `REPLAY_CALLS`;
`meta_replay_probe` resets it, runs the same `check_meta_replay` that
`replay_check` calls, and returns `(replays, declarations)`. m182 asserts
`replays == 4000`, `declarations == 0`; m183 asserts 400/1600 of each and
`4x packs == 4x declarations`. Neither file has `Instant`, `elapsed` or
`Duration`.

Mutant: `provenance.rs` swaps `lifecycle.replay(&event.op)?` for
`lifecycle = PackLifecycleTracker::from_log(&events[..=index])?`. m182 fails on
its count, **8002000 vs 4000** (prefix sum over 4000 installs; exit 101, 30 s).
m183 fails on 160400 vs 400 — the rebuild also replays `StoreBlob`, since
`replay` counts before `check` skips non-lifecycle ops. Restored sha256
`5e7ac18a…`; green 10 + 5, exit 0.

**TEST-276-01.** No `"tmp/` or `format!("tmp` in `crates/`; the 38
`.join("tmp")` sites carry a pid/nanos/seq suffix or only canonicalize;
`PersistScratch::new` uses `{label}-{nanos}-{pid}`, and the 13 `scratch()`
sites leave no fixed name to delete. Governor `fixed-tmp-persist-dir` high=0;
the concurrent-pair falsifier is inherited from round 2.

**TEST-276-03.** `from_query_error` maps `UnknownTerm` and `RelsShape` to
`QueryRejected`, keeping `UnknownRelation`. Reverting `RelsShape` to
`PolicyDenied` fails the test (exit 101, `got PolicyDenied { reason: "invalid
relation shape rels=2 k=3" }`); restore 7/7 green.

**Scope.** No product behaviour change. `cargo test --workspace` is inherited
from round 2 (green rerun; run 1 flaked in the pre-existing `m019_dur_a` lock)
and was not run here.
