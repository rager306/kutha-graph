//! M013 S05 / Phase 26: D-08 RED — dictionary entries on the same `as_of` walk as facts.
//! Admission, policy, and genesis allow live in fold side tables today. Do not edit
//! m012_*/m012a_*/m011_*/ff5/ff6 expects. Wiring is 26-02. Never edit `.kutha/STATE.md`.

use kutha_common::{rule_definition_hash, EventId, Op};
use kutha_runtime::Runtime;

fn leased_pin_and_derive() -> (Runtime, EventId, EventId, EventId) {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.fold().facts().len());

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    let pinned = rt
        .emit(Op::PinPolicy {
            definition: "leased-policy".into(),
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let pin_event = pinned.receipt.event_ids[0];

    let asserted = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let cause = asserted.receipt.event_ids[0];

    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: rule_definition_hash("derive_pq"),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    (rt, claim_q, cause, pin_event)
}

#[test]
fn admission_and_policy_visible_on_as_of_after_later_pin() {
    let (mut rt, claim_q, cause, _) = leased_pin_and_derive();
    rt.record_justification(
        claim_q,
        vec![cause],
        vec![cause],
        rule_definition_hash("derive_pq"),
        u64::MAX,
        2010,
    )
    .expect("record_justification");
    let cut_a = rt.log().len();

    rt.emit(Op::PinPolicy {
        definition: "later-policy".into(),
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();
    assert!(
        rt.log().len() > cut_a,
        "later PinPolicy must append after cut A ({cut_a})"
    );

    let policy = rt.intern("kutha:policy");
    let admission = rt.intern("kutha:admission");
    let triples = rt.fold().as_of(2010);
    assert!(
        triples.iter().any(|&(_, rel, _)| rel == policy),
        "as_of(2010) after later PinPolicy must contain a kutha:policy triple (cut A={cut_a})"
    );
    assert!(
        triples.iter().any(|&(_, rel, _)| rel == admission),
        "as_of(2010) after later PinPolicy must contain a kutha:admission triple (cut A={cut_a})"
    );
}

#[test]
fn genesis_allow_visible_on_as_of() {
    let mut rt = Runtime::default();
    let allows = rt.intern("kutha:allows");
    let triples = rt.fold().as_of(0);
    assert!(
        triples.iter().any(|&(_, rel, _)| rel == allows),
        "as_of(0) must contain a kutha:allows triple (genesis allow is a live fact, not a side-table entry)"
    );
}
