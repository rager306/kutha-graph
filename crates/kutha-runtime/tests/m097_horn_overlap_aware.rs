//! M097 S01 / Phase 168 — multi-premise Horn discovery is overlap-aware (CE #4).
//! Design: overlap-aware VT candidates, not retry-at-candidate-starts.

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm};
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

fn live_at(
    rt: &Runtime,
    s: kutha_common::TermId,
    r: kutha_common::TermId,
    o: kutha_common::TermId,
    vt: u64,
) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.is_live_at(u64::MAX, vt))
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

/// Late-starting AB after open-ended BC trigger must still mint AC on the overlap.
#[test]
fn late_starting_premise_fires_when_trigger_starts_earlier() {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&two_prem_horn(), 2010, None).unwrap();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");

    // AB starts later than the BC trigger's valid_from — point-gated search misses it.
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2015,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
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

    assert!(
        live_at(&rt, a, rel, c, 2017),
        "overlap [2015,2020) must mint relatedTo(A,C) even when trigger starts at 2010"
    );
    assert_eq!(
        live_vt(&rt, a, rel, c),
        Some((2015, Some(2020))),
        "derived VT must be the premise conjunction"
    );
    assert!(
        !live_at(&rt, a, rel, c, 2012),
        "must not invent coverage before AB starts"
    );
}

/// Same logical premises in reverse insert order must yield the same derived VT.
#[test]
fn horn_overlap_discovery_is_order_independent() {
    let mut early_trigger = Runtime::default();
    early_trigger
        .emit_rule_ast(&two_prem_horn(), 2010, None)
        .unwrap();
    let mut late_trigger = Runtime::default();
    late_trigger
        .emit_rule_ast(&two_prem_horn(), 2010, None)
        .unwrap();

    for rt in [&mut early_trigger, &mut late_trigger] {
        let _ = (
            rt.intern("A"),
            rt.intern("B"),
            rt.intern("C"),
            rt.intern("relatedTo"),
        );
    }
    let a = early_trigger.dictionary().id("A").unwrap();
    let b = early_trigger.dictionary().id("B").unwrap();
    let c = early_trigger.dictionary().id("C").unwrap();
    let rel = early_trigger.dictionary().id("relatedTo").unwrap();

    // Order 1: AB then BC (trigger starts earlier).
    early_trigger
        .emit(Op::Assert {
            subject: a,
            relation: rel,
            object: b,
            valid_from: 2015,
            valid_to: Some(2020),
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    early_trigger
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

    // Order 2: BC then AB (AB is the later trigger; already worked before M097).
    let a2 = late_trigger.dictionary().id("A").unwrap();
    let b2 = late_trigger.dictionary().id("B").unwrap();
    let c2 = late_trigger.dictionary().id("C").unwrap();
    let rel2 = late_trigger.dictionary().id("relatedTo").unwrap();
    late_trigger
        .emit(Op::Assert {
            subject: b2,
            relation: rel2,
            object: c2,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    late_trigger
        .emit(Op::Assert {
            subject: a2,
            relation: rel2,
            object: b2,
            valid_from: 2015,
            valid_to: Some(2020),
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();

    assert_eq!(
        live_vt(&early_trigger, a, rel, c),
        live_vt(&late_trigger, a2, rel2, c2),
        "same logical state must derive the same AC VT regardless of insert order"
    );
    assert_eq!(
        live_vt(&early_trigger, a, rel, c),
        Some((2015, Some(2020)))
    );
}
