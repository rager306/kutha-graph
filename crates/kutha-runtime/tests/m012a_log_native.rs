//! M012a S01 / LOG-01: quantum outcomes are log records; the outcomes file is a lease.

use kutha_common::Op;
use kutha_runtime::{store, OutcomeDisposition, Runtime};

#[test]
fn discard_outcomes_sidecar_keeps_reconstructible_disposition() {
    let zero = run_knows_budget(Runtime::new(0));
    let partial = run_knows_budget(Runtime::new(1));
    let full = run_knows_budget(Runtime::default());
    let full_fp = full.fold().fingerprint();

    assert_eq!(
        last_disposition_after_discard_outcomes(&zero),
        OutcomeDisposition::Zero
    );
    assert_eq!(
        last_disposition_after_discard_outcomes(&partial),
        OutcomeDisposition::Partial
    );

    let opened = persist_discard_open(&full);
    assert_eq!(
        opened.outcome_records().last().unwrap().disposition,
        OutcomeDisposition::Full
    );
    assert_eq!(
        opened.outcome_records().last().unwrap().events_in_quantum,
        2,
        "knows plus inverse; the outcome Event is not a cascade step"
    );
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::QuantumOutcome { .. })),
        "open log must contain at least one Op::QuantumOutcome"
    );
    assert_eq!(
        opened.fold().fingerprint(),
        full_fp,
        "fold fingerprint after open must match pre-persist"
    );
}

#[test]
fn discard_justifications_sidecar_keeps_admission_and_resume() {
    let (rt, jid) = derive_pq_with_justification();
    let fp = rt.fold().fingerprint();
    let dir = std::env::temp_dir().join(format!("kutha-m012a-s01-j-{}", uuid_like()));
    store::persist(&rt, &dir).unwrap();
    std::fs::remove_file(dir.join(store::JUSTIFICATIONS_REL)).unwrap();
    let opened = store::open(&dir).unwrap();
    opened
        .check_admission(&jid)
        .expect("admission stays Ok after discarding justifications.jsonl");
    assert!(
        opened
            .justification_records()
            .iter()
            .any(|j| j.justification_id == jid),
        "justification_id present after open"
    );
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::JustificationCite { .. })),
        "open log must contain Op::JustificationCite"
    );
    assert_eq!(opened.fold().fingerprint(), fp);
    let _ = std::fs::remove_dir_all(&dir);

    let partial = run_knows_budget(Runtime::new(1));
    let qid = partial
        .outcome_records()
        .last()
        .expect("Partial row")
        .quantum_id
        .clone();
    let pdir = std::env::temp_dir().join(format!("kutha-m012a-s01-r-{}", uuid_like()));
    store::persist(&partial, &pdir).unwrap();
    std::fs::remove_file(pdir.join(store::OUTCOMES_REL)).unwrap();
    let mut opened_p = store::open(&pdir).unwrap();
    assert_eq!(
        opened_p.outcome_records().last().map(|r| r.disposition),
        Some(OutcomeDisposition::Partial)
    );
    opened_p.record_resume(&qid).unwrap();
    store::persist(&opened_p, &pdir).unwrap();
    std::fs::remove_file(pdir.join(store::OUTCOMES_REL)).unwrap();
    let _ = std::fs::remove_file(pdir.join(store::JUSTIFICATIONS_REL));
    let mut opened_r = store::open(&pdir).unwrap();
    assert!(
        opened_r.outcome_records().iter().any(|row| {
            row.disposition == OutcomeDisposition::Resume && row.resume_of.as_deref() == Some(qid.as_str())
        }),
        "Resume reconstructs after discarding both sidecars"
    );
    assert!(
        opened_r.record_resume(&qid).is_err(),
        "duplicate resume_of is fail-closed"
    );
    let _ = std::fs::remove_dir_all(&pdir);
}

#[test]
fn provenance_fingerprint_moves_when_log_native_record_bytes_change() {
    let rt = run_knows_budget(Runtime::default());
    let mut cloned = rt.log().as_slice().to_vec();
    let mut flipped = false;
    for e in &mut cloned {
        if let Op::QuantumOutcome {
            receipt_digest_hex, ..
        } = &mut e.op
        {
            if receipt_digest_hex.is_empty() {
                continue;
            }
            let last = receipt_digest_hex.pop().expect("hex char");
            receipt_digest_hex.push(if last == '0' { '1' } else { '0' });
            flipped = true;
            break;
        }
    }
    assert!(flipped, "clone must contain a QuantumOutcome with receipt bytes");
    let rt2 = Runtime::from_dict_and_events(
        rt.dictionary().strings().to_vec(),
        cloned,
        rt.max_cascade,
    )
    .unwrap();
    assert_eq!(
        rt.fold().fingerprint(),
        rt2.fold().fingerprint(),
        "log-native field flip must not move the fold fingerprint"
    );
    assert_ne!(
        rt.provenance_fingerprint(),
        rt2.provenance_fingerprint(),
        "provenance mix must move when QuantumOutcome bytes change"
    );
    rt.replay_check().unwrap();
    rt2.replay_check().unwrap();
}

fn derive_pq_with_justification() -> (Runtime, String) {
    let mut rt = Runtime::default();
    let a = rt.intern("a");
    let b = rt.intern("b");
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let related = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let first = rt
        .emit(Op::Assert {
            subject: a,
            relation: related,
            object: p,
            valid_from: 2010,
            valid_to: None,
            claim: None,
        })
        .unwrap();
    let event_a = first.receipt.event_ids[0];
    let seq_a = rt.fold().facts().last().unwrap().seq;
    let claim_p = rt.fold().facts().last().unwrap().claim_id;
    let second = rt
        .emit(Op::Assert {
            subject: b,
            relation: related,
            object: p,
            valid_from: 2010,
            valid_to: None,
            claim: Some(claim_p),
        })
        .unwrap();
    let event_b = second.receipt.event_ids[0];
    let seq_b = rt.fold().facts().last().unwrap().seq;
    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            rule_version: "r1".into(),
            subject: q,
            relation: related,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    let t1 = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.claim_id == claim_q)
        .unwrap()
        .ingested_at;
    let jid = rt.record_justification(
        claim_q,
        vec![claim_p],
        vec![event_a, event_b],
        vec![seq_a, seq_b],
        "r1",
        t1,
        2017,
    );
    (rt, jid)
}

fn run_knows_budget(mut rt: Runtime) -> Runtime {
    let a = rt.intern("A");
    let b = rt.intern("B");
    let r = rt.intern("knows");
    let _ = rt
        .emit(Op::Assert {
            subject: a,
            relation: r,
            object: b,
            valid_from: 0,
            valid_to: None,
            claim: None,
        })
        .expect("emit Ok is not completion proof — disposition is the oracle");
    rt
}

fn last_disposition_after_discard_outcomes(rt: &Runtime) -> OutcomeDisposition {
    persist_discard_open(rt)
        .outcome_records()
        .last()
        .expect("reconstruct disposition from the event log")
        .disposition
}

fn persist_discard_open(rt: &Runtime) -> Runtime {
    let dir = std::env::temp_dir().join(format!("kutha-m012a-s01-{}", uuid_like()));
    store::persist(rt, &dir).unwrap();
    std::fs::remove_file(dir.join(store::OUTCOMES_REL)).unwrap();
    let opened = store::open(&dir).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    opened
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
