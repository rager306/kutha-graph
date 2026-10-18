//! Unindex of one Behavior must not walk every live Behavior id.

use kutha_common::Op;
use kutha_runtime::Runtime;

#[test]
fn unindex_one_behavior_live_visits_stay_constant() {
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
        rt.reset_unindex_live_examine_count();
        rt.correct(&format!("Bob-{}", d - 1), "knownBy", "Carol", 2010)
            .unwrap();
        counts.push(rt.unindex_live_examine_count());
    }
    assert!(
        counts[1] <= counts[0].saturating_add(2),
        "live-set visits must not grow with D: {counts:?}"
    );
    assert!(
        counts[1] < 8,
        "one Behavior unindex must touch a constant live-set count: {counts:?}"
    );
}
