//! M012 S01: relation allowlist entries as log facts (ALL-01..03).
//! Not ADR-050's six dictionary kinds.

use kutha_common::Op;
use kutha_runtime::{store, Runtime, RuntimeError};

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

#[test]
fn retract_allow_relation_drops_admit_at_later_cut() {
    let mut rt = Runtime::default();
    let leased = rt.intern("leasedRel");
    let allow = rt
        .emit(Op::AllowRelation {
            name: "leasedRel".into(),
            valid_from: 2017,
            valid_to: None,
        })
        .unwrap();
    let allow_id = allow.receipt.event_ids[0];
    let n = rt.log().len();

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

    rt.emit(Op::Retract { event_id: allow_id }).unwrap();

    let after = rt.log().len();
    let err = rt
        .emit(Op::Assert {
            subject,
            relation: leased,
            object,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "leasedRel"),
        "{err:?}"
    );
    assert_eq!(after, rt.log().len(), "fail-closed: log must not grow");

    let mut prefix = rt.fork_at(n);
    let rel = prefix.intern("leasedRel");
    let s = prefix.intern("alice-lease");
    let o = prefix.intern("bob-lease");
    prefix
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
}

#[test]
fn admit_consults_fold_allowlist_not_yaml_alone() {
    let mut rt = Runtime::default();
    let leased = rt.intern("leasedRel");
    rt.emit(Op::AllowRelation {
        name: "leasedRel".into(),
        valid_from: 2017,
        valid_to: None,
    })
    .unwrap();
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

    let dir = std::env::temp_dir().join(format!(
        "kutha-m012-s01-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(&e.op, Op::AllowRelation { name, .. } if name == "leasedRel")),
        "opened log must contain AllowRelation"
    );
    let rel = opened.intern("leasedRel");
    let s = opened.intern("alice-lease");
    let o = opened.intern("bob-lease");
    opened
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}
