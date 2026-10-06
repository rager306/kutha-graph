//! KNOWN-04 / D-HORN-KC: killed pin of ingest-at-head.
//! Horn remaining-premise lookup is `TT = u64::MAX` (log-head). `as_known_on` TT-hides
//! supports minted after the cut. Do not invert; `eval.rs` is unchanged.

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

/// D-HORN-KC killed pin: `horn_fires` → `search_prems(..., u64::MAX)`; `live_horns` uses `is_live_at(u64::MAX, vt)`.
/// Ingest mints at head; `as_known_on` TT-hides. Do not invert these asserts.
#[test]
fn known04_horn_fires_when_premise_hidden_on_early_cut() {
    let mut rt = Runtime::default();
    let ast = two_prem_horn();
    rt.emit_rule_ast(&ast, 2010, None).unwrap();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");

    rt.emit_tt_anchor(2014).unwrap();
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
    rt.emit_tt_anchor(2020).unwrap();

    let premise_ab = (a, rel, b);
    let derived_ac = (a, rel, c);
    assert!(
        rt.fold().facts().iter().any(|f| {
            f.subject == a && f.relation == rel && f.object() == c && f.invalidated_at.is_none()
        }),
        "F05: Horn at TT=MAX mints derived relatedTo(A,C) while A-B is after the 2014 cut"
    );
    assert!(
        rt.live_at(u64::MAX, 2010).contains(&derived_ac),
        "current fold (TT=MAX) keeps the Horn-derived support live"
    );

    let known_2014 = rt
        .as_known_on(2014)
        .expect("early TtAnchor 2014 must resolve");
    assert!(
        !known_2014.contains(&premise_ab),
        "as_known_on(2014) hides A-B (ingested after that cut)"
    );
    assert!(
        !known_2014.contains(&derived_ac),
        "as_known_on(2014) hides the derived support minted after that cut"
    );

    let known_2020 = rt
        .as_known_on(2020)
        .expect("late TtAnchor 2020 must resolve");
    assert!(
        known_2020.contains(&premise_ab),
        "as_known_on(2020) shows A-B"
    );
}
