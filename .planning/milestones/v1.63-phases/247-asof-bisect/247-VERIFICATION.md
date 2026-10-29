---
phase: 247-asof-bisect
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: git log -1 --format='%H %P %s' cec5b64
    exit: 0
  - cmd: git show cec5b64 -- crates/kutha-runtime/src/fold.rs | grep -nE 'LiveFactFilter|is_dictionary_rel|dictionary_rels'
    exit: 0
  - cmd: git diff crates/kutha-runtime/src/fold.rs
    exit: 0
  - cmd: grep -rn 'dictionary_rel_flags' crates/
    exit: 0
  - cmd: git merge-base --is-ancestor 7e79912 HEAD; same for cec5b64 and cdf1942
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 1
read_only_inputs:
  - docs/architecture/benchmark-baseline.md
  - tmp/bisect-parent.log
  - tmp/bisect-cec5b64.log
  - tmp/bisect-head-2k.log
  - tmp/bisect-head-fix-2k.log
  - tmp/bench-head-fix-10k.log
  - .kutha/STATE.md
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
---

# Phase 247 verification — as_of bisect (M169-S01)

Independent verifier context. The phase plan, its summary, and the phase context file were not read. Inputs: the 2026-10-29 M169 section of the benchmark note, five bench logs, the lease, the two runtime sources the cause names, and executed commands. Benches were not re-run.

## Checked

| Claim | Result |
|-------|--------|
| `7e79912` as_of.2n/4n 34,203 / 68,708 ns, ingest_plain.n 20,550,772; `cec5b64` 129,171 / 212,146, 82,151,209 | Exact match in `tmp/bisect-parent.log` and `tmp/bisect-cec5b64.log` |
| HEAD pre-fix 2n/4n 112,149 / 220,061, ingest_plain.n 20,526,065; post-fix 50,704 / 102,021, 35,123,706 | Exact in `tmp/bisect-head-2k.log` and `tmp/bisect-head-fix-2k.log` (the note's table does not name the second log) |
| N=10000 post-fix as_of.n 123,019, 2n 231,191, 4n 715,674, ingest_plain.n 128,494,291 | Exact match in `tmp/bench-head-fix-10k.log` |
| old median 91,335, pre-fix 303,355, ratios 1.35x and 3.32x | Both medians are the M168 table in this note; 123019/91335 = 1.347, 303355/91335 = 3.321 |
| `cec5b64` is the first bad commit | Its only parent is `7e79912`, measured good on the same key, and nothing lies between, so the good-parent argument closes. Both are ancestors of HEAD |
| Cause is a per-fact HashSet probe | `git show cec5b64` adds `LiveFactFilter::Product`, whose `keep` calls `fold.is_dictionary_rel`, i.e. `HashSet<TermId>::contains`, for every fact; `as_of` -> `live_at` -> `live_facts_at(Product)` -> `keep` per fact (`fold_query.rs`) |
| Fix = dense flags, semantics unchanged | Uncommitted diff swaps the set for `Vec<bool>` with the identical `starts_with("kutha:")` test at the same TermId index; out-of-range returns false, as membership did |
| Section does not claim B3 | Explicit: "This is not B3 and not a competitor comparison"; the 1.35x residual is an accepted cost, not a gate |

## Caveats

- Post-fix 2n `ingest_plain.n` is 35,123,706 vs 20,526,065 pre-fix at the same N = 1.71x. "Ingest unchanged" is scoped to N=10000 (128,494,291 vs 127,469,929 median, +0.8%); the 2n rise is unexplained.
- `as_of.n` at 2N is a single first call (18,094 post-fix vs 31,878 parent); as the note says, only 2n/4n carry the claim.
- Bisect width: parent, bad commit, pre-fix and post-fix HEAD, plus the earlier M168 A/B. Enough for first-bad by the good-parent argument; no independent mid-range sample.
- `fold.rs:1074` and `tests/m035_replay_dict_bind.rs:2` still say `dictionary_rels` after the rename; comment-only.
- `uv run kutha-gov precommit` (exit 1): this file passes `verification-independent`, and the only remaining HIGH is the sibling `248-fix-or-accept/248-VERIFICATION.md` (no frontmatter, cites plan and summary paths). Not this phase's artifact; not editable from this verifier.

## Verdict

**PASS.** Every table value traces to a log, `cec5b64` is first bad by a measured good parent, the per-fact HashSet probe is visible in the commit and still gates `as_of` through the Product filter, and the fix preserves the filter predicate. No B3 claim. Governor CI, ADR status, and capability are not claimed here.
