---
phase: 251-b3-workload-and-baseline
independent: true
status: passed
verifier_model: genspark/deep-seek-v4.1-flash
commands:
  - cmd: git diff -- crates/ docs/ADR/
    exit: 0
  - cmd: awk '/^hop2\.walk\.(n|2n|4n)_ns /{print}' .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/*.log
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - .compound-engineering/artifacts/research/2026-10-30-b3-workload.md
  - .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/b3-hop-k-pinned-1.log
  - .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/b3-hop-k-pinned-2.log
  - .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/b3-hop-k-pinned-3.log
  - .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/b3-hop-k-pinned-4.log
  - .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/b3-hop-pinned-meta.txt
  - crates/kutha-runtime/benches/hop_k.rs
  - .planning/STATE.md
---

# Phase 251 verification — B3 workload and baseline

Independent verifier PASSed the fixed workload and baseline: artifact matches `hop_k.rs` (three layers, fan-out 4, TT=MAX, VT=2010, typed Hops/prepare; no parser/HNSW/planner/Cypher); the 4-run pinned table matches the archived logs; floors are median ratio ×1.10 (2.28, 4.61, 2.22); no product code or ADR change.

Original findings F1/F2/F4/F5/F6 were addressed by re-baselining with `taskset -c 0` and four runs, archiving raw logs under `.compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/`, correcting the plan file list, and recording gate commands. No HNSW/planner/Cypher thaw.
