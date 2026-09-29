//! M011 S05: persisted quantum outcomes survive persist→open (ADR-014 / D-O1…D-O4).

use kutha_common::Op;
use kutha_runtime::{store, OutcomeDisposition, Runtime};

#[test]
fn budgets_0_1_2_distinguish_zero_partial_full_after_persist_open() {
    // Zero: Runtime::new(0) — env KUTHA_MAX_CASCADE=0 is filtered to 32 (D-O2 pitfall).
    let zero = run_knows_budget(Runtime::new(0));
    let partial = run_knows_budget(Runtime::new(1));
    let full = run_knows_budget(Runtime::default());

    assert_eq!(
        disposition_after_persist_open(&zero),
        OutcomeDisposition::Zero
    );
    assert_eq!(
        disposition_after_persist_open(&partial),
        OutcomeDisposition::Partial
    );
    assert_eq!(
        disposition_after_persist_open(&full),
        OutcomeDisposition::Full
    );
}

#[test]
fn crash_after_prefix_has_no_terminal_success_until_explicit_resume() {
    let mut rt = Runtime::new(1);
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
        .expect("Ok is not terminal success");
    let quantum_id = rt
        .outcome_records()
        .last()
        .expect("emit records a Partial row")
        .quantum_id
        .clone();
    assert_eq!(
        rt.outcome_records().last().unwrap().disposition,
        OutcomeDisposition::Partial
    );
    assert!(rt.log().len() > 0, "committed prefix on the log");

    let dir = std::env::temp_dir().join(format!("kutha-m011-s05-crash-{}", uuid_like()));
    store::persist(&rt, &dir).unwrap();
    // Crash before terminal success: events remain, outcomes sidecar gone (D-O3).
    std::fs::remove_file(dir.join(store::OUTCOMES_REL)).unwrap();

    let mut opened = store::open(&dir).unwrap();
    assert!(
        opened.outcome_records().is_empty(),
        "missing outcomes file must not invent rows"
    );
    assert!(
        !opened
            .outcome_records()
            .iter()
            .any(|row| row.disposition == OutcomeDisposition::Full),
        "open must not invent terminal Full from the event prefix"
    );

    opened.record_resume(&quantum_id).unwrap();
    store::persist(&opened, &dir).unwrap();
    let mut reopened = store::open(&dir).unwrap();
    assert!(
        reopened.outcome_records().iter().any(|row| {
            row.disposition == OutcomeDisposition::Resume
                && row.resume_of.as_deref() == Some(quantum_id.as_str())
        }),
        "Resume visible only after explicit record_resume"
    );
    assert!(
        reopened.record_resume(&quantum_id).is_err(),
        "duplicate resume_of is fail-closed"
    );

    let _ = std::fs::remove_dir_all(&dir);
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

fn disposition_after_persist_open(rt: &Runtime) -> OutcomeDisposition {
    let dir = std::env::temp_dir().join(format!("kutha-m011-s05-{}", uuid_like()));
    store::persist(rt, &dir).unwrap();
    assert!(
        dir.join(store::OUTCOMES_REL).exists(),
        "quantum_outcomes.jsonl must exist after persist"
    );
    std::fs::remove_file(dir.join("snapshot.json")).unwrap();
    let opened = store::open(&dir).unwrap();
    let rows = opened.outcome_records();
    assert!(
        !rows.is_empty(),
        "discarding snapshot must not erase outcome history"
    );
    let disp = rows.last().unwrap().disposition;
    let _ = std::fs::remove_dir_all(&dir);
    disp
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
