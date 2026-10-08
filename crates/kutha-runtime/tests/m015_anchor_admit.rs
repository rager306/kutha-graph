//! ANK-ADM-01: duplicate or backwards `TtAnchor` walls fail closed at admit.

use kutha_common::{WallTime};
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

fn seed_monotone_statute(
    rt: &mut Runtime,
) -> (
    kutha_common::TermId,
    kutha_common::TermId,
    kutha_common::TermId,
) {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit_tt_anchor(2014).unwrap();
    rt.emit(kutha_common::Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2020).unwrap();
    (article, in_force, limit_50)
}

#[test]
fn monotone_unique_anchors_persist_open() {
    let mut rt = Runtime::default();
    let statute = seed_monotone_statute(&mut rt);
    let known_d = rt.as_known_on(WallTime::from(2015)).expect("A1@2014 at or before 2015");
    assert!(!known_d.contains(&statute));
    assert!(rt.as_known_on(WallTime::from(2020)).expect("A2@2020").contains(&statute));
    let dir = std::path::Path::new("tmp/p34-02-store");
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    rt.persist(dir).unwrap();
    let opened = Runtime::open(dir).unwrap();
    let known_d = opened.as_known_on(WallTime::from(2015)).expect("A1 rebuilt");
    assert!(!known_d.contains(&statute));
    assert!(
        opened
            .as_known_on(WallTime::from(2020))
            .expect("A2 rebuilt")
            .contains(&statute)
    );
    let _ = std::fs::remove_dir_all(dir);
}
