//! M021 S01 / Phase 68 — TR-02 three moments and C01 pack-fact spike.
//! Does not change fold algebra. FIXG-01 / known04 / asof_elig01 stay authoritative.

use kutha_common::Op;
use kutha_common::RuleTerm;
use kutha_runtime::Runtime;

/// Force = VT; publication = TtAnchor / as_known_on; case decision = as_of(vt).
#[test]
fn tr02_force_vt_publication_anchor_decision_as_of() {
    let mut rt = Runtime::default();
    let act = rt.intern("act-1");
    let in_force = rt.intern("inForceAs");
    let true_ = rt.intern("true");

    rt.emit_tt_anchor(2015).unwrap();
    rt.emit(Op::Assert {
        subject: act,
        relation: in_force,
        object: true_,
        valid_from: 2016,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    let triple = (act, in_force, true_);
    assert!(
        !rt.as_of(2015).contains(&triple),
        "force clock: not in force before valid_from"
    );
    assert!(
        rt.as_of(2016).contains(&triple),
        "force clock: in force at valid_from (case decision via as_of)"
    );

    let known_2015 = rt.as_known_on(2015).expect("anchor 2015");
    assert!(
        !known_2015.contains(&triple),
        "publication/knowledge: ingest after wall 2015 is hidden on that cut"
    );
    let known_2017 = rt.as_known_on(2017).expect("anchor 2017");
    assert!(
        known_2017.contains(&triple),
        "publication/knowledge: later wall sees the ingested force fact"
    );
}

/// C01 application-end as pack Assert (`appliesTo` VT); still inexpressible as Horn Const.
#[test]
fn c01_applies_to_assert_encodes_application_window() {
    assert!(
        RuleTerm::constant("01.01.2028").is_none(),
        "calendar sunset remains inexpressible as Horn Const (FIXG-01)"
    );

    let mut rt = Runtime::default();
    rt.emit_allow("appliesTo", 2010, None).unwrap();
    let punkt = rt.intern("punkt-15-1");
    let applies = rt.intern("appliesTo");
    let cases = rt.intern("cases-in-window");

    rt.emit(Op::Assert {
        subject: punkt,
        relation: applies,
        object: cases,
        valid_from: 2010,
        valid_to: Some(2028),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let triple = (punkt, applies, cases);
    assert!(
        rt.as_of(2027).contains(&triple),
        "pack application window covers 2027"
    );
    assert!(
        !rt.as_of(2028).contains(&triple),
        "pack application window ends at half-open 2028 — not a fifth axis"
    );
}
