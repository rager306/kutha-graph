//! M017 S04 / PERF-01: Horn-visit scaling oracle (RED on HEAD).
//! Extra relatedTo ingest must not walk O(N) Horn rows (live_horns / search_at).

use kutha_common::Op;
use kutha_runtime::Runtime;

fn emit_plain_related(rt: &mut Runtime, i: u32) {
    let s = rt.intern(&format!("s{i}"));
    let rel = rt.intern("relatedTo");
    let obj = rt.intern("o");
    rt.emit(Op::Assert {
        subject: s,
        relation: rel,
        object: obj,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

#[test]
fn extra_relatedto_horn_visits_constant_in_n() {
    let mut counts = Vec::new();
    for &n in &[64u32, 256] {
        let mut rt = Runtime::default();
        for i in 0..n {
            emit_plain_related(&mut rt, i);
        }
        rt.fold().reset_horn_examine_count();
        emit_plain_related(&mut rt, n);
        counts.push(rt.fold().horn_examine_count());
    }
    assert!(
        counts[1] <= counts[0] + 8,
        "horn visits must not grow with N: {counts:?}"
    );
    assert!(
        counts[1] < 32,
        "horn visits on one extra relatedTo must stay bounded: {counts:?}"
    );
}
