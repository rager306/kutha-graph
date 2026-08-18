//! FF5: same log, two valid-times, different live triples.
//! Cosine/RAG is not the oracle. Named `as_of` has no “now” default.

use kutha_common::Op;
use kutha_runtime::Runtime;

/// Year-like valid-time instants for a statute-shaped fixture (not wall-clock).
const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

#[test]
fn ff5_as_of_t1_differs_from_as_of_t2_on_statute_log() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_30 = rt.intern("30-kmh");

    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_30,
        valid_from: VF_SECOND,
        valid_to: None,
    })
    .unwrap();

    let old = rt.fold().as_of(T_OLD);
    let new = rt.fold().as_of(T_NEW);
    assert_ne!(
        old, new,
        "AS OF T1 must differ from AS OF T2 on the same log"
    );

    assert!(old.contains(&(article, in_force, limit_50)));
    assert!(!old.contains(&(article, in_force, limit_30)));
    assert!(new.contains(&(article, in_force, limit_30)));
    assert!(!new.contains(&(article, in_force, limit_50)));

    rt.replay_check().unwrap();

    let csr_old = rt.csr_lease_at(u64::MAX, T_OLD);
    let csr_new = rt.csr_lease_at(u64::MAX, T_NEW);
    assert_ne!(
        csr_old.neighbors(article),
        csr_new.neighbors(article),
        "CSR cut must agree with the fold: T_old ≠ T_new"
    );
    assert_eq!(csr_old.neighbors(article), &[limit_50]);
    assert_eq!(csr_new.neighbors(article), &[limit_30]);
}

#[test]
fn ff3_csr_lease_drop_does_not_change_log_or_fold() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: None,
    })
    .unwrap();
    let fp = rt.fold().fingerprint();
    let n = rt.log().len();
    {
        let lease = rt.csr_lease_at(u64::MAX, T_OLD);
        assert_eq!(lease.neighbors(article), &[limit_50]);
    }
    assert_eq!(fp, rt.fold().fingerprint());
    assert_eq!(n, rt.log().len());
}
