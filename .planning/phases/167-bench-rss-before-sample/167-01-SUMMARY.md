---
phase: 167-bench-rss-before-sample
plan: 01
---

# Summary

`bytes_per_fact` now brackets only the dedicated sample runtime. Factor-loop
high-water no longer inflates the numerator. Without allocator trim the delta
may be 0 B when RSS already covers sample size — honest vs ~15x inflation.
