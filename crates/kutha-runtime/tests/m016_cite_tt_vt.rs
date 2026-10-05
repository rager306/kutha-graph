//! TH-02: JustificationCite stores distinct tt/vt; a swapped clone fails equality.

use kutha_common::{Op, TransactionTime, ValidTime, rule_definition_hash};
use kutha_runtime::{Justification, Runtime};

#[test]
fn swapped_justification_cite_tt_vt_fails_equality() {
    let mut rt = Runtime::default();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let q = rt.intern("Q");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let pin = rule_definition_hash("derive_pq");
    rt.emit_rule("derive_pq", 0, None).unwrap();
    rt.emit_policy("leased-policy", 0, None).unwrap();
    let minted = rt
        .emit(Op::Assert {
            subject: a,
            relation: rel,
            object: b,
            valid_from: 0,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let minting = minted.receipt.event_ids[0];
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: minting,
            rule_version: pin.clone(),
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
    assert_ne!(
        t1, 0,
        "cite tt must differ from vt=0 so a swap is observable"
    );
    let jid = rt
        .record_justification(
            claim_q,
            vec![minting],
            vec![minting],
            pin.clone(),
            TransactionTime::from(t1),
            ValidTime::from(0),
        )
        .expect("record_justification");

    let stored = rt
        .justification_records()
        .into_iter()
        .find(|j| j.justification_id == jid)
        .expect("stored JustificationCite");
    assert_eq!(stored.tt, t1, "cite.tt is the transaction-time argument");
    assert_eq!(stored.vt, 0, "cite.vt is the valid-time argument");
    assert_ne!(stored.tt, stored.vt);

    let swapped = Justification {
        tt: stored.vt,
        vt: stored.tt,
        ..stored.clone()
    };
    assert_ne!(
        swapped, stored,
        "swapping tt/vt must not equal the stored cite"
    );
    let log_cite = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find_map(|e| match &e.op {
            Op::JustificationCite { tt, vt, .. } => Some((*tt, *vt)),
            _ => None,
        })
        .expect("Op::JustificationCite on log");
    assert_eq!(log_cite, (stored.tt, stored.vt));
    assert_ne!(log_cite, (stored.vt, stored.tt));
}
