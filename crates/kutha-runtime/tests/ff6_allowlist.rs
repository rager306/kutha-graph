//! FF6 stub: one relation allowlist. Unknown relation does not append.
//! Not ADR-050's six dictionary kinds.

use kutha_common::Op;
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn ff6_unknown_relation_does_not_append() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let bogus = rt.intern("notALegalRelation");
    let limit = rt.intern("50-kmh");
    let n = rt.log().len();

    let err = rt
        .emit(Op::Assert {
            subject: article,
            relation: bogus,
            object: limit,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap_err();

    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "notALegalRelation"),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");
    assert_eq!(0, rt.fold().facts().len());
}

#[test]
fn ff6_allowlisted_relation_still_appends() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    assert_eq!(1, rt.log().len());
    rt.replay_check().unwrap();
}
