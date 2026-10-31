---
phase: 274-meta-lease-overhead-a-b-evidence
independent: true
status: passed
verifier_model: codebuddy/deepseek-v4.1-flash
commands:
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - docs/architecture/benchmark-baseline.md
  - CHANGELOG.md
  - .planning/phases/274-meta-lease-overhead-a-b-evidence/274-01-PLAN.md
  - .kutha/STATE.md
---

# Phase 274 verification — lease overhead A/B

The first independent verifier **blocked** this slice and was right: the recorded numbers matched the raw logs exactly, but the scope claim was false. `4db8770` is the v1.77 close, so it already contained v1.77's per-append upkeep; the A/B measured **v1.78 only** while the doc and CHANGELOG said "v1.77 and v1.78". The base worktree was also deleted, so the baseline arm could not be audited from disk.

Fixed before close: a second interleaved A/B against `05dfea0` (v1.76 close) isolates v1.77 + v1.78; base commit ids are recorded in the doc because the worktrees are removed; spreads are stated; and the doc now distinguishes the pre-registered keys from the ones that move further.

The second run changed the conclusion in an important way: `ingest_plain.n` is **−6.2%** (faster) against the pre-v1.77 base, but **`as_of.2n` is +19.0%**, reproduced at +16.3% in the other A/B. That is recorded as an **open, unexplained residual** above the 10% gate. It is not attributed to the leases (they act on append; `as_of` is a read), and the bench exercises no meta write at all, so the benefit side is cited from the v1.77 install measurement rather than measured here.

Final gate: `cargo test --workspace` green; `kutha-gov ci --jobs 1` exit 0.
