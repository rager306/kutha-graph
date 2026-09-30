//! M011 S06: typed CSR lease preserves relation labels and support multiplicity.

use kutha_common::Op;
use kutha_runtime::Runtime;

#[test]
fn typed_csr_preserves_relation_labels_and_support_multiplicity() {
    let mut rt = Runtime::default();
    let s = rt.intern("S");
    let o = rt.intern("O");
    let knows = rt.intern("knows");
    let related = rt.intern("relatedTo");
    let empty_s = rt.intern("EmptySubject");

    // Two independent supports on (s, knows, o) via claim: None → distinct claim_id.
    rt.emit(Op::Assert {
        subject: s,
        relation: knows,
        object: o,
        valid_from: 0,
        valid_to: None,
        claim: None,
    })
    .unwrap();
    rt.emit(Op::Assert {
        subject: s,
        relation: knows,
        object: o,
        valid_from: 0,
        valid_to: None,
        claim: None,
    })
    .unwrap();
    // Same endpoints, different relation.
    rt.emit(Op::Assert {
        subject: s,
        relation: related,
        object: o,
        valid_from: 0,
        valid_to: None,
        claim: None,
    })
    .unwrap();

    let typed = rt.typed_csr_lease_at(u64::MAX, 0);
    let edges = typed.edges_out(s);
    assert!(
        edges.len() >= 3,
        "typed lease must keep ≥2 supports + ≥1 other relation; got {}",
        edges.len()
    );

    let relations: Vec<_> = edges.iter().map(|e| e.relation).collect();
    assert!(
        relations.contains(&knows) && relations.contains(&related),
        "both relation TermIds must appear: {relations:?}"
    );

    let knows_edges: Vec<_> = edges.iter().filter(|e| e.relation == knows).collect();
    assert!(
        knows_edges.len() >= 2,
        "support multiplicity must remain visible on (s, knows, o)"
    );
    assert!(
        knows_edges[0].claim_id != knows_edges[1].claim_id
            || knows_edges[0].fact_seq != knows_edges[1].fact_seq,
        "supports must differ by claim_id and/or fact_seq"
    );
    for e in &knows_edges {
        assert_eq!(e.object, o);
        assert_eq!(e.relation, knows);
    }

    // Encoding: TermId / EventId identity, not string length.
    assert_ne!(knows, related);
    assert_ne!(knows_edges[0].claim_id, knows_edges[1].claim_id);

    // Same cut: untyped neighbor-set collapses to a single object.
    let untyped = rt.csr_lease_at(u64::MAX, 0);
    assert_eq!(untyped.neighbors(s), &[o]);

    // Empty subject with no live Facts → empty edges_out.
    assert!(typed.edges_out(empty_s).is_empty());
    assert!(untyped.neighbors(empty_s).is_empty());
}
