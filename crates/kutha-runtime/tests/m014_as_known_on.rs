//! ANK-03: “true on D” (`as_of` / valid-time) differs from “known on D” (`as_known_on`).
//! FF5 stays untouched; this file is the named divergence oracle.

use kutha_common::Op;
use kutha_runtime::Runtime;

const D: u64 = 2015;
const W1: u64 = 2014;
const W2: u64 = 2020;
const VF: u64 = 2010;

#[test]
fn true_on_d_differs_from_known_on_d() {
    let mut rt = Runtime::default();
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

    let statute = (article, in_force, limit_50);
    assert!(
        rt.as_of(D).contains(&statute),
        "true on D: as_of(2015) sees the statute (valid_from 2010)"
    );
    let known_2015 = rt.as_known_on(D).expect("anchor 2014 is at or before 2015");
    assert!(
        !known_2015.contains(&statute),
        "known on D: as_known_on(2015) uses 2014 tt, before the statute event"
    );
    let known_2020 = rt
        .as_known_on(W2)
        .expect("anchor 2020 is at or before 2020");
    assert!(
        known_2020.contains(&statute),
        "as_known_on(2020) uses the later anchor and sees the statute"
    );
}
