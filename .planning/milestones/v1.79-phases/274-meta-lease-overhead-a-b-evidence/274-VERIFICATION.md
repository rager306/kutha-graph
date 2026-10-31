---
phase: 274-meta-lease-overhead-a-b-evidence
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: uv run python tmp/m186-verify/recompute.py
    exit: 0
  - cmd: uv run python tmp/m186-verify/check_five.py
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - docs/architecture/benchmark-baseline.md
  - CHANGELOG.md
  - .kutha/STATE.md
  - .planning/MILESTONES.md
  - .planning/milestones/v1.79-REQUIREMENTS.md
  - .compound-engineering/artifacts/research/2026-10-31-m186-ab-samples.md
  - docs/process/beautifulrust.md
---

# Phase 274 verification — lease overhead A/B (re-verified after the correction)

**Scope history, kept.** The first independent verifier blocked this slice and
was right: the scope claim was false. `4db8770` is the v1.77 close, so the first
A/B measured **v1.78 only** while the documents said v1.77 + v1.78, and the base
worktree had been deleted. Fixed before close: a second interleaved A/B against
`05dfea0` (v1.76 close) isolates both milestones, both base commit ids resolve,
and spreads are stated.

**Retraction.** An earlier version of this file said "the recorded numbers
matched the raw logs exactly". That sentence was **wrong**. Four of the six A/B2
head medians had not been computed from sorted samples; recomputing them moved
`ingest_plain.n` from −6.2% to −4.2%, `as_of.n` from −7.7% to +1.7%,
`ingest_plain.4n` from +7.1% to +5.8%, and `ingest_keyed.n` from +9.3% to +12.1%.
Only `as_of.2n` (+19.0%) and `persist_append.n` (+30.1%) were already right.

**What this re-verification checked.** A fresh script recomputed all 36 medians
(6 metrics x 6 arms) from the 44 retained logs as sorted-sample medians; every
value matches `.compound-engineering/artifacts/research/2026-10-31-m186-ab-samples.md`.
A/B2 ratios recompute to −4.19%, +1.75%, +19.00%, +5.78%, +12.13%, +30.06%
(documents: −4.2%, +1.7%, +19.0%, +5.8%, +12.1%, +30.1%); A/B1 to +2.72%,
+5.69%, +16.31% (stated +2.7%, +5.7%, +16.3%); the toggle to +11.39%, −4.90%,
−2.23% (stated +11.4%, −4.9%, −2.2%).

The gate framing is now correct: AGENTS.md gates `ingest_plain.n` and `as_of.n`
with a 10% stop, no document calls `as_of.2n` or `ingest_keyed.n` "the 10% gate",
and `ingest_keyed.n` +12.1% is labelled secondary in `.kutha/STATE.md`,
`CHANGELOG.md`, `.planning/MILESTONES.md`, the milestone requirements and the
phase summary. No canonical document still cites 118824960, 121145614 or
455890543; those three now survive only in the audit review, and the old ratios
only in the intentional "(was ...)" history.

The reproducibility claim holds: no env-gated `log_ids` toggle exists in the tree
(`pipeline.rs` inserts unconditionally; the env reads in `crates/kutha-runtime/src`
are the pre-existing `KUTHA_FOLD_INDEX_MODE`, `KUTHA_MAX_CASCADE`, the migration
and tenant binaries, and the harness allow-file path), the probe target directory
and binary are gone, and the documents call the toggle run "recorded evidence, not
a reproducible procedure". The structural argument is verified: the
`05dfea0..HEAD` diff on `fold.rs`, `leapfrog.rs` and `fold_query.rs` is empty, as
is the benches diff.

**Re-verification of the fixes — two correction rounds.** Round 1 (`2070e1e`,
`5c899e2`) fixed the A/B2 `as_of.n` pre median (78536.5), the `ingest_keyed.n`
head minimum (119.8M = 119,831,330), the `persist_append.n` head minimum
(4.4M = 4,443,268), and reworded the median cause as unproven rather than
certain. Round 2 (`79c1640`) fixed the A/B1 base `as_of` spread lower bound
(72.1-253.1k; retained minimum 72,133) and the panic-site inventory (18
production `expect` calls with the file:line list). All five follow-up cells were
re-checked against the logs by `tmp/m186-verify/check_five.py`, which now exits
0. The count was independently recounted before the first top-level
`#[cfg(test)]` in each file: 7+1+2+1+2+3+2 = 18, every listed line is an
`expect` call, and `quantum_invariants.rs` is included only through
`quantum.rs:867` under `#[cfg(test)]`. No production `unwrap` remains (0).

**Status.** `passed`. No open gaps: the six A/B2 medians and ratios, the A/B1
and toggle numbers, the gate framing, the reproducibility limit, the structural
argument and both correction rounds verify against the retained logs. The
changes are documentation-only; no benchmark rerun was needed or done.

**Gate.** `uv run kutha-gov precommit` exit 0 (`run_high=0`), run after the
corrections; `check_five.py` exit 0. No cargo run: no code changed. Command log,
exit statuses and the recomputation table:
`.compound-engineering/artifacts/reviews/2026-10-31-m186-correction-verification.md`.
