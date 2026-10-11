//! M054 S04 / Phase 120 — scaling + bench honesty (CE #15 #16 #17 #19 #25 #27).

use kutha_common::{policy_version_hash, Op, RuleAst, RuleAtom, RuleTerm, ValidTime};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::path::PathBuf;

const BASELINE: &str = include_str!("../benches/baseline.rs");

fn two_prem_horn() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
}

fn emit_named(rt: &mut Runtime, rel_name: &str, i: u32) {
    let s = rt.intern(&format!("{rel_name}-s-{i}"));
    let o = rt.intern(&format!("{rel_name}-o-{i}"));
    let rel = rt.intern(rel_name);
    rt.emit(Op::Assert {
        subject: s,
        relation: rel,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m054-scaling-bench-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// #15 — after-RSS for bytes_per_fact must be read while the dedicated sample lives.
#[test]
fn bench_rss_samples_after_dedicated_runtime_is_built() {
    let sample_pos = BASELINE
        .find("let mut sample")
        .expect("dedicated N-sized sample runtime");
    let rest = &BASELINE[sample_pos..];
    assert!(
        rest.contains("vm_rss_bytes()"),
        "after RSS must be read while the dedicated sample is in scope (#15)"
    );
}

/// #25 — timed as_of triples must be bound and checked against an expected set.
#[test]
fn bench_validates_timed_as_of_triples() {
    assert!(
        !BASELINE.contains("let _ = rt.fold().as_of"),
        "timed as_of result must be consumed, not discarded (#25)"
    );
    assert!(
        BASELINE.contains("expected_as_of"),
        "bench must validate as_of against expected_as_of triples (#25)"
    );
}

/// #16 — retract of a decoy with no derived children must not walk O(N) log rows.
#[test]
fn retract_derived_visits_constant_in_n() {
    let mut counts = Vec::new();
    let mut child_counts = Vec::new();
    for &n in &[64u32, 256] {
        let mut rt = Runtime::default();
        for i in 0..n {
            emit_named(&mut rt, "inForceAs", i);
        }
        let decoy = rt
            .log()
            .as_slice()
            .iter()
            .rev()
            .find_map(|e| match &e.op {
                Op::Assert { .. } => Some(e.id),
                _ => None,
            })
            .expect("decoy assert");
        rt.reset_retract_examine_count();
        rt.emit(Op::Retract { event_id: decoy }).unwrap();
        counts.push(rt.retract_examine_count());

        let mut with_child = Runtime::default();
        for i in 0..n {
            emit_named(&mut with_child, "inForceAs", i);
        }
        let alice = with_child.intern("Alice");
        let bob = with_child.intern("Bob");
        let knows = with_child.intern("knows");
        let prem = with_child
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
            .unwrap()
            .receipt
            .event_ids[0];
        let kb = with_child.dictionary().id("knownBy").expect("knownBy");
        assert!(
            with_child.fold().facts().iter().any(|f| f.subject == bob
                && f.relation == kb
                && f.object() == alice
                && f.invalidated_at.is_none()),
            "inverse_knows must mint knownBy before retract"
        );
        with_child.reset_retract_examine_count();
        with_child.emit(Op::Retract { event_id: prem }).unwrap();
        assert!(
            !with_child.fold().facts().iter().any(|f| {
                f.subject == bob
                    && f.relation == kb
                    && f.object() == alice
                    && f.invalidated_at.is_none()
            }),
            "retract of the trigger must still drop the derived child"
        );
        child_counts.push(with_child.retract_examine_count());
    }
    assert!(
        counts[1] <= counts[0] + 8,
        "retract with no children must not grow with N: {counts:?}"
    );
    assert!(
        counts[1] < 32,
        "retract with no children must stay bounded: {counts:?}"
    );
    assert!(
        child_counts[1] <= child_counts[0] + 8,
        "retract of one Horn child must not grow with decoy N: {child_counts:?}"
    );
}

/// #17 — Horn search_at must probe the premise relation, not every live fact.
#[test]
fn horn_search_visits_constant_in_unrelated_n() {
    let mut counts = Vec::new();
    for &n in &[64u32, 256] {
        let mut rt = Runtime::default();
        let ast = two_prem_horn();
        rt.emit_rule_ast(&ast, 2010, None).unwrap();
        for i in 0..n {
            emit_named(&mut rt, "inForceAs", i);
        }
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
        rt.fold().reset_horn_examine_count();
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
            rt.fold().facts().iter().any(|f| {
                f.subject == a && f.relation == rel && f.object() == c && f.invalidated_at.is_none()
            }),
            "two-premise relatedTo Horn must still fire"
        );
        counts.push(rt.fold().horn_examine_count());
    }
    assert!(
        counts[1] <= counts[0] + 8,
        "Horn search must not rescan unrelated live facts: {counts:?}"
    );
    assert!(
        counts[1] < 64,
        "Horn search on a relatedTo pair must stay bounded: {counts:?}"
    );
}

/// #19 — incremental append must clone the undisclosed suffix, not the whole encoded log.
#[test]
fn persist_append_examines_suffix_only() {
    let mut counts = Vec::new();
    for &n in &[64u32, 256] {
        let mut rt = Runtime::default();
        for i in 0..n {
            emit_named(&mut rt, "relatedTo", i);
        }
        let dir = persist_dir(&format!("append-{n}"));
        store::persist(&rt, &dir).unwrap();
        let mut persister = store::Persister::attach(&rt, &dir).expect("attach");
        store::reset_append_examine_count();
        emit_named(&mut rt, "relatedTo", n);
        persister.append(&rt).expect("append one event");
        counts.push(store::append_examine_count());
        let _ = fs::remove_dir_all(&dir);
    }
    assert!(
        counts[1] <= counts[0] + 16,
        "append examine must not grow with durable N: {counts:?}"
    );
    assert!(
        counts[1] < 64,
        "one-event append must stay bounded: {counts:?}"
    );
}

/// #27 — rule/policy pin checks must not materialize every live fact.
#[test]
fn pin_lookups_constant_in_product_n() {
    let mut rule_counts = Vec::new();
    let mut policy_counts = Vec::new();
    for &n in &[64u32, 256] {
        let mut rt = Runtime::default();
        for i in 0..n {
            emit_named(&mut rt, "relatedTo", i);
        }
        let ast = two_prem_horn();
        let pin = ast.hash();
        rt.emit_rule_ast(&ast, 2010, None).unwrap();
        let policy = "m054-s04-policy";
        let policy_pin = policy_version_hash(policy);
        rt.emit_policy(policy, 2010, None).unwrap();

        rt.fold().reset_hot_examine_count();
        assert!(
            rt.fold().rule_hash_live_at(&pin, u64::MAX, 2010),
            "rule pin must stay live"
        );
        rule_counts.push(rt.fold().hot_examine_count());

        rt.fold().reset_hot_examine_count();
        assert!(
            rt.fold().policy_hash_live_at(&policy_pin, u64::MAX, 2010),
            "policy pin must stay live"
        );
        assert!(
            rt.fold().live_policy_pin_at(u64::MAX, 2010).as_deref() == Some(policy_pin.as_str()),
            "latest live policy pin must be the one just emitted"
        );
        policy_counts.push(rt.fold().hot_examine_count());
    }
    assert!(
        rule_counts[1] <= rule_counts[0] + 8,
        "rule_hash_live_at must not grow with product N: {rule_counts:?}"
    );
    assert!(
        rule_counts[1] < 32,
        "rule pin lookup must not walk all live facts: {rule_counts:?}"
    );
    assert!(
        policy_counts[1] <= policy_counts[0] + 8,
        "policy pin lookups must not grow with product N: {policy_counts:?}"
    );
    assert!(
        policy_counts[1] < 32,
        "policy pin lookup must not walk all live facts: {policy_counts:?}"
    );
}

/// Timed as_of contract used by the bench: exact interned relatedTo set, not live_count alone.
#[test]
fn as_of_matches_emitted_relatedto_triples() {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    let obj = rt.intern("o");
    let mut expected = Vec::new();
    for i in 0..8u32 {
        let s = rt.intern(&format!("s{i}"));
        rt.emit(Op::Assert {
            subject: s,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
        expected.push((s, rel, obj));
    }
    expected.sort_unstable();
    let mut got = rt.fold().as_of(ValidTime::from(2010));
    got.sort_unstable();
    assert_eq!(got, expected, "as_of must equal the emitted relatedTo set");
    assert_eq!(
        rt.fold().live_count(u64::MAX, 2010),
        expected.len(),
        "live_count must agree with the exact as_of set"
    );
}
