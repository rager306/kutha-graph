//! v0.99 audit residuals: newest policy pin, premise-rel unindex.

use kutha_common::{policy_version_hash, Op, RuleAst, RuleAtom, RuleTerm};
use kutha_runtime::Runtime;

fn single_prem(rel: &str) -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |name: &str, s, o| RuleAtom::new(name, s, o).unwrap();
    RuleAst::horn(atom("knownBy", v(0), v(1)), vec![atom(rel, v(0), v(1))]).unwrap()
}

/// Latest ingested live policy is chosen after one row visit, not a scan of older pins.
#[test]
fn newest_policy_pin_examines_one_live_row() {
    let mut rt = Runtime::default();
    let mut last = String::new();
    for i in 0..24 {
        let body = format!("body-{i}");
        rt.emit_policy(&body, 0, None).unwrap();
        last = policy_version_hash(&body);
    }
    rt.fold().reset_hot_examine_count();
    let pin = rt.fold().live_policy_pin_at(u64::MAX, 0);
    assert_eq!(pin.as_deref(), Some(last.as_str()));
    assert_eq!(
        rt.fold().hot_examine_count(),
        1,
        "newest-first election must stop on the first live policy"
    );
}

/// Unindex of one Behavior touches its premise relations, not every other rule's bucket.
#[test]
fn unindex_premise_examines_only_that_behaviors_rels() {
    let mut rt = Runtime::default();
    for i in 0..12u32 {
        let rel = format!("rel{i}");
        rt.emit_allow(&rel, 0, None).unwrap();
        rt.emit_rule_ast(&single_prem(&rel), 0, None).unwrap();
        let s = rt.intern(&format!("S{i}"));
        let o = rt.intern(&format!("O{i}"));
        let r = rt.intern(&rel);
        rt.emit(Op::Assert {
            subject: s,
            relation: r,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    }
    rt.reset_unindex_premise_examine_count();
    rt.correct("S11", "rel11", "Z", 2010).unwrap();
    let n = rt.unindex_premise_examine_count();
    assert!(
        n <= 2,
        "premise unindex must not walk every relation bucket: {n}"
    );
}
