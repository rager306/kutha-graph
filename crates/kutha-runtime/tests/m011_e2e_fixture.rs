//! M011 S08 candidate fixture; semantic-contract observations at named cuts.

use kutha_common::{EventId, Op, TermId};
use kutha_runtime::{store, Runtime};

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
    seq_a: u64,
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

    let first = rt
        .emit(Op::Assert {
            subject: a,
            relation: related,
            object: p,
            valid_from: VF_WIDE,
            valid_to: None,
            claim: None,
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
        })
        .unwrap();
    let event_b = second.receipt.event_ids[0];
    let seq_b = last_fact_seq(&rt);
    assert_eq!(rt.fold().facts().last().unwrap().claim_id, claim_p);

    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            rule_version: "r1".into(),
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
                fact_seq: self.seq_a,
                object: self.not_p,
                patch_from: PATCH_FROM,
                patch_to: Some(PATCH_TO),
            })
            .unwrap();
        self.t2 = Some(self.rt.log().iter().last().unwrap().ingested_at);
    }

    fn apply_t3_withdraw_b(&mut self) {
        self.rt.emit(Op::Retract { fact_seq: self.seq_b }).unwrap();
        self.t3 = Some(self.rt.log().iter().last().unwrap().ingested_at);
    }

    fn record_t1_justification(&mut self) -> String {
        self.rt.record_justification(
            self.claim_q,
            vec![self.claim_p],
            vec![self.event_a, self.event_b],
            vec![self.seq_a, self.seq_b],
            "r1",
            self.t1,
            VT_INTERIOR,
        )
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
    let t1_report = fx
        .rt
        .conflict_report_at(fx.claim_p, fx.t1, VT_INTERIOR, fx.p, fx.not_p);
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
    let t2_report = fx
        .rt
        .conflict_report_at(fx.claim_p, t2, VT_INTERIOR, fx.p, fx.not_p);
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
    let t3_report = fx
        .rt
        .conflict_report_at(fx.claim_p, t3, VT_INTERIOR, fx.p, fx.not_p);
    assert!(
        t3_report.positive_supports.is_empty(),
        "last positive-P support withdrawn"
    );
    assert!(fx.rt.fold().claim_supported_at(fx.claim_q, t3, VT_INTERIOR));
    assert!(
        fx.rt
            .fold()
            .live_at(t3, VT_LEFT)
            .contains(&(fx.a, fx.related, fx.p))
    );
    assert!(
        fx.rt
            .fold()
            .live_at(t3, VT_RIGHT)
            .contains(&(fx.a, fx.related, fx.p))
    );
    fx.rt.replay_check().unwrap();
}
