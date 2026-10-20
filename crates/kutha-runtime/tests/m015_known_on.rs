//! KNOWN-02: closed VT intervals survive `as_known_on` (FF5-shaped `valid_to`).
//! FF5 `as_of` assertions stay in `ff5_legal_pit.rs` (untouched).

use kutha_common::{Op, ValidTime, WallTime};
use kutha_runtime::Runtime;

fn closed_vt_known_on_store_dir() -> std::path::PathBuf {
    // Unique even when two Cargo processes run this binary together.
    // A fixed `tmp/p34-01-store` is removed by the other process mid-persist.
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seq = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "p34-01-store-{}-{nanos}-{seq}",
            std::process::id()
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

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
fn closed_vt_known_on_store_dirs_differ_across_calls() {
    let a = closed_vt_known_on_store_dir();
    let b = closed_vt_known_on_store_dir();
    assert_ne!(a, b, "parallel cargo must not share one persist directory");
    assert!(
        !a.ends_with("p34-01-store"),
        "fixed tmp/p34-01-store races across processes"
    );
    let _ = std::fs::remove_dir_all(a);
    let _ = std::fs::remove_dir_all(b);
}

#[test]
fn closed_vt_known_on_persist_open() {
    let mut rt = Runtime::default();
    let statute = seed_closed_interval(&mut rt);
    let dir = closed_vt_known_on_store_dir();
    rt.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    let known = opened.as_known_on(WallTime::from(T_OLD)).expect("anchor rebuilt on open");
    assert!(
        known.contains(&statute),
        "persist/open must keep the closed-interval speed fact on as_known_on"
    );
    let _ = std::fs::remove_dir_all(dir);
}
