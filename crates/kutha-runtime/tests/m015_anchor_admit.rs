//! ANK-ADM-01: duplicate or backwards `TtAnchor` walls fail closed at admit.

use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn duplicate_tt_anchor_wall_is_rejected() {
    let mut rt = Runtime::default();
    rt.emit_tt_anchor(2015).unwrap();
    let n = rt.log().len();
    let err = rt.emit_tt_anchor(2015).unwrap_err();
    assert!(
        matches!(
            err,
            RuntimeError::TtAnchorRejected {
                wall: 2015,
                max_wall: Some(2015)
            }
        ),
        "{err:?}"
    );
    assert_eq!(rt.log().len(), n, "rejected emit must not append");
}

#[test]
fn backwards_tt_anchor_wall_is_rejected() {
    let mut rt = Runtime::default();
    rt.emit_tt_anchor(2020).unwrap();
    let n = rt.log().len();
    let err = rt.emit_tt_anchor(2014).unwrap_err();
    assert!(
        matches!(
            err,
            RuntimeError::TtAnchorRejected {
                wall: 2014,
                max_wall: Some(2020)
            }
        ),
        "{err:?}"
    );
    assert_eq!(rt.log().len(), n, "rejected emit must not append");
}
