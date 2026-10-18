//! Correct of a non-behavior must not walk `live_behaviors`.

use kutha_common::Op;
use kutha_runtime::Runtime;

#[test]
fn unindex_non_behavior_skips_live_behavior_vec() {
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
        rt.emit_allow("lonely", 0, None).unwrap();
        let s = rt.intern("Solo");
        let o = rt.intern("None");
        let lonely = rt.intern("lonely");
        rt.emit(Op::Assert {
            subject: s,
            relation: lonely,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        rt.reset_unindex_live_examine_count();
        rt.correct("Solo", "lonely", "Z", 2010).unwrap();
        counts.push(rt.unindex_live_examine_count());
    }
    assert_eq!(
        counts,
        vec![0, 0],
        "correct of a non-behavior must not walk live_behaviors: {counts:?}"
    );
}
