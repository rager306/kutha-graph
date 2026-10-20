//! ANK-ADM-01: duplicate or backwards `TtAnchor` walls fail closed at admit.

use kutha_common::{WallTime};
use kutha_runtime::{Runtime, RuntimeError};

fn anchor_admit_store_dir() -> std::path::PathBuf {
    // Unique even when two Cargo processes run this binary together.
    // A fixed `tmp/p34-02-store` is removed by the other process mid-persist.
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
            "p34-02-store-{}-{nanos}-{seq}",
            std::process::id()
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

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
fn anchor_admit_store_dirs_differ_across_calls() {
    let a = anchor_admit_store_dir();
    let b = anchor_admit_store_dir();
    assert_ne!(a, b, "parallel cargo must not share one persist directory");
    assert!(
        !a.ends_with("p34-02-store"),
        "fixed tmp/p34-02-store races across processes"
    );
    let _ = std::fs::remove_dir_all(a);
    let _ = std::fs::remove_dir_all(b);
}

#[test]
fn monotone_unique_anchors_persist_open() {
    let mut rt = Runtime::default();
    let statute = seed_monotone_statute(&mut rt);
    let known_d = rt.as_known_on(WallTime::from(2015)).expect("A1@2014 at or before 2015");
    assert!(!known_d.contains(&statute));
    assert!(rt.as_known_on(WallTime::from(2020)).expect("A2@2020").contains(&statute));
    let dir = anchor_admit_store_dir();
    rt.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
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
