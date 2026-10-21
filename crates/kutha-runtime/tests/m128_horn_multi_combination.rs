//! M128 S01 / Phase 202 — multi-residual premise combinations under one trigger
//! (CE 2026-10-15 residual risk: no oracle that `search_at_all` enumerates).
//!
//! Kill: keep only the first `PremMatch` leaf in `search_at_all` (single-combination DFS).

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

fn live_acs(
    rt: &Runtime,
    a: kutha_common::TermId,
    rel: kutha_common::TermId,
    c: kutha_common::TermId,
) -> Vec<(u64, Option<u64>)> {
    let mut out: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| {
            f.subject == a && f.relation == rel && f.object() == c && f.invalidated_at.is_none()
        })
        .map(|f| (f.valid_from, f.valid_to))
        .collect();
    out.sort_by_key(|w| w.0);
    out
}

/// Two disjoint AB residuals overlapping an open BC trigger must mint two AC heads.
#[test]
fn two_disjoint_ab_windows_mint_two_ac_heads() {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&two_prem_horn(), 2000, None).unwrap();
    let a = rt.intern("A");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");

    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2010,
        valid_to: Some(2015),
        claim: None,
        delivery_key: Some("ab-early".into()),
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2020,
        valid_to: Some(2025),
        claim: None,
        delivery_key: Some("ab-late".into()),
        polarity: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: b,
        relation: rel,
        object: c,
        valid_from: 2000,
        valid_to: None,
        claim: None,
        delivery_key: Some("bc".into()),
        polarity: None,
    })
    .unwrap();

    let heads = live_acs(&rt, a, rel, c);
    assert_eq!(
        heads,
        vec![(2010, Some(2015)), (2020, Some(2025))],
        "each disjoint AB residual must mint its own AC head; got {heads:?}"
    );
}
