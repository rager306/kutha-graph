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
