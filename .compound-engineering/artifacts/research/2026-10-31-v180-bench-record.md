# v1.80 bench record — test honesty (M187)

Date: 2026-10-31. Base: `019aa69` (lease commit, before the code changes). Post: the working tree at close
(tests + `security.rs` error mapping + `pack_lifecycle.rs` replay counter).
Command, both arms: `KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline`.
Separate target dirs (`tmp/bench-pre-target` for the base worktree, `target/` for the tree). Eight interleaved
samples per arm (pre/post alternated within each iteration).

## Host state

The host was under external load during the run: `load average 28.62` at the start (other sessions running
`cargo test` for prime-agent and another workspace), down to `6.14` at the end. That is an `uptime` observation,
not a log field. Absolute times are therefore not comparable across arms; the spread below is the evidence.

## Numbers (ns; 8 samples per arm)

| metric | pre median | pre range | post median | post range | median delta |
|---|---:|---|---:|---|---:|
| `ingest_plain.n` | 145,261,167 | 101,418,260–412,192,147 | 114,620,266 | 102,108,116–244,647,726 | −21.1% |
| `as_of.n` | 79,789 | 73,096–101,109 | 86,030 | 69,850–137,286 | +7.8% |
| `as_of.2n` | 190,380 | 151,243–250,897 | 164,181 | 152,043–192,007 | −13.8% |
| `ingest_keyed.n` | 155,374,643 | 125,150,019–211,064,472 | 135,433,444 | 107,923,224–190,712,570 | −12.8% |

## Reading

- The ingest spread is 4× inside one arm; the medians move in *both* directions across metrics. Under this
  load the medians are not a measurement of the change.
- **No hot-path file changed.** `git diff --name-only 019aa69 -- crates/kutha-runtime/src/{fold,pipeline,commit,store,quantum,hydrate}.rs`
  is empty. The only `src/` changes are `security.rs` (4 lines, error mapping) and `pack_lifecycle.rs` (9 lines,
  a counter read on the replay path).
- Gate reading: `as_of.n` +7.8% is inside the 10% stop line and inside the observed spread; `ingest_plain.n`
  is not cited as an improvement. No bench number in this record is used as a regression or improvement claim.

## Raw logs

`tmp/bench-pre-1..8.log`, `tmp/bench-post-1..8.log` (gitignored). The base worktree and its target dir were
removed after the run; the numbers above are the durable copy.
