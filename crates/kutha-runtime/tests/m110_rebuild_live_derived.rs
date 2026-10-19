//! Rebuild must not put dead Behavior ids back into derived_by_cause.
use kutha_common::Op;
use kutha_runtime::{store, Runtime};
use std::fs;

fn behavior_id(rt: &Runtime) -> kutha_common::EventId {
    rt.log()
        .iter()
        .filter(|e| matches!(e.op, Op::Behavior { .. }))
        .last()
        .expect("inverse_knows Behavior")
        .id
}

#[test]
fn open_after_correct_keeps_dead_behavior_out_of_retract_index() {
    let mut rt = Runtime::default();
    let s = rt.intern("Alice");
    let o = rt.intern("Bob");
    let knows = rt.intern("knows");
    let premise = rt
        .emit(Op::Assert {
            subject: s,
            relation: knows,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];
    let head = behavior_id(&rt);
    rt.correct("Bob", "knownBy", "Carol", 2010).unwrap();

    let dir = std::env::temp_dir().join(format!("kutha-m110-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).expect("open after Correct of Behavior");

    opened.reset_retract_examine_count();
    opened.emit(Op::Retract { event_id: premise }).unwrap();
    let n = opened.retract_examine_count();
    assert_eq!(
        n, 0,
        "rebuild must not reindex dead Behavior {head} under its cause; examined {n}"
    );
    let _ = fs::remove_dir_all(&dir);
}
