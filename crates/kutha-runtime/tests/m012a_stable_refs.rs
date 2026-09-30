//! M012a S02 / REF-01: Retract targets the minting EventId, not fold-local seq.

use kutha_common::{Event, EventId, Op};
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

#[test]
fn rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads() {
    let mut rt = Runtime::default();
    let filler_s = rt.intern("filler");
    let target_s = rt.intern("target");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit(Op::Assert {
        subject: filler_s,
        relation: rel,
        object: true_,
        valid_from: 0,
        valid_to: None,
        claim: None,
    })
    .unwrap();
    let minted = rt
        .emit(Op::Assert {
            subject: target_s,
            relation: rel,
            object: true_,
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
    rt.emit(Op::Retract { event_id: minting }).unwrap();

    let live = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == minting)
        .expect("target fact on live runtime");
    assert_eq!(live.seq, 1, "filler occupies seq 0; target is seq 1");
    assert!(live.invalidated_at.is_some());

    let mut with_retract = rt.log().as_slice().to_vec();
    drop_first_assert(&mut with_retract);
    let rebuilt = Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        with_retract,
        rt.max_cascade,
    )
    .unwrap();
    let rebuilt_fact = rebuilt
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == minting)
        .expect("same EventId after rebuild");
    assert_eq!(
        rebuilt_fact.seq, 0,
        "dropping filler renumbers the target to seq 0"
    );
    assert!(rebuilt_fact.invalidated_at.is_some());
    assert!(
        rebuilt
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::Retract { event_id } if event_id == minting)),
        "rebuilt log Retract still carries the minting EventId"
    );
    let stale = rebuilt.check_admission(&jid).unwrap_err();
    assert!(
        matches!(
            stale,
            RuntimeError::AdmissionDenied {
                reason: "stale_support",
                ..
            }
        ),
        "{stale:?}"
    );

    let mut no_retract = rt.log().as_slice().to_vec();
    no_retract.retain(|e| !matches!(e.op, Op::Retract { .. }));
    drop_first_assert(&mut no_retract);
    let rebuilt_live = Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        no_retract,
        rt.max_cascade,
    )
    .unwrap();
    let live_renumbered = rebuilt_live
        .fold()
        .facts()
        .iter()
        .find(|f| f.event_id == minting)
        .expect("cite-only rebuild keeps the target Fact");
    assert_eq!(live_renumbered.seq, 0);
    assert_ne!(
        live.seq, live_renumbered.seq,
        "fold-local seqs differ across the pair"
    );
    rebuilt_live
        .check_admission(&jid)
        .expect("cite Event still admits after filler drop without Retract");
    rebuilt.replay_check().unwrap();
    rebuilt_live.replay_check().unwrap();
}

fn drop_first_assert(events: &mut Vec<Event>) {
    let i = events
        .iter()
        .position(|e| matches!(e.op, Op::Assert { .. }))
        .expect("filler Assert");
    events.remove(i);
}
