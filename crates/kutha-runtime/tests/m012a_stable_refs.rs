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
