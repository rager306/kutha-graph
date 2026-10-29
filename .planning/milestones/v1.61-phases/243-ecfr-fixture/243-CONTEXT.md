---
phase: "243"
name: "ecfr-fixture"
created: 2026-10-29
---

# Phase 243: eCFR fixture — Context

## Decisions

1. Source: eCFR Versioner API. Sections 304.1, 304.2, 304.10 of 1 CFR have three amendment dates: 2015-12-18, 2017-01-23, 2017-03-14.
2. Commit extracted text plus source metadata, not the full XML, to keep the fixture small and reviewable. The fetch script keeps the raw XML in gitignored `tmp/`.
3. eCFR is a US government work; record public-domain status (17 U.S.C. §105) and the exact URLs/dates. No proprietary law-nexus text.
4. The fetch script is a tool, not a CI step. CI uses only committed text.
5. No product code, no Op, no thaw.

## Discretion Areas

The exact text extraction (HEAD + P) and whether to keep XML alongside text; prefer text plus URL.

## Deferred Ideas

More sections/jurisdictions and legislation.gov.uk are later probes.
