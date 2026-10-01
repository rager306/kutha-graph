# Phase 23 Context: Legal fixture reconnaissance

**Milestone:** v0.06 / harness M013 S02
**Requirements:** REC-01, REC-02, REC-03
**Depends on:** nothing (independent of Phase 22)
**Source of gaps:** `docs/architecture/semantic-gap-review.md` addendum, R1 (riskiest assumption checked last) and F13
**Run rules:** `docs/process/gsd-auto-contract.md`

## Locked decisions

- **D-01:** Read-only phase. Output is one note: `docs/architecture/legal-fixture-recon.md`. `git diff --stat -- crates/` must be empty. No ontology, no pack, no copied law-nexus code.
- **D-02:** Sources (sister product, read in place, not copied): `/root/law-nexus/crates/ln-decode/tests/fixtures/npa/` (`fz44-001`…`fz44-042`, `syn-001/002`, `fz44_npa_tokens.json`), `npa-hostile/hostile-bounds.txt`, `npa-lawref/`, `npa-members/`, and `/root/law-nexus/law-source/consultant/*.xml` (eight laws; the edition date is in the file name, for example FZ-44 `ред. от 28.12.2025`).
- **D-03:** At least ten cases, covering at least: one sunset (`syn-001`: clause 15.1 valid until 2028-01-01), one amendment that creates a new edition, one specific-over-general derogation, one transitional or year-bound applicability (`fz44-010`: “в 2025 г. возможна закупка…”), one hostile-bounds case. Each case cites file path and a quoted fragment.
- **D-04:** Each case is classified against the current model (Assert, Retract, Correct, CorrectInterval, claim supports, `as_of`, `live_at`, `derivation_eligible_at`):
  **E** expressible now; **T** needs a typed clock or `tt_anchor` (M014); **R** needs an executable rule (M015); **X** not expressible for another stated reason.
- **D-05:** Name the clocks each case touches (promulgation, effective, application, edition, known-on) and say which of them today's two axes would silently conflate (ADR-090 TR-02).
- **D-06:** Propose exactly one case as the M015 golden fixture (smallest text that needs a transitional rule and an as-known-on query) and say what extra data it needs. This is a proposal; the lease decides.
- **D-07:** Failing to express a case is a result, not a defect to fix here.

## Tool contract

CBM project `law-nexus` and GitNexus repo `law-nexus` may be used to locate fixture loaders and test consumers (`search_graph`, `query`); source reads are the evidence for text. Kutha side: CBM `root-kutha-graph` `search_graph` / `get_code_snippet` for the model's public API. Subagents name `codebase-memory-mcp`; heavier model for classification (`cursor-grok-4.6-xhigh`).

## Verifier kill-tests

- A verifier who has not seen the note re-classifies three randomly chosen cases from the source text and the model API; at least two of three match or the disagreement is explained.
- Every case has a resolvable path and quote; `rg` finds each quote in its file.
- `git diff --stat -- crates/` is empty; the note names at least one T or R case, or states why none exist.

## Non-goals

Legal corpus pack, ADR-090 ontology, importing fixtures into this repo, parsing XML.

## Harness

Active Milestone M013, Active Slice **S02** while this phase runs (controller leases it).
