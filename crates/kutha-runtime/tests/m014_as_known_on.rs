//! ANK-03: “true on D” (`as_of` / valid-time) differs from “known on D” (`as_known_on`).
//! FF5 stays untouched; this file is the named divergence oracle.

use kutha_common::{ValidTime, WallTime};
mod common;

use kutha_common::Op;
use kutha_runtime::Runtime;

fn known_on_store_dir() -> std::path::PathBuf {
    // Unique even when two Cargo processes run this binary together.
    // A fixed `tmp/p32-02-store` is removed by the other process mid-persist.
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
            "p32-02-store-{}-{nanos}-{seq}",
            std::process::id()
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

const D: u64 = 2015;
const W1: u64 = 2014;
const W2: u64 = 2020;
const VF: u64 = 2010;

#[test]
fn true_on_d_differs_from_known_on_d() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");

    rt.emit_tt_anchor(W1).unwrap();
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(W2).unwrap();

    let statute = (article, in_force, limit_50);
    assert!(
        rt.as_of(ValidTime::from(D)).contains(&statute),
        "true on D: as_of(2015) sees the statute (valid_from 2010)"
    );
    let known_2015 = rt.as_known_on(WallTime::from(D)).expect("anchor 2014 is at or before 2015");
    assert!(
        !known_2015.contains(&statute),
        "known on D: as_known_on(2015) uses 2014 tt, before the statute event"
    );
    let known_2020 = rt
        .as_known_on(WallTime::from(W2))
        .expect("anchor 2020 is at or before 2020");
    assert!(
        known_2020.contains(&statute),
        "as_known_on(2020) uses the later anchor and sees the statute"
    );
}

#[test]
fn as_known_on_miss_without_anchor() {
    let rt = Runtime::default();
    assert!(
        rt.as_known_on(WallTime::from(D)).is_none(),
        "without an anchor at or before D, as_known_on is None"
    );
}

#[test]
fn as_known_on_uses_greatest_anchor_at_or_before() {
    let mut rt = Runtime::default();
    let statute = common::seed_statute_between_anchors(&mut rt);
    let known_d = rt
        .as_known_on(WallTime::from(D))
        .expect("A1@2014 is at or before D=2015 (W1 < D ≤ W2)");
    assert!(
        !known_d.contains(&statute),
        "W1 < D ≤ W2 resolves A1 tt, before the statute"
    );
    assert!(
        rt.as_known_on(WallTime::from(2013)).is_none(),
        "no anchor at or before 2013"
    );
}

#[test]
fn persist_open_store_dirs_differ_across_calls() {
    let a = known_on_store_dir();
    let b = known_on_store_dir();
    assert_ne!(a, b, "parallel cargo must not share one persist directory");
    assert!(
        !a.ends_with("p32-02-store"),
        "fixed tmp/p32-02-store races across processes"
    );
    let _ = std::fs::remove_dir_all(a);
    let _ = std::fs::remove_dir_all(b);
}

#[test]
fn as_known_on_persist_open_preserves_answers() {
    let mut rt = Runtime::default();
    assert!(rt.as_known_on(WallTime::from(D)).is_none());
    let statute = common::seed_statute_between_anchors(&mut rt);
    let dir = known_on_store_dir();
    rt.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    assert!(opened.as_known_on(WallTime::from(2013)).is_none());
    let known_d = opened.as_known_on(WallTime::from(D)).expect("A1 rebuilt on open");
    assert!(!known_d.contains(&statute));
    let known_w2 = opened.as_known_on(WallTime::from(W2)).expect("A2 rebuilt on open");
    assert!(known_w2.contains(&statute));
    let _ = std::fs::remove_dir_all(dir);
}
