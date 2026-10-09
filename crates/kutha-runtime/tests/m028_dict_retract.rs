//! M028 S01 / Phase 88 — trailing dictionary Retract must survive open
//! (CE review P0 #1: revocation fail-open).

use kutha_common::Op;
use kutha_runtime::{Runtime, RuntimeError, store};

fn persist_dir(label: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m028-dict-retract-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Persist→open with dict Retract as the last event: relation stays revoked.
/// (`persist` prefixes `Op::Define` from the intern picture; in-memory logs from
/// `intern_silent` lack those Defines.)
#[test]
fn trailing_emit_retract_dict_survives_persist_open() {
    let mut rt = Runtime::default();
    let name = "m028-open-rel";
    let allow = rt.emit_allow(name, 2017, None).unwrap();
    let allow_id = allow.receipt.event_ids[0];
    let leased = rt.intern(name);
    let subject = rt.intern("m028-s");
    let object = rt.intern("m028-o");
    rt.emit(Op::Assert {
        subject,
        relation: leased,
        object,
        valid_from: 2017,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_retract_dict(allow_id).unwrap();
    assert!(
        matches!(
            rt.log().as_slice().last().map(|e| &e.op),
            Some(Op::Retract { .. })
        ),
        "retract must be the trailing log op"
    );

    let dir = persist_dir("open");
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::Retract { event_id } if event_id == allow_id)),
        "open must keep trailing emit_retract_dict"
    );
    let rel = opened.intern(name);
    let s = opened.intern("m028-s2");
    let o = opened.intern("m028-o2");
    let err = opened
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "m028-open-rel"),
        "revoked kutha:allows must stay revoked after open: {err:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// Same kill without a User QuantumOutcome in the log (dictionary-only suffix).
#[test]
fn trailing_dict_retract_without_user_qo_survives_open() {
    let mut rt = Runtime::default();
    let name = "m028-solo-rel";
    let allow = rt.emit_allow(name, 2017, None).unwrap();
    let allow_id = allow.receipt.event_ids[0];
    rt.emit_retract_dict(allow_id).unwrap();
    let dir = persist_dir("solo");
    store::persist(&rt, &dir).unwrap();
    let mut opened = store::open(&dir).unwrap();
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::Retract { event_id } if event_id == allow_id)),
        "open must keep trailing dict Retract when no User QO exists"
    );
    let rel = opened.intern(name);
    let s = opened.intern("m028-solo-s");
    let o = opened.intern("m028-solo-o");
    let err = opened
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "m028-solo-rel"),
        "{err:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
