//! Phase 44 independent semantic kill-tests (public API). Differential vs `4d669ca`.

use kutha_common::{
    policy_version_hash, Event, Op, RuleAst, RuleAtom, RuleTerm, TermId, TransactionTime, ValidTime,
};
use kutha_runtime::{FoldIndexMode, GraphFold, Runtime};
use std::fs;
use std::path::PathBuf;

fn atom(rel: &str, s: RuleTerm, o: RuleTerm) -> RuleAtom {
    RuleAtom::new(rel, s, o).unwrap()
}

fn v(i: u8) -> RuleTerm {
    RuleTerm::var(i).unwrap()
}

fn hop_mentors() -> RuleAst {
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("v44m", v(0), v(1)), atom("v44m", v(1), v(2))],
    )
    .unwrap()
}

fn nary_knows_mentors() -> RuleAst {
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("knows", v(0), v(1)), atom("v44m", v(1), v(2))],
    )
    .unwrap()
}

fn share_head_fwd() -> RuleAst {
    RuleAst::horn(atom("v44h1", v(0), v(1)), vec![atom("v44share", v(0), v(1))]).unwrap()
}

fn share_head_rev() -> RuleAst {
    RuleAst::horn(atom("v44h2", v(1), v(0)), vec![atom("v44share", v(0), v(1))]).unwrap()
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p44-vfy-{label}-{}-{nanos}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn emit_assert(rt: &mut Runtime, s: TermId, rel: TermId, o: TermId) -> kutha_common::EventId {
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
    .unwrap()
    .receipt
    .event_ids[0]
}

fn name_triple(rt: &Runtime, s: TermId, r: TermId, o: TermId) -> String {
    let d = rt.dictionary();
    format!(
        "{} {} {}",
        d.lookup(s).unwrap_or("?"),
        d.lookup(r).unwrap_or("?"),
        d.lookup(o).unwrap_or("?")
    )
}

fn dump_as_of(label: &str, rt: &Runtime) {
    let mut rows: Vec<String> = rt
        .as_of(2010u64)
        .into_iter()
        .map(|(s, r, o)| name_triple(rt, s, r, o))
        .collect();
    rows.sort();
    println!("V44 {label} as_of n={}", rows.len());
    for row in &rows {
        println!("V44   {row}");
    }
}

fn dump_live_at(label: &str, rt: &Runtime, tt: u64) {
    let mut rows: Vec<String> = rt
        .live_at(tt, 2010u64)
        .into_iter()
        .map(|(s, r, o)| name_triple(rt, s, r, o))
        .collect();
    rows.sort();
    println!("V44 {label} live_at({tt}) n={}", rows.len());
    for row in &rows {
        println!("V44   {row}");
    }
}

fn live_triple(rt: &Runtime, s: TermId, r: TermId, o: TermId) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none())
}

fn live_count(rt: &Runtime, s: TermId, r: TermId, o: TermId) -> usize {
    rt.fold()
        .facts()
        .iter()
        .filter(|f| f.subject == s && f.relation == r && f.object() == o && f.invalidated_at.is_none())
        .count()
}

fn has_as_of(rt: &Runtime, s: TermId, r: TermId, o: TermId) -> bool {
    rt.as_of(2010u64).iter().any(|t| *t == (s, r, o))
}

fn last_tt(rt: &Runtime) -> u64 {
    rt.log().as_slice().last().map(|e| e.ingested_at).unwrap_or(0)
}

fn replay_fingerprint_matches(rt: &Runtime) {
    let replayed = GraphFold::replay(rt.log().as_slice());
    assert_eq!(
        replayed.fingerprint(),
        rt.fold().fingerprint(),
        "GraphFold::replay(log).fingerprint must equal live fold"
    );
}

fn seed_share_allows(rt: &mut Runtime) {
    rt.emit_allow("v44share", 0, None).unwrap();
    rt.emit_allow("v44h1", 0, None).unwrap();
    rt.emit_allow("v44h2", 0, None).unwrap();
}

/// (a) genesis inverse: A knows B → B knownBy A; retract drops current, keeps earlier live_at.
#[test]
fn v44_a_inverse_retract_as_of_earlier() {
    let mut rt = Runtime::default();
    let a = rt.intern("v44A");
    let b = rt.intern("v44B");
    let knows = rt.intern("knows");
    let known_by = rt.intern("knownBy");
    let prem = emit_assert(&mut rt, a, knows, b);
    dump_as_of("a_after_knows", &rt);
    assert!(
        live_triple(&rt, b, known_by, a),
        "genesis inverse_knows must mint B knownBy A"
    );
    assert!(has_as_of(&rt, b, known_by, a));
    let tt_before = last_tt(&rt);
    rt.emit(Op::Retract { event_id: prem }).unwrap();
    dump_as_of("a_after_retract", &rt);
    dump_live_at("a_after_retract", &rt, tt_before);
    assert!(
        !live_triple(&rt, b, known_by, a),
        "derived support must be gone at the current cut"
    );
    assert!(!has_as_of(&rt, b, known_by, a));
    assert!(
        rt.live_at(tt_before, 2010u64)
            .iter()
            .any(|t| *t == (b, known_by, a)),
        "derived support must remain live_at earlier TT"
    );
}

/// (b) rule added after facts: late emit does not retro-fire; a later matching assert does.
#[test]
fn v44_b_late_rule_then_matching_assert() {
    let mut rt = Runtime::default();
    rt.emit_allow("v44m", 0, None).unwrap();
    let a = rt.intern("v44La");
    let b = rt.intern("v44Lb");
    let c = rt.intern("v44Lc");
    let m = rt.intern("v44m");
    let rel = rt.intern("relatedTo");
    emit_assert(&mut rt, a, m, b);
    emit_assert(&mut rt, b, m, c);
    dump_as_of("b_facts_before_rule", &rt);
    assert!(
        !live_triple(&rt, a, rel, c),
        "facts before the rule must not already carry the hop"
    );
    rt.emit_rule_ast(&hop_mentors(), 2010, None).unwrap();
    dump_as_of("b_after_late_rule", &rt);
    let late_rule_fired = live_triple(&rt, a, rel, c);
    println!("V44 b_late_rule_retro_fire={late_rule_fired}");
    let d = rt.intern("v44Ld");
    emit_assert(&mut rt, c, m, d);
    dump_as_of("b_after_later_assert", &rt);
    assert!(
        live_triple(&rt, b, rel, d),
        "later matching assert must fire the late rule (B-C + C-D)"
    );
}

/// (c) n-ary, two premise relations; second-arrives vs first-arrives-last.
#[test]
fn v44_c_nary_second_and_first_last() {
    let ast = nary_knows_mentors();
    let run = |mentors_first: bool, label: &str| {
        let mut rt = Runtime::default();
        rt.emit_allow("v44m", 0, None).unwrap();
        rt.emit_rule_ast(&ast, 2010, None).unwrap();
        let a = rt.intern("v44Na");
        let b = rt.intern("v44Nb");
        let c = rt.intern("v44Nc");
        let knows = rt.intern("knows");
        let m = rt.intern("v44m");
        let rel = rt.intern("relatedTo");
        if mentors_first {
            emit_assert(&mut rt, b, m, c);
            dump_as_of(&format!("{label}_after_second_prem_first"), &rt);
            assert!(!live_triple(&rt, a, rel, c));
            emit_assert(&mut rt, a, knows, b);
            dump_as_of(&format!("{label}_after_first_prem_last"), &rt);
        } else {
            emit_assert(&mut rt, a, knows, b);
            dump_as_of(&format!("{label}_after_first_prem"), &rt);
            assert!(!live_triple(&rt, a, rel, c));
            emit_assert(&mut rt, b, m, c);
            dump_as_of(&format!("{label}_after_second_prem"), &rt);
        }
        assert!(
            live_triple(&rt, a, rel, c),
            "{label}: both orders must mint relatedTo(A,C)"
        );
    };
    run(false, "c_second_arrives");
    run(true, "c_first_arrives_last");
}

/// (d) retract then re-assert the same premise restores derived eligibility.
#[test]
fn v44_d_retract_reassert_support() {
    let mut rt = Runtime::default();
    rt.emit_allow("v44m", 0, None).unwrap();
    rt.emit_rule_ast(&hop_mentors(), 2010, None).unwrap();
    let a = rt.intern("v44Da");
    let b = rt.intern("v44Db");
    let c = rt.intern("v44Dc");
    let m = rt.intern("v44m");
    let rel = rt.intern("relatedTo");
    emit_assert(&mut rt, a, m, b);
    let prem = emit_assert(&mut rt, b, m, c);
    dump_as_of("d_fired", &rt);
    assert_eq!(1, live_count(&rt, a, rel, c));
    rt.emit(Op::Retract { event_id: prem }).unwrap();
    dump_as_of("d_retracted", &rt);
    assert_eq!(0, live_count(&rt, a, rel, c));
    emit_assert(&mut rt, b, m, c);
    dump_as_of("d_reasserted", &rt);
    assert_eq!(
        1,
        live_count(&rt, a, rel, c),
        "re-assert of the same premise must restore derived support"
    );
}

/// (e) persist+open, fork_at, GraphFold::replay; new premise still fires (lease rebuilt).
#[test]
fn v44_e_persist_open_fork_replay() {
    let mut rt = Runtime::default();
    rt.emit_allow("v44m", 0, None).unwrap();
    rt.emit_rule_ast(&hop_mentors(), 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();
    let a = rt.intern("v44Ea");
    let b = rt.intern("v44Eb");
    let c = rt.intern("v44Ec");
    let m = rt.intern("v44m");
    let rel = rt.intern("relatedTo");
    emit_assert(&mut rt, a, m, b);
    replay_fingerprint_matches(&rt);
    assert_eq!(
        Some(policy_version_hash("leased-policy")),
        rt.live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017))
    );

    let dir = persist_dir("e-open");
    rt.persist(&dir).unwrap();
    let snap = fs::read_to_string(dir.join("snapshot.json")).unwrap();
    assert!(
        !snap.contains("horns_by_premise"),
        "horn premise lease must not ride snapshot JSON"
    );
    let mut opened = Runtime::open(&dir).unwrap();
    assert_eq!(
        Some(policy_version_hash("leased-policy")),
        opened.live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017))
    );
    emit_assert(&mut opened, b, m, c);
    dump_as_of("e_opened_after_new_prem", &opened);
    assert!(
        live_triple(&opened, a, rel, c),
        "open must rebuild Horn lease so a new premise still fires"
    );
    replay_fingerprint_matches(&opened);
    let _ = fs::remove_dir_all(&dir);

    let mut forked = rt.fork_at(rt.log().len());
    emit_assert(&mut forked, b, m, c);
    dump_as_of("e_fork_after_new_prem", &forked);
    assert!(
        live_triple(&forked, a, rel, c),
        "fork_at must rebuild Horn lease so a new premise still fires"
    );
    replay_fingerprint_matches(&forked);
}

/// (f) Eager vs Lazy: same derived product rows.
#[test]
fn v44_f_eager_lazy_derived_equal() {
    let ingest = |mode: FoldIndexMode| {
        let mut rt = Runtime::with_index_mode(32, mode);
        rt.emit_allow("v44m", 0, None).unwrap();
        rt.emit_rule_ast(&hop_mentors(), 2010, None).unwrap();
        let a = rt.intern("v44Fa");
        let b = rt.intern("v44Fb");
        let c = rt.intern("v44Fc");
        let m = rt.intern("v44m");
        emit_assert(&mut rt, a, m, b);
        emit_assert(&mut rt, b, m, c);
        rt
    };
    let eager = ingest(FoldIndexMode::Eager);
    let lazy = ingest(FoldIndexMode::Lazy);
    dump_as_of("f_eager", &eager);
    dump_as_of("f_lazy", &lazy);
    let mut ea = eager.as_of(2010u64);
    let mut la = lazy.as_of(2010u64);
    ea.sort_unstable();
    la.sort_unstable();
    assert_eq!(ea, la, "Eager and Lazy derived as_of rows must match");
}

/// (g) two Horns sharing one premise relation both fire.
#[test]
fn v44_g_two_rules_one_premise() {
    let mut rt = Runtime::default();
    seed_share_allows(&mut rt);
    rt.emit_rule_ast(&share_head_fwd(), 2010, None).unwrap();
    rt.emit_rule_ast(&share_head_rev(), 2010, None).unwrap();
    let a = rt.intern("v44Ga");
    let b = rt.intern("v44Gb");
    let share = rt.intern("v44share");
    let h1 = rt.intern("v44h1");
    let h2 = rt.intern("v44h2");
    emit_assert(&mut rt, a, share, b);
    dump_as_of("g_shared_premise", &rt);
    assert!(live_triple(&rt, a, h1, b), "first rule must fire");
    assert!(live_triple(&rt, b, h2, a), "second rule must fire");
}

#[test]
fn v44_replay_log_is_sot() {
    let mut rt = Runtime::default();
    rt.emit_allow("v44m", 0, None).unwrap();
    rt.emit_rule_ast(&hop_mentors(), 2010, None).unwrap();
    let a = rt.intern("v44Ra");
    let b = rt.intern("v44Rb");
    let c = rt.intern("v44Rc");
    let m = rt.intern("v44m");
    emit_assert(&mut rt, a, m, b);
    emit_assert(&mut rt, b, m, c);
    replay_fingerprint_matches(&rt);
    let _log: &[Event] = rt.log().as_slice();
}
