//! KNOWN-02: closed VT intervals survive `as_known_on` (FF5-shaped `valid_to`).
//! FF5 `as_of` assertions stay in `ff5_legal_pit.rs` (untouched).

use kutha_common::{Op, ValidTime, WallTime};
use kutha_runtime::Runtime;

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

#[test]
fn closed_vt_interval_survives_as_known_on() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");

    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();

    let statute = (article, in_force, limit_50);
    assert!(
        rt.as_of(ValidTime::from(T_OLD)).contains(&statute),
        "as_of(2015) sees the closed-interval speed fact in-window"
    );
    let known = rt
        .as_known_on(WallTime::from(T_OLD))
        .expect("anchor 2015 is at or before 2015");
    assert!(
        known.contains(&statute),
        "as_known_on(2015) must keep the closed VT interval that as_of sees in-window"
    );
}

fn seed_closed_interval(
    rt: &mut Runtime,
) -> (
    kutha_common::TermId,
    kutha_common::TermId,
    kutha_common::TermId,
) {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();
    (article, in_force, limit_50)
}

#[test]
fn closed_vt_known_on_persist_open() {
    let mut rt = Runtime::default();
    let statute = seed_closed_interval(&mut rt);
    let dir = std::path::Path::new("tmp/p34-01-store");
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    rt.persist(dir).unwrap();
    let opened = Runtime::open(dir).unwrap();
    let known = opened.as_known_on(WallTime::from(T_OLD)).expect("anchor rebuilt on open");
    assert!(
        known.contains(&statute),
        "persist/open must keep the closed-interval speed fact on as_known_on"
    );
    let _ = std::fs::remove_dir_all(dir);
}
