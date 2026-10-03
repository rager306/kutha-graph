//! FIXG-01 C01 sunset: application-end is inexpressible as Horn without folding into VT.

use kutha_common::Op;
use kutha_common::RuleTerm;
use kutha_runtime::Runtime;

/// law-nexus syn-001 quote (C01). Do not copy the corpus file.
const C01: &str = "пункт 15.1 действует до 01.01.2028";

#[test]
fn fixg01_c01_sunset_inexpressible_as_horn() {
    assert!(
        RuleTerm::constant("01.01.2028").is_none(),
        "{C01}: RuleTerm Const is rule-ident only; a calendar sunset cannot be a Horn atom"
    );
    assert!(
        RuleTerm::constant("2028").is_none(),
        "year digits are not a rule ident"
    );

    let mut rt = Runtime::default();
    let punkt = rt.intern("punkt-15-1");
    let in_force = rt.intern("inForceAs");
    let true_ = rt.intern("true");
    rt.emit_tt_anchor(2020).unwrap();
    rt.emit(Op::Assert {
        subject: punkt,
        relation: in_force,
        object: true_,
        valid_from: 2010,
        valid_to: Some(2028),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2026).unwrap();

    let triple = (punkt, in_force, true_);
    assert!(rt.as_of(2027).contains(&triple), "VT folding sees 2027");
    assert!(
        !rt.as_of(2028).contains(&triple),
        "half-open valid_to=2028 is not a transitional rule"
    );
    let known = rt.as_known_on(2026).expect("anchor 2026");
    assert!(
        known.contains(&triple),
        "as_known_on answers at a fixed wall; it does not encode application-end"
    );
}
