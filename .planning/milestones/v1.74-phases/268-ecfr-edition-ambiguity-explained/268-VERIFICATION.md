---
phase: 268-ecfr-edition-ambiguity-explained
independent: true
status: passed
verifier_model: codebuddy/deepseek-v4.1-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m167_ecfr_pit
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/m167_ecfr_pit.rs
  - crates/kutha-runtime/tests/fixtures/ecfr/README.md
  - docs/architecture/ecfr-edition-ambiguity.md
  - .kutha/STATE.md
---

# Phase 268 verification — eCFR ambiguity

Independent verifier PASSed. The citation-only claim was checked against raw XML for all eight part-304 sections, a live re-fetch, and Federal Register document 2017-00891: the rule was published 2017-01-23 and effective 2017-03-14, so the middle row carries only a link note. No fixture text was edited or fabricated; all nine committed texts reproduce byte-exactly from live eCFR through the committed extractor.

The new assertion is falsifiable: the count assert uniquely catches a lost edition, while a gained edition trips the pre-existing equality asserts. M167 parity (AS OF cuts, as_known_on, norm_view, persist/checkpoint/reopen) is untouched.

Verifier findings fixed before close: the effective-date mechanism is now in the doc, the comment names which assert catches which direction, the evidence sources for later dates are distinguished, the section lists match, and the phase directory was renamed to the roadmap name.

Final gate: eCFR PIT 1/1, workspace 801 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
