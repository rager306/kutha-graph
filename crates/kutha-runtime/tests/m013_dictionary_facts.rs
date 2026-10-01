//! M013 S05 / Phase 26: D-08 GREEN — dictionary entries on `dictionary_as_of`.
//! Product `as_of` excludes `kutha:`. Never edit `.kutha/STATE.md`.

use kutha_common::{rule_definition_hash, EventId, Op, KUTHA_ALLOWS};
use kutha_runtime::{Runtime, RuntimeError};

fn leased_pin_and_derive() -> (Runtime, EventId, EventId, EventId) {
    let mut rt = Runtime::default();
    assert_eq!(0, rt.fold().live_count(u64::MAX, 0));

    let p = rt.intern("P");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");

    rt.emit_rule("derive_pq", 2010, None).unwrap();
    let pinned = rt.emit_policy("leased-policy", 2010, None).unwrap();
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
    let jid = rt
        .record_justification(
            claim_q,
            vec![cause],
            vec![cause],
            rule_definition_hash("derive_pq"),
            u64::MAX,
            2010,
        )
        .expect("record_justification");
    let cut_a = rt.log().len();

    rt.emit_policy("later-policy", 2010, None).unwrap();
    assert!(
        rt.log().len() > cut_a,
        "later PinPolicy must append after cut A ({cut_a})"
    );

    let policy = rt.intern("kutha:policy");
    let admission = rt.intern("kutha:admission");
    let triples = rt.fold().dictionary_as_of(2010);
    assert!(
        triples.iter().any(|&(_, rel, _)| rel == policy),
        "dictionary_as_of(2010) after later PinPolicy must contain a kutha:policy triple (cut A={cut_a})"
    );
    assert!(
        triples.iter().any(|&(_, rel, _)| rel == admission),
        "dictionary_as_of(2010) after later PinPolicy must contain a kutha:admission triple (cut A={cut_a})"
    );

    let admission_id: EventId = jid.parse().expect("admission EventId");
    rt.emit(Op::Retract {
        event_id: admission_id,
    })
    .unwrap();
    let later = rt.fold().dictionary_as_of(2010);
    assert!(
        !later.iter().any(|&(_, rel, _)| rel == admission),
        "dictionary_as_of after Retract must lack kutha:admission"
    );
    let fork = rt.fork_at(cut_a);
    let fork_triples = fork.fold().dictionary_as_of(2010);
    assert!(
        fork_triples.iter().any(|&(_, rel, _)| rel == admission),
        "fork_at(cut A) must still contain kutha:admission"
    );
}

#[test]
fn genesis_allow_visible_on_as_of() {
    let mut rt = Runtime::default();
    let allows = rt.intern("kutha:allows");
    let triples = rt.fold().dictionary_as_of(0);
    assert!(
        triples.iter().any(|&(_, rel, _)| rel == allows),
        "dictionary_as_of(0) must contain a kutha:allows triple (genesis allow is a live fact)"
    );

    let retract_id = rt.log().iter().find_map(|e| match &e.op {
        Op::Assert {
            delivery_key: Some(name),
            relation,
            ..
        } if name == "relatedTo" && rt.dictionary().lookup(*relation) == Some(KUTHA_ALLOWS) => {
            Some(e.id)
        }
        _ => None,
    });
    let cut = rt.log().len();
    if let Some(id) = retract_id {
        rt.emit(Op::Retract { event_id: id }).unwrap();
    }
    let relation = rt.intern("relatedTo");
    let subject = rt.intern("alice-dict");
    let object = rt.intern("bob-dict");
    let n = rt.log().len();
    let err = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "relatedTo"),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");
    let mut prefix = rt.fork_at(cut);
    let rel = prefix.intern("relatedTo");
    let s = prefix.intern("alice-dict");
    let o = prefix.intern("bob-dict");
    prefix
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
}
