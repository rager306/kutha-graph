---
phase: 273-enrichment-admission-without-log-replay
independent: true
status: passed
verifier_model: codebuddy/deepseek-v4.1-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m185_enrichment_admission --test m178_enrichment --test m180_replay_meta --test m182_meta_threats
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/enrichment.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m185_enrichment_admission.rs
  - .kutha/STATE.md
---

# Phase 273 verification — enrichment admission leases

The first independent verifier **blocked** this slice and was right on all three counts: the cache-key lease was never updated on commit (the tests passed only because the emit pre-check still scanned the log), admission cloned the whole key set per write (O(K) per proposal), and the source predicate used `event_by_id`, which also sees compacted rows, so admission was broader than replay.

Fixed before close: `check_proposal` is read-only and the caller inserts after commit; admission borrows the leases; source existence uses the visible-log id set; the emit pre-check uses the same leases so the write path no longer scans the log.

A second verifier round passed the fixes and found two residual gaps, also closed: `intern` appended `Op::Define` outside the commit choke point, so a live lease was missing Define ids (fail-closed, but not a faithful mirror) — now every append goes through `append_indexed`; and the checkpoint test did not actually create a checkpoint — now a real `checkpoint_sealed_prefix` store proves a tail proposal citing a dropped source is rejected by admission and replay alike.

Recorded limit: replay still exempts proposals placed before the first checkpoint from source existence, because the rewrite may legitimately drop the cited product rows. That is documented at the call site and bounded to derived data.

Final gate: enrichment admission 6/6, workspace 832 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0 with `run_high=0 run_low=0`.
