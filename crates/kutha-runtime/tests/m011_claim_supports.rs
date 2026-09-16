//! M011 S01: independent supports share a claim_id; retract one, the other remains.

use kutha_common::{EventId, Op};
use kutha_runtime::Runtime;

#[test]
fn retracting_one_support_leaves_claim_supported() {
    let mut rt = Runtime::default();
    let p = rt.intern("P");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    let first = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
        })
        .unwrap();
    let claim: EventId = first.receipt.event_ids[0];
    let seq_a = rt.fold().facts()[0].seq;
    assert_eq!(rt.fold().facts()[0].claim_id, claim);
    assert_eq!(1, rt.fold().live_support_count(claim, u64::MAX, 2017));

    rt.emit(Op::Assert {
        subject: p,
        relation: rel,
        object: true_,
        valid_from: 2010,
        valid_to: None,
        claim: Some(claim),
    })
    .unwrap();
    assert_eq!(2, rt.fold().live_support_count(claim, u64::MAX, 2017));
    assert!(rt.fold().claim_supported_at(claim, u64::MAX, 2017));

    rt.emit(Op::Retract { fact_seq: seq_a }).unwrap();
    assert_eq!(1, rt.fold().live_support_count(claim, u64::MAX, 2017));
    assert!(
        rt.fold().claim_supported_at(claim, u64::MAX, 2017),
        "independent support must survive withdrawal of the other"
    );
    rt.replay_check().unwrap();
}
