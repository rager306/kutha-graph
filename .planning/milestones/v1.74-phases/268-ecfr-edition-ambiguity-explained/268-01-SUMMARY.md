---
phase: 268-ecfr-edition-ambiguity-explained
plan: 01
status: complete
requirements: [ECFR-01]
---

# Summary — eCFR edition ambiguity explained

- Checked the eCFR Versioner list and raw XML for all eight part-304 sections: the 2017-01-23 edition differs only by an `<XREF>` "amendment published" note.
- FR doc 2017-00891 was published 2017-01-23 and effective 2017-03-14, which is why the middle eCFR row exists without a text change.
- The PIT probe now asserts exactly two distinct extracted texts per section; the count assert catches a lost edition and the equality asserts catch a gained one.
- Added `docs/architecture/ecfr-edition-ambiguity.md`; fixture README matches the checked section list.
- No fixture `.txt` was edited or fabricated; no legal pack or corpus claim.
- Tests: eCFR PIT 1/1, workspace 801 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
