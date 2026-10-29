---
phase: 245-bench-ab
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: parse tmp/bench-*.log `.n` lines; 2-run and 3-run medians; ratios
    exit: 0
  - cmd: ls --time-style=full-iso -l tmp/bench-*.log
    exit: 0
  - cmd: git -C tmp/bench-cdf1942 rev-parse HEAD; nproc; MemTotal from /proc/meminfo
    exit: 0
  - cmd: uv run kutha-gov precommit (before this file existed)
    exit: 1
  - cmd: uv run kutha-gov precommit (after); both FAIL/HIGH checks clear
    exit: 0
read_only_inputs:
  - docs/architecture/benchmark-baseline.md
  - .kutha/STATE.md
  - tmp/bench-old-1.log
  - tmp/bench-old-2.log
  - tmp/bench-head-1.log
  - tmp/bench-head-2.log
  - tmp/bench-head-3.log
---

# Phase 245 verification — interleaved bench A/B vs cdf1942

Independent context. Plan and summary not read; bench not rerun. The lease names M168-S01
performance truth: A/B of HEAD vs `cdf1942`, then `bytes_per_fact` to `n/a` on zero RSS delta.

| Claim | Result |
|-------|--------|
| 30 table cells (6 keys x old1/old2/HEAD1-3) equal the logs | Pass, exact integer match after comma removal |
| Old is `cdf1942`, separate target dir | Pass: `tmp/bench-cdf1942` at `cdf1942bf`, log binary inside `tmp/bench-target-cdf1942` |
| Order Old1, HEAD1, Old2, HEAD2, HEAD3 | Pass: mtimes 20:20:56, 20:22:24, 20:25:30, 20:26:32, 20:27:22 |
| Host 12 cores / 47 GB | Pass: `nproc` 12, MemTotal 47 GB |
| Medians (2 old, 3 HEAD) | Pass, 4 of 6 exact. Two are `.5` and printed rounded: replay_check old 11,129,976.5 to 11,129,977; bytes_per_fact old 13,682.5 to 13,682. No stated ratio moves |
| ingest **+12.6%** | 127,469,929 / 113,244,540 - 1 = +12.56%, pass |
| as_of **3.32x** | 303,355 / 91,335 = 3.3213, pass |
| open **-3.0%** | 3,231,444,822 / 3,332,235,768 - 1 = -3.02%, pass |
| persist **-67.0%** | 3,045,344,997 / 9,215,293,745 - 1 = -66.95%, pass |
| replay_check **+1.1%** | 11,249,035 / 11,129,976.5 = +1.07%, pass |
| Old reproduces 2026-10-01 `as_of.n` | Pass: baseline 83,195 ns; old 84,146 / 98,524 ns (the stated 84-99 us) |
| Old reproduces 2026-10-01 `bytes_per_fact` | Pass: baseline 13,674 B; old1 13,674 B exact, old2 13,691 B (13.7 KB) |
| HEAD1 4.6 ms `as_of.n` outlier explained | Pass: HEAD1 4,607,638 ns; its 2N/4N 674,438 / 1,300,945 ns = 674 us / 1.30 ms; HEAD2/3 303,355 / 285,853 = 303/286 us. HEAD1 scaling is non-monotone (n beyond 2n), so the load reading fits the numbers |
| Not B3, not competitor | Pass: stated verbatim; no competitor engine and no multi-hop workload in the logs |

## Caveats

- `precommit` was exit 1 with two HIGHs, both one cause: no phase verification file existed
  when it ran (`verification-independent` glob-empty, `artifact-budget` min_hits). Re-run
  after this file existed: exit 0, both checks high=0, nothing else was red.
- Comparability is weaker than the table implies: HEAD logs add `fold_index_mode` and
  `persist_append.n`, and `events_per_assert` is 3.031 against 3.0002 on old. The section does
  not say the two harnesses differ, so the ingest and persist ratios may carry that too.
- HEAD `bytes_per_fact` is 0 B in all three logs, so its median 0 is degenerate, not a measurement.
- Spread: persist HEAD 2.54-4.76 s over three runs; a 2-run old median is weak evidence.

**PASS.**
