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
