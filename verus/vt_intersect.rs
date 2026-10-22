// Verus proof: valid-time interval intersection (Kutha V1a).
//
// Production: `vt_intersect` in crates/kutha-runtime/src/fold.rs. The body below
// is that body line for line; the governor check `verus-proof-sync` pins the
// body lines in both files. Change one, change the other, re-run Verus.
//
// Run (pinned release in tools/verus/VERSION, rustc 1.99.1):
//   RUSTUP_TOOLCHAIN=1.99.1 "$KUTHA_VERUS_BIN" verus/vt_intersect.rs
//
// Model: half-open interval [from, to); `None` is an open end, encoded as
// u64::MAX exactly like production. u64::MAX itself is therefore the sentinel,
// not a real instant, so the point contract covers x < u64::MAX.
//
// Not proven here: callers (conjunct_vt, windows_cover), serde, the fold.
use vstd::prelude::*;

verus! {

pub open spec fn end_of(to: Option<u64>) -> int {
    match to {
        Some(t) => t as int,
        None => 0xFFFF_FFFF_FFFF_FFFFint,
    }
}

pub open spec fn contains(from: u64, to: Option<u64>, t: int) -> bool {
    from as int <= t && t < end_of(to)
}

pub fn vt_intersect(
    a_from: u64,
    a_to: Option<u64>,
    b_from: u64,
    b_to: Option<u64>,
) -> (r: Option<(u64, Option<u64>)>)
    ensures
        // Soundness and completeness: a point is in the result iff it is in both inputs.
        match r {
            Some((f, t)) => forall|x: int| 0 <= x < 0xFFFF_FFFF_FFFF_FFFFint ==>
                (contains(f, t, x) <==> (contains(a_from, a_to, x) && contains(b_from, b_to, x))),
            None => forall|x: int| 0 <= x < 0xFFFF_FFFF_FFFF_FFFFint ==>
                !(contains(a_from, a_to, x) && contains(b_from, b_to, x)),
        },
        // Non-empty and normalized: Some(interval) is never empty, and a closed end is
        // never the open-end sentinel.
        match r {
            Some((f, t)) => (f as int) < end_of(t) && t != Some(u64::MAX),
            None => true,
        },
{
    let from = a_from.max(b_from);
    let a_end = a_to.unwrap_or(u64::MAX);
    let b_end = b_to.unwrap_or(u64::MAX);
    let to_raw = a_end.min(b_end);
    if from >= to_raw {
        return None;
    }
    let to = if to_raw == u64::MAX {
        None
    } else {
        Some(to_raw)
    };
    Some((from, to))
}

fn main() {}

}
