//! TH-03..TH-05: Eager/Lazy known-on, fork_at, mid-cut snapshot+tail, GraphFold::replay.

use kutha_common::{WallTime};
mod common;

use kutha_common::Op;
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime};

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
    assert_eq!(eager.as_known_on(WallTime::from(D)), lazy.as_known_on(WallTime::from(D)));
    assert_eq!(eager.as_known_on(WallTime::from(W2)), lazy.as_known_on(WallTime::from(W2)));
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
    let fork = rt.fork_at(cut);
    let live_w2 = rt.as_known_on(WallTime::from(W2));
    let fork_w2 = fork.as_known_on(WallTime::from(W2));
    assert_ne!(
        fork_w2, live_w2,
        "fork before later TtAnchor must not share live as_known_on(W2)"
    );
    let dir = std::path::Path::new("tmp/p36-02-fork");
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    fork.persist(dir).unwrap();
    let opened = Runtime::open(dir).unwrap();
    assert_eq!(opened.as_known_on(WallTime::from(W2)), fork_w2);
    let _ = std::fs::remove_dir_all(dir);
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
    assert_eq!(restored.as_known_on(WallTime::from(W1)), rt.as_known_on(WallTime::from(W1)));
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
    let mut replayed = GraphFold::replay(rt.log().as_slice());
    assert!(
        replayed.as_known_on(WallTime::from(W2)).is_some(),
        "GraphFold::replay must apply TtAnchor before dictionary bind"
    );
    replayed.bind_reserved(rt.dictionary());
    assert_eq!(replayed.as_known_on(WallTime::from(D)), rt.fold().as_known_on(WallTime::from(D)));
    assert_eq!(replayed.as_known_on(WallTime::from(W2)), rt.fold().as_known_on(WallTime::from(W2)));
    let known_w2 = replayed.as_known_on(WallTime::from(W2)).expect("replay applies TtAnchor");
    assert!(known_w2.contains(&statute));
}
