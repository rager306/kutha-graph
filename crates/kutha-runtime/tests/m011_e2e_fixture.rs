//! M011 S08 candidate fixture; semantic-contract observations at named cuts.

use kutha_common::{rule_definition_hash, EventId, Op, SupportPolarity, TermId};
use kutha_runtime::{store, GraphFold, Runtime, RuntimeError};

/// Year-like valid-time instants (same as S04).
const VF_WIDE: u64 = 2010;
const PATCH_FROM: u64 = 2015;
const PATCH_TO: u64 = 2020;
const VT_LEFT: u64 = 2012;
const VT_INTERIOR: u64 = 2017;
const VT_RIGHT: u64 = 2021;

struct Fixture {
    rt: Runtime,
    a: TermId,
    b: TermId,
    p: TermId,
    #[allow(dead_code)]
    q: TermId,
    related: TermId,
    not_p: TermId,
    claim_p: EventId,
    claim_q: EventId,
    event_a: EventId,
    event_b: EventId,
    #[allow(dead_code)]
    seq_a: u64,
    #[allow(dead_code)]
    seq_b: u64,
    t1: u64,
    t2: Option<u64>,
    t3: Option<u64>,
}

fn last_fact_seq(rt: &Runtime) -> u64 {
    rt.fold().facts().last().expect("fact after emit").seq
}

fn last_fact_claim(rt: &Runtime) -> EventId {
    rt.fold().facts().last().expect("fact after emit").claim_id
}

fn build_through_t1() -> Fixture {
    let mut rt = Runtime::default();
    let a = rt.intern("a");
    let b = rt.intern("b");
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let related = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let not_p = rt.intern("not-P");
    let pin = rule_definition_hash("derive_pq");
    rt.emit(Op::RegisterRule {
        definition: "derive_pq".into(),
        valid_from: VF_WIDE,
        valid_to: None,
    })
    .unwrap();
    rt.emit(Op::PinPolicy {
        definition: "leased-policy".into(),
        valid_from: VF_WIDE,
        valid_to: None,
    })
    .unwrap();

    let first = rt
        .emit(Op::Assert {
            subject: a,
            relation: related,
            object: p,
            valid_from: VF_WIDE,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: Some(SupportPolarity::Positive),
        })
        .unwrap();
    let event_a = first.receipt.event_ids[0];
    let seq_a = last_fact_seq(&rt);
    let claim_p = last_fact_claim(&rt);
    assert_eq!(
        event_a, claim_p,
        "RESEARCH A1: receipt.event_ids[0] matches fold claim_id"
    );

    let second = rt
        .emit(Op::Assert {
            subject: b,
            relation: related,
            object: p,
            valid_from: VF_WIDE,
            valid_to: None,
            claim: Some(claim_p),
            delivery_key: None,
            polarity: Some(SupportPolarity::Positive),
        })
        .unwrap();
    let event_b = second.receipt.event_ids[0];
    let seq_b = last_fact_seq(&rt);
    assert_eq!(rt.fold().facts().last().unwrap().claim_id, claim_p);

    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            rule_version: pin,
            subject: q,
            relation: related,
            object: true_,
            valid_from: VF_WIDE,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    assert_eq!(last_fact_claim(&rt), claim_q);

    let t1 = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.claim_id == claim_q)
        .unwrap()
        .ingested_at;

    Fixture {
        rt,
        a,
        b,
        p,
        q,
        related,
        not_p,
        claim_p,
        claim_q,
        event_a,
        event_b,
        seq_a,
        seq_b,
        t1,
        t2: None,
        t3: None,
    }
}

impl Fixture {
    fn apply_t2_conflict(&mut self) {
        self.rt
            .emit(Op::CorrectInterval {
                event_id: self.event_a,
                object: self.not_p,
                patch_from: PATCH_FROM,
                patch_to: Some(PATCH_TO),
            })
            .unwrap();
        self.t2 = Some(self.rt.log().iter().last().unwrap().ingested_at);
    }

    fn apply_t3_withdraw_b(&mut self) {
        self.rt
            .emit(Op::Retract {
                event_id: self.event_b,
            })
            .unwrap();
        self.t3 = Some(self.rt.log().iter().last().unwrap().ingested_at);
    }

    fn record_t1_justification(&mut self) -> String {
        self.rt
            .record_justification(
                self.claim_q,
                vec![self.claim_p],
                vec![self.event_a, self.event_b],
                rule_definition_hash("derive_pq"),
                self.t1,
                VT_INTERIOR,
            )
            .expect("record_justification")
    }
}

fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

fn temp_dir() -> std::path::PathBuf {
    std::env::temp_dir().join(format!("kutha-m011-s08-{}", uuid_like()))
}

#[test]
fn e2e_fixture_supports_and_conflict_at_named_cuts() {
    let mut fx = build_through_t1();

    assert_eq!(
        2,
        fx.rt
            .fold()
            .live_support_count(fx.claim_p, fx.t1, VT_INTERIOR)
    );
    assert!(fx.rt.derivation_eligible_at(fx.claim_q, fx.t1, VT_INTERIOR));
    let t1_report = fx.rt.conflict_report_at(fx.claim_p, fx.t1, VT_INTERIOR);
    assert!(!t1_report.positive_supports.is_empty());
    assert!(t1_report.negative_supports.is_empty());

    let jid = fx.record_t1_justification();
    fx.rt.check_admission(&jid).unwrap();
    let dir = temp_dir();
    store::persist(&fx.rt, &dir).unwrap();
    assert!(dir.join(store::JUSTIFICATIONS_REL).exists());
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    opened.check_admission(&jid).unwrap();
    assert!(!opened.justification_records().is_empty());
    let _ = std::fs::remove_dir_all(&dir);

    fx.apply_t2_conflict();
    let t2 = fx.t2.expect("t2");
    assert!(fx
        .rt
        .fold()
        .live_at(t2, VT_INTERIOR)
        .contains(&(fx.b, fx.related, fx.p)));
    assert!(fx
        .rt
        .fold()
        .live_at(t2, VT_INTERIOR)
        .contains(&(fx.a, fx.related, fx.not_p)));
    let t2_report = fx.rt.conflict_report_at(fx.claim_p, t2, VT_INTERIOR);
    assert!(!t2_report.positive_supports.is_empty());
    assert!(!t2_report.negative_supports.is_empty());
    assert!(
        fx.rt
            .fold()
            .live_at(t2, VT_LEFT)
            .contains(&(fx.a, fx.related, fx.p)),
        "VT 2012 residual of a must keep P"
    );
    assert!(
        fx.rt
            .fold()
            .live_at(t2, VT_RIGHT)
            .contains(&(fx.a, fx.related, fx.p)),
        "VT 2021 residual of a must keep P"
    );

    fx.apply_t3_withdraw_b();
    let t3 = fx.t3.expect("t3");
    let t3_report = fx.rt.conflict_report_at(fx.claim_p, t3, VT_INTERIOR);
    assert!(
        t3_report.positive_supports.is_empty(),
        "last positive-P support withdrawn"
    );
    assert!(fx.rt.fold().claim_supported_at(fx.claim_q, t3, VT_INTERIOR));
    assert!(fx
        .rt
        .fold()
        .live_at(t3, VT_LEFT)
        .contains(&(fx.a, fx.related, fx.p)));
    assert!(fx
        .rt
        .fold()
        .live_at(t3, VT_RIGHT)
        .contains(&(fx.a, fx.related, fx.p)));
    fx.rt.replay_check().unwrap();
}

fn assert_stale(rt: &Runtime, jid: &str) {
    let err = rt.check_admission(jid).unwrap_err();
    assert!(
        matches!(
            err,
            RuntimeError::AdmissionDenied {
                reason: "stale_support",
                ..
            }
        ),
        "{err:?}"
    );
}

fn assert_cut_agrees(rt: &Runtime, replayed: &GraphFold, fx: &Fixture, tt: u64, vt: u64) {
    assert_eq!(rt.fold().live_at(tt, vt), replayed.live_at(tt, vt));
    assert_eq!(
        rt.fold().live_support_count(fx.claim_p, tt, vt),
        replayed.live_support_count(fx.claim_p, tt, vt)
    );
    assert_eq!(
        rt.fold().live_support_count(fx.claim_q, tt, vt),
        replayed.live_support_count(fx.claim_q, tt, vt)
    );
    let report = rt.conflict_report_at(fx.claim_p, tt, vt);
    let mut pos = Vec::new();
    let mut neg = Vec::new();
    for f in replayed.live_supports(fx.claim_p, tt, vt) {
        match f.polarity {
            Some(SupportPolarity::Positive) => pos.push(f.seq),
            Some(SupportPolarity::Negative) => neg.push(f.seq),
            None => {}
        }
    }
    assert_eq!(report.positive_supports, pos);
    assert_eq!(report.negative_supports, neg);
}

#[test]
fn e2e_justification_cites_sources_and_rejects_stale_admission() {
    let mut fx = build_through_t1();
    let jid = fx.record_t1_justification();
    let row = fx
        .rt
        .justification_records()
        .iter()
        .find(|j| j.justification_id == jid)
        .expect("t1 row");
    assert_eq!(row.source_event_ids, vec![fx.event_a, fx.event_b]);
    assert_eq!(row.rule_version, rule_definition_hash("derive_pq"));
    assert_eq!(row.target_claim, fx.claim_q);
    fx.rt.check_admission(&jid).unwrap();

    let dir = temp_dir();
    store::persist(&fx.rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    std::fs::remove_file(dir.join(store::JUSTIFICATIONS_REL)).unwrap();
    let opened = store::open(&dir).unwrap();
    opened.check_admission(&jid).unwrap();
    assert!(!opened.justification_records().is_empty());
    let _ = std::fs::remove_dir_all(&dir);

    fx.apply_t2_conflict();
    assert_stale(&fx.rt, &jid);

    let t2 = fx.t2.expect("t2");
    let jid2 = fx
        .rt
        .record_justification(
            fx.claim_q,
            vec![fx.claim_p],
            vec![fx.event_b],
            rule_definition_hash("derive_pq"),
            t2,
            VT_INTERIOR,
        )
        .expect("record_justification");
    fx.rt.check_admission(&jid2).unwrap();

    fx.apply_t3_withdraw_b();
    assert_stale(&fx.rt, &jid);
    assert_stale(&fx.rt, &jid2);

    let unknown = fx.rt.check_admission("no-such-justification").unwrap_err();
    assert!(
        matches!(
            unknown,
            RuntimeError::AdmissionDenied {
                reason: "unknown_justification",
                ..
            }
        ),
        "{unknown:?}"
    );
}

#[test]
fn e2e_incremental_matches_reconstruct_after_discarding_leases() {
    let mut fx = build_through_t1();
    let jid = fx.record_t1_justification();
    fx.apply_t2_conflict();
    let t2 = fx.t2.expect("t2");
    let jid2 = fx
        .rt
        .record_justification(
            fx.claim_q,
            vec![fx.claim_p],
            vec![fx.event_b],
            rule_definition_hash("derive_pq"),
            t2,
            VT_INTERIOR,
        )
        .expect("record_justification");
    fx.apply_t3_withdraw_b();
    let t3 = fx.t3.expect("t3");

    let replayed = GraphFold::replay(fx.rt.log().as_slice());
    assert_eq!(replayed.fingerprint(), fx.rt.fold().fingerprint());
    for (tt, vt) in [
        (fx.t1, VT_INTERIOR),
        (t2, VT_INTERIOR),
        (t3, VT_INTERIOR),
        (t2, VT_LEFT),
        (t3, VT_LEFT),
        (t2, VT_RIGHT),
        (t3, VT_RIGHT),
    ] {
        assert_cut_agrees(&fx.rt, &replayed, &fx, tt, vt);
        let eligible = fx.rt.derivation_eligible_at(fx.claim_q, tt, vt);
        assert_eq!(
            eligible,
            replayed.claim_supported_at(fx.claim_q, tt, vt)
                && replayed.claim_supported_at(fx.claim_p, tt, vt)
        );
    }

    let dir = temp_dir();
    store::persist(&fx.rt, &dir).unwrap();
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), fx.rt.fold().fingerprint());
    assert_eq!(
        opened.justification_records(),
        fx.rt.justification_records()
    );
    assert_stale(&opened, &jid);
    assert_stale(&opened, &jid2);
    for (tt, vt) in [(fx.t1, VT_INTERIOR), (t2, VT_INTERIOR), (t3, VT_INTERIOR)] {
        assert_eq!(
            opened.conflict_report_at(fx.claim_p, tt, vt),
            fx.rt.conflict_report_at(fx.claim_p, tt, vt)
        );
        assert_eq!(
            opened.derivation_eligible_at(fx.claim_q, tt, vt),
            fx.rt.derivation_eligible_at(fx.claim_q, tt, vt)
        );
    }
    let _ = std::fs::remove_dir_all(&dir);

    let untyped_before = fx.rt.csr_lease_at(t3, VT_INTERIOR);
    let typed_before = fx.rt.typed_csr_lease_at(t3, VT_INTERIOR);
    let fp_before = fx.rt.fold().fingerprint();
    {
        let _drop_u = fx.rt.csr_lease_at(t3, VT_INTERIOR);
        let _drop_t = fx.rt.typed_csr_lease_at(t3, VT_INTERIOR);
    }
    let untyped_after = fx.rt.csr_lease_at(t3, VT_INTERIOR);
    let typed_after = fx.rt.typed_csr_lease_at(t3, VT_INTERIOR);
    assert_eq!(
        untyped_before.neighbors(fx.a),
        untyped_after.neighbors(fx.a)
    );
    assert_eq!(
        untyped_before.neighbors(fx.b),
        untyped_after.neighbors(fx.b)
    );
    assert_eq!(typed_before.edges_out(fx.a), typed_after.edges_out(fx.a));
    assert_eq!(typed_before.edges_out(fx.b), typed_after.edges_out(fx.b));
    assert_eq!(fp_before, fx.rt.fold().fingerprint());
    fx.rt.replay_check().unwrap();
}
