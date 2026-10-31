# M186 evidence correction — independent verification (2026-10-31)

Subject: commit `957a8d5` "docs(bench): correct M186 A/B2 medians and record
reproducibility limits", branch `work/main-line`, repo `/root/kutha-graph`.
Verifier context: fresh subagent, did not write the correction. Repository was
read-only for this pass; the only repo files changed are this report and
`.planning/milestones/v1.79-phases/274-meta-lease-overhead-a-b-evidence/274-VERIFICATION.md`.
No lease existed (`.kutha/STATE.md`: Active Milestone/Slice = None); no product
change was needed or made.

Scratch: `tmp/m186-verify/recompute.py` (gitignored; retained so the recorded
command stays re-runnable).
Disk at start: `df -h /root` -> 8.7G available (97% used).

## Commands and exit statuses

| # | command | exit |
|---|---------|-----:|
| 1 | `git show --stat --oneline 957a8d5` | 0 |
| 2 | `git show 957a8d5 -- docs .kutha/STATE.md CHANGELOG.md .planning/...` | 0 |
| 3 | `git cat-file -t 05dfea0; git cat-file -t 4db8770` | 0 / 0 |
| 4 | `git worktree list` (only `/root/kutha-graph`) | 0 |
| 5 | `uv run python tmp/m186-verify/recompute.py` | 0 |
| 6 | `rg -n 'env::var\|std::env' crates/kutha-runtime/src` | 0 |
| 7 | `git log -S log_ids --oneline -- crates/` | 0 |
| 8 | `ls -d tmp/ab-probe-target; ls tmp/ab-probe` (both absent) | 2 |
| 9 | `git diff --stat 05dfea0..HEAD -- crates/kutha-runtime/src/{fold,leapfrog,fold_query}.rs` | 0 (empty) |
| 10 | `git diff --stat 05dfea0..HEAD -- crates/kutha-runtime/benches/` | 0 (empty) |
| 11 | `rg -n '−6\.2%\|118824960\|121145614\|455890543'` (excl. `tmp/`, `.planning-archive/`) | 0 (matches explained) |
| 12 | `rg -n '10% gate'` (repo-wide) | 0 (matches explained) |
| 13 | `node .cursor/gsd-core/bin/gsd-tools.cjs validate health --raw` | 0 (`healthy`) |
| 14 | `uv run kutha-gov precommit` | 0 (105 checks OK, no nonzero `high`) |
| 15 | `git status --porcelain` (final) | 0 (only this report + the VERIFICATION file) |

No cargo was run: no code changed. No `graph index`, no `.kutha` writes.

## Recomputation

Parsed the 44 retained logs (`tmp/ab-{base,head,pre,head2}-{1..6}.log`,
`tmp/probe-{on,off}-{1..10}.log`) as `metric value ns` triples; every log prints
`validated yes`. All 44 logs name the same bench binary
`baseline-ad506c2ed323b589`, consistent with "bench source unchanged". Medians are
sorted-sample medians; even counts are the mean of the two middle samples.

| arm | ingest_plain.n | as_of.n | as_of.2n | ingest_plain.4n | ingest_keyed.n | persist_append.n |
|---|---:|---:|---:|---:|---:|---:|
| A/B1 base `4db8770` | 124,571,494.5 | 73,632.0 | 152,187.5 | 420,149,406.0 | 118,710,653.5 | 5,888,089.0 |
| A/B1 head | 127,965,659.0 | 77,824.5 | 177,007.5 | 488,745,933.5 | 125,038,532.5 | 9,890,538.0 |
| A/B2 base `05dfea0` | 126,665,565.5 | 78,536.5 | 160,599.0 | 425,505,687.5 | 110,857,378.5 | 5,612,319.5 |
| A/B2 head | 121,352,717.0 | 79,908.5 | 191,120.5 | 450,096,298.5 | 124,302,089.5 | 7,299,392.5 |
| toggle ON | 135,091,050.5 | 72,775.5 | 168,223.0 | 512,482,062.5 | 145,490,354.0 | 6,727,847.5 |
| toggle OFF | 121,275,306.0 | 76,522.5 | 172,055.5 | 455,133,695.5 | 128,636,652.5 | 7,859,327.0 |

All 36 medians equal the per-sample record in
`.compound-engineering/artifacts/research/2026-10-31-m186-ab-samples.md`; the 44
per-sample rows in that file also equal the logs value-for-value (0 mismatches).

A/B2 ratios (head/base - 1):

| metric | recomputed | stated in docs | match |
|---|---:|---:|:--:|
| `ingest_plain.n` | -4.19% | -4.2% | yes |
| `as_of.n` | +1.75% | +1.7% | yes |
| `as_of.2n` | +19.00% | +19.0% | yes |
| `ingest_plain.4n` | +5.78% | +5.8% | yes |
| `ingest_keyed.n` | +12.13% | +12.1% | yes |
| `persist_append.n` | +30.06% | +30.1% (table ratio 1.301) | yes |

A/B1: +2.72% / +5.69% / +16.31% (docs +2.7% / +5.7% / +16.3%). Toggle ON vs OFF:
+11.39% / -4.90% / -2.23% (docs +11.4% / -4.9% / -2.2%). The "reproduced at
+16.3% in the other A/B" sentence is correct.

The old published values are confirmed wrong: `118824960` = mean of A/B2 head
samples 3+4 (119,019,318 and 118,630,602), i.e. the unsorted positional median;
`121145614` = mean of samples 2+4 (120,202,792 and 122,088,437, truncated).
`72495` and `455890543` are not derivable from any subset of the retained six
samples, so the "unsorted positional samples" diagnosis is exact for one old
value, near-exact for a second, and unproven for two (see M5).

## Findings

**M1 (gap, canonical doc).** `docs/architecture/benchmark-baseline.md:129` gives
the A/B2 head range for `ingest_keyed.n` as 113.2-139.5M. Retained minimum is
119,831,330 (119.8M); the maximum 139,478,938 matches. The 113.2M value appears in
no retained log and in no other arm.

**M2 (gap, canonical doc).** `docs/architecture/benchmark-baseline.md:130` gives
the A/B2 head range for `persist_append.n` as 3.9-15.2M. Retained minimum is
4,443,268 (4.4M); the maximum 15,189,087 matches. 3.9M appears in no retained log.
The now-removed decision bullet repeated "head 3.9-15.2M".

M1 and M2 are the only substantive numeric mismatches found. Both predate commit `957a8d5`
(introduced in `57d03c2`) and were not in the correction's recomputed scope
(medians and ratios). They are documentation-only and fixable by writing 119.8M
and 4.4M. Because a canonical table still carries two false numbers, the phase
verification is `gaps_found`, not `passed`.

**M3 (cosmetic).** The A/B2 table prints the `as_of.n` pre median as 78536 (exact
78536.5) and `ingest_plain.n` as 126665566 (exact 126665565.5). Half-values are
rounded inconsistently across cells; other cells keep `.5`.

**M4 (cosmetic).** The A/B1 "spread" row prints base `as_of` as 73.1-85.7k while
the same table keeps sample 4 at 253,099. The prose explains the outlier but the
cell does not say it is the cluster range.

**M5 (provenance narrative).** `four of six head medians ... computed from
unsorted positional samples` is supported for `ingest_plain.n` exactly and for
`ingest_keyed.n` approximately; `as_of.n` 72,495 and `ingest_plain.4n` 455,890,543
cannot be reconstructed from the retained six. The corrected numbers are right;
the causal story is only partly checkable.

**O1 (outside the enumerated checks; same commit).** `docs/process/beautifulrust.md`
changed "only 4 are production sites" to "the four production panic sites are
`expect` calls". The direction is right — no production `unwrap` remains (the
`leapfrog.rs:103`, `materializer.rs:88` and `allow.rs:119` unwraps are inside
`#[cfg(test)]`) — but "the four production panic sites" undercounts:
`pipeline.rs:451` and `pipeline.rs:537` are production `expect` calls too, so HEAD
has six. Consider "the four production `unwrap` sites are now `expect` calls"
instead. This does not touch the M186 numbers.

## Gate framing, reproducibility, stale copies

- Gate framing: correct everywhere. AGENTS.md gates `ingest_plain.n` and
  `as_of.n`; `as_of.2n` is never called "the 10% gate" after the correction, and
  `ingest_keyed.n` +12.1% is labelled secondary in `.kutha/STATE.md`, `CHANGELOG.md`,
  `.planning/MILESTONES.md`, the milestone requirements, and the phase summary.
  Other "10% gate" hits in the repo are unrelated historical sections (Phase 22,
  D-07, hop_k) and older CHANGELOG entries, not M186 claims.
- Reproducibility: no env-gated `log_ids` toggle exists. The env reads in
  `crates/kutha-runtime/src` are `KUTHA_FOLD_INDEX_MODE` (`fold.rs:217`, added in
  `027a2ce`, long before `05dfea0`), `KUTHA_MAX_CASCADE`, the migrate/tenant
  binaries, and the harness allow-file path; `pipeline.rs:245` inserts `log_ids`
  unconditionally. `tmp/ab-probe/` and `tmp/ab-probe-target/` are gone. The docs
  say "recorded evidence, not a reproducible procedure". Note: the ON/OFF labels
  rest only on file names; nothing inside the probe logs distinguishes them.
- Structural: empty diff for `fold.rs`, `leapfrog.rs`, `fold_query.rs` across
  `05dfea0..HEAD`; empty benches diff. Both base commits resolve; only the main
  worktree exists.
- Stale copies: `−6.2%` survives only in (a) the intentional "(was −6.2%)" history
  in `benchmark-baseline.md:136`, (b) the audit review that quotes the old values,
  (c) an unrelated M037 CHANGELOG entry (line 3013). `118824960`, `121145614`,
  `455890543` survive only in the audit review. No canonical document cites them
  as current.

## Verdict

**PASS on the correction.** Every A/B2 median, every ratio in the six-metric
table, the A/B1 and toggle numbers, the gate framing, the reproducibility limit,
and the structural argument verify against the retained logs.

**BLOCKED (narrow, documentation-only) on the phase document** because two
head-range minima in the same canonical table are still false (M1: 113.2M vs
119.8M; M2: 3.9M vs 4.4M). Fix those two cells and the verification can be
re-issued as `passed`; no benchmark rerun is needed.
