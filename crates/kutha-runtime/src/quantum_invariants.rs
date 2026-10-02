//! In-crate Runtime invariants (moved out of quantum.rs so src files stay ≤800).
use super::*;
use kutha_common::Op;
use std::time::Instant;

#[test]
fn assert_then_fold_has_live_edge() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert_eq!(rt.fold().live_count(u64::MAX, 0), 2); // knows + inverse
    rt.replay_check().unwrap();
}

#[test]
fn retract_keeps_loser() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: r,
        object: b,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let minting = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.relation == r)
        .expect("knows fact")
        .event_id;
    rt.emit(Op::Retract { event_id: minting }).unwrap();
    assert_eq!(rt.fold().live_count(u64::MAX, 0), 1); // inverse still live
    assert!(!rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.relation == r)
        .unwrap()
        .is_live_at(u64::MAX, 0));
    assert!(rt.fold().facts().iter().any(|f| f.invalidated_at.is_some()));
    rt.replay_check().unwrap();
}

#[test]
fn cascade_idles_without_inverse_loop() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    let q = rt
        .emit(Op::Assert {
            subject: a,
            relation: r,
            object: b,
            valid_from: 0,
            valid_to: None,

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    assert_eq!(q.events_in_quantum, 2);
    assert!(!q.receipt.aborted_on_budget);
    assert_eq!(rt.fold().live_count(u64::MAX, 0), 2);
}

#[test]
fn budget_aborts_storm() {
    let mut rt = Runtime::new(1);
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    let q = rt
        .emit(Op::Assert {
            subject: a,
            relation: r,
            object: b,
            valid_from: 0,
            valid_to: None,

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    assert!(q.receipt.aborted_on_budget);
    assert_eq!(q.events_in_quantum, 1);
}

#[test]
fn replay_detects_tamper() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: r,
        object: b,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.tamper_fold();
    let err = rt.replay_check().unwrap_err();
    assert!(matches!(err, RuntimeError::ReplayDivergence { .. }));
}

#[test]
fn replay_time_vs_log_length() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    for i in 0..200u32 {
        let s = rt.intern(&format!("n{i}"));
        let o = rt.intern(&format!("n{}", i + 1));
        rt.emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 0,
            valid_to: None,

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
    let start = Instant::now();
    rt.replay_check().unwrap();
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    eprintln!(
        "kutha P0 measure: replay_check log_len={} live={} elapsed_ms={:.3}",
        rt.log().len(),
        rt.fold().live_count(u64::MAX, 0),
        ms
    );
    assert!(ms < 1000.0, "replay unexpectedly slow: {ms} ms");
}

#[test]
fn snapshot_plus_tail_matches_full_replay() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    for i in 0..10u32 {
        let s = rt.intern(&format!("s{i}"));
        let o = rt.intern(&format!("o{i}"));
        rt.emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 0,
            valid_to: None,

            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
    let snap = rt.snapshot();
    let events = rt.log().as_slice().to_vec();
    let restored = Runtime::from_snapshot(snap, events);
    assert_eq!(restored.fold().fingerprint(), rt.fold().fingerprint());
    restored.replay_check().unwrap();
}

#[test]
fn persist_open_round_trip() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: r,
        object: b,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let dir = std::env::temp_dir().join(format!("kutha-p0-{}", uuid_like()));
    crate::store::persist(&rt, &dir).unwrap();
    let opened = crate::store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.dictionary().lookup(a), Some("A"));
    opened.replay_check().unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn csr_drop_rebuild_and_seek() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: a,
        relation: r,
        object: b,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    // Fixture facts use valid_from=0; name that cut (no silent “now”).
    let csr = rt.csr_lease_at(u64::MAX, 0);
    assert_eq!(csr.neighbors(a), &[b]);
    assert_eq!(csr.seek(a, b), Some(b));
    assert_eq!(csr.seek(a, b.saturating_add(1)), None);
    drop(csr);
    let csr2 = rt.csr_lease_at(u64::MAX, 0);
    assert_eq!(csr2.neighbors(a), &[b]);
}

#[test]
fn fork_at_isolates_prefix() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let n0 = rt.intern("n0");
    let n1 = rt.intern("n1");
    let n2 = rt.intern("n2");
    rt.emit(Op::Assert {
        subject: n0,
        relation: rel,
        object: n1,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let cut = rt.log().len();
    rt.emit(Op::Assert {
        subject: n1,
        relation: rel,
        object: n2,
        valid_from: 0,
        valid_to: None,

        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let fork = rt.fork_at(cut);
    assert_eq!(fork.log().len(), cut);
    assert!(fork.fold().live_count(u64::MAX, 0) < rt.fold().live_count(u64::MAX, 0));
    fork.replay_check().unwrap();
}

#[test]
fn as_known_on_after_anchor_is_some_earlier_wall_is_none() {
    let mut rt = Runtime::default();
    let facts_before = rt.fold().facts().len();
    rt.emit_tt_anchor(2015).unwrap();
    assert_eq!(rt.fold().facts().len(), facts_before);
    assert!(rt.as_known_on(2015).is_some());
    assert!(rt.as_known_on(2014).is_none());
    let _as_of: Vec<_> = rt.as_of(kutha_common::ValidTime::from(2015));
    assert!(rt.as_known_on(kutha_common::WallTime::from(2015)).is_some());
}

#[test]
fn as_known_on_survives_persist_open_and_matches_fold() {
    let mut rt = Runtime::default();
    rt.emit_tt_anchor(2015).unwrap();
    assert_eq!(rt.as_known_on(2015), rt.fold().as_known_on(2015));
    let dir = std::path::Path::new("tmp/p32-01-store");
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    rt.persist(dir).unwrap();
    let opened = Runtime::open(dir).unwrap();
    assert!(opened.as_known_on(2015).is_some());
    assert_eq!(opened.as_known_on(2015), opened.fold().as_known_on(2015));
    let _ = std::fs::remove_dir_all(dir);
}

fn uuid_like() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
