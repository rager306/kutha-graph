//! M104 S01 / Phase 175 — unindex_behavior must not scan every cause bucket
//! (CE whole-repo 2026-10-15 residual).

use kutha_common::Op;
use kutha_runtime::Runtime;

fn behavior_id(rt: &Runtime) -> kutha_common::EventId {
    rt.log()
        .iter()
        .filter(|e| matches!(e.op, Op::Behavior { .. }))
        .last()
        .expect("inverse_knows Behavior")
        .id
}

/// Correct of one Behavior examines a constant number of cause buckets as live D grows.
#[test]
fn unindex_cause_examines_constant_in_live_d() {
    let mut counts = Vec::new();
    for &d in &[32u32, 128] {
        let mut rt = Runtime::default();
        for i in 0..d {
            let s = rt.intern(&format!("Alice-{i}"));
            let o = rt.intern(&format!("Bob-{i}"));
            let knows = rt.intern("knows");
            rt.emit(Op::Assert {
                subject: s,
                relation: knows,
                object: o,
                valid_from: 2010,
                valid_to: None,
                claim: None,
                delivery_key: None,
                polarity: None,
            })
            .unwrap();
        }
        let live_d = rt
            .log()
            .iter()
            .filter(|e| matches!(e.op, Op::Behavior { .. }))
            .count();
        assert!(
            live_d >= d as usize,
            "expected ≥{d} inverse_knows Behaviors, got {live_d}"
        );

        let head = behavior_id(&rt);
        rt.reset_unindex_cause_examine_count();
        rt.correct(&format!("Bob-{}", d - 1), "knownBy", "Carol", 2010)
            .unwrap();
        let _ = head;
        counts.push(rt.unindex_cause_examine_count());
    }
    assert!(
        counts[1] <= counts[0] + 2,
        "unindex must not grow with live D cause buckets: {counts:?}"
    );
    assert!(
        counts[1] < 8,
        "unindex of one Behavior must touch a constant cause bucket count: {counts:?}"
    );
}
