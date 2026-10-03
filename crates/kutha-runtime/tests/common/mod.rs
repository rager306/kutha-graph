//! Shared log-backed fixtures for integration tests (D-04). Not a `[[test]]` crate.

use kutha_common::Op;
use kutha_runtime::Runtime;

const W1: u64 = 2014;
const W2: u64 = 2020;
const VF: u64 = 2010;

pub fn seed_statute_between_anchors(
    rt: &mut Runtime,
) -> (
    kutha_common::TermId,
    kutha_common::TermId,
    kutha_common::TermId,
) {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit_tt_anchor(W1).unwrap();
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(W2).unwrap();
    (article, in_force, limit_50)
}
