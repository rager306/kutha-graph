//! M022 S02 — admission_status_at resolves via admission EventId (delivery_key = policy).

use kutha_common::{Op, policy_version_hash};
use kutha_runtime::Runtime;

#[test]
fn adm_pin01_admission_status_via_event_id() {
    let mut rt = Runtime::default();
    let policy_body = "leased-policy";
    let pin = policy_version_hash(policy_body);
    rt.emit_policy(policy_body, 2010, None).unwrap();
    let a = rt.intern("a");
    let knows = rt.intern("knows");
    let b = rt.intern("b");
    let claim = rt
        .emit(Op::Assert {
            subject: a,
            relation: knows,
            object: b,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];
    let jid = rt
        .record_justification(claim, vec![claim], vec![claim], "rule-v1", u64::MAX, 2010)
        .unwrap();
    assert!(
        rt.admission_status_at(&jid, u64::MAX, 2010).is_some(),
        "admission EventId string must resolve (true or false)"
    );
    let admission = rt
        .log()
        .iter()
        .find_map(|e| match &e.op {
            Op::Assert {
                delivery_key: Some(k),
                ..
            } if e.id.to_string() == jid => Some(k.clone()),
            _ => None,
        })
        .expect("admission Assert in log");
    assert_eq!(admission, pin);
}
