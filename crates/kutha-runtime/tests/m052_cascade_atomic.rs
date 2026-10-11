//! M052 S02 / Phase 113: user cascade is atomic (CE whole-repo P1 #8).
//! A later-hop `rule_follow_ons` error must not leave a live prefix without QuantumOutcome.

use kutha_common::{Op, RuleAst, RuleAtom, RuleTerm};
use kutha_runtime::{Runtime, RuntimeError};

fn var(i: u8) -> RuleTerm {
    RuleTerm::var(i).expect("var")
}

fn atom(rel: &str, s: RuleTerm, o: RuleTerm) -> RuleAtom {
    RuleAtom::new(rel, s, o).expect("atom")
}

/// `ghostRel(Y,X) :- knownBy(X,Y)` — head string is never interned.
fn ghost_from_known_by() -> RuleAst {
    RuleAst::horn(
        atom("ghostRel", var(1), var(0)),
        vec![atom("knownBy", var(0), var(1))],
    )
    .expect("horn")
}

fn snapshot(rt: &Runtime) -> (usize, usize, usize) {
    (
        rt.log().len(),
        rt.fold().facts().len(),
        rt.outcome_records().len(),
    )
}

#[test]
fn later_hop_unknown_head_leaves_log_fold_outcomes_unchanged() {
    let mut rt = Runtime::default();
    rt.emit_rule_ast(&ghost_from_known_by(), 2010, None)
        .expect("register later-hop Horn");

    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let before = snapshot(&rt);

    let err = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect_err("later-hop unknown head must fail closed");
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "ghostRel"),
        "{err}"
    );

    let after = snapshot(&rt);
    assert_eq!(
        before, after,
        "Err must not commit a prefix: log/fold/outcomes must match the pre-emit snapshot"
    );
}

#[test]
fn inverse_knows_cascade_writes_quantum_outcome() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    let known_by = rt
        .dictionary()
        .id("knownBy")
        .expect("genesis intern knownBy");

    let out = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .expect("single-hop inverse cascade");
    assert!(
        out.events_in_quantum >= 2,
        "knows Assert plus inverse_knows Behavior"
    );
    assert!(
        rt.log()
            .iter()
            .any(|e| matches!(e.op, Op::QuantumOutcome { .. })),
        "successful cascade must persist QuantumOutcome"
    );
    assert!(
        !rt.outcome_records().is_empty(),
        "outcome buffer must record the quantum"
    );
    assert!(
        rt.fold().facts().iter().any(|f| {
            f.subject == bob
                && f.relation == known_by
                && f.object() == alice
                && f.invalidated_at.is_none()
        }),
        "knownBy inverse must be live"
    );
}
