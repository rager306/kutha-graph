# Test-strategy prior art (M019 S04 / Phase 59)

Date: 2026-10-05. D-02(e) options with one-paragraph verdicts. Sources verified via jina `search_web` + `read_url` (not built-in WebSearch). This note is not an ADR and does not lease S05–S07.

Repo facts used below: single crate pair (`kutha-common`, `kutha-runtime`), research stage, no new Cargo deps, seeded LCG already in `m017_determinism.rs`, EventIds are UUID v7, `cargo-mutants 27.1.0` is installed on this host (checked `cargo mutants --version`). D-02(d): cargo-mutants is a verifier/controller tool only — never a repository dependency and never a governor gate.

## Options (verdict per D-02(e))

**Property-based testing (proptest / Hypothesis).** proptest is Hypothesis-like shrinking for Rust; current crate on crates.io is 1.11.0, MSRV 1.88, still passively maintained ([proptest README](https://github.com/proptest-rs/proptest)). It wants a `proptest` dev-dependency and a strategy/shrink corpus. **Verdict: defer as a crate dep.** The fitness this repo already needs (seeded mix of Assert/Retract/Correct/anchors, compare derived triples not EventIds) is the hand-rolled LCG in `m017_determinism.rs`. A future DIFF-01 (M019 S07) should copy that pattern, not add proptest/bolero. Revisit only if an owner accepts a new test-only crate.

**Mutation testing (cargo-mutants).** cargo-mutants injects bugs and reports mutants that tests do not kill ([mutants.rs](https://mutants.rs/), [sourcefrog/cargo-mutants](https://github.com/sourcefrog/cargo-mutants)). **Verdict: verifier tool only, already on the host (27.1.0).** D-02(d) already allows this: the verifier may run a mutation list per slice; the matrix records `kill_mutation` text. Do not add it to CI, `Cargo.toml`, or `observe_cargo`. A green `cargo mutants` run is not a honeycomb capability.

**Deterministic simulation testing (TigerBeetle VOPR / FoundationDB Simulation).** FoundationDB Simulation runs a whole cluster in one deterministic thread (Flow swaps time/net/disk); they cite ~1e12 CPU-hours and ~10:1 real-to-sim time ([FoundationDB testing](https://apple.github.io/foundationdb/testing.html)). TigerBeetle DST is protocol-aware (consensus + storage invariants inside the replica); VOPR is the long-running DST fleet, distinct from the later non-deterministic Vortex harness ([protocol-aware DST](https://tigerbeetle.com/blog/2026-08-20-protocol-aware-dst), [Vortex 2025-02-13](https://tigerbeetle.com/blog/2025-02-13-a-descent-into-the-vortex)). **Verdict: does not fit this spike.** Kutha is not a replicated cluster and has no simulated clock/net/disk layer. Failpoints already in `m002_seg01` / `m002_snap01` are the local cousin. A VOPR-class harness would be a later research cell, not S04–S07.

**Crash-consistency testing (ALICE / CrashMonkey).** ALICE (OSDI'14) builds crash states from a syscall trace and checks application recovery ([Pillai et al.](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-pillai.pdf)). CrashMonkey + Ace (OSDI'18) do bounded black-box crash testing of POSIX file systems ([utsaslab/crashmonkey](https://github.com/utsaslab/crashmonkey)). **Verdict: adapt the idea, not the tools.** This repo already has failpoint crash matrices (`rollover_crash_matrix_reopens_to_prefix`, `migrate_crash_matrix_reopens_without_loss`) and tmp+rename oracles. Kernel modules, 20 GB disks, and ext4/btrfs workloads are out of scope. Remaining holes (exact-frame-end truncate, wrong-role active, two-writer) are S05/S06 named tests, not CrashMonkey.

**Differential testing.** McKeeman-style: same input, two implementations, any output mismatch is a bug ([Wikipedia: Differential testing](https://en.wikipedia.org/wiki/Differential_testing)). **Verdict: fits, no new dep.** DIFF-01 (M019 S07) is fold vs CSR lease vs clean rebuild on a seeded LCG log — two in-process oracles, not a second engine. Already used in the small: `lazy_as_of_matches_eager_after_same_ingest`, `det01_*` (sorted triples). Do not vendor Graphiti/Neo4j as a cross-check (D1 / D6).

**Model-based testing.** Generate tests from an EFSM (e.g. ModelJUnit: guard + action per transition, [Artho et al.](https://people.kth.se/~artho/papers/modeljunit-long.pdf)). **Verdict: defer.** The product fold is the model; a second Java/XState machine would drift. Harness FSM (`fsm.yaml`) is process, not a product test generator. If M018 ever decides Horn/C01, a tiny explicit state table for those rules could be a characterization oracle — not this slice.

**Approval / snapshot tests.** Capture output, re-approve on change ([approvaltests.com](https://approvaltests.com/)). **Verdict: reject as a durability or semantic oracle.** Snapshots hide decision changes inside a blob and fight EventId instability (UUID v7). Characterization tests must say so in the fn name or doc comment (D-02(b)), with field asserts, not golden files. Fine for a future pretty-printer of receipts; not for commit/segment/Horn.

## What this repo keeps now

Decision oracles (named `#[test]`, mutation recorded). Characterization oracles (labelled; may be replaced when the owner decides). Seeded property/differential (LCG, no new dep). Crash/failpoint oracles. Kill-tests (verifier-authored). Governor oracles (dictionary checks). Bench as a **controller** gate, not a coverage number.

## Could not verify

- TigerBeetle **VOPR** internals (scheduler, seed format, fleet config) beyond the public DST/Vortex posts above. The 2025-02-13 page is Vortex, not a VOPR spec.
- ALICE source tree / current maintainers; the OSDI'14 PDF claims were read, not reproduced.
- Whether `cargo-mutants 27.1.0` has been run on this tree (not run in S04; not a gate).
- proptest 1.11.0 crate page was not re-fetched beyond the GitHub README + crates.io snippet from search.
