//! M101 S01 / Phase 172 — rule-pin discovery is overlap-aware
//! (CE whole-repo 2026-10-15 residual: live_horns point-gated at trigger start).

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm};
use kutha_runtime::Runtime;

fn knows_to_related() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(1)),
        vec![atom("knows", v(0), v(1))],
    )
    .unwrap()
}

fn live_vt(
    rt: &Runtime,
    s: kutha_common::TermId,
    r: kutha_common::TermId,
    o: kutha_common::TermId,
) -> Option<(u64, Option<u64>)> {
    rt.fold().facts().iter().find_map(|f| {
        if f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none() {
            Some((f.valid_from, f.valid_to))
        } else {
            None
        }
    })
}

/// A pin that starts after the premise still mints the head on the intersection.
#[test]
fn late_starting_rule_pin_fires_on_overlap() {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&knows_to_related(), 2015, Some(2020))
        .unwrap();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let related = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    assert_eq!(
        live_vt(&rt, alice, related, bob),
        Some((2015, Some(2020))),
        "pin [2015,2020) overlapping premise [2010,None) must mint that intersection"
    );
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.subject == alice
                && f.relation == related
                && f.object() == bob
                && f.is_live_at(u64::MAX, 2017)),
        "head is live inside the pin window"
    );
    assert!(
        !rt.fold().facts().iter().any(|f| f.subject == alice
            && f.relation == related
            && f.object() == bob
            && f.is_live_at(u64::MAX, 2012)),
        "head is absent before the pin starts"
    );
}
