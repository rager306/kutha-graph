---
phase: 243-ecfr-fixture
plan: 01
subsystem: legal-fixture
tags: [ecfr, pit, public-domain, fixture]
requires: []
provides:
  - "3 eCFR sections x 3 dates"
  - "fetch script with retries"
affects: [244-pit-probe]
actuals:
  tokens: 1200
  tasks: 5
  commits: 0
tech-stack:
  added: []
  patterns: [real-text-probe]
key-files:
  created:
    - tools/fetch_ecfr_fixture.py
    - crates/kutha-runtime/tests/fixtures/ecfr/README.md
  modified: []
key-decisions:
  - "eCFR 1 CFR 304.1/304.2/304.10 at 2015-12-18, 2017-01-23, 2017-03-14; extracted text only."
  - "Public domain under 17 U.S.C. 105; not a legal pack."
requirements-completed: [LP-01]
coverage:
  - id: D1
    description: "A small public-domain eCFR fixture with three dated editions per section."
    requirement: LP-01
    verification:
      - kind: integration
        ref: "tools/fetch_ecfr_fixture.py --check"
        status: pass
    human_judgment: false
---

# Phase 243 summary — eCFR fixture

- Added `tools/fetch_ecfr_fixture.py` with gzip, User-Agent, 5 retries, and HTML/503 fail-closed.
- Committed 9 text files: 304.1, 304.2, 304.10 at 2015-12-18, 2017-01-23, 2017-03-14.
- README records URLs, dates, public-domain status, and the equal/different boundaries.
- 304.2 changes substantively at 2017-03-14; 304.1 and 304.10 change by a few bytes.
- No product code, no Op, no thaw.
