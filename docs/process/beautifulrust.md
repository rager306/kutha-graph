# BeautifulRust: advisory Rust review policy

Status: **advisory only**. BeautifulRust is not product truth, not an ADR source, not a
governor check, and not a CI gate. `docs/ADR/` and the `.kutha` process ledger remain the
authority. A BeautifulRust finding is evidence for a review, not a verdict.

Package: [rager306/BeautifulRust](https://github.com/rager306/BeautifulRust), fork of
`morganlinton/BeautifulRust` (MIT). The verified install is commit `f863cd3` in
`$ALLSPARK_HOME/.prime/agent/git/github.com/rager306/BeautifulRust`; use
`ALLSPARK_HOME=/root/.allspark-home`. The package README documents the nested-home bug:
without the override the launcher creates a second home with an empty `auth.json` and the
provider rejects the request with HTTP 400.

## What to use

| Surface | Use | Boundary |
|---|---|---|
| `rust-best-practices` | Checklist when writing or changing Rust: ownership, errors/panics, API shape, unsafe, tests, performance. | Generic guidance only; ADR-000 D1–D10 and the current slice win. |
| `rust-review` | Prioritized review of a file, crate, or diff. Use its `Must fix / Should fix / Consider` shape and `file:line` evidence. | Add Kutha invariants; do not accept library-API ceremony for an internal `publish = false` crate. |
| `rust_review` Python inventory | A quick map of files, public API, panic sites, unsafe, tests, and clones. | It counts inline `#[cfg(test)]` code and `.clone()` without judgment. It is a map, not a finding list. |
| `lints/lints.toml` | Source of ideas for a future, narrow lint slice. | Do not paste wholesale; see "Do not use" below. |
| `rust-port` | Future external ports only: port the original tests first and prove behaviour parity. | Not for Kutha's Rust-native crates and not for Python harness/tooling. |

## What not to use, or use only partially

| Do not | Why |
|---|---|
| Do not paste the full `[lints]` block or `clippy.toml` into Cargo. | The measured baseline is 1,566 warnings for a selected high-signal subset and 959 for `pedantic`; a `-D warnings` gate fails immediately. |
| Do not enable `missing_docs` globally. | 178 missing struct-field docs, 55 method docs, and 35 variant docs at HEAD `89b0b9c`. Add docs on new public API instead. |
| Do not enable `as_conversions` / `cast_*` globally. | 96 `as` conversions in `src`; many are lossless on 64-bit. Audit casts by data path, not by lint count. |
| Do not set `max-fn-params-bools = 0`. | 17 bool parameters in `src`; several are deliberate API flags. The default threshold, or no bool lint, is less noisy. |
| Do not make `cargo fmt --check` a gate. | HEAD is not rustfmt-clean: 81 files, 143 hunks. Formatting is a separate slice, not a side effect of review. |
| Do not use `--all-features` for CI clippy. | It compiles `librocksdb-sys`. Kutha CI runs with `KUTHA_GOV_SKIP_ROCKS=1` on a 7 GB runner. Use default features or a dedicated job. |
| Do not treat `rust_review` counts as bug counts. | The runtime inventory reports 48 `unwrap`, but only 4 are production sites once tests are excluded (`wal.rs:209`, `wal.rs:210`, `leapfrog.rs:51` twice). It also reports 95 `.clone()` without judging redundancy. |
| Do not use `rust-port` for `tools/` or harness Python. | Python stays on the harness plane; porting it to Rust would collapse the product/harness split. |
| Do not add a CI phase or governor check from this document alone. | A new CI phase requires the `.kutha/META.md` and `docs/process/governor-intake.md` intake. This document changes no gate. |

## Measured baseline (2026-10-29, HEAD `89b0b9c`)

Logs: `tmp/beautifulrust-*` (gitignored).

| Command | Result |
|---|---|
| `cargo clippy --workspace --all-targets` | 108 warnings; 51 in `src` |
| `cargo clippy ... -- -W clippy::pedantic` | 959 warnings; 367 in `src`, 576 in tests |
| Selected BeautifulRust lints with `CLIPPY_CONF_DIR=lints` | 1,566 warnings; 523 in `src`, 999 in tests |
| `cargo fmt --all -- --check` | exit 1; 81 files, 143 hunks |
| `rust_review("crates/kutha-runtime/src")` | 30 files, 11,818 lines, 374 public API, 48 `unwrap`, 20 `expect`, 1 unsafe block, 95 `.clone()` |

Notable production signal:

- `crates/kutha-runtime/src/allow.rs:30` uses `unsafe { std::env::set_var(...) }` in
  `apply_dotenv()`. The SAFETY comment itself admits a race with a host that reads the
  environment concurrently. `apply_dotenv()` is a safe function called from
  `Runtime::new` / `FoldIndexMode`; treat this as a soundness caveat, not as proof that
  the current startup path is unsafe.
- `wal.rs:209/210` slice fixed 8-byte headers with `try_into().unwrap()`; replacing the
  two four-byte slices with `expect("frame header has a 4-byte length prefix")` and
  `expect("frame header has a 4-byte checksum")` is low-risk and lint-clean.
- `leapfrog.rs:51` unwraps `mn`/`mx` after a guard proves the rows are non-empty; an
  `expect("non-empty rows")` documents the invariant without changing behaviour.

## Review procedure for Kutha

1. Read the relevant skill in the installed package. Do not rely on a summary alone.
2. For a targeted audit, run `rust_review` on the file or directory, then ignore inline
   test noise. Use `cargo clippy --workspace --all-targets` (default features) and the
   relevant tests as the mechanical layer.
3. Classify the target as an **internal library**, `publish = false`, research stage.
   Use the simplicity rules for application code; do not demand public-API ceremony for
   every re-export.
4. Add Kutha checks that the generic skill does not know: event log is SoT; graph is a
   deterministic fold; AS OF / temporal validity; derived state never enters the log;
   D1–D10 and the current ADR cell. Cite the ADR or named test for each such finding.
5. Keep findings evidence-first. A review may propose a patch, but do not rewrite code
   unless the user asked for a fix. A green review is not governor green, not ADR
   Accepted, and not capability.
6. If a lint slice is later approved, start narrow: `unsafe_op_in_unsafe_fn`,
   `undocumented_unsafe_blocks`, `multiple_unsafe_ops_per_block`, `unwrap_used` for
   production code, `allow_attributes_without_reason`, and
   `missing_debug_implementations`. Add `missing_errors_doc` / `panic_in_result_fn` for
   new public APIs only. Leave `missing_docs`, `as_conversions`, `pedantic`, and
   `print_*` out of the first gate.

## Prompt template for a Kutha review

```text
Use the rust-review skill. Review <path or diff> as an internal Kutha library
(publish = false, research stage). Apply rust-best-practices as the rule source, but
keep ADR-000 D1-D10 and docs/ADR/ above the skill. Do not propose the full lints.toml,
a cargo fmt gate, or public-API ceremony. Report Must fix / Should fix / Consider,
with file:line and the rule or ADR each finding cites. Treat rust_review output as a
map, not a verdict; exclude inline test code from production findings. Do not edit files.
```

## Supply chain

- Pin the package commit or tag; do not let a floating `allspark package update` silently
  change review behaviour. Re-run `python3 allspark/check.py` after an update.
- Skills are prompts, not code execution. They can still steer an agent; keep the
  Kutha contract in `AGENTS.md` and the ADR as the higher authority.
- `rust_review` reads local files and prints a report; it has no network path. It is
  nonetheless an external Python package in the kernel venv, so pin and review updates.
