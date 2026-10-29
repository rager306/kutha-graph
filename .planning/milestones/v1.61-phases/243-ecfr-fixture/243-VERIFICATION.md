---
phase: 243-ecfr-fixture
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: uv run python tools/fetch_ecfr_fixture.py --check
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 1
  - cmd: sha256sum + diff/cmp pairs over crates/kutha-runtime/tests/fixtures/ecfr/*.txt
    exit: 0
read_only_inputs:
  - tools/fetch_ecfr_fixture.py
  - crates/kutha-runtime/tests/fixtures/ecfr/README.md
  - .kutha/STATE.md
---

# Phase 243 verification — eCFR PIT fixture (M167-S01)

Independent verifier context. PLAN and SUMMARY were not read; findings come from the
fetch script, the 9 fixture files, the fixture README, the lease, and executed commands.

## Checked

| Claim | Result |
|-------|--------|
| Exactly 3 sections x 3 dates | 9 text files: 304.1 / 304.2 / 304.10 x 2015-12-18, 2017-01-23, 2017-03-14; no extra files |
| Every file nonempty | 544–2028 chars each; `--check` prints 9 `OK`, exit 0 |
| README records source URLs, dates, public-domain status | Versioner `full/{date}/title-1.xml?part=304&section={section}` URL, all 3 dates, 17 U.S.C. §105 public-domain note with re-check caveat |
| 304.1, 304.2, 304.10: 2015-12-18 == 2017-01-23 | `diff -q` identical for all three; matching sha256 |
| 304.1, 304.2, 304.10: 2017-01-23 != 2017-03-14 | `cmp` differs for all three; distinct sha256 |
| Script fails closed on HTML / 503 | Non-section body (HTML, empty, no DIV8) → `ValueError`; 503 → `HTTPError` (a `URLError`) → 5 attempts → `SystemExit(str)` → exit 1; file written only after extraction, so nothing is overwritten |
| Script not run in CI | no reference in `.github/workflows/ci.yml`, `.pre-commit-config.yaml` (ruff hooks scoped `^scripts/`), or governor checks |
| Lease authorizes this slice | `.kutha/STATE.md`: M167-S01 legal PIT probe, "3 sections × 3 editions", no thaw |

`kutha-gov precommit` exit 1: 103 OK, 2 FAIL, both `glob .planning/phases/*-*/*-VERIFICATION.md
matched no files` (`artifact-budget`, `verification-independent`) — expected until this file
exists. No other check failed. Nothing was fixed.

## Non-blocking notes

1. README boundary sizes are character counts labelled "bytes", newest edition first:
   304.1 says "1985 vs 2028 bytes" where files are 2006 B (2017-03-14) and 2049 B
   (2017-01-23); 304.2 "1071 vs 544" vs 1072 B / 545 B; 304.10 "801 vs 800" vs 806 B / 805 B.
   The fetch script's own `wrote … (N bytes)` message uses the same char count. Equal/differ
   facts hold; only the unit label and ordering are loose.
2. Extraction keeps HEAD + P only, so `<XREF>` pointers are dropped: the raw 2017-01-23 XML
   differs from 2015-12-18 by one "Link to an amendment published at 82 FR 7632, Jan. 23, 2017."
   line. "Identical" is therefore true of the committed section text, not of the raw edition.
   Correct for a PIT probe, but unstated in the README.
3. `tools/fetch_ecfr_fixture.py` and the fixture directory are untracked at verification time.
4. `--check` asserts only presence and non-emptiness; the equal/differ boundaries are not
   self-asserted by any test or by `--check`.
