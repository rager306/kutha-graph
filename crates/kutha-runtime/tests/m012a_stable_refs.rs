//! M012a S02 / REF-01: Retract targets the minting EventId, not fold-local seq.

use kutha_common::{EventId, Op};
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn retract_by_event_id_invalidates_support() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let rel = rt.intern("relatedTo");
    let minted = rt
        .emit(Op::Assert {
            subject: a,
            relation: rel,
            object: b,
            valid_from: 0,
            valid_to: None,
            claim: None,
        })
        .unwrap();
    let minting = minted.receipt.event_ids[0];

    let n = rt.log().len();
    let err = rt
        .emit(Op::Retract {
            event_id: EventId::nil(),
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownFact { event_id } if event_id == EventId::nil()),
        "{err:?}"
    );
    assert_eq!(
        n,
        rt.log().len(),
        "fail-closed: unknown EventId must not append"
    );

    rt.emit(Op::Retract { event_id: minting }).unwrap();
    let fact = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == minting)
        .expect("minted fact remains");
    assert!(
        fact.invalidated_at.is_some(),
        "Retract must invalidate the Fact minted by that EventId"
    );
    assert_eq!(
        rt.fold().facts().len(),
        1,
        "relatedTo has no inverse cascade; loser row stays"
    );
    let last_retract = rt
        .log()
        .iter()
        .filter(|e| matches!(e.op, Op::Retract { .. }))
        .last()
        .expect("Retract on log");
    assert!(
        matches!(last_retract.op, Op::Retract { event_id } if event_id == minting),
        "last Retract must carry the minting EventId"
    );
    rt.replay_check().unwrap();
}

#[test]
fn justification_cites_source_event_ids_survive_fork() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let minted = rt
        .emit(Op::Assert {
            subject: a,
            relation: rel,
            object: b,
            valid_from: 0,
            valid_to: None,
            claim: None,
        })
        .unwrap();
    let minting = minted.receipt.event_ids[0];
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: minting,
            rule_version: "r1".into(),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 0,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    let t1 = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.claim_id == claim_q)
        .unwrap()
        .ingested_at;
    let jid = rt.record_justification(claim_q, vec![minting], vec![minting], "r1", t1, 0);
    rt.check_admission(&jid).unwrap();

    let fork = rt.fork_at(rt.log().len());
    rt.emit(Op::Retract { event_id: minting }).unwrap();
    let parent_err = rt.check_admission(&jid).unwrap_err();
    assert!(
        matches!(
            parent_err,
            RuntimeError::AdmissionDenied {
                reason: "stale_support",
                ..
            }
        ),
        "{parent_err:?}"
    );
    fork.check_admission(&jid)
        .expect("fork prefix still admits the cited EventId");
    let cited = fork
        .justification_records()
        .iter()
        .find(|j| j.justification_id == jid)
        .expect("hydrated cite");
    assert_eq!(cited.source_event_ids, vec![minting]);
    rt.replay_check().unwrap();
    fork.replay_check().unwrap();
}
