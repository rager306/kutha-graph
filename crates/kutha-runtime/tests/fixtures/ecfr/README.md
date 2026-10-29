# eCFR PIT fixture (test data, not a legal pack)

Three sections of **1 CFR Part 304** at three amendment dates, fetched from the
eCFR Versioner API on 2026-10-29. This is a small real-text probe for
`as_of` / `as_known_on` / `norm_view`; it is **not** a legal pack, not legal
advice, and not a claim of complete regulatory coverage.

## Source

- API: `https://www.ecfr.gov/api/versioner/v1/full/{date}/title-1.xml?part=304&section={section}`
- Dates: 2015-12-18, 2017-01-23, 2017-03-14
- Sections: 304.1, 304.2, 304.10 (3 × 3 = 9 text files)
- Raw XML was fetched with `tools/fetch_ecfr_fixture.py` and kept in gitignored `tmp/ecfr-raw/`.
- The eCFR Versioner `versions/title-1.json` listed these as three amendment dates for the sections.

## Status of the text

eCFR is a work of the US federal government. Under 17 U.S.C. §105, US
government works are not subject to copyright in the United States; the eCFR
site describes the content as public domain. Re-check the current eCFR terms
before any redistribution beyond this research repository.

## Boundaries

- 304.1: 2015-12-18 and 2017-01-23 extracted text is identical; 2017-03-14 differs (about 1985 vs 2028 characters).
- 304.2: 2015-12-18 and 2017-01-23 extracted text is identical; 2017-03-14 differs (about 1071 vs 544 characters).
- 304.10: 2015-12-18 and 2017-01-23 extracted text is identical; 2017-03-14 differs in one schedule citation (about 801 vs 800 characters).

The probe must preserve these equal/different facts; do not "normalize" the text. Extraction keeps HEAD and P text only, so raw XML citation/XREF lines are dropped; "identical" refers to the committed extracted text, not to the raw edition XML.
