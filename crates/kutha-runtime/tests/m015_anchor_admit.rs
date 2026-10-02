//! ANK-ADM-01: duplicate or backwards `TtAnchor` walls fail closed at admit.

use kutha_runtime::Runtime;

#[test]
fn duplicate_tt_anchor_wall_is_rejected() {
    let mut rt = Runtime::default();
    rt.emit_tt_anchor(2015).unwrap();
    let n = rt.log().len();
    assert!(
        rt.emit_tt_anchor(2015).is_err(),
        "duplicate wall must fail closed at admit"
    );
    assert_eq!(rt.log().len(), n, "rejected emit must not append");
}

#[test]
fn backwards_tt_anchor_wall_is_rejected() {
    let mut rt = Runtime::default();
    rt.emit_tt_anchor(2020).unwrap();
    let n = rt.log().len();
    assert!(
        rt.emit_tt_anchor(2014).is_err(),
        "wall earlier than max prior must fail closed at admit"
    );
    assert_eq!(rt.log().len(), n, "rejected emit must not append");
}
