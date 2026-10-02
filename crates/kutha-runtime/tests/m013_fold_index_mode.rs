//! FoldIndexMode: Eager (accept write-cost) vs Lazy (thin write / lazy lease).

use kutha_common::{Op, TransactionTime, ValidTime};
use kutha_runtime::{FoldIndexMode, Runtime};

fn fill(rt: &mut Runtime, n: u32) {
    let rel = rt.intern("relatedTo");
    let obj = rt.intern("o");
    for i in 0..n {
        let s = rt.intern(&format!("s{i}"));
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
}

fn live_sorted(rt: &Runtime) -> Vec<(u32, u32, u32)> {
    let mut v = rt.fold().as_of(ValidTime::from(2010));
    v.sort_unstable();
    v
}

#[test]
fn lazy_as_of_matches_eager_after_same_ingest() {
    let n = 64u32;
    let mut eager = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    let mut lazy = Runtime::with_index_mode(32, FoldIndexMode::Lazy);
    fill(&mut eager, n);
    fill(&mut lazy, n);

    let a = live_sorted(&eager);
    let b = live_sorted(&lazy);
    assert_eq!(a.len(), n as usize);
    assert_eq!(a, b, "Lazy scan path must match Eager lease as_of");
    // EventIds differ across independent runtimes — compare shape, not fingerprint.
    assert_eq!(eager.fold().facts().len(), lazy.fold().facts().len());
    assert_eq!(
        eager
            .fold()
            .live_count(TransactionTime::MAX, ValidTime::from(2010)),
        lazy.fold()
            .live_count(TransactionTime::MAX, ValidTime::from(2010))
    );

    lazy.ensure_fold_indexes();
    assert_eq!(
        a,
        live_sorted(&lazy),
        "after ensure_fold_indexes, as_of still matches"
    );
}

#[test]
fn set_index_mode_roundtrips_eager_lazy() {
    let mut rt = Runtime::with_index_mode(1, FoldIndexMode::Eager);
    assert_eq!(rt.index_mode(), FoldIndexMode::Eager);
    fill(&mut rt, 8);
    let before = live_sorted(&rt);
    rt.set_index_mode(FoldIndexMode::Lazy);
    assert_eq!(rt.index_mode(), FoldIndexMode::Lazy);
    assert_eq!(before, live_sorted(&rt));
    rt.set_index_mode(FoldIndexMode::Eager);
    assert_eq!(rt.index_mode(), FoldIndexMode::Eager);
    assert_eq!(before, live_sorted(&rt));
}

#[test]
fn lazy_admit_does_not_scan_product_facts() {
    let n = 128u32;
    let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Lazy);
    fill(&mut rt, n);
    rt.fold().reset_hot_examine_count();
    let rel = rt.intern("relatedTo");
    let extra = rt.intern("s-lazy-extra");
    let obj = rt.intern("o");
    rt.emit(Op::Assert {
        subject: extra,
        relation: rel,
        object: obj,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let examined = rt.fold().hot_examine_count();
    assert!(
        examined < 32,
        "Lazy product maps stay cold; admit must use the allows lease: examined={examined}"
    );
    assert_eq!(live_sorted(&rt).len(), (n + 1) as usize);
}
