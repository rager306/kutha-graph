---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed-with-findings
score: 11/11
exit: 0
commands:
  - {cmd: "cargo fmt --all --check", exit: 0}
  - {cmd: "cargo test --workspace --offline", exit: 0}
  - {cmd: "uv run kutha-gov precommit --check test-matrix", exit: 0}
read_only_inputs:
  - crates/kutha-runtime/tests/common/sem.rs
  - crates/kutha-runtime/tests/m019_sem_diff.rs
  - crates/kutha-runtime/tests/m019_sem_model.rs
  - crates/kutha-runtime/tests/m019_sem_meta.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/csr.rs
  - docs/architecture/test-matrix.yaml
  - CHANGELOG.md
---
# 62-VERIFICATION — M019-S07 (independent)

Judged `git log da655d8..HEAD` at `d64dab3`. Did not read CONTEXT/PLAN or the phase SUMMARY file. CBM `list_projects`; cited tests `not_tracked`. No `index_repository`. Scratch `tmp/p62v-mut`.

**Verdict: PASS-WITH-FINDINGS.** Eight claimed kills reproduced (rc≠0). Honeycomb Proposed. Owner-gated rows still blocked. Named oracles (FF5/FF6/M010–M017/m002/m019_dur/rocks) not weakened. Matrix 55/6/0/6/4/1=72. Gates at that HEAD: fmt 0; clippy no `-D` 0 (61 warnings); workspace tests 0; rocks 0; py/precommit/ci/selftest 0; pytest 181.

**Claimed kills:** CSR `relation=0`; `is_live_at` skip tt; Lazy reverse follow-ons; `push_fact` LWW; `replay` skip Correct; `encoded_log` pop last; Assert `valid_from=2010`; Horn fires on Correct.

**Own survivors (weak oracles, not a false killed row):** (1) hydrate drop-last QO/Define — facts fingerprint unchanged. (2) sealed CRC disabled — oracle tampers jsonl, not middle-segment bytes. (3) Correct skip-invalidate — char only pinned inverse stays live.

**Medium:** persist cap `min(12)` ignores `KUTHA_SEEDS`; not every failure path shrinks; isolated `--test-threads=1` sum 37.8 s. **Low:** model 208 physical lines; `SystemTime` in persist dirs; VT 2009 always outside; BehaviorFree persist 0% Horn.

Executor follow-up after this file is out of scope for this verdict.
