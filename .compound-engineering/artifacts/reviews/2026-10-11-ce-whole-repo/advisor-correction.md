# Advisor correction — finding #1 (2026-10-11)

Controller / advisor note after source re-verification. Does not rewrite the
frozen validator batch; updates actionable triage for delivery planning.

## Verdict on #1

**Not a defect. Intended documented wire break.**

Evidence:

- `docs/process/op-compat.md` § Decision: **Policy: documented break; serde
  backfill deferred.** Follow-on migrate is explicitly unleased.
- `crates/kutha-runtime/tests/m050_op_compat.rs` requires legacy
  `AllowRelation` Event JSON to fail deserialize (fail-open).
- M050 / GSD v0.44 shipped that policy pin (CE full-branch #14 minimum path).

The whole-repo validator “confirmed” the open-fail behavior — that confirms the
**policy**, not a missing decode path. Treating #1 as P1 / fix-first / actionable
contradicts the leased policy.

## Corrected counts (planning SoT)

| Bucket | Report (raw) | After advisor |
|--------|--------------|---------------|
| P1 actionable | 12 (#1–#12) | **11** (#2–#12); #1 → deferred residual |
| Actionable total | 31 | **30** (+ #1 residual, not fix-first) |
| Decision-gate before code | #1 then #24 | **#24 only** (EventId UUID); Op migrate stays unleased follow-on in `op-compat.md` |

## Fix-first order (corrected)

1. Cascade atomicity (**#8**)
2. Horn lineage / VT / dedup (**#4**, **#3**, **#2**, **#5**)
3. Persist/open durability (**#7**, **#6**, **#9**, **#10**, **#11**, then #29/#30)
4. Governor honesty (**#23**, **#22**, **#14**, **#34**)
5. Scaling + bench (**#19**, **#16**/#17/#27, **#25**/#15)
6. ID contract (**#24**) + structural splits (**#12**, **#21**, **#20**)
7. Hygiene (**#35**, **#36**)

## Withdrawals re-checked this session

| Claim | Status | Evidence |
|-------|--------|----------|
| Outcomes dropped on hydrate | **Withdrawn** | `hydrate_from_log` rebuilds via `outcomes_from_events` (`hydrate.rs:14-15`, called at `:150`) |
| m028 `persist_dir` CWD-coupled | **Withdrawn** | Anchored on `CARGO_MANIFEST_DIR` + PID + nanos (`m028_dict_retract.rs:7-18`) |
| Rollover-shrink abandons layout | **Withdrawn** | `try_persist_with_layout` keeps original layout; rejects candidates outside seam (`sem.rs:1021-1023`) |

## Residual (not leased)

Serde/open migrate for five legacy Op variants remains a **named follow-on** in
`docs/process/op-compat.md`. Do not silent-alias TermIds.
