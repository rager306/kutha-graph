# M186 evidence correction — independent verification (2026-10-31)

Subject: commit `957a8d5` "docs(bench): correct M186 A/B2 medians and record
reproducibility limits", plus follow-ups `2070e1e` (range cells) and `5c899e2`
(cause wording), branch `work/main-line`, repo `/root/kutha-graph`. Re-verified
at HEAD `5c899e2`.
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
| 16 | `git log --oneline -4` (HEAD `5c899e2`) | 0 |
| 17 | `uv run python tmp/m186-verify/check_five.py` (five-cell re-check) | 1 (two cells still wrong, see below) |
| 18 | `uv run kutha-gov precommit` (after the re-verification edits) | 0 (105 checks OK) |

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

## Findings (original pass at `957a8d5`; status re-checked at HEAD `5c899e2`)

**M1 — FIXED.** The A/B2 head range for `ingest_keyed.n` said 113.2-139.5M; the
retained minimum is 119,831,330 (119.8M). HEAD now reads 119.8-139.5M.

**M2 — FIXED.** The A/B2 head range for `persist_append.n` said 3.9-15.2M; the
retained minimum is 4,443,268 (4.4M). HEAD now reads 4.4-15.2M.

**M3 — FIXED.** The `as_of.n` pre median now reads 78536.5 (exact value). The
`ingest_plain.n` pre median still prints 126665566 from an exact 126665565.5; that
is a half-up display round, not a wrong number.

**M4 — HALF FIXED.** The A/B1 base `as_of` spread is now 73.1-253.1k. The upper
bound is right (253,099 is now inside the range), but the lower bound is still
wrong: the retained minimum is 72,133 (72.1k), and 73.1k is the second-smallest
sample (73,095). Correct cell: 72.1-253.1k.

**M5 — FIXED.** The correction note now says the four medians "did not match the
retained logs — they appear to have been taken from unsorted positional samples"
and names only `ingest_plain.n` as the exact positional case (118,824,960 = mean
of positional samples 3 and 4). That matches the evidence. The samples artifact
carries the same wording.

**O1 — NOT FIXED.** `docs/process/beautifulrust.md` now says "the six production
panic sites", adding `pipeline.rs:451` and `pipeline.rs:537`. Those two are real
production `expect` calls, but the count is still wrong: HEAD has **18**
production `expect` sites (before the top-level `#[cfg(test)]` boundary, excluding
the test-only `quantum_invariants.rs`): `eval.rs:14,15,17,20,21,23,24` (7),
`gpml.rs:494`, `leapfrog.rs:52,53`, `pack_manifest.rs:95`, `pipeline.rs:451,537`,
`quantum.rs:434,437,602`, `wal.rs:210,215`. No production `unwrap` remains (0).
The row should either say 18 or stop counting: "the four production `unwrap` sites
are now `expect` calls", which is what the original four referred to.

**Open at HEAD `5c899e2`: M4 (lower bound) and O1 (count).** M1, M2, M3 and M5 are
closed. Both open items are documentation-only.

## Re-verification of the five fixed cells (HEAD `5c899e2`)

`tmp/m186-verify/check_five.py` parses the doc cells and compares them with the
logs; it exits 0 only when all five are correct (currently 1).

| # | cell | doc at HEAD | recomputed | verdict |
|---|------|-------------|------------|---------|
| 1 | `as_of.n` pre median | 78536.5 | 78536.5 | OK |
| 2 | `ingest_keyed.n` head range min | 119.8M | 119,831,330 -> 119.8M | OK |
| 3 | `persist_append.n` head range min | 4.4M | 4,443,268 -> 4.4M | OK |
| 4 | A/B1 base `as_of` spread min | 73.1k | 72,133 -> 72.1k | MISMATCH |
| 5 | beautifulrust panic-site count | six | 18 production expects | MISMATCH |

The wording change at `5c899e2` (medians "did not match the retained logs";
positional cause exact only for `ingest_plain.n`) was checked against the logs and
the old values: it is accurate.

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
the structural argument, and the new cause wording verify against the retained
logs. Three of the five follow-up cells are fixed; a fourth is half fixed (upper
bound only).

**BLOCKED (narrow, documentation-only) on two cells at HEAD `5c899e2`:** the
A/B1 base `as_of` spread lower bound (doc 73.1k, logs 72.1k) and the
`beautifulrust.md` panic-site count (doc six, tree 18; or restate the row as the
four former production `unwrap` sites). Fix those two and the phase verification
becomes `passed`; no benchmark rerun is needed.
