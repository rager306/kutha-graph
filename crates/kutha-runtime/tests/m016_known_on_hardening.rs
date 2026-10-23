//! TH-03..TH-05: Eager/Lazy known-on, fork_at, mid-cut snapshot+tail, GraphFold::replay.

use kutha_common::WallTime;
mod common;

use kutha_common::Op;
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime};

fn fork_known_on_store_dir() -> std::path::PathBuf {
    // Unique even when two Cargo processes run this binary together.
    // A fixed `tmp/p36-02-fork` is removed by the other process mid-persist.
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let seq = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p36-02-fork-{}-{nanos}-{seq}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("persist dir");
    dir
}

const D: u64 = 2015;
const W1: u64 = 2014;
const W2: u64 = 2020;
const VF: u64 = 2010;

#[test]
fn as_known_on_eager_equals_lazy() {
    let mut eager = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    let mut lazy = Runtime::with_index_mode(32, FoldIndexMode::Lazy);
    let s_e = common::seed_statute_between_anchors(&mut eager);
    let s_l = common::seed_statute_between_anchors(&mut lazy);
    assert_eq!(s_e, s_l, "same intern order yields the same TermIds");
    assert_eq!(
        eager.as_known_on(WallTime::from(D)),
        lazy.as_known_on(WallTime::from(D))
    );
    assert_eq!(
        eager.as_known_on(WallTime::from(W2)),
        lazy.as_known_on(WallTime::from(W2))
    );
}

#[test]
fn fork_at_before_later_anchor_differs_and_persist_open() {
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
    let cut = rt.log().len();
    rt.emit_tt_anchor(W2).unwrap();
    let fork = rt.fork_at(cut).unwrap();
    let live_w2 = rt.as_known_on(WallTime::from(W2));
    let fork_w2 = fork.as_known_on(WallTime::from(W2));
    assert_ne!(
        fork_w2, live_w2,
        "fork before later TtAnchor must not share live as_known_on(W2)"
    );
    let dir = fork_known_on_store_dir();
    fork.persist(&dir).unwrap();
    let opened = Runtime::open(&dir).unwrap();
    assert_eq!(opened.as_known_on(WallTime::from(W2)), fork_w2);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn fork_known_on_store_dirs_differ_across_calls() {
    let a = fork_known_on_store_dir();
    let b = fork_known_on_store_dir();
    assert_ne!(a, b, "parallel cargo must not share one persist directory");
    assert!(
        !a.ends_with("p36-02-fork"),
        "fixed tmp/p36-02-fork races across processes"
    );
    let _ = std::fs::remove_dir_all(a);
    let _ = std::fs::remove_dir_all(b);
}

#[test]
fn snapshot_mid_cut_tail_assert_and_anchor_recovers_known_on() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let s0 = rt.intern("s0");
    let o0 = rt.intern("o0");
    rt.emit(Op::Assert {
        subject: s0,
        relation: rel,
        object: o0,
        valid_from: 0,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let snap = rt.snapshot();
    let offset = snap.log_offset;
    let s1 = rt.intern("s1");
    let o1 = rt.intern("o1");
    rt.emit(Op::Assert {
        subject: s1,
        relation: rel,
        object: o1,
        valid_from: 0,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(W1).unwrap();
    assert!(
        rt.graph_len() > offset,
        "tail must contain a fold-affecting Assert"
    );
    let restored = Runtime::from_snapshot(snap, rt.log().as_slice().to_vec());
    assert_eq!(
        restored.as_known_on(WallTime::from(W1)),
        rt.as_known_on(WallTime::from(W1))
    );
    let known = restored
        .as_known_on(WallTime::from(W1))
        .expect("tail TtAnchor must hydrate");
    assert!(known.contains(&(s0, rel, o0)));
    assert!(known.contains(&(s1, rel, o1)));
}

#[test]
fn graph_fold_replay_as_known_on_matches_live() {
    let mut rt = Runtime::default();
    let statute = common::seed_statute_between_anchors(&mut rt);
    let replayed = GraphFold::replay_with_dict(rt.log().as_slice(), rt.dictionary());
    assert!(
        replayed.as_known_on(WallTime::from(W2)).is_some(),
        "GraphFold::replay_with_dict must apply TtAnchor and bind reserved"
    );
    assert_eq!(
        replayed.as_known_on(WallTime::from(D)),
        rt.fold().as_known_on(WallTime::from(D))
    );
    assert_eq!(
        replayed.as_known_on(WallTime::from(W2)),
        rt.fold().as_known_on(WallTime::from(W2))
    );
    let known_w2 = replayed
        .as_known_on(WallTime::from(W2))
        .expect("replay applies TtAnchor");
    assert!(known_w2.contains(&statute));
}
