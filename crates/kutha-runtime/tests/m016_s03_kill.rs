//! Independent RULE-02 kill-tests. Lazy empty-pin cascade or retract-leave-inverse fail these.

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm};
use kutha_runtime::Runtime;

fn inverse_pin() -> String {
    let head = RuleAtom::new(
        "knownBy",
        RuleTerm::var(1).unwrap(),
        RuleTerm::var(0).unwrap(),
    )
    .unwrap();
    let prem = RuleAtom::new(
        "knows",
        RuleTerm::var(0).unwrap(),
        RuleTerm::var(1).unwrap(),
    )
    .unwrap();
    RuleAst::horn(head, vec![prem]).unwrap().hash()
}

#[test]
fn inverse_is_hashed_horn_not_empty_pin() {
    let mut rt = Runtime::default();
    let pin = inverse_pin();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let knows = rt.intern("knows");
    let out = rt
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
        .unwrap();
    let cause = out.receipt.event_ids[0];
    let inv = rt
        .log()
        .iter()
        .find_map(|e| match &e.op {
            Op::Behavior {
                name,
                caused_by,
                rule_version,
                ..
            } if name == "inverse_knows" => Some((e.id, *caused_by, rule_version.clone())),
            _ => None,
        })
        .expect("inverse_knows");
    assert_eq!(cause, inv.1);
    assert_eq!(pin, inv.2);
    assert_ne!("", inv.2);

    let n = rt.log().len();
    let related = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: related,
        object: b,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let inverses = rt
        .log()
        .iter()
        .filter(|e| matches!(&e.op, Op::Behavior { name, .. } if name == "inverse_knows"))
        .count();
    assert_eq!(1, inverses);
    assert!(rt.log().len() > n);

    rt.emit(Op::Retract { event_id: cause }).unwrap();
    let known_by = rt.intern("knownBy");
    assert!(
        !rt.fold()
            .facts()
            .iter()
            .any(|f| f.relation == known_by && f.invalidated_at.is_none()),
        "retract of knows must drop live inverse"
    );
}
