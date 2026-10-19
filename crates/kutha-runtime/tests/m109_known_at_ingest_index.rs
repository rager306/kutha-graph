//! known_at must not examine facts ingested after the cut when hot maps are warm.
use kutha_common::{Op, TransactionTime};
use kutha_runtime::{FoldIndexMode, Runtime};

#[test]
fn known_at_examines_stay_constant_vs_late_decoys() {
    let mut counts = Vec::new();
    let mut early_lens = Vec::new();
    for &d in &[32u32, 128] {
        let mut rt = Runtime::default();
        rt.set_index_mode(FoldIndexMode::Eager);
        let knows = rt.intern("knows");
        let early_s = rt.intern("Alice");
        let early_o = rt.intern("Bob");
        rt.emit(Op::Assert {
            subject: early_s,
            relation: knows,
            object: early_o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        let cut = rt
            .fold()
            .facts()
            .iter()
            .map(|f| f.ingested_at)
            .max()
            .unwrap();
        let early_len = rt
            .fold()
            .facts()
            .iter()
            .filter(|f| f.ingested_at <= cut)
            .count();
        for i in 0..d {
            let s = rt.intern(&format!("late-{i}"));
            let o = rt.intern(&format!("decoy-{i}"));
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
        assert!(
            rt.fold().facts().len() > early_len + (d as usize),
            "late decoys must add facts after the cut"
        );
        rt.fold().reset_hot_examine_count();
        let got = rt.fold().known_at(TransactionTime::from(cut));
        assert!(
            !got.is_empty(),
            "early cut must keep the early cohort"
        );
        assert!(
            got.iter().all(|&(s, _, o)| s == early_s || o == early_s || s == early_o || o == early_o),
            "late decoy subjects must not appear at the early cut"
        );
        counts.push(rt.fold().hot_examine_count());
        early_lens.push(early_len as u64);
    }
    assert!(
        counts[1] <= counts[0].saturating_add(2),
        "known_at visits must not grow with late decoys: {counts:?}"
    );
    assert!(
        counts[1] <= early_lens[1].saturating_add(4),
        "warm known_at must examine only the early ingest cohort: counts={counts:?} early={early_lens:?}"
    );
    assert!(
        counts[1] < 32,
        "warm known_at must not walk the late decoy set: {counts:?}"
    );
}
