//! M012 S01: relation allowlist entries as log facts (ALL-01..03).
//! Not ADR-050's six dictionary kinds.

use kutha_common::Op;
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn allow_relation_appends_as_log_fact() {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.fold().facts().len());

    let leased = rt.intern("leasedRel");
    let n = rt.log().len();

    rt.emit(Op::AllowRelation {
        name: "leasedRel".into(),
        valid_from: 2017,
        valid_to: None,
    })
    .unwrap();

    assert_eq!(n + 1, rt.log().len(), "AllowRelation appends one log event");
    let last = rt.log().as_slice().last().expect("appended");
    assert!(
        matches!(
            &last.op,
            Op::AllowRelation {
                name,
                valid_from: 2017,
                valid_to: None
            } if name == "leasedRel"
        ),
        "{:?}",
        last.op
    );
    assert_eq!(0, rt.fold().facts().len());
    assert_eq!(0, rt.graph_len());
    assert!(rt.fold().relation_allowed_at("leasedRel", u64::MAX, 2017));

    let subject = rt.intern("alice-lease");
    let object = rt.intern("bob-lease");
    rt.emit(Op::Assert {
        subject,
        relation: leased,
        object,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert_eq!(1, rt.fold().facts().len());

    let bogus = rt.intern("notALegalRelation");
    let n = rt.log().len();
    let err = rt
        .emit(Op::Assert {
            subject,
            relation: bogus,
            object,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "notALegalRelation"),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");
}

#[test]
fn yaml_seed_still_admits_in_force_as() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert_eq!(1, rt.fold().facts().len());
}

#[test]
fn empty_allow_relation_name_does_not_append() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let err = rt
        .emit(Op::AllowRelation {
            name: String::new(),
            valid_from: 2017,
            valid_to: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name.is_empty()),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");
}
