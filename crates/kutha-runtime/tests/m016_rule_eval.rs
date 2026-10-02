//! RULE-02: n-ary Horn eval; premise retract drops Horn-derived support.

use kutha_common::{Event, Op, RuleAst, RuleAtom, RuleTerm};
use kutha_runtime::Runtime;

fn two_prem_horn() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn derived_related<'a>(log: impl IntoIterator<Item = &'a Event>, pin: &str) -> Vec<&'a Event> {
    log.into_iter()
        .filter(|e| {
            matches!(
                &e.op,
                Op::Behavior {
                    rule_version,
                    name,
                    ..
                } if rule_version == pin && name == "relatedTo"
            )
        })
        .collect()
}

#[test]
fn nary_horn_fires_when_both_premises_live() {
    let mut rt = Runtime::default();
    let ast = two_prem_horn();
    let pin = ast.hash();
    rt.emit_rule_ast(&ast, 2010, None).unwrap();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert_eq!(
        0,
        derived_related(rt.log().iter(), &pin).len(),
        "one premise must not fire the 2-prem Horn"
    );
    let mid = rt
        .emit(Op::Assert {
            subject: b,
            relation: rel,
            object: c,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let mid_id = mid.receipt.event_ids[0];
    assert_eq!(
        1,
        derived_related(rt.log().iter(), &pin).len(),
        "both premises live must mint derived relatedTo"
    );

    rt.emit(Op::Retract { event_id: mid_id }).unwrap();
    assert!(
        !rt.fold().facts().iter().any(|f| {
            f.subject == a && f.object() == c && f.relation == rel && f.invalidated_at.is_none()
        }),
        "retract of one Horn premise must drop derived support"
    );
}
