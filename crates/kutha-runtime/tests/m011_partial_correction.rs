//! M011 S04: explicit interval patch leaves residual VT versions of source `a`.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{Runtime, RuntimeError};

/// Year-like valid-time instants for the residual oracle (not wall-clock).
const VF_WIDE: u64 = 2010;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;
const VT_LEFT: u64 = 2012;
const VT_RIGHT: u64 = 2021;
const VT_INTERIOR: u64 = 2017;

#[test]
fn interval_patch_leaves_vt_2012_and_2021_residuals() {
    let mut rt = Runtime::default();
    let related = rt.intern("relatedTo");
    let a = rt.intern("a");
    let p = rt.intern("P");
    let p_prime = rt.intern("P-prime");

    rt.emit(Op::Assert {
        subject: a,
        relation: related,
        object: p,
        valid_from: VF_WIDE,
        valid_to: None,
        claim: None,
    })
    .unwrap();

    let original = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.object() == p && f.invalidated_at.is_none())
        .expect("live Assert fact for P")
        .clone();
    let fact_seq = original.seq;
    let claim_id: EventId = original.claim_id;
    let orig_ingested = original.ingested_at;

    rt.emit(Op::CorrectInterval {
        fact_seq,
        object: p_prime,
        patch_from: PATCH_FROM,
        patch_to: Some(PATCH_TO),
    })
    .unwrap();

    let as_left = rt.fold().as_of(VT_LEFT);
    let as_right = rt.fold().as_of(VT_RIGHT);
    let as_mid = rt.fold().as_of(VT_INTERIOR);
    assert!(
        as_left.contains(&(a, related, p)),
        "VT 2012 residual of source a must keep P"
    );
    assert!(
        as_right.contains(&(a, related, p)),
        "VT 2021 residual of source a must keep P"
    );
    assert!(
        as_mid.contains(&(a, related, p_prime)),
        "interior VT 2017 must see the replacement"
    );
    assert!(
        !as_mid.contains(&(a, related, p)),
        "interior VT 2017 must not keep the original object"
    );

    for vt in [VT_LEFT, VT_INTERIOR, VT_RIGHT] {
        for f in rt.fold().live_supports(claim_id, u64::MAX, vt) {
            assert_eq!(
                f.claim_id, claim_id,
                "residuals and replacement share the pre-patch claim_id"
            );
            assert_eq!(f.subject, a);
            assert_eq!(f.relation, related);
        }
    }

    let loser = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.seq == fact_seq)
        .expect("original row remains");
    assert!(loser.invalidated_at.is_some());
    assert_eq!(loser.valid_from, VF_WIDE);
    assert_eq!(loser.valid_to, None);

    let old_tt = rt.fold().live_at(orig_ingested, VT_INTERIOR);
    assert!(
        old_tt.contains(&(a, related, p)),
        "old TT must still see P; original VT bounds are not clipped"
    );

    let round_trip = Event::new(
        Op::CorrectInterval {
            fact_seq,
            object: p_prime,
            patch_from: PATCH_FROM,
            patch_to: Some(PATCH_TO),
        },
        0,
    );
    let value = serde_json::to_value(&round_trip).unwrap();
    let back: Event = serde_json::from_value(value).unwrap();
    assert_eq!(round_trip.op, back.op);

    rt.replay_check().unwrap();
}

#[test]
fn interval_patch_unknown_fact_does_not_append() {
    let mut rt = Runtime::default();
    let p_prime = rt.intern("P-prime");
    let n = rt.log().len();
    let err = rt
        .emit(Op::CorrectInterval {
            fact_seq: 99,
            object: p_prime,
            patch_from: PATCH_FROM,
            patch_to: Some(PATCH_TO),
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownFact { fact_seq: 99 }),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: unknown fact_seq must not append"
    );
}

#[test]
fn interval_patch_non_intersect_does_not_append() {
    let mut rt = Runtime::default();
    let related = rt.intern("relatedTo");
    let a = rt.intern("a");
    let p = rt.intern("P");
    let p_prime = rt.intern("P-prime");
    rt.emit(Op::Assert {
        subject: a,
        relation: related,
        object: p,
        valid_from: VF_WIDE,
        valid_to: Some(PATCH_FROM),
        claim: None,
    })
    .unwrap();
    let fact_seq = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.object() == p && f.invalidated_at.is_none())
        .expect("live Assert")
        .seq;
    let n = rt.log().len();
    let err = rt
        .emit(Op::CorrectInterval {
            fact_seq,
            object: p_prime,
            patch_from: PATCH_FROM,
            patch_to: Some(PATCH_TO),
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::IntervalPatchRejected { fact_seq: seq } if seq == fact_seq),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: half-open touching [2010,2015) vs [2015,2020) must not append"
    );
}

#[test]
fn interval_patch_inverted_does_not_append() {
    let mut rt = Runtime::default();
    let related = rt.intern("relatedTo");
    let a = rt.intern("a");
    let p = rt.intern("P");
    let p_prime = rt.intern("P-prime");
    rt.emit(Op::Assert {
        subject: a,
        relation: related,
        object: p,
        valid_from: VF_WIDE,
        valid_to: None,
        claim: None,
    })
    .unwrap();
    let fact_seq = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.object() == p && f.invalidated_at.is_none())
        .expect("live Assert")
        .seq;
    let n = rt.log().len();
    let err = rt
        .emit(Op::CorrectInterval {
            fact_seq,
            object: p_prime,
            patch_from: PATCH_TO,
            patch_to: Some(PATCH_FROM),
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::IntervalPatchRejected { fact_seq: seq } if seq == fact_seq),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: inverted patch_from/patch_to must not append"
    );
}

#[test]
fn interval_patch_not_live_does_not_append() {
    let mut rt = Runtime::default();
    let related = rt.intern("relatedTo");
    let a = rt.intern("a");
    let p = rt.intern("P");
    let p_prime = rt.intern("P-prime");
    rt.emit(Op::Assert {
        subject: a,
        relation: related,
        object: p,
        valid_from: VF_WIDE,
        valid_to: None,
        claim: None,
    })
    .unwrap();
    let fact_seq = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.object() == p && f.invalidated_at.is_none())
        .expect("live Assert")
        .seq;
    rt.emit(Op::Retract { fact_seq }).unwrap();
    let n = rt.log().len();
    let err = rt
        .emit(Op::CorrectInterval {
            fact_seq,
            object: p_prime,
            patch_from: PATCH_FROM,
            patch_to: Some(PATCH_TO),
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::IntervalPatchRejected { fact_seq: seq } if seq == fact_seq),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: already-invalidated target must not append"
    );
}
